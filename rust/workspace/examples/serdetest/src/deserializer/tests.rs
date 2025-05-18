use serde::{
    de::{Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;
use std::marker::PhantomData;

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

#[test]
fn test_deserialize_seq() {
    struct VecVisitor<T> {
        marker: PhantomData<T>,
    }

    impl<'de, T> Visitor<'de> for VecVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut values = Vec::<T>::new();

            while let Some(value) = seq.next_element()? {
                values.push(value);
            }

            Ok(values)
        }
    }

    let mut d = serde_json::Deserializer::from_str("[1, 2, 3]");
    let visitor: VecVisitor<i32> = VecVisitor {
        marker: PhantomData,
    };
    let res = d.deserialize_seq(visitor);
    assert_eq!(res.unwrap(), vec![1, 2, 3]);
}
