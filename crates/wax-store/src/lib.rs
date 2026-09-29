use redb::{Database, ReadableDatabase, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use xxhash_rust::xxh3::xxh3_64;

const CLIPS: TableDefinition<u64, &[u8]> = TableDefinition::new("clips");
const HISTORY: TableDefinition<u64, u64> = TableDefinition::new("history");
const PINNED: TableDefinition<u64, u64> = TableDefinition::new("pinned");
const HASH_TS: TableDefinition<u64, u64> = TableDefinition::new("hash_ts");

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
    pub max_db_bytes: u64,
    pub max_images_bytes: u64,
    pub ttl_secs: Option<u64>,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_db_bytes: u64::MAX,
            max_images_bytes: u64::MAX,
            ttl_secs: None,
        }
    }
}

pub struct ClipStore {
    db: Database,
    db_path: PathBuf,
    images_dir: PathBuf,
    images_dir_dim: AtomicU64,
    limits: Limits,
}

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
    read_cache_from(&cache_path(), n)
}

pub fn read_cache_from(path: &Path, n: usize) -> Option<Vec<String>> {
    let bytes = std::fs::read(path).ok()?;
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

impl ClipStore {
    pub fn open(path: impl AsRef<Path>, limits: Limits) -> Result<Self, redb::Error> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let mut db = Database::builder()
            .set_cache_size(256 * 1024)
            .create(&path)?;

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
                if hash_ts.len()? == 0 && history.len()? > 0 {
                    for entry in history.iter()? {
                        let (k, v) = entry?;
                        hash_ts.insert(v.value(), k.value())?;
                    }
                }
            }
            txn.commit()?;
        }

        let metadata = std::fs::metadata(&path)?;
        if metadata.len() > limits.max_db_bytes {
            db.compact()?;
        }

        let images_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("wax/images");

        let images_dir_dim = dir_size(images_dir.as_path());

        let store = Self {
            db,
            db_path: path,
            images_dir,
            images_dir_dim: AtomicU64::new(images_dir_dim),
            limits,
        };
        store.enforce_limits();
        store.rebuild_cache();
        Ok(store)
    }

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

    pub fn push_image(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let hash = xxh3_64(data);
        std::fs::create_dir_all(&self.images_dir)?;
        let path = self.images_dir.join(format!("{}.png", hash));
        if !path.exists() {
            std::fs::write(&path, data)?;
            self.images_dir_dim
                .fetch_add(data.len() as u64, Ordering::Relaxed);
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

    fn push(&self, clip: Clip) -> Result<bool, Box<dyn std::error::Error>> {
        let hash_key = match &clip.content {
            ClipContent::Text(t) => xxh3_64(t.as_bytes()),
            ClipContent::Image(p) => xxh3_64(p.as_bytes()),
        };

        let txn = self.db.begin_write()?;
        let changed;
        {
            let mut clips = txn.open_table(CLIPS)?;
            let mut history = txn.open_table(HISTORY)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;

            let last_hash = history.last()?.map(|e| e.1.value());
            if last_hash == Some(hash_key) {
                changed = false;
            } else if clips.get(hash_key)?.is_none() {
                let bytes = bincode::serialize(&clip)?;
                clips.insert(hash_key, bytes.as_slice())?;
                let ts = unique_micros();
                history.insert(ts, hash_key)?;
                hash_ts.insert(hash_key, ts)?;
                changed = true;
            } else {
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
        let changed = self.delete_by_hash(xxh3_64(path.as_bytes()), Some(path))?;
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
            self.images_dir_dim
                .fetch_sub(std::fs::metadata(path)?.len(), Ordering::Relaxed);
            std::fs::remove_file(path).ok();
        }
        Ok(removed)
    }

    fn trim_oldest(&self, n: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut image_paths: Vec<String> = Vec::new();

        let txn = self.db.begin_write()?;
        {
            let mut history = txn.open_table(HISTORY)?;
            let mut clips = txn.open_table(CLIPS)?;
            let pinned = txn.open_table(PINNED)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;

            let to_remove: Vec<(u64, u64)> = history
                .iter()?
                .take(n)
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    Some((k.value(), v.value()))
                })
                .collect();

            for (ts, _) in &to_remove {
                history.remove(ts)?;
            }

            let still_referenced: HashSet<u64> = history
                .iter()?
                .filter_map(|e| e.ok().map(|(_, v)| v.value()))
                .collect();

            let pinned_hashes: HashSet<u64> = pinned
                .iter()?
                .filter_map(|e| e.ok().map(|(k, _)| k.value()))
                .collect();

            for (_, hash) in &to_remove {
                if !still_referenced.contains(hash) {
                    hash_ts.remove(hash)?;
                    if !pinned_hashes.contains(hash) {
                        if let Ok(Some(data)) = clips.get(hash)
                            && let Ok(clip) = bincode::deserialize::<Clip>(data.value())
                            && let ClipContent::Image(path) = clip.content
                        {
                            image_paths.push(path);
                        }
                        clips.remove(hash)?;
                    }
                }
            }
        }
        txn.commit()?;

        for path in &image_paths {
            self.images_dir_dim
                .fetch_sub(std::fs::metadata(path)?.len(), Ordering::Relaxed);
            std::fs::remove_file(path).ok();
        }

        Ok(())
    }

    fn check_expire(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ttl_secs = match self.limits.ttl_secs {
            Some(t) => t,
            None => return Ok(()),
        };
        let cutoff = now_micros().saturating_sub(ttl_secs * 1_000_000);

        let mut image_paths: Vec<String> = Vec::new();

        let txn = self.db.begin_write()?;
        {
            let mut history = txn.open_table(HISTORY)?;
            let mut clips = txn.open_table(CLIPS)?;
            let pinned = txn.open_table(PINNED)?;
            let mut hash_ts = txn.open_table(HASH_TS)?;

            let to_remove: Vec<(u64, u64)> = history
                .range(..cutoff)?
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    Some((k.value(), v.value()))
                })
                .collect();

            for (ts, _) in &to_remove {
                history.remove(ts)?;
            }

            let still_referenced: HashSet<u64> = history
                .iter()?
                .filter_map(|e| e.ok().map(|(_, v)| v.value()))
                .collect();

            let pinned_hashes: HashSet<u64> = pinned
                .iter()?
                .filter_map(|e| e.ok().map(|(k, _)| k.value()))
                .collect();

            for (_, hash) in &to_remove {
                if !still_referenced.contains(hash) {
                    hash_ts.remove(hash)?;
                    if !pinned_hashes.contains(hash) {
                        if let Ok(Some(data)) = clips.get(hash)
                            && let Ok(clip) = bincode::deserialize::<Clip>(data.value())
                            && let ClipContent::Image(path) = clip.content
                        {
                            image_paths.push(path);
                        }
                        clips.remove(hash)?;
                    }
                }
            }
        }
        txn.commit()?;

        for path in &image_paths {
            self.images_dir_dim
                .fetch_sub(std::fs::metadata(path)?.len(), Ordering::Relaxed);
            std::fs::remove_file(path).ok();
        }

        Ok(())
    }

    fn enforce_limits(&self) {
        let db_size = std::fs::metadata(&self.db_path)
            .map(|m| m.len())
            .unwrap_or(0);
        if db_size > self.limits.max_db_bytes {
            self.trim_oldest(50).ok();
        }

        if self.images_dir_dim.load(Ordering::Relaxed) > self.limits.max_images_bytes {
            self.trim_oldest(50).ok();
        }

        if self.limits.ttl_secs.is_some() {
            self.check_expire().ok();
        }
    }

    fn rebuild_cache(&self) {
        let clips = match self.get(1000) {
            Ok(c) => c,
            Err(_) => return,
        };
        let mut content = Vec::new();
        for c in &clips {
            match &c.content {
                ClipContent::Text(t) => content.extend_from_slice(t.as_bytes()),
                ClipContent::Image(p) => {
                    content.extend_from_slice(b"[img] ");
                    content.extend_from_slice(p.as_bytes());
                }
            }
            content.push(b'\0');
        }
        let tmp = cache_path().with_extension("tmp");
        if std::fs::write(&tmp, &content).is_ok() {
            std::fs::rename(&tmp, cache_path()).ok();
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
            self.images_dir_dim
                .fetch_sub(std::fs::metadata(path)?.len(), Ordering::Relaxed);
            std::fs::remove_file(path).ok();
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
}

fn dir_size(path: &Path) -> u64 {
    std::fs::read_dir(path)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| e.metadata().ok())
                .map(|m| m.len())
                .sum()
        })
        .unwrap_or(0)
}

fn clip_hash(text: &str) -> u64 {
    let key = text.strip_prefix("[img] ").unwrap_or(text);
    xxh3_64(key.as_bytes())
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
