use macro_examples::IntoStringHashMap;

#[derive(IntoStringHashMap)]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {

}