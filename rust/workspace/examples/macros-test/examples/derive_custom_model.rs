use macro_examples::{DeriveCustomModel, IntoStringHashMap};
use std::collections::HashMap;

#[derive(DeriveCustomModel)]
#[custom_model(model(
    name = "UserName",
    fields(first_name, last_name),
    extra_derives(IntoStringHashMap)
))]
#[custom_model(model(name = "UserInfo", fields(username, age), extra_derives(Debug)))]
pub struct User2 {
    username: String,
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    let user_name = UserName {
        first_name: "first_name".to_string(),
        last_name: "last_name".to_string(),
    };
    let hash_map = HashMap::<String, String>::from(user_name);

    dbg!(hash_map);

    let user_info = UserInfo {
        username: "username".to_string(),
        age: 27,
    };

    dbg!(user_info);
}
