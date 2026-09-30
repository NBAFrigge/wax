//! Whole-store integrity checks.
//!
//! These do not test any single operation. They assert the relationships that
//! must hold between tables after *any* sequence of them, which is where the
//! damaging bugs have lived: a history row without its clip, a `hash_ts` index
//! that drifted from `history`, or a clip nothing references any more.

use super::common::{temp_store, temp_store_with_images, temp_store_with_ttl};
use crate::{CLIPS, Clip, ClipContent, ClipStore, HASH_TS, HISTORY, PINNED};
use redb::{ReadableDatabase, ReadableTable, ReadableTableMetadata};
use std::collections::HashSet;
use std::path::Path;

/// Assert every cross-table invariant, naming the specific violation if any.
fn check(store: &ClipStore) {
    let txn = store.db.begin_read().unwrap();
    let clips = txn.open_table(CLIPS).unwrap();
    let history = txn.open_table(HISTORY).unwrap();
    let pinned = txn.open_table(PINNED).unwrap();
    let hash_ts = txn.open_table(HASH_TS).unwrap();

    let history_rows: Vec<(u64, u64)> = history
        .iter()
        .unwrap()
        .map(|e| {
            let (k, v) = e.unwrap();
            (k.value(), v.value())
        })
        .collect();
    let history_hashes: HashSet<u64> = history_rows.iter().map(|(_, h)| *h).collect();

    // 1. Every history row points at a clip that exists.
    for (ts, hash) in &history_rows {
        assert!(
            clips.get(*hash).unwrap().is_some(),
            "history ts={ts} references hash {hash} with no CLIPS row"
        );
    }

    // 2. One history row per hash. `push` maintains this by removing the old
    //    timestamp before inserting the new one.
    assert_eq!(
        history_hashes.len(),
        history_rows.len(),
        "a hash appears in HISTORY more than once"
    );

    // 3. `hash_ts` is a faithful reverse index of `history`.
    assert_eq!(
        hash_ts.len().unwrap(),
        history_hashes.len() as u64,
        "HASH_TS has {} entries for {} history rows",
        hash_ts.len().unwrap(),
        history_hashes.len()
    );
    for (ts, hash) in &history_rows {
        let indexed = hash_ts
            .get(*hash)
            .unwrap()
            .unwrap_or_else(|| panic!("hash {hash} missing from HASH_TS"));
        assert_eq!(
            indexed.value(),
            *ts,
            "HASH_TS says ts={} for hash {hash}, HISTORY says {ts}",
            indexed.value()
        );
    }

    // 4. No orphaned clips: everything stored is either in history or pinned.
    for entry in clips.iter().unwrap() {
        let (k, _) = entry.unwrap();
        let hash = k.value();
        assert!(
            history_hashes.contains(&hash) || pinned.get(hash).unwrap().is_some(),
            "orphaned clip {hash}: in neither HISTORY nor PINNED"
        );
    }

    // 5. Every pin resolves to a real clip that is still in history.
    for entry in pinned.iter().unwrap() {
        let (k, _) = entry.unwrap();
        let hash = k.value();
        assert!(
            clips.get(hash).unwrap().is_some(),
            "pin {hash} has no CLIPS row"
        );
        assert!(
            history_hashes.contains(&hash),
            "pin {hash} has no HISTORY row, so it is invisible to the picker"
        );
    }

    // 6. Every image clip still has its file on disk.
    for entry in clips.iter().unwrap() {
        let (_, v) = entry.unwrap();
        if let Ok(clip) = bincode::deserialize::<Clip>(v.value())
            && let ClipContent::Image(path) = clip.content
        {
            assert!(
                Path::new(&path).exists(),
                "image clip points at missing file {path}"
            );
        }
    }
}

#[test]
fn invariants_hold_on_an_empty_store() {
    check(&temp_store());
}

#[test]
fn invariants_hold_through_a_realistic_sequence() {
    let store = temp_store();
    check(&store);

    for i in 0..30 {
        store.push_text(&format!("entry {i}")).unwrap();
        check(&store);
    }

    // Re-pushing an existing entry moves it to the newest position.
    store.push_text("entry 5").unwrap();
    check(&store);

    store.pin_text("entry 5").unwrap();
    store.pin_text("entry 7").unwrap();
    check(&store);

    store.unpin_text("entry 7").unwrap();
    check(&store);

    store.delete_text("entry 3").unwrap();
    check(&store);

    // Deleting a pinned entry unpins it too.
    store.delete_text("entry 5").unwrap();
    check(&store);

    store.clear().unwrap();
    check(&store);
}

