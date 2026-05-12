use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static READY: AtomicBool = AtomicBool::new(false);
static mut DATA: i32 = 0; // 模拟共享数据，实际中可能是一块缓冲区

fn main() {
    let writer = thread::spawn(|| {
        // 1. 准备数据
        unsafe {
            DATA = 42;
        }
        // 2. 通知读取者
        READY.store(true, Ordering::Relaxed);
    });

    let reader = thread::spawn(|| {
        // 3. 自旋等待就绪标志
        while !READY.load(Ordering::Relaxed) {
            // 忙等
        }
        // 4. 一旦看到标志，直接使用数据
        let d = unsafe { DATA };
        println!("读取到的数据: {}", d);
        // 期望 d == 42，但 Relaxed 下可能看到 0 或其他结果
    });

    writer.join().unwrap();
    reader.join().unwrap();
}
