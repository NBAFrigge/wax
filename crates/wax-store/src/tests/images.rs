//! Coverage for the `images_dir_dim` counter.
//!
//! The counter replaces a per-copy `readdir` in the image-limit check, so it
//! has to stay in step with what is actually on disk. TODO #5: only
//! `trim_oldest` and `delete_by_hash` decrement it today.

use super::common::temp_store_with_images;
use crate::{ClipContent, Limits};

/// A tiny but valid PNG: `png_dimensions` only reads the 24-byte header.
fn png(seed: u8) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(24);
    bytes.extend_from_slice(b"\x89PNG\r\n\x1a\n");
    bytes.extend_from_slice(&13u32.to_be_bytes());
    bytes.extend_from_slice(b"IHDR");
    bytes.extend_from_slice(&(100u32 + seed as u32).to_be_bytes());
    bytes.extend_from_slice(&(200u32 + seed as u32).to_be_bytes());
    bytes
}

fn dim(store: &crate::ClipStore) -> u64 {
    store
        .images_dir_dim
        .load(std::sync::atomic::Ordering::Relaxed)
}

#[test]
fn a_new_store_starts_at_zero() {
    let (store, _images) = temp_store_with_images(Limits::default());
    assert_eq!(dim(&store), 0);
}

#[test]
fn pushing_an_image_increments_the_counter() {
    let (store, _images) = temp_store_with_images(Limits::default());
    let data = png(1);
    store.push_image(&data).unwrap();
    assert_eq!(dim(&store), data.len() as u64);
}

#[test]
fn several_images_accumulate() {
    let (store, _images) = temp_store_with_images(Limits::default());
    let mut expected = 0;
    for seed in 0..5u8 {
        let data = png(seed);
        store.push_image(&data).unwrap();
        expected += data.len() as u64;
    }
    assert_eq!(dim(&store), expected);
}

#[test]
fn re_pushing_the_same_image_does_not_double_count() {
    // The write is skipped when the file already exists, so the counter must be
    // skipped too. Otherwise the total drifts upward on every re-copy.
    let (store, _images) = temp_store_with_images(Limits::default());
    let data = png(7);
    store.push_image(&data).unwrap();
    let after_first = dim(&store);

    store.push_image(&data).unwrap();
    store.push_image(&data).unwrap();
    assert_eq!(dim(&store), after_first);
}

#[test]
fn deleting_an_image_decrements_the_counter() {
    let (store, images) = temp_store_with_images(Limits::default());
    let data = png(2);
    store.push_image(&data).unwrap();
    let expected = dim(&store);
    assert!(expected > 0);

    let path = images.join(format!("{}.png", xxhash_rust::xxh3::xxh3_64(&data)));
    store.delete_image(path.to_str().unwrap()).unwrap();

    assert_eq!(dim(&store), 0, "counter should return to baseline");
    assert!(!path.exists(), "file should be gone from disk");
}

#[test]
fn deleting_one_of_several_images_leaves_the_rest_counted() {
    let (store, images) = temp_store_with_images(Limits::default());
    let mut sizes = Vec::new();
    for seed in 0..3u8 {
        let data = png(seed);
        store.push_image(&data).unwrap();
        sizes.push(data.len() as u64);
    }
    let total: u64 = sizes.iter().sum();

    let data = png(0);
    let path = images.join(format!("{}.png", xxhash_rust::xxh3::xxh3_64(&data)));
    store.delete_image(path.to_str().unwrap()).unwrap();
    assert_eq!(dim(&store), total - sizes[0]);
}

#[test]
fn the_stored_clip_still_reports_its_path() {
    let (store, images) = temp_store_with_images(Limits::default());
    let data = png(3);
    store.push_image(&data).unwrap();

    let clips = store.get(10).unwrap();
    assert_eq!(clips.len(), 1);
    match &clips[0].content {
        ClipContent::Image(path) => {
            assert!(
                images.join(path).exists(),
                "path should be absolute: {}",
                path
            );
        }
        ClipContent::Text(_) => panic!("expected an image clip"),
    }
}

#[test]
fn clear_returns_the_counter_to_baseline() {
    let (store, _images) = temp_store_with_images(Limits::default());
    for seed in 0..3u8 {
        store.push_image(&png(seed)).unwrap();
    }
    assert!(dim(&store) > 0);

    store.clear().unwrap();
    assert_eq!(dim(&store), 0);
    assert!(store.get(10).unwrap().is_empty());
}

#[test]
fn expiry_returns_the_counter_to_baseline() {
    let (store, images) = temp_store_with_images(Limits {
        max_db_bytes: u64::MAX,
        max_images_bytes: u64::MAX,
        ttl_secs: Some(60),
    });

    // The image itself has to be the entry that ages out; expiring a separate
    // text entry leaves the image untouched and the counter legitimately held.
    store
        .push_image_at(&png(4), crate::now_micros() - 120 * crate::MICROS_PER_SEC)
        .unwrap();
    assert_eq!(dim(&store), 24, "one 24-byte header written");

    store.check_expire().unwrap();
    assert!(store.get(10).unwrap().is_empty(), "entry should be gone");
    assert_eq!(dim(&store), 0, "counter should follow the file");
    assert!(
        images.read_dir().unwrap().next().is_none(),
        "image file should be deleted"
    );
}
