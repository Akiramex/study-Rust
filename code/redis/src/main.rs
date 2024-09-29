use r2d2_redis::redis::{Commands, RedisError};
use core::time;
use std::{sync::LazyLock, thread::sleep};
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;


static RS_POOL: LazyLock<Pool<RedisConnectionManager>> = LazyLock::new(|| {
    let manager = RedisConnectionManager::new("redis://127.0.0.1").unwrap();
    Pool::builder().build(manager).unwrap()
});

fn set_value(key: &str, value: &str) -> Result<(), RedisError> {
    let mut conn = RS_POOL.get().unwrap();
    conn.set_ex::<&str, &str, ()>(key, value, 10)
}

fn get_value(key: &str) -> Result<String, RedisError> {
    let mut pconn = RS_POOL.get().unwrap();
    pconn.get::<&str, String>(key)
}

fn main() {

    let result = set_value("my_key", "akira");

    match result {
        Ok(_) => (),
        Err(e)=>println!("{e}")
    }

    let result = get_value("my_key");

    match result {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}")
    }

    sleep(time::Duration::from_secs(12));

    let result = get_value("my_key");

    match result {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}")
    }
}
