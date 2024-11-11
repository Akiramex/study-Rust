use chrono::Local;
use std::thread;
use tokio::{self, task, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn sleep_task(n: u64)
{
    time::sleep(time::Duration::from_secs(n)).await;
    println!("task over: {}", now());
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
   
    rt.block_on(async {
        let res =tokio::join!(sleep_task(1), sleep_task(2));
    });

    //let rt = Runtime::new().unwrap();
    //let _guard = rt.enter();
    /* 
    task::spawn(async {
        time::sleep(time::Duration::from_secs(3)).await;
        println!("task over: {}", now());
    });

    task::spawn(async {
        time::sleep(time::Duration::from_secs(2)).await;
        println!("task over: {}", now());
    });
    */


    //thread::sleep(time::Duration::from_secs(4));
}