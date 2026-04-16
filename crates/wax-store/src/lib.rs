use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use xxhash_rust::xxh3::xxh3_64;

const CLIPS: TableDefinition<u64, &[u8]> = TableDefinition::new("clips");
const HISTORY: TableDefinition<u64, u64> = TableDefinition::new("history");

#[derive(Serialize, Deserialize)]
pub enum ClipContent {
    Text(String),
    Image(String),
}

#[derive(Serialize, Deserialize)]
pub struct Clip {
    pub content: ClipContent,
}

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
        let db = Database::create(&path)?;

        let txn = db.begin_write()?;
        txn.open_table(CLIPS)?;
        txn.open_table(HISTORY)?;
        txn.commit()?;

        let images_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("wax/images");

        let store = Self {
            db,
            db_path: path,
            images_dir,
            limits,
        };
        store.rebuild_cache();
        Ok(store)
    }

    pub fn push_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let inserted = self.push(Clip {
            content: ClipContent::Text(text.to_string()),
        })?;
        if inserted {
            self.prepend_cache(text.as_bytes());
            self.enforce_limits();
        }
        Ok(())
    }

    pub fn push_image(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let hash = xxh3_64(data);
        std::fs::create_dir_all(&self.images_dir)?;
        let path = self.images_dir.join(format!("{}.png", hash));
        std::fs::write(&path, data)?;

        let path_str = path.to_string_lossy().into_owned();
        let inserted = self.push(Clip {
            content: ClipContent::Image(path_str.clone()),
        })?;
        if inserted {
            self.prepend_cache(format!("[img] {}", path_str).as_bytes());
            self.enforce_limits();
        }
        Ok(())
    }

    fn push(&self, clip: Clip) -> Result<bool, Box<dyn std::error::Error>> {
        let hash_key = match &clip.content {
            ClipContent::Text(t) => xxh3_64(t.as_bytes()),
            ClipContent::Image(p) => xxh3_64(p.as_bytes()),
        };

        let txn = self.db.begin_write()?;
        let inserted;
        {
            let mut history = txn.open_table(HISTORY)?;
            let last_hash = history.last()?.map(|e| e.1.value());
            inserted = last_hash != Some(hash_key);

            if inserted {
                let mut clips = txn.open_table(CLIPS)?;
                if clips.get(hash_key)?.is_none() {
                    let bytes = bincode::serialize(&clip)?;
                    clips.insert(hash_key, bytes.as_slice())?;
                }
                history.insert(now_micros(), hash_key)?;
            }
        }
        txn.commit()?;
        Ok(inserted)
    }

    pub fn get(&self, last_n: usize) -> Result<Vec<Clip>, redb::Error> {
        let txn = self.db.begin_read()?;
        let clips = txn.open_table(CLIPS)?;
        let history = txn.open_table(HISTORY)?;

        Ok(history
            .iter()?
            .rev()
            .take(last_n)
            .filter_map(|e| {
                let hash = e.ok()?.1.value();
                let data = clips.get(hash).ok()??;
                bincode::deserialize::<Clip>(data.value()).ok()
            })
            .collect())
    }

    pub fn delete_text(&self, text: &str) -> Result<(), redb::Error> {
        self.delete_matching(
            |clip| matches!(&clip.content, ClipContent::Text(t) if t == text),
            None,
        )?;
        self.rebuild_cache();
        Ok(())
    }

    pub fn delete_image(&self, path: &str) -> Result<(), redb::Error> {
        let path = path.to_string();
        self.delete_matching(
            |clip| matches!(&clip.content, ClipContent::Image(p) if p == &path),
            Some(&path),
        )?;
        self.rebuild_cache();
        Ok(())
    }

    fn delete_matching(
        &self,
        predicate: impl Fn(&Clip) -> bool,
        file_to_remove: Option<&str>,
    ) -> Result<(), redb::Error> {
        let txn = self.db.begin_write()?;
        {
            let clips_ro = txn.open_table(CLIPS)?;
            let to_remove: Vec<u64> = clips_ro
                .iter()?
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    let clip: Clip = bincode::deserialize(v.value()).ok()?;
                    predicate(&clip).then_some(k.value())
                })
                .collect();
            drop(clips_ro);

            let mut clips = txn.open_table(CLIPS)?;
            let mut history = txn.open_table(HISTORY)?;

            for hash in &to_remove {
                clips.remove(hash)?;
            }
            let ts_to_remove: Vec<u64> = history
                .iter()?
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    to_remove.contains(&v.value()).then_some(k.value())
                })
                .collect();
            for ts in ts_to_remove {
                history.remove(ts)?;
            }
        }
        txn.commit()?;
        if let Some(path) = file_to_remove {
            std::fs::remove_file(path).ok();
        }
        Ok(())
    }

    fn trim_oldest(&self, n: usize) -> Result<(), Box<dyn std::error::Error>> {
        let txn = self.db.begin_write()?;
        {
            let mut history = txn.open_table(HISTORY)?;
            let mut clips = txn.open_table(CLIPS)?;

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

            let still_referenced: std::collections::HashSet<u64> = history
                .iter()?
                .filter_map(|e| e.ok().map(|(_, v)| v.value()))
                .collect();

            for (_, hash) in &to_remove {
                if !still_referenced.contains(hash) {
                    if let Ok(Some(data)) = clips.get(hash) {
                        if let Ok(clip) = bincode::deserialize::<Clip>(data.value()) {
                            if let ClipContent::Image(path) = clip.content {
                                std::fs::remove_file(&path).ok();
                            }
                        }
                    }
                    clips.remove(hash)?;
                }
            }
        }
        txn.commit()?;
        self.rebuild_cache();
        Ok(())
    }

    fn check_expire(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ttl_secs = match self.limits.ttl_secs {
            Some(t) => t,
            None => return Ok(()),
        };
        let cutoff = now_micros().saturating_sub(ttl_secs * 1_000_000);

        let txn = self.db.begin_write()?;
        {
            let mut history = txn.open_table(HISTORY)?;
            let mut clips = txn.open_table(CLIPS)?;

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

            let still_referenced: std::collections::HashSet<u64> = history
                .iter()?
                .filter_map(|e| e.ok().map(|(_, v)| v.value()))
                .collect();

            for (_, hash) in &to_remove {
                if !still_referenced.contains(hash) {
                    if let Ok(Some(data)) = clips.get(hash) {
                        if let Ok(clip) = bincode::deserialize::<Clip>(data.value()) {
                            if let ClipContent::Image(path) = clip.content {
                                std::fs::remove_file(&path).ok();
                            }
                        }
                    }
                    clips.remove(hash)?;
                }
            }
        }
        txn.commit()?;
        self.rebuild_cache();
        Ok(())
    }

    fn enforce_limits(&self) {
        let db_size = std::fs::metadata(&self.db_path)
            .map(|m| m.len())
            .unwrap_or(0);
        if db_size > self.limits.max_db_bytes {
            self.trim_oldest(50).ok();
        }

        let images_size = dir_size(&self.images_dir);
        if images_size > self.limits.max_images_bytes {
            self.trim_oldest(50).ok();
        }

        if self.limits.ttl_secs != None {
            self.check_expire().ok();
        }
    }

    fn prepend_cache(&self, entry: &[u8]) {
        let existing = std::fs::read(cache_path()).unwrap_or_default();
        let mut content = Vec::with_capacity(entry.len() + 1 + existing.len());
        content.extend_from_slice(entry);
        content.push(b'\0');
        content.extend_from_slice(&existing);
        let tmp = cache_path().with_extension("tmp");
        if std::fs::write(&tmp, &content).is_ok() {
            std::fs::rename(&tmp, cache_path()).ok();
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
            if clips.get(hash_key)?.is_none() {
                let bytes = bincode::serialize(&Clip {
                    content: ClipContent::Text(text.to_string()),
                })?;
                clips.insert(hash_key, bytes.as_slice())?;
            }
            history.insert(timestamp_micros, hash_key)?;
        }
        txn.commit()?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), redb::Error> {
        let txn = self.db.begin_write()?;
        {
            let mut clips = txn.open_table(CLIPS)?;
            let keys: Vec<u64> = clips
                .iter()?
                .filter_map(|e| e.ok().map(|(k, _)| k.value()))
                .collect();
            for k in keys {
                clips.remove(k)?;
            }

            let mut history = txn.open_table(HISTORY)?;
            let keys: Vec<u64> = history
                .iter()?
                .filter_map(|e| e.ok().map(|(k, _)| k.value()))
                .collect();
            for k in keys {
                history.remove(k)?;
            }
        }
        txn.commit()?;
        std::fs::remove_dir_all(&self.images_dir).ok();
        self.rebuild_cache();
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

fn now_micros() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> ClipStore {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        ClipStore::open(
            format!(
                "/tmp/wax_test_{}_{}.redb",
                now_micros(),
                N.fetch_add(1, Ordering::Relaxed)
            ),
            Limits::default(),
        )
        .unwrap()
    }

    #[test]
    fn test_push_and_get_text() {
        let store = temp_store();
        store.push_text("hello world").unwrap();
        let results = store.get(10).unwrap();
        assert_eq!(results.len(), 1);
        assert!(matches!(&results[0].content, ClipContent::Text(t) if t == "hello world"));
    }

    #[test]
    fn test_reverse_order() {
        let store = temp_store();
        store.push_text("first").unwrap();
        store.push_text("second").unwrap();
        store.push_text("third").unwrap();
        let results = store.get(10).unwrap();
        let texts: Vec<&str> = results
            .iter()
            .map(|c| {
                if let ClipContent::Text(t) = &c.content {
                    t.as_str()
                } else {
                    ""
                }
            })
            .collect();
        assert_eq!(texts, vec!["third", "second", "first"]);
    }

    #[test]
    fn test_contiguous_dedup() {
        let store = temp_store();
        store.push_text("duplicate").unwrap();
        store.push_text("duplicate").unwrap();
        store.push_text("duplicate").unwrap();
        assert_eq!(store.get(10).unwrap().len(), 1);
    }

    #[test]
    fn test_non_contiguous_dedup() {
        let store = temp_store();
        store.push_text("A").unwrap();
        store.push_text("B").unwrap();
        store.push_text("A").unwrap();
        assert_eq!(store.get(10).unwrap().len(), 3);
    }

    #[test]
    fn test_get_last_n() {
        let store = temp_store();
        for i in 0..20 {
            store.push_text(&format!("clip {}", i)).unwrap();
        }
        let results = store.get(5).unwrap();
        assert_eq!(results.len(), 5);
        assert!(matches!(&results[0].content, ClipContent::Text(t) if t == "clip 19"));
    }

    #[test]
    fn test_clear() {
        let store = temp_store();
        store.push_text("one").unwrap();
        store.push_text("two").unwrap();
        store.clear().unwrap();
        assert!(store.get(10).unwrap().is_empty());
    }

    #[test]
    fn test_empty_db() {
        assert!(temp_store().get(10).unwrap().is_empty());
    }

    #[test]
    fn test_unicode() {
        let store = temp_store();
        store.push_text("こんにちは 🦀 àèìòù").unwrap();
        assert!(
            matches!(&store.get(10).unwrap()[0].content, ClipContent::Text(t) if t == "こんにちは 🦀 àèìòù")
        );
    }

    #[test]
    fn test_delete_text() {
        let store = temp_store();
        store.push_text("keep").unwrap();
        store.push_text("remove me").unwrap();
        store.push_text("keep").unwrap();
        store.delete_text("remove me").unwrap();
        assert!(
            store
                .get(10)
                .unwrap()
                .iter()
                .all(|c| !matches!(&c.content, ClipContent::Text(t) if t == "remove me"))
        );
    }

    #[test]
    fn test_trim_oldest() {
        let store = temp_store();
        for i in 0..20 {
            store.push_text(&format!("entry {}", i)).unwrap();
        }
        store.trim_oldest(10).unwrap();
        let results = store.get(20).unwrap();
        assert_eq!(results.len(), 10);
        assert!(matches!(&results[0].content, ClipContent::Text(t) if t == "entry 19"));
    }

    #[test]
    fn test_enforce_limits() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let store = ClipStore::open(
            format!(
                "/tmp/wax_limits_{}_{}.redb",
                now_micros(),
                N.fetch_add(1, Ordering::Relaxed)
            ),
            Limits {
                max_db_bytes: 1,
                max_images_bytes: u64::MAX,
                ttl_secs: None,
            },
        )
        .unwrap();
        for i in 0..60 {
            store.push_text(&format!("entry {}", i)).unwrap();
        }
        assert!(store.get(100).unwrap().len() < 60);
    }

    fn temp_store_with_ttl(ttl_secs: u64) -> ClipStore {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        ClipStore::open(
            format!(
                "/tmp/wax_ttl_{}_{}.redb",
                now_micros(),
                N.fetch_add(1, Ordering::Relaxed)
            ),
            Limits {
                max_db_bytes: u64::MAX,
                max_images_bytes: u64::MAX,
                ttl_secs: Some(ttl_secs),
            },
        )
        .unwrap()
    }

    #[test]
    fn test_ttl_removes_old_entries() {
        let store = temp_store_with_ttl(60);
        let old_ts = now_micros() - 120 * 1_000_000; // 2 minuti fa
        store.push_text_at("old entry", old_ts).unwrap();
        store.check_expire().unwrap();
        assert!(store.get(10).unwrap().is_empty());
    }

    #[test]
    fn test_ttl_keeps_recent_entries() {
        let store = temp_store_with_ttl(60);
        let recent_ts = now_micros() - 10 * 1_000_000; // 10 secondi fa
        store.push_text_at("recent entry", recent_ts).unwrap();
        store.check_expire().unwrap();
        assert_eq!(store.get(10).unwrap().len(), 1);
    }

    #[test]
    fn test_ttl_mixed_old_and_new() {
        let store = temp_store_with_ttl(60);
        let old_ts = now_micros() - 120 * 1_000_000;
        let recent_ts = now_micros() - 10 * 1_000_000;
        store.push_text_at("old", old_ts).unwrap();
        store.push_text_at("recent", recent_ts).unwrap();
        store.check_expire().unwrap();
        let results = store.get(10).unwrap();
        assert_eq!(results.len(), 1);
        assert!(matches!(&results[0].content, ClipContent::Text(t) if t == "recent"));
    }

    #[test]
    fn test_ttl_none_does_not_expire() {
        let store = temp_store(); // ttl_secs: None
        let old_ts = now_micros() - 999 * 1_000_000;
        store.push_text_at("old entry", old_ts).unwrap();
        store.check_expire().unwrap();
        assert_eq!(store.get(10).unwrap().len(), 1);
    }
}
