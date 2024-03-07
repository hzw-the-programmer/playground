D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\builder.rs
```
fn build_threaded_runtime(&mut self) -> io::Result<Runtime>
// Spawn the thread pool workers
let _enter = handle.enter();
launch.launch();
```

D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\scheduler\multi_thread\worker.rs
```
impl Launch {
    pub(crate) fn launch(mut self) {
        for worker in self.0.drain(..) {
            runtime::spawn_blocking(move || run(worker));
        }
    }
}
```

D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\runtime.rs
```
pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    self.handle.spawn_blocking(func)
}
```

D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\handle.rs
```
pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    self.inner.blocking_spawner().spawn_blocking(self, func)
}
```

D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\scheduler\mod.rs
```
pub(crate) fn blocking_spawner(&self) -> &blocking::Spawner {
    match_flavor!(self, Handle(h) => &h.blocking_spawner)
}
```

D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\scheduler\multi_thread\handle.rs
```
/// Handle to the multi thread scheduler
pub(crate) struct Handle {
    /// Task spawner
    pub(super) shared: worker::Shared,

    /// Resource driver handles
    pub(crate) driver: driver::Handle,

    /// Blocking pool spawner
    pub(crate) blocking_spawner: blocking::Spawner,

    /// Current random number generator seed
    pub(crate) seed_generator: RngSeedGenerator,
}
```

D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\blocking\pool.rs
```
pub(crate) struct Spawner {
    inner: Arc<Inner>,
}

impl Spawner {
    pub(crate) fn spawn_blocking<F, R>(&self, rt: &Handle, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
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
    }

    fn spawn_task(&self, task: Task, rt: &Handle) -> Result<(), SpawnError> {
    }

    fn spawn_thread(
        &self,
        shutdown_tx: shutdown::Sender,
        rt: &Handle,
        id: usize,
    ) -> std::io::Result<thread::JoinHandle<()>> {
    }
}

impl Inner {
    fn run(&self, worker_thread_id: usize) {
    }
}
```
