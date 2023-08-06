use serde::Serialize;
use serde_json::Result;

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
}

fn main() -> Result<()> {
    let address = Address {
        street: "Caobao".to_string(),
        city: "Shanghai".to_string(),
    };

    let j = serde_json::to_string(&address)?;
    println!("{}", j);
    Ok(())
}
