use std::hash::RandomState;

use cache::SizeLimitedCache;
use criterion::{criterion_group, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion};
use rand::{Rng, SeedableRng};

fn single_threaded_comparison(c: &mut Criterion) {
    let mut single_thread_benchmark_group = c.benchmark_group("single_thread");

    benchmark_cache_single_threaded(
        BenchmarkId::from_parameter("example"),
        &mut single_thread_benchmark_group,
        example_sieve_cache::SieveCache::new(),
    );

    benchmark_cache_single_threaded(
        BenchmarkId::from_parameter("k-cache"),
        &mut single_thread_benchmark_group,
        KCache(k_cache::Cache::new(RandomState::new(), cache::MAX_SIZE)),
    );
}

fn benchmark_cache_single_threaded(
    id: BenchmarkId,
    group: &mut BenchmarkGroup<'_, WallTime>,
    mut cache: impl SizeLimitedCache<i64, i64>,
) {
    let mut random = rand::rngs::StdRng::seed_from_u64(37);
    group.throughput(criterion::Throughput::Elements(2));
    group.bench_function(id, |bencher| {
        bencher.iter(|| {
            let store = random.gen_range(0..(2 * cache::MAX_SIZE)) as i64;
            let load = random.gen_range(0..(2 * cache::MAX_SIZE)) as i64;

            cache.set(store, store);
            criterion::black_box(cache.get(&load));
        });
    });
}

struct KCache(k_cache::Cache<i64, i64, RandomState>);
impl SizeLimitedCache<i64, i64> for KCache {
    fn get(&mut self, key: &i64) -> Option<i64> {
        self.0.get(key).cloned()
    }

    fn set(&mut self, key: i64, value: i64) {
        self.0.put(key, value);
    }
}

criterion_group!(single_thread, single_threaded_comparison);
