fn main() {
    test1();
    test2();
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn test1() {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

use serde::ser::SerializeStruct;
use serde::Serializer;

struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Person", 3)?;
        s.serialize_field("NAME", &self.name)?;
        s.serialize_field("aGe", &self.age)?;
        s.serialize_field("phones", &self.phones)?;
        s.end()
    }
}

fn test2() {
    let p = Person {
        name: "hzw".to_string(),
        age: 36,
        phones: vec!["1234".to_string(), "5678".to_string()],
    };
    let serialized = serde_json::to_string(&p).unwrap();
    println!("serialized = {}", serialized);
}
