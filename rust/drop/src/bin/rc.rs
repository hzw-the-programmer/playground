use drop::Object;
use std::mem;

fn test0() {
    let rc = Rc::new(Object { id: 1 });
    {
        let rc = rc.clone();
        {
            let rc = rc.clone();
            {
                let rc = rc.clone();
                {
                    let rc = rc.clone();
                    println!("{:?}", rc);
                }
                println!("{:?}", rc);
            }
            println!("{:?}", rc);
        }
        println!("{:?}", rc);
    }
    println!("{:?}", rc);
}

fn test1() {}

fn test2() {}

fn test3() {}

fn test4() {}

fn test5() {}
fn test6() {}
fn test7() {}
fn test8() {}
fn test9() {}
fn test10() {}

#[derive(Debug)]
struct Rc<T> {
    ptr: *const T,
    count: i32,
}

impl<T> Rc<T> {
    fn new(t: T) -> Self {
        Rc {
            ptr: Box::into_raw(Box::new(t)),
            count: 1,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        println!("S::drop");
        self.count -= 1;
        if self.count == 0 {
            unsafe {
                Box::from_raw(self.ptr as *mut T);
            }
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            count: self.count + 1,
        }
    }
}

fn main() {
    let tests: Vec<fn()> = vec![
        test0, test1, test2, test3, test4, test5, test6, test7, test8, test9,
        test10, // test11,
               // test12, test13, test14, test15, test16, test17, test18, test19, test20, test21, test22,
               // test23, test24,
    ];

    drop::tests(&tests);
}
