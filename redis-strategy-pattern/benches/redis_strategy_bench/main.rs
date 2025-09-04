use criterion::{criterion_group, criterion_main, Criterion};
use redis::Client;
use redis_strategy_pattern::cache_context::CacheContext;
use redis_strategy_pattern::cache_strategies::CacheStrategy;
use redis_strategy_pattern::strategies::{
    lazy_invalidation::LazyInvalidationCache,
    read_through::ReadThroughCache,
    write_through::WriteThroughCache,
};

fn benchmark_cache_strategies(c: &mut Criterion) {
    let redis_url = "redis://127.0.0.1/";
    let client = Client::open(redis_url).unwrap();

    let strategies: Vec<(&str, Box<dyn CacheStrategy + Send + Sync>)> = vec![
        ("WriteThrough", Box::new(WriteThroughCache::new(client.clone()))),
        ("ReadThrough", Box::new(ReadThroughCache::new(client.clone()))),
        ("LazyInvalidation", Box::new(LazyInvalidationCache::new(client.clone()))),
];

   let keys: Vec<String> = (0..100)
    .map(|_| format!("bench_key_{}", rand::random::<u64>()))
    .collect();


    for (name, strategy) in strategies {
        let context = CacheContext::new(strategy);

        c.bench_function(&format!("Set with {}", name), |b| {
            b.iter(|| {
                for key in &keys {
                    let _ = context.set(key, "value");
                }
            })
        });

        c.bench_function(&format!("Get with {}", name), |b| {
            for key in &keys {
                let _ = context.set(key, "value");
            }
            b.iter(|| {
                for key in &keys {
                    let _ = context.get(key);
                }
            })
        });

        c.bench_function(&format!("Delete with {}", name), |b| {
            for key in &keys {
                let _ = context.set(key, "value");
            }
            b.iter(|| {
                for key in &keys {
                    let _ = context.delete(key);
                }
            })
        });
    }
}

criterion_group!(benches, benchmark_cache_strategies);
criterion_main!(benches);