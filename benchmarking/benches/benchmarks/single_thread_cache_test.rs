use cache::SizeLimitedCache;
use criterion::{measurement::WallTime, BenchmarkGroup, BenchmarkId};
use rand::{Rng, SeedableRng};

pub fn benchmark_cache_single_threaded(
    id: BenchmarkId,
    group: &mut BenchmarkGroup<'_, WallTime>,
    mut cache: impl SizeLimitedCache<String, String>,
) {
    let words: Vec<String> = (0..2 * cache::MAX_SIZE)
        .map(|i| format!("value {i}"))
        .collect();
    let mut random = rand::rngs::StdRng::seed_from_u64(37);

    group.bench_function(id, |bencher| {
        bencher.iter(|| {
            let store = random.gen_range(0..(2 * cache::MAX_SIZE)) as usize;
            let load = random.gen_range(0..(2 * cache::MAX_SIZE)) as usize;

            cache.set(words[store].clone(), words[store].clone());
            criterion::black_box(cache.get(&words[load]));
        });
    });
}
