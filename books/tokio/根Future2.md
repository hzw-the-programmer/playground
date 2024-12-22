## Future有两个角色。

### 包装到Task，成为根Future。
* 存储Task信息的Cell结构体中保存了Future，即 cell.core.stage.stage
```rust
#[repr(C)]
pub(super) struct Cell<T: Future, S> {
    /// Hot task state data
    pub(super) header: Header,

    /// Either the future or output, depending on the execution stage.
    pub(super) core: Core<T, S>,

    /// Cold data
    pub(super) trailer: Trailer,
}

#[repr(C)]
pub(super) struct Core<T: Future, S> {
    /// Scheduler used to drive this future.
    pub(super) scheduler: S,

    /// The task's ID, used for populating `JoinError`s.
    pub(super) task_id: Id,

    /// Either the future or the output.
    pub(super) stage: CoreStage<T>,
}

pub(super) struct CoreStage<T: Future> {
    stage: UnsafeCell<Stage<T>>,
}

#[repr(C)] // https://github.com/rust-lang/miri/issues/3780
pub(super) enum Stage<T: Future> {
    Running(T),
    Finished(super::Result<T::Output>),
    Consumed,
}
```

* 通过 task::spawn 将根 Future 存储到 Cell 中
tokio\src\task\spawn.rs
```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    // preventing stack overflows on debug mode, by quickly sending the
    // task to the heap.
    if cfg!(debug_assertions) && std::mem::size_of::<F>() > BOX_FUTURE_THRESHOLD {
        spawn_inner(Box::pin(future), None)
    } else {
        spawn_inner(future, None)
    }
}

pub(super) fn spawn_inner<T>(future: T, name: Option<&str>) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    use crate::runtime::{context, task};

    #[cfg(all(
        tokio_unstable,
        tokio_taskdump,
        feature = "rt",
        target_os = "linux",
        any(
            target_arch = "aarch64",
            target_arch = "x86",
            target_arch = "x86_64"
        )
    ))]
    let future = task::trace::Trace::root(future);
    let id = task::Id::next();
    let task = crate::util::trace::task(future, "task", name, id.as_u64());

    match context::with_current(|handle| handle.spawn(task, id)) {
        Ok(join_handle) => join_handle,
        Err(e) => panic!("{}", e),
    }
}
```

tokio\src\runtime\scheduler\mod.rs
```rust
impl Handle {
    pub(crate) fn spawn<F>(&self, future: F, id: Id) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        match self {
            Handle::CurrentThread(h) => current_thread::Handle::spawn(h, future, id),

            #[cfg(feature = "rt-multi-thread")]
            Handle::MultiThread(h) => multi_thread::Handle::spawn(h, future, id),

            #[cfg(all(tokio_unstable, feature = "rt-multi-thread"))]
            Handle::MultiThreadAlt(h) => multi_thread_alt::Handle::spawn(h, future, id),
        }
    }
}
```

tokio\src\runtime\scheduler\multi_thread\handle.rs
```rust
impl Handle {
    /// Spawns a future onto the thread pool
    pub(crate) fn spawn<F>(me: &Arc<Self>, future: F, id: task::Id) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Self::bind_new_task(me, future, id)
    }

    pub(super) fn bind_new_task<T>(me: &Arc<Self>, future: T, id: task::Id) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let (handle, notified) = me.shared.owned.bind(future, me.clone(), id);

        me.schedule_option_task_without_yield(notified);

        handle
    }
}
```

* 在根 Future 中被 poll 或 .await 的 Future
