use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }];

        for foo in foos {
            tx.send(foo).unwrap();
            println!("{:?}: after send", thread::current().id());
            thread::sleep(Duration::from_secs(1));
        }

        Foo { id: 4 }
    });

    for _foo in rx {
        println!("{:?}: recv", thread::current().id());
    }

    let _res = handle.join().unwrap();
    println!("{:?}: finished", thread::current().id());
}

struct Foo {
    id: u64,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?}: Foo {} dropped", thread::current().id(), self.id);
    }
}
