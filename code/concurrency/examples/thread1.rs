use anyhow::{anyhow, Result};
use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

const NUM_PRODUCES: usize = 4;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    // 创建producers
    for i in 0..NUM_PRODUCES {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let comsumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }

        42
    });

    let secret = comsumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    println!("{secret}");
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exist", idx);
            break;
        }
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
