use serde::{
    de::{Error, Visitor},
    Deserializer,
};
use std::fmt;

#[test]
fn test_deserialize_bool() {
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let mut d = serde_json::Deserializer::from_str("false");
    let res = d.deserialize_bool(BoolVisitor);
    assert!(!res.unwrap());

    let mut d = serde_json::Deserializer::from_str("true");
    let res = d.deserialize_bool(BoolVisitor);
    assert!(res.unwrap());

    let mut d = serde_json::Deserializer::from_str("");
    let res = d.deserialize_bool(BoolVisitor);
    println!("{}", res.unwrap_err());

    let mut d = serde_json::Deserializer::from_str(" ");
    let res = d.deserialize_bool(BoolVisitor);
    println!("{}", res.unwrap_err());

    let mut d = serde_json::Deserializer::from_str("a");
    let res = d.deserialize_bool(BoolVisitor);
    println!("{}", res.unwrap_err());

    // D:\github\serde-rs\json\src\de.rs:peek_invalid_type
    // D:\github\serde-rs\serde\serde\src\de\mod.rs:invalid_type
    // D:\github\serde-rs\serde\serde\src\de\mod.rs:trait Expected
    let mut d = serde_json::Deserializer::from_str("[");
    let res = d.deserialize_bool(BoolVisitor);
    println!("{}", res.unwrap_err());
}
