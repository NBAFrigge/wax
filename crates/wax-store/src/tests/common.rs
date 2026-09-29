//! Shared fixtures for the store tests.

use crate::{ClipStore, Limits, now_micros};
use std::sync::atomic::{AtomicU64, Ordering};

/// A store backed by a throwaway database under `/tmp`, with no limits set.
pub fn temp_store() -> ClipStore {
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

/// A store with only a TTL set, so size limits never interfere with TTL tests.
pub fn temp_store_with_ttl(ttl_secs: u64) -> ClipStore {
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
