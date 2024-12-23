# block_on 是怎么工作的？
block_on在当前线程中loop执行Future。
tokio\src\runtime\park.rs
```rust
impl CachedParkThread {
    pub(crate) fn block_on<F: Future>(&mut self, f: F) -> Result<F::Output, AccessError> {
        use std::task::Context;
        use std::task::Poll::Ready;

        let waker = self.waker()?;
        let mut cx = Context::from_waker(&waker);

        pin!(f);

        loop {
            if let Ready(v) = crate::runtime::coop::budget(|| f.as_mut().poll(&mut cx)) {
                return Ok(v);
            }

            self.park();
        }
    }
}
```
Future返回Pending，用条件变量park当前线程。
tokio\src\runtime\park.rs
```rust
impl Inner {
    fn park(&self) {
        if self
            .state
            .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
            .is_ok()
        {
            return;
        }

        let mut m = self.mutex.lock();

        match self.state.compare_exchange(EMPTY, PARKED, SeqCst, SeqCst) {
            Ok(_) => {}
            Err(NOTIFIED) => {
                return;
            }
            Err(actual) => panic!("inconsistent park state; actual = {}", actual),
        }

        loop {
            m = self.condvar.wait(m).unwrap();

            if self
                .state
                .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
                .is_ok()
            {
                return;
            }
        }
    }
}
```
返回Ready，结束while循环。
返回Pending时注册得waker会在另一个线程用条件变量unpark block_on线程。
tokio\src\runtime\park.rs
```rust
impl Inner {
    fn unpark(&self) {
        match self.state.swap(NOTIFIED, SeqCst) {
            EMPTY => return,    // no one was waiting
            NOTIFIED => return, // already unparked
            PARKED => {}        // gotta go wake someone up
            _ => panic!("inconsistent state in unpark"),
        }
        drop(self.mutex.lock());
        self.condvar.notify_one();
    }
}
```

block_on没有将Future包装到任务，直接poll Future，说明没有复杂的状态要管理。
