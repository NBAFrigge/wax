//! Pinning versus trimming and expiry.
//!
//! Pinning means "keep this", so a pinned entry must keep its place in history
//! when the history is trimmed and when the TTL fires. Previously the history
//! row was removed even though the clip was kept, so a pinned entry silently
//! vanished from the picker — and once unpinned, nothing could ever reclaim it.

use super::common::{temp_store, temp_store_with_images, temp_store_with_ttl};
use crate::{ClipContent, Limits, MICROS_PER_SEC};

/// The text of every clip currently in history, newest first.
fn texts(store: &crate::ClipStore) -> Vec<String> {
    store
        .get(10_000)
        .unwrap()
        .iter()
        .filter_map(|c| match &c.content {
            ClipContent::Text(t) => Some(t.clone()),
            ClipContent::Image(_) => None,
        })
        .collect()
}

fn push_range(store: &crate::ClipStore, n: usize) {
    for i in 0..n {
        store.push_text(&format!("entry {i}")).unwrap();
    }
}

fn pin_range(store: &crate::ClipStore, n: usize) {
    for i in 0..n {
        store.pin_text(&format!("entry {i}")).unwrap();
    }
}

#[test]
fn trim_keeps_a_pinned_entry_in_history() {
    let store = temp_store();
    push_range(&store, 20);
    store.pin_text("entry 0").unwrap(); // the oldest entry

    store.trim_oldest(10).unwrap();

    let got = texts(&store);
    assert!(got.contains(&"entry 0".to_string()), "pin should survive");
    assert!(
        !got.contains(&"entry 1".to_string()),
        "the next-oldest unpinned entry should be removed instead"
    );
    assert_eq!(got.len(), 10, "20 pushed, 10 removed, pin is among the 10");
}

#[test]
fn trim_skips_pinned_entries_and_keeps_going() {
    // The point of filtering rather than stopping: a batch of 10 must still
    // remove 10 entries when the first 5 are pinned.
    let store = temp_store();
    push_range(&store, 20);
    pin_range(&store, 5);

    store.trim_oldest(10).unwrap();

    let got = texts(&store);
    assert_eq!(got.len(), 10, "10 pinned plus 5 newest unpinned");
    for i in 0..5 {
        assert!(
            got.contains(&format!("entry {i}")),
            "pin {i} should survive"
        );
    }
    for i in 5..15 {
        assert!(
            !got.contains(&format!("entry {i}")),
            "entry {i} should be gone"
        );
    }
    assert!(got.contains(&"entry 19".to_string()));
}

#[test]
fn trim_removes_nothing_when_every_entry_is_pinned() {
    let store = temp_store();
    push_range(&store, 5);
    pin_range(&store, 5);

    store.trim_oldest(50).unwrap();
    assert_eq!(
        texts(&store).len(),
        5,
        "all pinned, so nothing is removable"
    );
}

#[test]
fn a_pinned_entry_is_still_there_after_being_unpinned() {
    // The old bug's lasting damage: the history row was gone but the clip
    // remained, so unpinning stranded an entry nothing referenced.
    let store = temp_store();
    push_range(&store, 20);
    store.pin_text("entry 0").unwrap();

    store.trim_oldest(10).unwrap();
    store.unpin_text("entry 0").unwrap();

    assert!(
        texts(&store).contains(&"entry 0".to_string()),
        "unpinning must not strand the entry"
    );
}

#[test]
fn unpinning_leaves_the_entry_eligible_for_a_later_trim() {
    // ...and once unpinned it is just an ordinary entry again.
    let store = temp_store();
    push_range(&store, 20);
    store.pin_text("entry 0").unwrap();
    store.trim_oldest(10).unwrap();
    store.unpin_text("entry 0").unwrap();

    store.trim_oldest(10).unwrap();
    assert!(
        !texts(&store).contains(&"entry 0".to_string()),
        "unpinned entries are ordinary and should be trimmable"
    );
}

#[test]
fn expiry_leaves_pinned_entries_alone() {
    let store = temp_store_with_ttl(60);
    let old = crate::now_micros() - 120 * MICROS_PER_SEC;
    store.push_text_at("old pinned", old).unwrap();
    store.push_text_at("old unpinned", old - 1).unwrap(); // distinct timestamp
    store.pin_text("old pinned").unwrap();

    store.check_expire().unwrap();

    let got = texts(&store);
    assert!(
        got.contains(&"old pinned".to_string()),
        "pin should survive TTL"
    );
    assert!(
        !got.contains(&"old unpinned".to_string()),
        "unpinned old entry should expire"
    );
}

#[test]
fn expiry_removes_nothing_when_every_expired_entry_is_pinned() {
    let store = temp_store_with_ttl(60);
    let old = crate::now_micros() - 120 * MICROS_PER_SEC;
    for i in 0..5 {
        store
            .push_text_at(&format!("old {i}"), old - i as u64)
            .unwrap();
        store.pin_text(&format!("old {i}")).unwrap();
    }

    store.check_expire().unwrap();
    assert_eq!(texts(&store).len(), 5);
}

#[test]
fn a_pinned_image_survives_a_trim_with_its_file_intact() {
    let (store, images) = temp_store_with_images(Limits::default());
    let mut png = Vec::with_capacity(24);
    png.extend_from_slice(b"\x89PNG\r\n\x1a\n");
    png.extend_from_slice(&13u32.to_be_bytes());
    png.extend_from_slice(b"IHDR");
    png.extend_from_slice(&64u32.to_be_bytes());
    png.extend_from_slice(&64u32.to_be_bytes());
    store.push_image(&png).unwrap();

    let path = match &store.get(1).unwrap()[0].content {
        ClipContent::Image(p) => p.clone(),
        ClipContent::Text(_) => unreachable!(),
    };
    store.pin_text(&format!("[img] {path}")).unwrap();

    for i in 0..20 {
        store.push_text(&format!("entry {i}")).unwrap();
    }
    store.trim_oldest(50).unwrap();

    let kept = store.get(10_000).unwrap();
    assert!(
        kept.iter()
            .any(|c| matches!(&c.content, ClipContent::Image(p) if *p == path)),
        "pinned image should survive"
    );
    assert!(
        images.join(&path).exists(),
        "image file should not be deleted"
    );
}
