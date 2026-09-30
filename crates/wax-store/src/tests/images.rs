//! Image storage: what gets written, and when it is removed.
//!
//! The byte counter that used to live here is gone; history length is now the
//! only limit, so these tests cover file lifecycle rather than accounting.

use super::common::temp_store_with_images;
use crate::{ClipContent, Limits};
use std::io::Write;
use tempfile::NamedTempFile;

/// A tiny but structurally valid PNG. Only the 24-byte header matters here.
fn png(seed: u8) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(24);
    bytes.extend_from_slice(b"\x89PNG\r\n\x1a\n");
    bytes.extend_from_slice(&13u32.to_be_bytes());
    bytes.extend_from_slice(b"IHDR");
    bytes.extend_from_slice(&(100u32 + seed as u32).to_be_bytes());
    bytes.extend_from_slice(&(200u32 + seed as u32).to_be_bytes());
    bytes
}

fn image_files(images: &std::path::Path) -> usize {
    images.read_dir().unwrap().count()
}

#[test]
fn an_image_clip_records_its_absolute_path() {
    let (store, images) = temp_store_with_images(Limits::default());
    store.push_image(&png(1)).unwrap();

    let clips = store.get(10).unwrap();
    assert_eq!(clips.len(), 1);
    match &clips[0].content {
        ClipContent::Image(path) => {
            assert!(path.ends_with(".png"), "unexpected path: {}", path);
            assert!(images.join(path).exists(), "path should resolve: {}", path);
        }
        ClipContent::Text(_) => panic!("expected an image clip"),
    }
}

#[test]
fn the_same_image_twice_stores_one_file_and_one_entry() {
    let (store, images) = temp_store_with_images(Limits::default());
    let data = png(2);

    store.push_image(&data).unwrap();
    store.push_image(&data).unwrap();
    store.push_image(&data).unwrap();

    assert_eq!(image_files(&images), 1, "content-addressed, so one file");
    assert_eq!(store.get(10).unwrap().len(), 1, "one deduplicated entry");
}

#[test]
fn different_images_get_different_files() {
    let (store, images) = temp_store_with_images(Limits::default());
    for seed in 0..3u8 {
        store.push_image(&png(seed)).unwrap();
    }
    assert_eq!(image_files(&images), 3);
    assert_eq!(store.get(10).unwrap().len(), 3);
}

#[test]
fn deleting_an_image_removes_its_file() {
    let (store, images) = temp_store_with_images(Limits::default());
    store.push_image(&png(3)).unwrap();
    assert_eq!(image_files(&images), 1);

    let path = match &store.get(1).unwrap()[0].content {
        ClipContent::Image(p) => p.clone(),
        ClipContent::Text(_) => unreachable!(),
    };
    store.delete_image(&path).unwrap();

    assert_eq!(image_files(&images), 0, "file should be gone");
    assert!(store.get(10).unwrap().is_empty());
}

#[test]
fn deleting_one_image_leaves_the_others() {
    let (store, images) = temp_store_with_images(Limits::default());
    for seed in 0..3u8 {
        store.push_image(&png(seed)).unwrap();
    }

    let path = match &store.get(1).unwrap()[0].content {
        ClipContent::Image(p) => p.clone(),
        ClipContent::Text(_) => unreachable!(),
    };
    store.delete_image(&path).unwrap();

    assert_eq!(image_files(&images), 2);
    assert_eq!(store.get(10).unwrap().len(), 2);
}

#[test]
fn clear_removes_every_image_file() {
    let (store, images) = temp_store_with_images(Limits::default());
    for seed in 0..4u8 {
        store.push_image(&png(seed)).unwrap();
    }
    assert_eq!(image_files(&images), 4);

    store.clear().unwrap();

    assert_eq!(image_files(&images), 0, "no orphaned files left behind");
    assert!(store.get(10).unwrap().is_empty());
}

#[test]
fn a_missing_file_on_disk_does_not_break_listing() {
    // The picker reads the cache, not the file, so a deleted image should still
    // list rather than vanish or error.
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(&png(5)).unwrap();
    file.flush().unwrap();

    let (store, _images) = temp_store_with_images(Limits::default());
    let clip = format!("[img] {}", file.path().to_str().unwrap());
    store.push_text(&clip).unwrap();

    std::fs::remove_file(file.path()).unwrap();
    assert_eq!(store.get(10).unwrap().len(), 1);
}

#[test]
fn expiry_removes_stale_image_files() {
    let (store, images) = temp_store_with_images(Limits {
        max_entries: u64::MAX,
        ttl_secs: Some(60),
    });

    // The image itself has to be the entry that ages out; expiring a separate
    // text entry leaves the image untouched.
    store
        .push_image_at(&png(4), crate::now_micros() - 120 * crate::MICROS_PER_SEC)
        .unwrap();
    assert_eq!(image_files(&images), 1);

    store.check_expire().unwrap();

    assert!(store.get(10).unwrap().is_empty(), "entry should be gone");
    assert_eq!(image_files(&images), 0, "file should follow the entry");
}
