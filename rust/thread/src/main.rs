use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![10, 9, 8, 7];
    let handle = thread::spawn(move || {
        println!("child: {:?}", v);
        for i in 1..10 {
            println!("child: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //drop(v);

    for i in 1..5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
