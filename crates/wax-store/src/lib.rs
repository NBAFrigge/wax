use redb::{Database, ReadableDatabase, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use xxhash_rust::xxh3::xxh3_64;

/// hash -> clip (bincode-encoded `Clip`)
const CLIPS: TableDefinition<u64, &[u8]> = TableDefinition::new("clips");
/// ts -> hash
const HISTORY: TableDefinition<u64, u64> = TableDefinition::new("history");
/// hash -> ts
const PINNED: TableDefinition<u64, u64> = TableDefinition::new("pinned");
/// hash -> ts
const HASH_TS: TableDefinition<u64, u64> = TableDefinition::new("hash_ts");

const DELETE_PERCENTAGE: u64 = 10;

/// How many clips the picker cache holds. This is a hard ceiling on what
/// `wax list` can return, independent of `max_entries`
const CACHE_ENTRY_LIMIT: usize = 1000;

/// redb's in-memory page cache. Kept small deliberately: the daemon is a
/// long-lived background process that only reads a few pages per copy.
const DB_CACHE_BYTES: usize = 256 * 1024;

const MICROS_PER_SEC: u64 = 1_000_000;

const IMAGE_PREFIX: &str = "[img]";

/// images directory
fn default_images_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("wax/images")
}

// db directory
pub fn default_db_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("wax/db.redb")
}

pub fn cache_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("wax/history.cache")
}

pub fn read_cache(n: usize) -> Option<Vec<String>> {
    let bytes = std::fs::read(cache_path()).ok()?;
    if bytes.is_empty() {
        return Some(vec![]);
    }
    Some(
        bytes
            .split(|&b| b == b'\0')
            .filter(|s| !s.is_empty())
            .take(n)
            .filter_map(|s| std::str::from_utf8(s).ok().map(|s| s.to_owned()))
            .collect(),
    )
}

#[derive(Serialize, Deserialize)]
pub enum ClipContent {
    Text(String),
    Image(String),
}

#[derive(Serialize, Deserialize)]
pub struct Clip {
    pub content: ClipContent,
}

#[derive(Clone)]
pub struct Limits {
    /// How many entries to keep in history. `u64::MAX` means unlimited.
    pub max_entries: u64,
    pub ttl_secs: Option<u64>,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_entries: u64::MAX,
            ttl_secs: None,
        }
    }
}

pub struct ClipStore {
    db: Database,
    images_dir: PathBuf,
    limits: Limits,
}

impl ClipStore {
    pub fn open(path: impl AsRef<Path>, limits: Limits) -> Result<Self, redb::Error> {
        let images_dir = default_images_dir();
        Self::open_at(path.as_ref(), &images_dir, limits)
    }

    /// Open a store with an explicit images directory.
    fn open_at(path: &Path, images_dir: &Path, limits: Limits) -> Result<Self, redb::Error> {
        if let Some(parent) = path.parent()
            && let Err(e) = std::fs::create_dir_all(parent)
            && e.kind() != std::io::ErrorKind::AlreadyExists
        {
            eprintln!("wax-store: could not create {}: {e}", parent.display());
        }
        // create/open database
        let db = Database::builder()
            .set_cache_size(DB_CACHE_BYTES)
            .create(path)?;

        // create tables
        {
            let txn = db.begin_write()?;
            txn.open_table(CLIPS)?;
            txn.open_table(HISTORY)?;
            txn.open_table(PINNED)?;
            txn.open_table(HASH_TS)?;
            txn.commit()?;
        }

        {
            let txn = db.begin_write()?;
            {
                let history = txn.open_table(HISTORY)?;
                let mut hash_ts = txn.open_table(HASH_TS)?;

                // migration from timestamp - hash to hash - timestamp table
                if hash_ts.len()? == 0 && history.len()? > 0 {
                    for entry in history.iter()? {
                        let (k, v) = entry?;
                        hash_ts.insert(v.value(), k.value())?;
                    }
                }
            }
            txn.commit()?;
        }

        let store = Self {
            db,
            images_dir: images_dir.to_path_buf(),
            limits,
        };

        store.enforce_limits();
        store.rebuild_cache();
        Ok(store)
    }

