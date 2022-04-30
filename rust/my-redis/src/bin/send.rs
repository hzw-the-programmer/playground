use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    /*
    let rc = Rc::new("hello");
    yield_now().await;
    println!("{}", rc);
    */
    let handle = tokio::spawn(async {
        //let rc = Rc::new("hello");
        {
            let rc = Rc::new("hello");
        }
        yield_now().await;
        //println!("{}", rc);
    });
    handle.await;
}
