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