    // wrap text into a clip and call push
    pub fn push_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let changed = self.push(Clip {
            content: ClipContent::Text(text.to_string()),
        })?;

        if changed {
            self.enforce_limits();
            self.rebuild_cache();
        }
        Ok(())
    }

    // save the image, wrap the path int oa clip and call push
    pub fn push_image(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let hash = xxh3_64(data);

        std::fs::create_dir_all(&self.images_dir)?;

        let path = self.images_dir.join(format!("{}.png", hash));
        if !path.exists() {
            std::fs::write(&path, data)?;
        }
        let path_str = path.to_string_lossy().into_owned();

        let changed = self.push(Clip {
            content: ClipContent::Image(path_str),
        })?;

        if changed {
            self.enforce_limits();
            self.rebuild_cache();
        }
        Ok(())
    }

    // save the clip into the db
    fn push(&self, clip: Clip) -> Result<bool, Box<dyn std::error::Error>> {
        let hash_key = match &clip.content {
            ClipContent::Text(t) => xxh3_64(t.as_bytes()),
            ClipContent::Image(p) => xxh3_64(format!("{IMAGE_PREFIX} {p}").as_bytes()),
        };

        let txn = self.db.begin_write()?;
        let changed;
        {
            let mut clips = txn.open_table(CLIPS)?;
            let mut history = txn.open_table(HISTORY)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;

            let last_hash = history.last()?.map(|e| e.1.value());

            if last_hash == Some(hash_key) {
                // duplicate skipping
                changed = false;
            } else if clips.get(hash_key)?.is_none() {
                // clip never seen before
                let bytes = bincode::serialize(&clip)?;
                clips.insert(hash_key, bytes.as_slice())?;
                let ts = unique_micros();
                history.insert(ts, hash_key)?;
                hash_ts.insert(hash_key, ts)?;

                changed = true;
            } else {
                // seen before ->  update the ts
                let old_ts = if let Some(e) = hash_ts.get(hash_key)? {
                    Some(e.value())
                } else {
                    history.iter()?.find_map(|e| {
                        let (k, v) = e.ok()?;
                        (v.value() == hash_key).then_some(k.value())
                    })
                };
                if let Some(old_ts) = old_ts {
                    history.remove(old_ts)?;
                }
                let ts = unique_micros();
                history.insert(ts, hash_key)?;
                hash_ts.insert(hash_key, ts)?;
                changed = true;
            }
        }
        txn.commit()?;
        Ok(changed)
    }

    // get last n clips
    pub fn get(&self, last_n: usize) -> Result<Vec<Clip>, redb::Error> {
        let txn = self.db.begin_read()?;
        let clips = txn.open_table(CLIPS)?;
        let history = txn.open_table(HISTORY)?;
        let pinned = txn.open_table(PINNED)?;

        let pinned_hashes: HashSet<u64> = pinned
            .iter()?
            .filter_map(|e| e.ok().map(|(k, _)| k.value()))
            .collect();

        let mut pinned_with_ts: Vec<(u64, Clip)> = pinned
            .iter()?
            .filter_map(|e| {
                let (k, v) = e.ok()?;
                let hash = k.value();
                let ts = v.value();
                let data = clips.get(hash).ok()??;
                let clip = bincode::deserialize::<Clip>(data.value()).ok()?;
                Some((ts, clip))
            })
            .collect();

        pinned_with_ts.sort_by(|a, b| b.0.cmp(&a.0));
        let pinned_clips: Vec<Clip> = pinned_with_ts.into_iter().map(|(_, c)| c).collect();

        let normal_clips: Vec<Clip> = history
            .iter()?
            .rev()
            .filter_map(|e| {
                let (_, v) = e.ok()?;
                let hash = v.value();
                if pinned_hashes.contains(&hash) {
                    return None;
                }
                let data = clips.get(hash).ok()??;
                bincode::deserialize::<Clip>(data.value()).ok()
            })
            .take(last_n.saturating_sub(pinned_clips.len()))
            .collect();

        Ok(pinned_clips.into_iter().chain(normal_clips).collect())
    }

    pub fn delete_text(&self, text: &str) -> Result<(), redb::Error> {
        let changed = self.delete_by_hash(xxh3_64(text.as_bytes()), None)?;
        if changed {
            self.rebuild_cache();
        }
        Ok(())
    }

    pub fn delete_image(&self, path: &str) -> Result<(), redb::Error> {
        let changed = self.delete_by_hash(
            xxh3_64(format!("{IMAGE_PREFIX} {path}").as_bytes()),
            Some(path),
        )?;
        if changed {
            self.rebuild_cache();
        }
        Ok(())
    }

    fn delete_by_hash(&self, hash: u64, file_to_remove: Option<&str>) -> Result<bool, redb::Error> {
        let txn = self.db.begin_write()?;
        let removed;
        {
            let mut clips = txn.open_table(CLIPS)?;
            let mut history = txn.open_table(HISTORY)?;
            let mut pinned = txn.open_table(PINNED)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;

            removed = clips.remove(hash)?.is_some();
            pinned.remove(hash)?;

            let ts_opt = hash_ts.get(hash)?.map(|e| e.value());
            hash_ts.remove(hash)?;

            if let Some(ts) = ts_opt {
                history.remove(ts)?;
            } else {
                let ts_to_remove: Vec<u64> = history
                    .iter()?
                    .filter_map(|e| {
                        let (k, v) = e.ok()?;
                        (v.value() == hash).then_some(k.value())
                    })
                    .collect();
                for ts in ts_to_remove {
                    history.remove(ts)?;
                }
            }
        }
        txn.commit()?;
        if let Some(path) = file_to_remove {
            remove_image_file(path);
        }
        Ok(removed)
    }

    fn trim_oldest(&self, n: usize) -> Result<u64, Box<dyn std::error::Error>> {
        let mut image_paths: Vec<String> = Vec::new();
        let txn = self.db.begin_write()?;
        let mut rows_removed = 0;
        {
            let mut history = txn.open_table(HISTORY)?;
            let mut clips = txn.open_table(CLIPS)?;
            let pinned = txn.open_table(PINNED)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;

            let pinned_hashes: HashSet<u64> = pinned
                .iter()?
                .filter_map(|e| e.ok().map(|(k, _)| k.value()))
                .collect();

            // Pinned entries are skipped rather than counted, so a heavily-pinned
            // history still trims
            let to_remove: Vec<(u64, u64)> = history
                .iter()?
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    let (ts, hash) = (k.value(), v.value());
                    (!pinned_hashes.contains(&hash)).then_some((ts, hash))
                })
                .take(n)
                .collect();

            for (ts, _) in &to_remove {
                history.remove(ts)?;
                rows_removed += 1;
            }

            for (_, hash) in &to_remove {
                hash_ts.remove(hash)?;
                if let Ok(Some(data)) = clips.get(hash)
                    && let Ok(clip) = bincode::deserialize::<Clip>(data.value())
                    && let ClipContent::Image(path) = clip.content
                {
                    image_paths.push(path);
                }
                clips.remove(hash)?;
            }
        }
        txn.commit()?;

        for path in &image_paths {
            remove_image_file(path);
        }

        Ok(rows_removed)
    }

    fn check_expire(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ttl_secs = match self.limits.ttl_secs {
            Some(t) => t,
            None => return Ok(()),
        };
        let cutoff = now_micros().saturating_sub(ttl_secs * MICROS_PER_SEC);

        let mut image_paths: Vec<String> = Vec::new();

        let txn = self.db.begin_write()?;
        {
            let mut history = txn.open_table(HISTORY)?;
            let mut clips = txn.open_table(CLIPS)?;
            let pinned = txn.open_table(PINNED)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;

            let pinned_hashes: HashSet<u64> = pinned
                .iter()?
                .filter_map(|e| e.ok().map(|(k, _)| k.value()))
                .collect();

            // Pinned entries are exempt from the TTL, for the same reason they
            // are exempt from trimming
            let to_remove: Vec<(u64, u64)> = history
                .range(..cutoff)?
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    let (ts, hash) = (k.value(), v.value());
                    (!pinned_hashes.contains(&hash)).then_some((ts, hash))
                })
                .collect();

            for (ts, _) in &to_remove {
                history.remove(ts)?;
            }

            for (_, hash) in &to_remove {
                hash_ts.remove(hash)?;
                if let Ok(Some(data)) = clips.get(hash)
                    && let Ok(clip) = bincode::deserialize::<Clip>(data.value())
                    && let ClipContent::Image(path) = clip.content
                {
                    image_paths.push(path);
                }
                clips.remove(hash)?;
            }
        }
        txn.commit()?;

        for path in &image_paths {
            remove_image_file(path);
        }

        Ok(())
    }

    fn history_len(&self) -> Result<u64, redb::Error> {
        let txn = self.db.begin_read()?;
        Ok(txn.open_table(HISTORY)?.len()?)
    }

    fn enforce_limits(&self) {
        match self.history_len() {
            Ok(count) if count > self.limits.max_entries => {
                let trim_bactch =
                    (count - (self.limits.max_entries * DELETE_PERCENTAGE / 100)) as usize;
                if let Err(e) = self.trim_oldest(trim_bactch) {
                    eprintln!("wax-store: trim failed: {e}");
                }
            }
            Ok(_) => {}
            Err(e) => eprintln!("wax-store: cannot read history length: {e}"),
        }

        if self.limits.ttl_secs.is_some()
            && let Err(e) = self.check_expire()
        {
            eprintln!("wax-store: expiry failed: {e}");
        }
    }

    fn rebuild_cache(&self) {
        let clips = match self.get(CACHE_ENTRY_LIMIT) {
            Ok(c) => c,
            Err(_) => return,
        };
        let mut content = Vec::new();
        for c in &clips {
            match &c.content {
                ClipContent::Text(t) => content.extend_from_slice(t.as_bytes()),
                ClipContent::Image(p) => {
                    content.extend_from_slice(IMAGE_PREFIX.as_bytes());
                    content.extend_from_slice(p.as_bytes());
                }
            }
            content.push(b'\0');
        }
        let tmp = cache_path().with_extension("tmp");
        match std::fs::write(&tmp, &content) {
            Ok(()) => {
                if let Err(e) = std::fs::rename(&tmp, cache_path()) {
                    eprintln!("wax-store: could not replace the cache file: {e}");
                }
            }
            Err(e) => eprintln!("wax-store: could not write the cache file: {e}"),
        }
    }

    pub fn clear(&self) -> Result<(), redb::Error> {
        let mut image_paths: Vec<String> = Vec::new();

        let txn = self.db.begin_write()?;
        {
            let pinned_hashes: HashSet<u64> = {
                let pinned = txn.open_table(PINNED)?;
                pinned
                    .iter()?
                    .filter_map(|e| e.ok().map(|(k, _)| k.value()))
                    .collect()
            };

            let mut history = txn.open_table(HISTORY)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;

            let ts_to_remove: Vec<(u64, u64)> = history
                .iter()?
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    (!pinned_hashes.contains(&v.value())).then_some((k.value(), v.value()))
                })
                .collect();

            for (ts, hash) in &ts_to_remove {
                history.remove(ts)?;
                hash_ts.remove(hash)?;
            }

            let mut clips = txn.open_table(CLIPS)?;
            let clips_to_remove: Vec<u64> = clips
                .iter()?
                .filter_map(|e| {
                    let (k, _) = e.ok()?;
                    (!pinned_hashes.contains(&k.value())).then_some(k.value())
                })
                .collect();

            for hash in &clips_to_remove {
                if let Ok(Some(data)) = clips.get(hash)
                    && let Ok(clip) = bincode::deserialize::<Clip>(data.value())
                    && let ClipContent::Image(path) = clip.content
                {
                    image_paths.push(path);
                }
                clips.remove(hash)?;
            }
        }
        txn.commit()?;

        for path in &image_paths {
            remove_image_file(path);
        }

        self.rebuild_cache();
        Ok(())
    }

    pub fn get_pinned(&self) -> Result<HashSet<u64>, redb::Error> {
        let txn = self.db.begin_read()?;
        Ok(txn
            .open_table(PINNED)?
            .iter()?
            .filter_map(|e| e.ok().map(|(k, _)| k.value()))
            .collect())
    }

    pub fn get_pinned_clips(&self) -> Result<Vec<Clip>, redb::Error> {
        let txn = self.db.begin_read()?;
        let clips = txn.open_table(CLIPS)?;
        let pinned = txn.open_table(PINNED)?;

        let mut entries: Vec<(u64, Clip)> = pinned
            .iter()?
            .filter_map(|e| {
                let (k, v) = e.ok()?;
                let hash = k.value();
                let ts = v.value();
                let data = clips.get(hash).ok()??;
                let clip = bincode::deserialize::<Clip>(data.value()).ok()?;
                Some((ts, clip))
            })
            .collect();
        entries.sort_by(|a, b| b.0.cmp(&a.0));
        Ok(entries.into_iter().map(|(_, c)| c).collect())
    }

    pub fn pin_text(&self, text: &str) -> Result<(), redb::Error> {
        let hash = clip_hash(text);
        let txn = self.db.begin_write()?;
        let clips = txn.open_table(CLIPS)?;
        let exists = clips.get(hash)?.is_some();
        drop(clips);
        if exists {
            txn.open_table(PINNED)?.insert(hash, unique_micros())?;
        }
        txn.commit()?;
        if exists {
            self.rebuild_cache();
        }
        Ok(())
    }

    pub fn unpin_text(&self, text: &str) -> Result<(), redb::Error> {
        let hash = clip_hash(text);
        let txn = self.db.begin_write()?;
        let removed = txn.open_table(PINNED)?.remove(hash)?.is_some();
        txn.commit()?;
        if removed {
            self.rebuild_cache();
        }
        Ok(())
    }

    #[cfg(test)]
    fn push_text_at(
        &self,
        text: &str,
        timestamp_micros: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hash_key = xxh3_64(text.as_bytes());
        let txn = self.db.begin_write()?;
        {
            let mut clips = txn.open_table(CLIPS)?;
            let mut history = txn.open_table(HISTORY)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;
            if clips.get(hash_key)?.is_none() {
                let bytes = bincode::serialize(&Clip {
                    content: ClipContent::Text(text.to_string()),
                })?;
                clips.insert(hash_key, bytes.as_slice())?;
            }
            history.insert(timestamp_micros, hash_key)?;
            hash_ts.insert(hash_key, timestamp_micros)?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Test-only: store an image with an explicit history timestamp, so TTL
    /// expiry of image entries can be exercised. `push_text_at` covers text
    /// only, and expiry is the only path that removes image files on a timer.
    #[cfg(test)]
    fn push_image_at(
        &self,
        data: &[u8],
        timestamp_micros: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hash = xxh3_64(data);
        std::fs::create_dir_all(&self.images_dir)?;
        let path = self.images_dir.join(format!("{}.png", hash));
        if !path.exists() {
            std::fs::write(&path, data)?;
        }
        let path_str = path.to_string_lossy().into_owned();
        let hash_key = xxh3_64(format!("{IMAGE_PREFIX} {path_str}").as_bytes());

        let txn = self.db.begin_write()?;
        {
            let mut clips = txn.open_table(CLIPS)?;
            let mut history = txn.open_table(HISTORY)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;
            if clips.get(hash_key)?.is_none() {
                let bytes = bincode::serialize(&Clip {
                    content: ClipContent::Image(path_str),
                })?;
                clips.insert(hash_key, bytes.as_slice())?;
            }
            history.insert(timestamp_micros, hash_key)?;
            hash_ts.insert(hash_key, timestamp_micros)?;
        }
        txn.commit()?;
        Ok(())
    }
}

fn clip_hash(text: &str) -> u64 {
    xxh3_64(text.as_bytes())
}

/// Delete a stored image file, logging anything other than "already gone".
///
/// A missing file is normal rather than a problem: the clip row was removed by
/// an earlier trim, or the file was cleaned up out of band. Only a real
/// failure is worth reporting, since it means the bytes are still on disk with
/// no database row pointing at them.
fn remove_image_file(path: &str) {
    match std::fs::remove_file(path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => eprintln!("wax-store: could not delete image {path}: {e}"),
    }
}

fn now_micros() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

fn unique_micros() -> u64 {
    static LAST: AtomicU64 = AtomicU64::new(0);
    let now = now_micros();
    loop {
        let last = LAST.load(Ordering::Relaxed);
        let next = if now > last { now } else { last + 1 };
        if LAST
            .compare_exchange_weak(last, next, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            return next;
        }
    }
}

#[cfg(test)]
mod tests;
