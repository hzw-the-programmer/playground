use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    test1();
    test2();
}

#[derive(Hash)]
struct Rustacean {
    name: String,
    country: String,
}

fn test1() {
    let r = Rustacean {
        name: "hzw".into(),
        country: "CN".into(),
    };

    let mut hasher = DefaultHasher::new();
    r.hash(&mut hasher);
    println!("{:x}", hasher.finish());
}

struct Person {
    id: u32,
    name: String,
    phone: u64,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
        self.phone.hash(hasher);
    }
}

fn test2() {
    let p = Person {
        id: 1,
        name: "hzw".to_string(),
        phone: 123,
    };
    let mut hasher = DefaultHasher::new();
    p.hash(&mut hasher);
    println!("{:x}", hasher.finish());
}
