use std::sync::mpsc::channel;
use std::thread;

fn main() {
    test1();
    test2();
    test3();
    test4();
}

fn test1() {
    print!("\ntest1\n\n");

    let (sender, receiver) = channel();
    let sender2 = sender.clone();

    thread::spawn(move || {
        sender.send(1).unwrap();
    });

    thread::spawn(move || {
        sender2.send(2).unwrap();
    });

    let msg = receiver.recv().unwrap();
    let msg2 = receiver.recv().unwrap();
    assert_eq!(3, msg + msg2);

    println!("finish");
}

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

fn test2() {
    print!("\ntest2\n\n");

    let (sender, receiver) = channel();
    thread::spawn(move || {
        let msg = Foo { id: 1 };
        sender.send(1).unwrap();
    });

    println!("before recv");
    let msg = receiver.recv().unwrap();

    println!("finish");
}

fn test3() {
    print!("\ntest3\n\n");

    let (sender, receiver) = channel();
    thread::spawn(move || {
        let msg = Foo { id: 1 };
        println!("{:016p}", &msg);
        println!("{:016p}", &msg.id);
        sender.send(msg).unwrap();
    });

    println!("before recv");
    let msg = receiver.recv().unwrap();
    println!("{:016p}", &msg);
    println!("{:016p}", &msg.id);

    println!("finish");
}

fn test4() {
    print!("\ntest4\n\n");

    let msg = Foo { id: 1 };
    println!("{:016p}", &msg);

    let (sender, receiver) = channel();
    thread::spawn(move || {
        println!("{:016p}", &msg);
        sender.send(1).unwrap();
    });

    println!("before recv");
    let msg = receiver.recv().unwrap();

    println!("finish");
}
