fn thread_local_function() {
    use thread_local::ThreadLocal;
    use std::sync::Arc;
    use std::cell::Cell;
    use std::thread;

    let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];

    // 创建多个线程
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            // 将计数器加1
            // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
            // 这只能在线程退出后发生，因此不会导致任何竞争条件
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }
    for handle in v {
        handle.join().unwrap();
    }

    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
        // 因为一些线程已退出，并且其他线程会回收退出线程的对象
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });

    // 和为5
    assert_eq!(total, 5);
}

fn function2() {
    use std::thread;
    use std::sync::{Arc, Mutex, Condvar};

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

use std::{env, thread};
use std::sync::Once;

static mut VAL: usize = 0;
static INIT: Once = Once::new();
fn function3() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 1;
            }
        });
    });
    let handle2 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 2;
            }
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}

use study_thread::ExProgram;
use std::time::Duration;

fn main() {
    //function3();

    let sleep_program = ExProgram{path: "C:\\Akira\\1_Code\\rust\\study-Rust\\code\\studyDemo\\a-print-program.exe", opt: None};
    
    thread::spawn(move ||{
        loop {
            let mut child = sleep_program.run();

            child.wait().expect("msg");

            thread::sleep(Duration::from_secs(1));
        }
    });

    // 获取当前工作目录
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // 打印当前工作目录
    println!("Current directory: {:?}", current_dir);

    thread::sleep(Duration::from_secs(20));
}
