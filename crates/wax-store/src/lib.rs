use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
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
    images_dir: std::path::PathBuf,
}

impl ClipStore {
    pub fn open(path: &str) -> Result<Self, redb::Error> {
        let db = Database::create(path)?;

        let txn = db.begin_write()?;
        txn.open_table(CLIPS)?;
        txn.open_table(HISTORY)?;
        txn.commit()?;

        let images_dir = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
            .join("wax/images");

        Ok(Self { db, images_dir })
    }

    pub fn push_text(&self, text: &str) -> Result<(), redb::Error> {
        let clip = Clip {
            content: ClipContent::Text(text.to_string()),
            timestamp: now_micros(),
        };
        self.push(clip)
    }

    pub fn push_image(&self, data: &[u8]) -> Result<(), redb::Error> {
        let hash = xxh3_64(data);
        std::fs::create_dir_all(&self.images_dir).ok();
        let path = self.images_dir.join(format!("{}.png", hash));
        std::fs::write(&path, data).ok();

        let clip = Clip {
            content: ClipContent::Image(path.to_string_lossy().to_string()),
            timestamp: now_micros(),
        };
        self.push(clip)
    }

    fn push(&self, clip: Clip) -> Result<(), redb::Error> {
        let content_bytes = match &clip.content {
            ClipContent::Text(t) => t.as_bytes().to_vec(),
            ClipContent::Image(p) => p.as_bytes().to_vec(),
        };

        let hash_key = xxh3_64(&content_bytes);
        let bytes = bincode::serialize(&clip).unwrap();
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

        let records = history
            .iter()?
            .rev()
            .take(last_n)
            .filter_map(|e| {
                let hash = e.ok()?.1.value();
                let data = clips.get(hash).ok()??;
                bincode::deserialize::<Clip>(data.value()).ok()
            })
            .collect();

        Ok(records)
    }

    pub fn delete_text(&self, text: &str) -> Result<(), redb::Error> {
        let txn = self.db.begin_write()?;
        {
            let clips = txn.open_table(CLIPS)?;
            let to_remove: Vec<u64> = clips
                .iter()?
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    let clip: Clip = bincode::deserialize(v.value()).ok()?;
                    if let ClipContent::Text(t) = clip.content {
                        if t == text {
                            return Some(k.value());
                        }
                    }
                    None
                })
                .collect();

            drop(clips);
            let mut clips = txn.open_table(CLIPS)?;
            let mut history = txn.open_table(HISTORY)?;

            for hash in to_remove {
                clips.remove(hash)?;
                let ts_to_remove: Vec<u64> = history
                    .iter()?
                    .filter_map(|e| {
                        let (k, v) = e.ok()?;
                        if v.value() == hash {
                            Some(k.value())
                        } else {
                            None
                        }
                    })
                    .collect();
                for ts in ts_to_remove {
                    history.remove(ts)?;
                }
            }
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
        Ok(())
    }
}

fn now_micros() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> ClipStore {
        let ts = now_micros();
        ClipStore::open(&format!("/tmp/wax_test_{}.redb", ts)).unwrap()
    }

    #[test]
    fn test_push_and_get_text() {
        let store = temp_store();
        store.push_text("hello world").unwrap();

        let results = store.get(10).unwrap();
        assert_eq!(results.len(), 1);
        if let ClipContent::Text(t) = &results[0].content {
            assert_eq!(t, "hello world");
        } else {
            panic!("expected text");
        }
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
        if let ClipContent::Text(t) = &results[0].content {
            assert_eq!(t, "clip 19");
        }
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
        let store = temp_store();
        assert!(store.get(10).unwrap().is_empty());
    }

    #[test]
    fn test_unicode() {
        let store = temp_store();
        store.push_text("こんにちは 🦀 àèìòù").unwrap();

        let results = store.get(10).unwrap();
        if let ClipContent::Text(t) = &results[0].content {
            assert_eq!(t, "こんにちは 🦀 àèìòù");
        }
    }
}

