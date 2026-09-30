//! Store behaviour once the database spans many redb pages.
//!
//! The single-page tests can hide bugs that only appear when btree pages split
//! and merges, so these fill the store with enough data to cross page
//! boundaries repeatedly. redb's default page size is 4096 bytes.

use super::common::{clip_of, temp_store_with_images, temp_store_with_ttl};
use crate::{ClipContent, ClipStore, Limits, now_micros};

const PAGE_BYTES: u64 = 4096;
const CLIP_SIZE: usize = 2048;

/// Fill a store with `count` clips of roughly `CLIP_SIZE` bytes each.
fn fill(store: &ClipStore, count: usize) {
    for i in 0..count {
        store.push_text(&clip_of(i, CLIP_SIZE)).unwrap();
    }
}

fn texts(store: &ClipStore, n: usize) -> Vec<String> {
    store
        .get(n)
        .unwrap()
        .iter()
        .filter_map(|c| match &c.content {
            ClipContent::Text(t) => Some(t.clone()),
            ClipContent::Image(_) => None,
        })
        .collect()
}

/// Total bytes of clip content the store actually holds.
fn live_bytes(store: &ClipStore) -> u64 {
    store
        .get(10_000)
        .unwrap()
        .iter()
        .map(|c| match &c.content {
            ClipContent::Text(t) => t.len() as u64,
            ClipContent::Image(p) => p.len() as u64,
        })
        .sum()
}

#[test]
fn a_filled_store_really_does_span_many_pages() {
    // Guards the premise of every other test in this file: if the fixture ever
    // stops crossing page boundaries, the rest silently stop testing anything.
    //
    // Asserted against live data volume, not the file length. The file length is
    // not a reliable proxy for either pages used or data stored (TODO #1), and
    // redb can shrink the file on its own during a run.
    let store = temp_store_with_images(Limits::default()).0;
    assert_eq!(live_bytes(&store), 0, "starts empty");

    fill(&store, 60);

    let live = live_bytes(&store);
    assert_eq!(live, 60 * CLIP_SIZE as u64, "all 60 clips present");
    let pages = live / PAGE_BYTES;
    assert!(
        pages >= 24,
        "expected 24+ pages of user data, got {} pages",
        pages
    );
}

#[test]
fn get_returns_every_clip_newest_first_across_page_boundaries() {
    let store = temp_store_with_images(Limits::default()).0;
    fill(&store, 60);

    let got = texts(&store, 60);
    assert_eq!(got.len(), 60);
    for (i, text) in got.iter().enumerate() {
        assert_eq!(text, &clip_of(59 - i, CLIP_SIZE), "wrong position {}", i);
    }
}

#[test]
fn get_is_bounded_by_the_requested_count_across_page_boundaries() {
    let store = temp_store_with_images(Limits::default()).0;
    fill(&store, 60);

    assert_eq!(texts(&store, 5).len(), 5);
    assert_eq!(texts(&store, 1).len(), 1);
    assert_eq!(texts(&store, 10_000).len(), 60);
}

#[test]
fn trim_removes_the_oldest_across_page_boundaries() {
    let store = temp_store_with_images(Limits::default()).0;
    fill(&store, 60);

    store.trim_oldest(20).unwrap();

    let got = texts(&store, 100);
    assert_eq!(got.len(), 40, "should drop exactly 20");
    // Oldest 20 are gone.
    assert!(!got.contains(&clip_of(0, CLIP_SIZE)));
    assert!(!got.contains(&clip_of(19, CLIP_SIZE)));
    // Newest survive, still newest-first.
    assert_eq!(got[0], clip_of(59, CLIP_SIZE));
    assert_eq!(got[39], clip_of(20, CLIP_SIZE));
}

#[test]
fn trimming_more_than_exists_is_harmless() {
    let store = temp_store_with_images(Limits::default()).0;
    fill(&store, 10);

    store.trim_oldest(1_000).unwrap();
    assert!(store.get(100).unwrap().is_empty());
}

#[test]
fn delete_reaches_entries_sitting_on_later_pages() {
    let store = temp_store_with_images(Limits::default()).0;
    fill(&store, 60);

    // Delete from the start, the middle and the end.
    for index in [0usize, 30, 59] {
        store.delete_text(&clip_of(index, CLIP_SIZE)).unwrap();
    }

    let got = texts(&store, 100);
    assert_eq!(got.len(), 57);
    for index in [0usize, 30, 59] {
        assert!(
            !got.contains(&clip_of(index, CLIP_SIZE)),
            "{} survived",
            index
        );
    }
}

#[test]
fn expiry_removes_old_entries_across_page_boundaries() {
    let store = temp_store_with_ttl(60);
    let now = now_micros();

    // Half the entries are older than the TTL, half are recent. Inserted
    // directly so the whole set lands in one database before expiring.
    //
    // Each entry needs its own timestamp: HISTORY is keyed by timestamp, so two
    // clips sharing one would overwrite each other. Production avoids this with
    // unique_micros(); this test helper takes timestamps verbatim.
    for i in 0..60 {
        let ts = if i < 30 {
            now - 120 * 1_000_000 + i as u64
        } else {
            now - 10 * 1_000_000 + (i - 30) as u64
        };
        store.push_text_at(&clip_of(i, CLIP_SIZE), ts).unwrap();
    }
    assert_eq!(store.get(100).unwrap().len(), 60);

    store.check_expire().unwrap();

    let remaining = texts(&store, 100);
    assert_eq!(remaining.len(), 30, "the 30 recent entries should remain");
    assert_eq!(remaining[0], clip_of(59, CLIP_SIZE));
    assert!(!remaining.contains(&clip_of(0, CLIP_SIZE)));
}

#[test]
fn clear_empties_a_multi_page_store() {
    let store = temp_store_with_images(Limits::default()).0;
    fill(&store, 60);

    store.clear().unwrap();
    assert!(store.get(100).unwrap().is_empty());
    assert_eq!(live_bytes(&store), 0);
}
