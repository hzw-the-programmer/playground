use std::thread;
use std::time::Duration;
use std::sync::{Arc, Barrier};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    // t1();
    // t2();
    t3();
}

fn t1() {
    let v = vec![10, 9, 8, 7];
    let handle = thread::spawn(move || {
        println!("child: {:?}", v);
        for i in 1..10 {
            println!("child: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //drop(v);

    for i in 1..5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}

fn t2() {
    let num = 10;
    let mut handles = Vec::with_capacity(num);
    let barrier = Arc::new(Barrier::new(num));
    let mut j = 0;
    for i in 0..num {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            j+=1;
            println!("{} brefore wait: j = {}", i, j);
            barrier.wait();
            println!("{} after wait: j = {}", i, j);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("j = {}", j);
}

fn t3() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);
    
    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1).expect("channel will be there waiting for pool");
        });
    }
    println!("{}", rx.iter().take(n_jobs).fold(0, |a, b| a + b));
}
