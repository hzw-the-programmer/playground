#[tokio::main]
async fn main() {
    let handle = tokio::spawn(process());
    match handle.await {
        Ok(ok) => println!("main Ok: {:?}", ok),
        Err(err) => println!("main Err: {:?}", err),
    }
}

async fn process() -> Result<String, String> {
    //Ok("success".into())
    //Err("failed".into())
    panic!("panic");
}