#[test]
fn invariants_hold_through_trimming() {
    let store = temp_store();
    for i in 0..40 {
        store.push_text(&format!("entry {i}")).unwrap();
    }
    store.pin_text("entry 0").unwrap();
    store.pin_text("entry 1").unwrap();
    check(&store);

    // 4 rounds x 5 = 20 removed. The two pins are skipped, so they survive
    // alongside the 18 newest unpinned.
    for _ in 0..4 {
        store.trim_oldest(5).unwrap();
        check(&store);
    }
    assert_eq!(store.get(1_000).unwrap().len(), 20, "40 pushed, 20 trimmed");
    assert_eq!(store.get_pinned().unwrap().len(), 2, "both pins survived");

    // Pin what is left, so nothing is removable at all.
    for i in 22..40 {
        store.pin_text(&format!("entry {i}")).unwrap();
    }
    check(&store);

    store.trim_oldest(1_000).unwrap();
    check(&store);
    assert_eq!(
        store.get(1_000).unwrap().len(),
        20,
        "all pinned, nothing removable"
    );
}

#[test]
fn invariants_hold_through_expiry() {
    let store = temp_store_with_ttl(60);
    let now = crate::now_micros();

    for i in 0..20 {
        // Every third entry is stale, each with a distinct timestamp.
        let age = if i % 3 == 0 {
            120 * crate::MICROS_PER_SEC
        } else {
            5 * crate::MICROS_PER_SEC
        };
        store
            .push_text_at(&format!("entry {i}"), now - age - i as u64)
            .unwrap();
    }
    // A pinned stale entry must survive expiry.
    store.pin_text("entry 0").unwrap();
    check(&store);

    store.check_expire().unwrap();
    check(&store);

    assert!(
        store.get(1_000).unwrap().len() < 20,
        "some entries should expire"
    );
}

#[test]
fn invariants_hold_through_image_operations() {
    let (store, _images) = temp_store_with_images(crate::Limits::default());

    let png = |seed: u8| {
        let mut b = Vec::with_capacity(24);
        b.extend_from_slice(b"\x89PNG\r\n\x1a\n");
        b.extend_from_slice(&13u32.to_be_bytes());
        b.extend_from_slice(b"IHDR");
        b.extend_from_slice(&(10u32 + seed as u32).to_be_bytes());
        b.extend_from_slice(&(10u32 + seed as u32).to_be_bytes());
        b
    };

    for seed in 0..5u8 {
        store.push_image(&png(seed)).unwrap();
        check(&store);
    }

    let path = match &store.get(1).unwrap()[0].content {
        ClipContent::Image(p) => p.clone(),
        ClipContent::Text(_) => unreachable!(),
    };
    store.delete_image(&path).unwrap();
    check(&store);

    store.clear().unwrap();
    check(&store);
}

/// Deterministic PRNG so a failure is reproducible without a dev-dependency.
struct Rng(u64);

impl Rng {
    fn below(&mut self, n: u64) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) % n
    }
}

#[test]
fn invariants_hold_under_a_random_workload() {
    let store = temp_store();
    let mut rng = Rng(0x5EED_1234);
    let alphabet = 40u64;

    for step in 0..600 {
        let n = rng.below(alphabet);
        match rng.below(10) {
            0..=3 => {
                store.push_text(&format!("clip {n}")).unwrap();
            }
            4..=5 => {
                store.pin_text(&format!("clip {n}")).unwrap();
            }
            6 => {
                store.unpin_text(&format!("clip {n}")).unwrap();
            }
            7 => {
                store.delete_text(&format!("clip {n}")).unwrap();
            }
            8 => {
                store.trim_oldest((1 + rng.below(4)) as usize).unwrap();
            }
            _ => {
                if step % 97 == 0 {
                    store.clear().unwrap();
                }
            }
        }
        // Checking every step is the point: drift shows up in a single
        // intermediate state, not only at the end.
        check(&store);
    }
}
