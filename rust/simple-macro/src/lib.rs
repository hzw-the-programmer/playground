use proc_macro::TokenStream;

#[proc_macro_derive(A)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    assert_eq!(input, "struct A;");
    r#"
        impl A {
            fn a(&self) -> String {
                format!("hello from impl A")
            }
        }
    "#
    .parse()
    .unwrap()
}

#[proc_macro_attribute]
pub fn attr_with_args(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    assert_eq!("\"Hello Rust!\"", args);
    let input = input.to_string();
    assert_eq!("fn foo() { println!(\"hhh\"); }", input);
    r#"
        fn foo() -> String { "Hello Rust!".to_string() }
    "#
    .parse()
    .unwrap()
}
