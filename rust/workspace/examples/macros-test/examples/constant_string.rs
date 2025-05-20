use macro_examples::constant_string;

constant_string!("SOME_CONSTANT_STRING_VALUE");

fn main() {
    println!("{}", SOME_CONSTANT_STRING_VALUE);
}
