//! Size-limit and TTL behaviour.
//!
//! These reach private methods, so they only work from inside the crate.

use super::common::{temp_store, temp_store_with_ttl};
use crate::now_micros;

#[test]
fn test_trim_oldest() {
    let store = temp_store();
    for i in 0..20 {
        store.push_text(&format!("entry {}", i)).unwrap();
    }
    store.trim_oldest(10).unwrap();
    let results = store.get(20).unwrap();
    assert_eq!(results.len(), 10);
    assert!(matches!(&results[0].content, crate::ClipContent::Text(t) if t == "entry 19"));
}

#[test]
fn test_ttl_removes_old_entries() {
    let store = temp_store_with_ttl(60);
    let old_ts = now_micros() - 120 * 1_000_000;
    store.push_text_at("old entry", old_ts).unwrap();
    store.check_expire().unwrap();
    assert!(store.get(10).unwrap().is_empty());
}

#[test]
fn test_ttl_keeps_recent_entries() {
    let store = temp_store_with_ttl(60);
    let recent_ts = now_micros() - 10 * 1_000_000;
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
    assert!(matches!(&results[0].content, crate::ClipContent::Text(t) if t == "recent"));
}

#[test]
fn test_ttl_none_does_not_expire() {
    let store = temp_store();
    let old_ts = now_micros() - 999 * 1_000_000;
    store.push_text_at("old entry", old_ts).unwrap();
    store.check_expire().unwrap();
    assert_eq!(store.get(10).unwrap().len(), 1);
}
