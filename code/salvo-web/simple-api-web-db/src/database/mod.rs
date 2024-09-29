pub mod user;

use std::env;
use std::sync::LazyLock;
use sqlx::{PgPool, Pool};

use r2d2::Pool as r2d2Pool;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::redis::{Commands, RedisError};

const OVETTIME: usize = 60 * 10;

static DB_POOL: LazyLock<PgPool> = LazyLock::new(||{
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is no found");
    
    // 异步函数不能用于闭包，这里用tokio把异步转为同步，不需要从main函数调用set方法
    tokio::runtime::Handle::current().block_on(Pool::connect(&database_url)).unwrap()
});

pub fn db_pool() -> &'static PgPool {
    &DB_POOL
}

static RS_POOL: LazyLock<r2d2Pool<RedisConnectionManager>> = LazyLock::new(|| {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is no found");

    let manager = RedisConnectionManager::new(redis_url).unwrap();
    r2d2Pool::builder().build(manager).unwrap()
});

pub fn set_value(key: &str, value: &str) -> Result<(), RedisError> {
    let mut conn = RS_POOL.get().unwrap();
    conn.set_ex::<&str, &str, ()>(key, value, OVETTIME)
}

pub fn get_value(key: &str) -> Result<String, RedisError> {
    let mut pconn = RS_POOL.get().unwrap();
    pconn.get::<&str, String>(key)
}
