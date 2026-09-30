//! Shared fixtures for the store tests.
//!
//! Each store gets its own `TempDir` holding both the database and the images
//! directory. The previous fixtures used fixed paths under `/tmp`, which meant
//! every test run left a database behind (427 files, 23 MB after a few dozen
//! runs) and wrote images into the real user data directory.
//!
//! The `TempDir` handles are parked in thread-local storage so they outlive the
//! store they belong to and are cleaned up when the test thread exits. That
//! keeps `temp_store()` returning a plain `ClipStore`, so the existing tests
//! need no changes.

use crate::{ClipStore, Limits};
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use tempfile::TempDir;

thread_local! {
    static KEEP_ALIVE: RefCell<Vec<TempDir>> = const { RefCell::new(Vec::new()) };
}

fn new_dir(prefix: &str) -> PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().to_path_buf();
    KEEP_ALIVE.with(|keep| keep.borrow_mut().push(dir));
    // The counter only exists so failures are easier to correlate with a run.
    let _ = (prefix, N.fetch_add(1, Ordering::Relaxed));
    path
}

/// A store with no limits set.
pub fn temp_store() -> ClipStore {
    temp_store_with(Limits::default())
}

/// A store with the given limits.
pub fn temp_store_with(limits: Limits) -> ClipStore {
    ClipStore::open(new_dir("store").join("wax.redb"), limits).expect("open store")
}

/// A store with only a TTL set, so size limits never interfere with TTL tests.
pub fn temp_store_with_ttl(ttl_secs: u64) -> ClipStore {
    ClipStore::open(
        new_dir("ttl").join("wax.redb"),
        Limits {
            max_db_bytes: u64::MAX,
            max_images_bytes: u64::MAX,
            ttl_secs: Some(ttl_secs),
        },
    )
    .expect("open store")
}

/// A store plus the path of its images directory, for tests that push images.
pub fn temp_store_with_images(limits: Limits) -> (ClipStore, PathBuf) {
    let dir = new_dir("images");
    let images_dir = dir.join("images");
    let store = ClipStore::open_at(&dir.join("wax.redb"), &images_dir, limits).expect("open store");
    (store, images_dir)
}

/// Bytes of filler needed to push a database past a single redb page.
pub const FILLER: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

/// A clip string of roughly `len` bytes, distinct per index.
pub fn clip_of(index: usize, len: usize) -> String {
    let mut out = format!("clip {index} ");
    while out.len() < len {
        out.push_str(FILLER);
    }
    out.truncate(len);
    out
}
