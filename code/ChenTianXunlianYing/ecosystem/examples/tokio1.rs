use std::{thread, time::Duration};

use tokio::{fs, runtime::Builder, time::sleep};

fn main() {
    let handle = thread::spawn(||{
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        
        // 把这个future放到队列里面
        rt.spawn(async {
            println!("Future 1");
            let content = fs::read_to_string("Cargo.toml").await.unwrap();
            println!("content length: {}", content.len())
        });
        // 把这个future放到队列里面
        rt.spawn(async {
            println!("Future 2");
            foo();
            println!("foo finish")
        });

        // 运行runtime直到该future返回
        rt.block_on(async {
            sleep(Duration::from_millis(900)).await;
        });
    });

    handle.join().unwrap();
}

fn foo() {
    thread::sleep(Duration::from_millis(800));
}