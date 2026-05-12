// 还能探测到错误吗？
// 可以。

// Loom 会同时探索读线程在写线程之前运行的情况（此时 ready 为 false，直接忽略，测试通过）。

// 也会探索读线程在写线程之后运行的情况（ready 为 true，进入 if 块，检查 data 的值）。

// 在 relaxed_can_cause_wrong_value 中，Loom 可能会模拟出“写线程的 ready.store 被重排到 data.store 之前，而读线程恰好在此时看到 true” 的交错，此时 d 读到的仍是旧值 0，断言失败，错误被捕获。而在 release_acquire_is_correct 中，Release-Acquire 保证了正确的 happens-before 关系，永远不会触发断言。

use loom::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use loom::thread;
use std::sync::Arc;

#[test]
fn relaxed_can_cause_wrong_value() {
    loom::model(|| {
        let ready = Arc::new(AtomicBool::new(false));
        let data = Arc::new(AtomicI32::new(0));

        let writer = {
            let ready = ready.clone();
            let data = data.clone();
            thread::spawn(move || {
                data.store(42, Ordering::Relaxed);
                ready.store(true, Ordering::Relaxed);
            })
        };

        let reader = {
            let ready = ready.clone();
            let data = data.clone();
            thread::spawn(move || {
                // 只检查一次，不循环
                if ready.load(Ordering::Relaxed) {
                    let d = data.load(Ordering::Relaxed);
                    assert_eq!(d, 42, "Reader saw an invalid value!");
                }
            })
        };

        writer.join().unwrap();
        reader.join().unwrap();
    });
}

#[test]
fn release_acquire_is_correct() {
    loom::model(|| {
        let ready = Arc::new(AtomicBool::new(false));
        let data = Arc::new(AtomicI32::new(0));

        let writer = {
            let ready = ready.clone();
            let data = data.clone();
            thread::spawn(move || {
                data.store(42, Ordering::Relaxed);
                ready.store(true, Ordering::Release);
            })
        };

        let reader = {
            let ready = ready.clone();
            let data = data.clone();
            thread::spawn(move || {
                if ready.load(Ordering::Acquire) {
                    let d = data.load(Ordering::Relaxed);
                    assert_eq!(d, 42);
                }
            })
        };

        writer.join().unwrap();
        reader.join().unwrap();
    });
}

/*
// thread 'test::test_relaxed_bug' (23160) panicked at D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\loom-0.7.2\src\rt\path.rs:247:13:
// Model exceeded maximum number of branches. This is often caused by an algorithm requiring the processor to make progress, e.g. spin locks.
#[test]
fn test_relaxed_bug() {
    loom::model(|| {
        let flag = Arc::new(AtomicBool::new(false));
        let data = Arc::new(AtomicI32::new(0));

        // 线程1：先写 flag，再写 data（Relaxed 允许重排）
        let t1 = {
            let flag = flag.clone();
            let data = data.clone();
            thread::spawn(move || {
                data.store(100, Ordering::Relaxed);
                flag.store(true, Ordering::Relaxed);
            })
        };

        // 线程2：读 flag，再读 data
        let t2 = {
            let flag = flag.clone();
            let data = data.clone();
            thread::spawn(move || {
                loop {
                    if flag.load(Ordering::Relaxed) {
                        let d = data.load(Ordering::Relaxed);
                        // Loom 会触发：flag=true 但 d=0
                        if d == 0 {
                            panic!("BUG: flag=true but data=0 (Relaxed 乱序)");
                        }
                        break;
                    }
                }
            })
        };

        t1.join().unwrap();
        t2.join().unwrap();
    });
}
*/

/*
// thread 'test::test_release_acquire_correct' (24600) panicked at D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\loom-0.7.2\src\rt\path.rs:247:13:
// Model exceeded maximum number of branches. This is often caused by an algorithm requiring the processor to make progress, e.g. spin locks.
#[test]
fn test_release_acquire_correct() {
    loom::model(|| {
        let flag = Arc::new(AtomicBool::new(false));
        let data = Arc::new(AtomicI32::new(0));

        // 线程1：Release 保证 data 先写，flag 后写
        let t1 = {
            let flag = flag.clone();
            let data = data.clone();
            thread::spawn(move || {
                data.store(100, Ordering::Release);
                flag.store(true, Ordering::Release);
            })
        };

        // 线程2：Acquire 保证读到 flag=true 后，data 一定可见
        let t2 = {
            let flag = flag.clone();
            let data = data.clone();
            thread::spawn(move || {
                loop {
                    if flag.load(Ordering::Acquire) {
                        let d = data.load(Ordering::Acquire);
                        assert_eq!(d, 100); // 永远成功
                        break;
                    }
                }
            })
        };

        t1.join().unwrap();
        t2.join().unwrap();
    });
}
*/
