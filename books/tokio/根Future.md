# Future有两种：根Future；在根Future中poll的Future。
## 根Future，存储在任务的Cell结构中，即cell.core.stage.stage。
tokio\src\runtime\task\core.rs
```rust
pub(super) struct Cell<T: Future, S> {
    pub(super) header: Header,
    pub(super) core: Core<T, S>,
    pub(super) trailer: Trailer,
}
pub(super) struct Core<T: Future, S> {
    pub(super) scheduler: S,
    pub(super) task_id: Id,
    pub(super) stage: CoreStage<T>,
}
pub(super) struct CoreStage<T: Future> {
    stage: UnsafeCell<Stage<T>>,
}
pub(super) enum Stage<T: Future> {
    Running(T),
    Finished(super::Result<T::Output>),
    Consumed,
}
```
### 根Future有两种：
1. 包装闭包的BlockingTask。
tokio\src\task\blocking.rs
```rust
pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    crate::runtime::spawn_blocking(f)
}
```
tokio\src\runtime\mod.rs
```rust
mod blocking;
pub(crate) use blocking::spawn_blocking;
```
tokio\src\runtime\blocking\mod.rs
```rust
mod pool;
pub(crate) use pool::{spawn_blocking, BlockingPool, Spawner};
```
tokio\src\runtime\blocking\pool.rs
```rust
pub(crate) fn spawn_blocking<F, R>(func: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let rt = Handle::current();
    rt.spawn_blocking(func)
}
```
tokio\src\runtime\handle.rs
```rust
impl Handle {
    pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.inner.blocking_spawner().spawn_blocking(self, func)
    }
}
```
tokio\src\runtime\scheduler\mod.rs
```rust
impl Handle {
    pub(crate) fn blocking_spawner(&self) -> &blocking::Spawner {
        match_flavor!(self, Handle(h) => &h.blocking_spawner)
    }
}
```
tokio\src\runtime\blocking\pool.rs
```rust
impl Spawner {
    pub(crate) fn spawn_blocking<F, R>(&self, rt: &Handle, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.spawn_blocking_inner(func, Mandatory::NonMandatory, None, rt)
    }
    pub(crate) fn spawn_blocking_inner<F, R>(
        &self,
        func: F,
        is_mandatory: Mandatory,
        name: Option<&str>,
        rt: &Handle,
    ) -> (JoinHandle<R>, Result<(), SpawnError>)
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let fut = BlockingTask::new(func);
        let id = task::Id::next();
        let (task, handle) = task::unowned(fut, BlockingSchedule::new(rt), id);
        let spawned = self.spawn_task(Task::new(task, is_mandatory), rt);
        (handle, spawned)
    }
}
```

2. 用户定义的根Future，即传给task::spawn的Future。

```rust

```
