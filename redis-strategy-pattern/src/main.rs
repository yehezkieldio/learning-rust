use redis::Client;

use crate::{cache_context::CacheContext, strategies::write_through::WriteThroughCache};

mod cache_context;
mod cache_strategies;
mod strategies;

fn main() {
    let redis_url = "redis://127.0.0.1/";
    let client = Client::open(redis_url).expect("Failed to create Redis client");

    let strategy = WriteThroughCache::new(client.clone());
    let context = CacheContext::new(Box::new(strategy));

    context.set("user_123", "elizielx".into());

    let value = context.get("user_123");
    println!("Cached value: {:?}", value);
}
