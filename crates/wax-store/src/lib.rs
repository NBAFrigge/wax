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
    pub timestamp: u64,
}

pub struct ClipStore {
    db: Database,
    images_dir: PathBuf,
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

impl ClipStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, redb::Error> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let db = Database::create(path)?;

        let txn = db.begin_write()?;
        txn.open_table(CLIPS)?;
        txn.open_table(HISTORY)?;
        txn.commit()?;

        let images_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("wax/images");

        let store = Self { db, images_dir };
        store.rebuild_cache();
        Ok(store)
    }

    pub fn push_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.push(Clip {
            content: ClipContent::Text(text.to_string()),
            timestamp: now_micros(),
        })?;
        self.rebuild_cache();
        Ok(())
    }

    pub fn push_image(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let hash = xxh3_64(data);
        std::fs::create_dir_all(&self.images_dir)?;
        let path = self.images_dir.join(format!("{}.png", hash));
        std::fs::write(&path, data)?;

        self.push(Clip {
            content: ClipContent::Image(path.to_string_lossy().to_string()),
            timestamp: now_micros(),
        })?;
        self.rebuild_cache();
        Ok(())
    }

    fn push(&self, clip: Clip) -> Result<(), Box<dyn std::error::Error>> {
        let content_bytes = match &clip.content {
            ClipContent::Text(t) => t.as_bytes().to_vec(),
            ClipContent::Image(p) => p.as_bytes().to_vec(),
        };

        let hash_key = xxh3_64(&content_bytes);
        let bytes = bincode::serialize(&clip)?;
        let txn = self.db.begin_write()?;

        {
            let mut clips = txn.open_table(CLIPS)?;
            clips.insert(hash_key, bytes.as_slice())?;

            let mut history = txn.open_table(HISTORY)?;
            let last_hash = history.last()?.map(|e| e.1.value());
            if last_hash != Some(hash_key) {
                history.insert(now_micros(), hash_key)?;
            }
        }
        txn.commit()?;
        Ok(())
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

            for hash in to_remove {
                clips.remove(hash)?;
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
            std::fs::remove_file(path).ok();
        }
        Ok(())
    }

    fn rebuild_cache(&self) {
        let clips = match self.get(1000) {
            Ok(c) => c,
            Err(_) => return,
        };
        let content: Vec<u8> = clips
            .iter()
            .flat_map(|c| {
                let entry = match &c.content {
                    ClipContent::Text(t) => t.as_bytes().to_vec(),
                    ClipContent::Image(p) => format!("[img] {}", p).into_bytes(),
                };
                entry.into_iter().chain(std::iter::once(b'\0'))
            })
            .collect();
        let tmp = cache_path().with_extension("tmp");
        if std::fs::write(&tmp, &content).is_ok() {
            std::fs::rename(&tmp, cache_path()).ok();
        }
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
        ClipStore::open(format!(
            "/tmp/wax_test_{}_{}.redb",
            now_micros(),
            N.fetch_add(1, Ordering::Relaxed)
        ))
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
}
