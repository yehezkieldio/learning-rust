use crate::cache_strategies::CacheStrategy;

pub struct CacheContext {
    strategy: Box<dyn CacheStrategy + Send + Sync>,
}

impl CacheContext {
    pub fn new(strategy: Box<dyn CacheStrategy + Send + Sync>) -> Self {
        CacheContext { strategy }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.strategy.get(key)
    }

    pub fn set(&self, key: &str, value: String) {
        self.strategy.set(key, value);
    }
}