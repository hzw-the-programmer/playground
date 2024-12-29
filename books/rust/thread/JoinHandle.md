std\src\thread\mod.rs
```rust
pub struct JoinHandle<T>(JoinInner<'static, T>);
impl<T> JoinHandle<T> {
    pub fn join(self) -> Result<T> {
        self.0.join()
    }
}

struct JoinInner<'scope, T> {
    native: imp::Thread,
    thread: Thread,
    packet: Arc<Packet<'scope, T>>,
}
impl<'scope, T> JoinInner<'scope, T> {
    fn join(mut self) -> Result<T> {
        self.native.join();
        Arc::get_mut(&mut self.packet).unwrap().result.get_mut().take().unwrap()
    }
}

struct Packet<'scope, T> {
    scope: Option<Arc<scoped::ScopeData>>,
    result: UnsafeCell<Option<Result<T>>>,
    _marker: PhantomData<Option<&'scope scoped::ScopeData>>,
}
impl<'scope, T> Drop for Packet<'scope, T> {
    fn drop(&mut self) {
        let unhandled_panic = matches!(self.result.get_mut(), Some(Err(_)));
        if let Err(_) = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            *self.result.get_mut() = None;
        })) {
            rtabort!("thread result panicked on drop");
        }
        if let Some(scope) = &self.scope {
            scope.decrement_num_running_threads(unhandled_panic);
        }
    }
}

impl Builder {
    pub fn spawn<F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        unsafe { self.spawn_unchecked(f) }
    }
    
    pub unsafe fn spawn_unchecked<'a, F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'a,
        T: Send + 'a,
    {
        Ok(JoinHandle(unsafe { self.spawn_unchecked_(f, None) }?))
    }
    
    unsafe fn spawn_unchecked_<'a, 'scope, F, T>(
        self,
        f: F,
        scope_data: Option<Arc<scoped::ScopeData>>,
    ) -> io::Result<JoinInner<'scope, T>>
    where
        F: FnOnce() -> T,
        F: Send + 'a,
        T: Send + 'a,
        'scope: 'a,
    {
        let Builder { name, stack_size } = self;

        let stack_size = stack_size.unwrap_or_else(thread::min_stack);

        let my_thread = Thread::new(name.map(|name| {
            CString::new(name).expect("thread name may not contain interior null bytes")
        }));
        let their_thread = my_thread.clone();

        let my_packet: Arc<Packet<'scope, T>> = Arc::new(Packet {
            scope: scope_data,
            result: UnsafeCell::new(None),
            _marker: PhantomData,
        });
        let their_packet = my_packet.clone();

        let output_capture = crate::io::set_output_capture(None);
        crate::io::set_output_capture(output_capture.clone());

        // Pass `f` in `MaybeUninit` because actually that closure might *run longer than the lifetime of `F`*.
        // See <https://github.com/rust-lang/rust/issues/101983> for more details.
        // To prevent leaks we use a wrapper that drops its contents.
        #[repr(transparent)]
        struct MaybeDangling<T>(mem::MaybeUninit<T>);
        impl<T> MaybeDangling<T> {
            fn new(x: T) -> Self {
                MaybeDangling(mem::MaybeUninit::new(x))
            }
            fn into_inner(self) -> T {
                // SAFETY: we are always initialized.
                let ret = unsafe { self.0.assume_init_read() };
                // Make sure we don't drop.
                mem::forget(self);
                ret
            }
        }
        impl<T> Drop for MaybeDangling<T> {
            fn drop(&mut self) {
                // SAFETY: we are always initialized.
                unsafe { self.0.assume_init_drop() };
            }
        }

        let f = MaybeDangling::new(f);
        let main = move || {
            if let Some(name) = their_thread.cname() {
                imp::Thread::set_name(name);
            }

            crate::io::set_output_capture(output_capture);

            // SAFETY: we constructed `f` initialized.
            let f = f.into_inner();
            // SAFETY: the stack guard passed is the one for the current thread.
            // This means the current thread's stack and the new thread's stack
            // are properly set and protected from each other.
            thread_info::set(unsafe { imp::guard::current() }, their_thread);
            let try_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                crate::sys_common::backtrace::__rust_begin_short_backtrace(f)
            }));
            // SAFETY: `their_packet` as been built just above and moved by the
            // closure (it is an Arc<...>) and `my_packet` will be stored in the
            // same `JoinInner` as this closure meaning the mutation will be
            // safe (not modify it and affect a value far away).
            unsafe { *their_packet.result.get() = Some(try_result) };
            // Here `their_packet` gets dropped, and if this is the last `Arc` for that packet that
            // will call `decrement_num_running_threads` and therefore signal that this thread is
            // done.
            drop(their_packet);
            // Here, the lifetime `'a` and even `'scope` can end. `main` keeps running for a bit
            // after that before returning itself.
        };

        if let Some(scope_data) = &my_packet.scope {
            scope_data.increment_num_running_threads();
        }

        let main = Box::new(main);
        // SAFETY: dynamic size and alignment of the Box remain the same. See below for why the
        // lifetime change is justified.
        let main = unsafe { Box::from_raw(Box::into_raw(main) as *mut (dyn FnOnce() + 'static)) };

        Ok(JoinInner {
            // SAFETY:
            //
            // `imp::Thread::new` takes a closure with a `'static` lifetime, since it's passed
            // through FFI or otherwise used with low-level threading primitives that have no
            // notion of or way to enforce lifetimes.
            //
            // As mentioned in the `Safety` section of this function's documentation, the caller of
            // this function needs to guarantee that the passed-in lifetime is sufficiently long
            // for the lifetime of the thread.
            //
            // Similarly, the `sys` implementation must guarantee that no references to the closure
            // exist after the thread has terminated, which is signaled by `Thread::join`
            // returning.
            native: unsafe { imp::Thread::new(stack_size, main)? },
            thread: my_thread,
            packet: my_packet,
        })
    }
}
```
