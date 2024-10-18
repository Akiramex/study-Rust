use std::env;
use std::sync::LazyLock;

use r2d2::Pool as r2d2Pool;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::redis::{Commands, RedisError};

const OVETTIME: usize = 60 * 10;

static RS_POOL: LazyLock<r2d2Pool<RedisConnectionManager>> = LazyLock::new(|| {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is no found");

    let manager = RedisConnectionManager::new(redis_url).unwrap();
    r2d2Pool::builder().build(manager).unwrap()
});

pub fn rs_set_value(key: &str, value: &str) -> Result<(), RedisError> {
    let mut pconn = RS_POOL.get().unwrap();
    pconn.set_ex::<&str, &str, ()>(key, value, OVETTIME)
}

pub fn rs_delete_key(key: &str) -> Result<String, RedisError> {
    let mut pconn = RS_POOL.get().unwrap();
    pconn.del::<&str, String>(key)
}

pub fn rs_get_value(key: &str) -> Result<String, RedisError> {
    let mut pconn = RS_POOL.get().unwrap();
    pconn.get::<&str, String>(key)
}
