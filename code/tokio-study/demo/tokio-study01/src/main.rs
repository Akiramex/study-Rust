use std::thread;

use tokio;

async fn foo() {
    
    let task = tokio::time::sleep(tokio::time::Duration::from_secs(2));

    // do something
    println!("madamda");
    // start the task --> sleep 2 secs
    task.await;

    println!("Zzzz")
}

fn main() {
    // 创建runtime
    //let rt = tokio::runtime::Runtime::new().unwrap();

    //std::thread::sleep(std::time::Duration::from_secs(10));

    //rt.block_on(async {
    //    println!("hello");
    //});

    //#[tokio::main] 默认多线程, 线程数为机器拥有线程数

    //创建单线程异步 #[tokio::main(flavor = "current_thread")]

    //等价于 #[tokio::main(flavor = "multi_thread", worker_threads = 8))]
    tokio::runtime::Builder::new_multi_thread()
    .worker_threads(8)  
    .enable_all()
    .build()
    .unwrap()
    .block_on( foo());


    let _ = thread::spawn(||{
        tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tokio::spawn(async {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                println!("Zzzzz")
            }).await;

            tokio::spawn(async {
                println!("Studying")
            }).await;

        })
    });

    std::thread::sleep(std::time::Duration::from_secs(5));
    
}