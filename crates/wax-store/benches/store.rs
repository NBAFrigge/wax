use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use wax_store::ClipStore;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Point `XDG_DATA_HOME` at a temporary directory, once.
///
/// `cache_path()` and the images directory both resolve through
/// `dirs::data_dir()`, so without this the benches would read and overwrite the
/// real `~/.local/share/wax/history.cache`. Every bench calls this first.
fn isolate_data_dir() {
    static DIR: OnceLock<tempfile::TempDir> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        // SAFETY: called from criterion's main thread before any bench work
        // starts, so no other thread is reading the environment.
        unsafe { std::env::set_var("XDG_DATA_HOME", dir.path()) };
        dir
    });
}

fn temp_store() -> ClipStore {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros();
    ClipStore::open(
        format!("/tmp/wax_bench_{}_{}.redb", ts, id),
        wax_store::Limits::default(),
    )
    .unwrap()
}

fn store_with_entries(n: usize) -> ClipStore {
    let store = temp_store();
    for i in 0..n {
        store
            .push_text(&format!("bench entry number {}", i))
            .unwrap();
    }
    store
}

/// Overwrite the (isolated) cache file with `n` entries.
fn write_cache_with_entries(n: usize) {
    let path = wax_store::cache_path();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let content: String = (0..n)
        .map(|i| format!("bench entry number {i}\0"))
        .collect();
    std::fs::write(path, content).unwrap();
}

fn bench_read_cache(c: &mut Criterion) {
    isolate_data_dir();
    let mut group = c.benchmark_group("read_cache");
    for &size in &[50usize, 500, 5_000, 50_000] {
        write_cache_with_entries(size);
        group.bench_with_input(BenchmarkId::new("entries", size), &size, |b, _| {
            b.iter(|| wax_store::read_cache(50));
        });
    }
    group.finish();
}

fn bench_get(c: &mut Criterion) {
    isolate_data_dir();
    let mut group = c.benchmark_group("get");
    for &size in &[50usize, 500, 5_000] {
        let store = store_with_entries(size);
        group.bench_with_input(BenchmarkId::new("db_entries", size), &size, |b, _| {
            b.iter(|| store.get(50).unwrap());
        });
    }
    group.finish();
}

fn bench_push_text(c: &mut Criterion) {
    isolate_data_dir();
    let mut group = c.benchmark_group("push_text");
    for &size in &[0usize, 100, 1_000] {
        group.bench_with_input(BenchmarkId::new("db_entries", size), &size, |b, &size| {
            b.iter_batched(
                || {
                    let store = store_with_entries(size);
                    let unique = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos()
                        .to_string();
                    (store, unique)
                },
                |(store, entry)| store.push_text(&entry).unwrap(),
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_read_cache, bench_get, bench_push_text);
criterion_main!(benches);
