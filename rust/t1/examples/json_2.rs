use serde_json::{Result, Value};

fn main() -> Result<()> {
    untyped_example()?;

    Ok(())
}

fn untyped_example() -> Result<()> {
    let data = r#"{
        "name": "hzw",
        "age": 36,
        "phones": [
            "+86 12345678901",
            "+86 12345678902"
        ]
    }"#;

    let res: Value = serde_json::from_str(data)?;

    println!("Please call {} at the number {}", res["name"], res["phones"][0]);

    Ok(())
}
