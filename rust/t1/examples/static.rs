use rand;

static mut SHUTDOWN: bool = false;

fn main() {
    loop {
        unsafe {
            SHUTDOWN = rand::random();
        }

        println!(".");

        if unsafe { SHUTDOWN } {
            break;
        }
    }
}
