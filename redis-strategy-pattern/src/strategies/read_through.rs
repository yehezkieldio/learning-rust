use redis::Client;

use crate::cache_strategies::CacheStrategy;

pub struct ReadThroughCache {
    redis: Client
}

impl ReadThroughCache {
    pub fn new(client: Client) -> Self {
        Self { redis: client }
    }
}

impl CacheStrategy for ReadThroughCache {
    fn get(&self, key: &str) -> Option<String> {
        let mut con = self.redis.get_connection().expect("Failed to connect to Redis");
        let result: redis::RedisResult<String> = redis::cmd("GET").arg(key).query(&mut con);
        result.ok()
    }

    fn set(&self, key: &str, value: String) {
        let mut con = self.redis.get_connection().expect("Failed to connect to Redis");
        let _: () = redis::cmd("SET").arg(key).arg(value).query(&mut con).expect("Failed to set value in Redis");
    }

    fn delete(&self, key: &str) {
        let mut con = self.redis.get_connection().expect("Failed to connect to Redis");
        let _: () = redis::cmd("DEL").arg(key).query(&mut con).expect("Failed to delete key in Redis");
    }
}