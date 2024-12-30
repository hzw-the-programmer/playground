use std::thread;

pub fn test() {
    test1();
}

fn test1() {
    let mut x = 0;
    // closure may outlive the current function, but it borrows `x`, which is owned by the current function
    // let jh = thread::spawn(|| {
    //     x += 1;
    // });
    // jh.join();

    unsafe {
        // use of unstable library feature 'thread_spawn_unchecked'
        let jh = thread::Builder::new().spawn_unchecked(|| {
            x += 1;
        });
        jh.unwrap().join().unwrap();
    }
}
