use tokio::runtime::Runtime;

pub fn test() {
    test1();
}

fn test1() {
    let rt = Runtime::new().unwrap();
    let _enter = rt.enter();
    // there is no reactor running, must be called from the context of a Tokio 1.x runtime
    tokio::spawn(async {});
}
