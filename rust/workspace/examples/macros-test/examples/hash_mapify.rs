use macro_examples::hash_mapify;

fn main() {
    test_hashmap();
}

fn test_hashmap() {
    let some_variable = "Some variable value";

    let hash_map = hash_mapify!(
        &str,
        "first_key" = "first_value",
        "second_variable" = some_variable,
        some_key = "value for variable key",
    );

    let number_hash_map =
        hash_mapify!(usize, "first_key" = 1, "second_variable" = 2, some_key = 3,);

    dbg!(hash_map);
    dbg!(number_hash_map);
}
