//! Core store behaviour: push, retrieval, deduplication, deletion, clearing.

use super::common::{temp_store, temp_store_with};
use crate::{ClipContent, Limits};

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
    assert_eq!(store.get(10).unwrap().len(), 2);
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
fn test_enforce_limits() {
    let store = temp_store_with(Limits {
        max_db_bytes: 1,
        max_images_bytes: u64::MAX,
        ttl_secs: None,
    });
    for i in 0..60 {
        store.push_text(&format!("entry {}", i)).unwrap();
    }
    assert!(store.get(100).unwrap().len() < 60);
}
