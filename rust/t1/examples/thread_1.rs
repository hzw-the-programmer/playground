use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::Builder::new()
        .name("thread 1".to_string())
        .spawn(move || {
            let foo1 = Foo { id: 1 };
            let foo2 = Foo { id: 2 };
            let _foo3 = Foo { id: 3 };
            println!("{:?}: before send", thread::current().id());
            tx.send(foo1).unwrap();
            println!("{:?}: after send", thread::current().id());
            foo2
        })
        .unwrap();

    println!("{:?}: before recv", thread::current().id());
    {
        let _foo = rx.recv().unwrap();
    }
    println!("{:?}: after recv", thread::current().id());

    let _res = handle.join().unwrap();
    println!("{:?}: finish", thread::current().id());
}

struct Foo {
    id: u64,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?}: Foo {} droped", thread::current().id(), self.id);
    }
}
