

D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.50.0\src\net\tcp\listener.rs
```rust
pub struct TcpListener {
    io: PollEvented<mio::net::TcpListener>,
}

pub fn poll_accept(&self, cx: &mut Context<'_>) -> Poll<io::Result<(TcpStream, SocketAddr)>> {
    loop {
        let ev = ready!(self.io.registration().poll_read_ready(cx))?;

        match self.io.accept() {
            Ok((io, addr)) => {
                let io = TcpStream::new(io)?;
                return Poll::Ready(Ok((io, addr)));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                self.io.registration().clear_readiness(ev);
            }
            Err(e) => return Poll::Ready(Err(e)),
        }
    }
}
```

D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.50.0\src\io\poll_evented.rs
```rust
pub(crate) struct PollEvented<E: Source> {
    io: Option<E>,
    registration: Registration,
}
```

D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.50.0\src\runtime\io\registration.rs
```rust
pub(crate) struct Registration {
    /// Handle to the associated runtime.
    ///
    /// TODO: this can probably be moved into `ScheduledIo`.
    handle: scheduler::Handle,

    /// Reference to state stored by the driver.
    shared: Arc<ScheduledIo>,
}

pub(crate) fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<ReadyEvent>> {
    self.poll_ready(cx, Direction::Read)
}

fn poll_ready(
    &self,
    cx: &mut Context<'_>,
    direction: Direction,
) -> Poll<io::Result<ReadyEvent>> {
    ready!(crate::trace::trace_leaf(cx));
    // Keep track of task budget
    let coop = ready!(crate::task::coop::poll_proceed(cx));
    let ev = ready!(self.shared.poll_readiness(cx, direction));

    if ev.is_shutdown {
        return Poll::Ready(Err(gone()));
    }

    coop.made_progress();
    Poll::Ready(Ok(ev))
}
```

D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.50.0\src\runtime\io\scheduled_io.rs
```rust
pub(crate) struct ScheduledIo {
    pub(super) linked_list_pointers: UnsafeCell<linked_list::Pointers<Self>>,

    /// Packs the resource's readiness and I/O driver latest tick.
    readiness: AtomicUsize,

    waiters: Mutex<Waiters>,
}

pub(super) fn poll_readiness(
    &self,
    cx: &mut Context<'_>,
    direction: Direction,
) -> Poll<ReadyEvent> {
    let curr = self.readiness.load(Acquire);

    let ready = direction.mask() & Ready::from_usize(READINESS.unpack(curr));
    let is_shutdown = SHUTDOWN.unpack(curr) != 0;

    if ready.is_empty() && !is_shutdown {
        // Update the task info
        let mut waiters = self.waiters.lock();
        let waker = match direction {
            Direction::Read => &mut waiters.reader,
            Direction::Write => &mut waiters.writer,
        };

        // Avoid cloning the waker if one is already stored that matches the
        // current task.
        match waker {
            Some(waker) => waker.clone_from(cx.waker()),
            None => *waker = Some(cx.waker().clone()),
        }

        // Try again, in case the readiness was changed while we were
        // taking the waiters lock
        let curr = self.readiness.load(Acquire);
        let ready = direction.mask() & Ready::from_usize(READINESS.unpack(curr));
        let is_shutdown = SHUTDOWN.unpack(curr) != 0;
        if is_shutdown {
            Poll::Ready(ReadyEvent {
                tick: TICK.unpack(curr) as u8,
                ready: direction.mask(),
                is_shutdown,
            })
        } else if ready.is_empty() {
            Poll::Pending
        } else {
            Poll::Ready(ReadyEvent {
                tick: TICK.unpack(curr) as u8,
                ready,
                is_shutdown,
            })
        }
    } else {
        Poll::Ready(ReadyEvent {
            tick: TICK.unpack(curr) as u8,
            ready,
            is_shutdown,
        })
    }
}
```

D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.50.0\src\runtime\io\driver.rs
```rust
fn turn(&mut self, handle: &Handle, max_wait: Option<Duration>) {
    debug_assert!(!handle.registrations.is_shutdown(&handle.synced.lock()));

    handle.release_pending_registrations();

    let events = &mut self.events;

    // Block waiting for an event to happen, peeling out how many events
    // happened.
    match self.poll.poll(events, max_wait) {
        Ok(()) => {}
        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
        #[cfg(target_os = "wasi")]
        Err(e) if e.kind() == io::ErrorKind::InvalidInput => {
            // In case of wasm32_wasi this error happens, when trying to poll without subscriptions
            // just return from the park, as there would be nothing, which wakes us up.
        }
        Err(e) => panic!("unexpected error when polling the I/O driver: {e:?}"),
    }

    // Process all the events that came in, dispatching appropriately
    let mut ready_count = 0;
    for event in events.iter() {
        let token = event.token();

        if token == TOKEN_WAKEUP {
            // Nothing to do, the event is used to unblock the I/O driver
        } else if token == TOKEN_SIGNAL {
            self.signal_ready = true;
        } else {
            let ready = Ready::from_mio(event);
            let ptr = super::EXPOSE_IO.from_exposed_addr(token.0);

            // Safety: we ensure that the pointers used as tokens are not freed
            // until they are both deregistered from mio **and** we know the I/O
            // driver is not concurrently polling. The I/O driver holds ownership of
            // an `Arc<ScheduledIo>` so we can safely cast this to a ref.
            let io: &ScheduledIo = unsafe { &*ptr };

            io.set_readiness(Tick::Set, |curr| curr | ready);
            io.wake(ready);

            ready_count += 1;
        }
    }

    #[cfg(all(
        tokio_unstable,
        feature = "io-uring",
        feature = "rt",
        feature = "fs",
        target_os = "linux",
    ))]
    {
        let mut guard = handle.get_uring().lock();
        let ctx = &mut *guard;
        ctx.dispatch_completions();
    }

    handle.metrics.incr_ready_count_by(ready_count);
}
```
