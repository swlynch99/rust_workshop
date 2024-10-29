use std::{
    sync::{Arc, Barrier},
    time::Instant,
};

use cache::ShareableCache;
use criterion::{measurement::WallTime, BenchmarkGroup, BenchmarkId};
use rand::{Rng, SeedableRng};

pub fn benchmark_cache_multi_threaded(
    id: BenchmarkId,
    group: &mut BenchmarkGroup<'_, WallTime>,
    thread_count: usize,
    cache: impl ShareableCache<String, String>,
) {
    let thread_count = thread_count as u64;

    // We're going to use this from multiple threads. Ownership is easiest when you own your data,
    // so we'll use a smart pointer to share ownership. Each smart pointer is owned - so it makes
    // sharing easy. However, you can't get a mutable reference to the data inside an Arc, so you
    // must have a SharableCache in order to share!
    let cache = Arc::new(cache);

    group.bench_function(id, |bencher| {
        bencher.iter_custom(|iterations| {
            std::thread::scope(|scope| {
                let thread_count = thread_count.min(iterations);
                let barrier = Arc::new(Barrier::new(1 + thread_count as usize));
                for _ in 0..thread_count {
                    let iterations_per_thread = iterations / thread_count;
                    let thread_barrier = barrier.clone();
                    let cache = cache.clone();
                    scope.spawn(move || {
                        let words: Vec<String> = (0..2 * cache::MAX_SIZE)
                            .map(|i| format!("value {i}"))
                            .collect();
                        thread_barrier.wait();
                        let mut random = rand::rngs::StdRng::from_entropy();
                        for _ in 0..iterations_per_thread {
                            let load = random.gen_ratio(4, 5);
                            let key = random.gen_range(0..(2 * cache::MAX_SIZE));
                            if load {
                                criterion::black_box(cache.get(&words[key]));
                            } else {
                                cache.set(words[key].clone(), words[key].clone());
                            }
                        }
                    });
                }
                barrier.wait();
                Instant::now()
            })
            .elapsed()
        });
    });
}
