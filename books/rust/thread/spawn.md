如下代码是安全的，但编译器报错：
closure may outlive the current function, but it borrows `x`, which is owned by the current function
```rust
fn test1() {
    let mut x = 0;
    let jh = thread::spawn(|| {
        x += 1;
    });
    jh.join();
}
```
闭包引用了x，它的生命周期与x一样，即从x定义到test1结束。
spawn要求闭包拥有'static生命周期：
```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}
```
'static生命周期从程序开始执行到结束。
x的生命周期在test1函数返回时结束，显然没有'static长。
所以编译器报错。从编译器的角度看，它已经做到最好了。
从人的角度看，代码是安全的：
新线程执行过程中，test1函数在join处阻塞，没有返回，x一直在test1函数的栈中，新线程可以安全访问x。
这个安全性是可以由程序员保证的。所以标准库提供了另一个函数：
```rust
impl Builder {
    #[unstable(feature = "thread_spawn_unchecked", issue = "55132")]
    pub unsafe fn spawn_unchecked<'a, F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'a,
        T: Send + 'a,
    {
        Ok(JoinHandle(unsafe { self.spawn_unchecked_(f, None) }?))
    }
}
```
该函数要求闭包有'a周期，即调用时，闭包有什么生命周期，'a就是那个生命周期。
不要求闭包有'static周期。
该函数是不安全的，编译器不负责保证安全性，由程序员保证安全性。
该函数还是不稳定的feature。如果要调用该函数，需要加crate属性：#![feature(thread_spawn_unchecked)]
由程序员保证正确性的做法：
```rust
#![feature(thread_spawn_unchecked)]

fn test1() {
    let mut x = 0;
    unsafe {
        let jh = thread::Builder::new().spawn_unchecked(|| {
            x += 1;
        });
        jh.unwrap().join().unwrap();
    }
}
```
