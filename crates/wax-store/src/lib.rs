// wax-store/src/lib.rs

use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use std::time::{SystemTime, UNIX_EPOCH};
use xxhash_rust::xxh3::xxh3_64;

const CLIPS: TableDefinition<u64, &[u8]> = TableDefinition::new("clips");
const HISTORY: TableDefinition<u64, u64> = TableDefinition::new("history");

pub struct ClipStore {
    db: Database,
}

impl ClipStore {
    pub fn open(path: &str) -> Result<Self, redb::Error> {
        let db = Database::create(path)?;

        let txn = db.begin_write()?;
        txn.open_table(CLIPS)?;
        txn.open_table(HISTORY)?;
        txn.commit()?;

        Ok(Self { db })
    }

    pub fn push(&self, string: &str) -> Result<(), redb::Error> {
        let content = bincode::serialize(string).unwrap();
        let hash_key = xxh3_64(&content);

        let txn = self.db.begin_write()?;
        {
            let mut clips = txn.open_table(CLIPS)?;
            clips.insert(hash_key, content.as_slice())?;

            let mut history = txn.open_table(HISTORY)?;
            let last_hash = history.last()?.map(|e| e.1.value());

            if last_hash != Some(hash_key) {
                let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as u64;
                history.insert(ts, hash_key)?;
            }
        }
        txn.commit()?;
        Ok(())
    }

    pub fn get(&self, last_n: usize) -> Result<Vec<String>, redb::Error> {
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
                let value: String = bincode::deserialize(data.value()).unwrap();
                Some(value)
            })
            .collect();

        Ok(records)
    }

    pub fn delete(&self, string: &str) -> Result<(), redb::Error> {
        let content = bincode::serialize(string).unwrap();
        let hash_key = xxh3_64(&content);

        let txn = self.db.begin_write()?;
        {
            txn.open_table(CLIPS)?.remove(hash_key)?;

            let mut history = txn.open_table(HISTORY)?;
            let to_remove: Vec<u64> = history
                .iter()?
                .filter_map(|e| {
                    let (k, v) = e.ok()?;
                    if v.value() == hash_key {
                        Some(k.value())
                    } else {
                        None
                    }
                })
                .collect();

            for ts in to_remove {
                history.remove(ts)?;
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> ClipStore {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        ClipStore::open(&format!("/tmp/wax_test_{}.redb", ts)).unwrap()
    }

    #[test]
    fn test_push_and_get() {
        let store = temp_store();
        store.push("hello world").unwrap();

        let results = store.get(10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "hello world");
    }

    #[test]
    fn test_reverse_order() {
        let store = temp_store();
        store.push("first").unwrap();
        store.push("second").unwrap();
        store.push("third").unwrap();

        let results = store.get(10).unwrap();
        assert_eq!(results[0], "third");
        assert_eq!(results[1], "second");
        assert_eq!(results[2], "first");
    }

    #[test]
    fn test_contiguous_dedup() {
        let store = temp_store();
        store.push("duplicate").unwrap();
        store.push("duplicate").unwrap();
        store.push("duplicate").unwrap();

        let results = store.get(10).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_non_contiguous_dedup() {
        let store = temp_store();
        store.push("A").unwrap();
        store.push("B").unwrap();
        store.push("A").unwrap(); // non-contiguous → should appear again

        let results = store.get(10).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], "A");
        assert_eq!(results[1], "B");
        assert_eq!(results[2], "A");
    }

    #[test]
    fn test_get_last_n() {
        let store = temp_store();
        for i in 0..20 {
            store.push(&format!("clip {}", i)).unwrap();
        }

        let results = store.get(5).unwrap();
        assert_eq!(results.len(), 5);
        assert_eq!(results[0], "clip 19");
    }

    #[test]
    fn test_delete() {
        let store = temp_store();
        store.push("keep this").unwrap();
        store.push("delete this").unwrap();
        store.delete("delete this").unwrap();

        let results = store.get(10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "keep this");
    }

    #[test]
    fn test_clear() {
        let store = temp_store();
        store.push("one").unwrap();
        store.push("two").unwrap();
        store.clear().unwrap();

        assert!(store.get(10).unwrap().is_empty());
    }

    #[test]
    fn test_empty_db() {
        let store = temp_store();
        assert!(store.get(10).unwrap().is_empty());
    }

    #[test]
    fn test_empty_string() {
        let store = temp_store();
        store.push("").unwrap();

        let results = store.get(10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "");
    }

    #[test]
    fn test_unicode() {
        let store = temp_store();
        store.push("こんにちは 🦀 àèìòù").unwrap();

        let results = store.get(10).unwrap();
        assert_eq!(results[0], "こんにちは 🦀 àèìòù");
    }
}
