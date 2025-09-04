use std::{collections::HashMap, sync::Mutex};

use redis::Client;

use crate::cache_strategies::CacheStrategy;

pub struct LazyInvalidationCache {
    redis: Client,
    invalid_keys: Mutex<HashMap<String, bool>>
}

impl LazyInvalidationCache {
    pub fn new(client: Client) -> Self {
        Self { redis: client, invalid_keys: Mutex::new(HashMap::new()) }
    }
}

impl CacheStrategy for LazyInvalidationCache {
    fn get(&self, key: &str) -> Option<String> {
        let invalid = self.invalid_keys.lock().unwrap().get(key).copied().unwrap_or(false);
        if invalid {
            return None;
        }

        let mut con = self.redis.get_connection().expect("Failed to connect to Redis");
        let result: redis::RedisResult<String> = redis::cmd("GET").arg(key).query(&mut con);
        result.ok()
    }

    fn set(&self, key: &str, value: String) {
        let mut con = self.redis.get_connection().expect("Failed to connect to Redis");
        let _: () = redis::cmd("SET").arg(key).arg(value).query(&mut con).expect("Failed to set value in Redis");
        self.invalid_keys.lock().unwrap().insert(key.to_string(), false);
    }
}

