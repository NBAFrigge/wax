use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::sync::atomic::{AtomicU64, Ordering};
use wax_store::ClipStore;

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_store() -> ClipStore {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros();
    ClipStore::open(format!("/tmp/wax_bench_{}_{}.redb", ts, id)).unwrap()
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

fn cache_file_with_entries(n: usize) -> tempfile::NamedTempFile {
    use std::io::Write;
    let mut f = tempfile::NamedTempFile::new().unwrap();
    for i in 0..n {
        write!(f, "bench entry number {}\0", i).unwrap();
    }
    f
}

fn bench_read_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_cache");
    for &size in &[50usize, 500, 5_000, 50_000] {
        let f = cache_file_with_entries(size);
        group.bench_with_input(BenchmarkId::new("entries", size), &size, |b, _| {
            b.iter(|| wax_store::read_cache_from(f.path(), 50));
        });
    }
    group.finish();
}

fn bench_get(c: &mut Criterion) {
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
