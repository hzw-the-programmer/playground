use macro_examples::IntoStringHashMap;

#[derive(IntoStringHashMap, Debug)]
// #[derive(IntoStringHashMap)]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    // age: u32,
}

fn main() {
    let user = User {
        username: "username".to_string(),
        first_name: "First".to_string(),
        last_name: "Last".to_string(),
    };

    let hash_map = std::collections::HashMap::<String, String>::from(user);

    dbg!(hash_map);
}
