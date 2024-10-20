use tokio;
fn main() {
    // 创建runtime
    //let rt = tokio::runtime::Runtime::new().unwrap();

    //std::thread::sleep(std::time::Duration::from_secs(10));

    //rt.block_on(async {
    //    println!("hello");
    //});


    //等价于 #[tokio::main(flavor = "multi_thread", worker_threads = 8))]
    tokio::runtime::Builder::new_multi_thread()
    .worker_threads(8)  
    .enable_all()
    .build()
    .unwrap()
    .block_on(async { println!("hello"); });
}
