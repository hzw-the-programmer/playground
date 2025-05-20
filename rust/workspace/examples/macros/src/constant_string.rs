use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{LitStr, parse_macro_input};

pub fn constant_string_impl(item: TokenStream) -> TokenStream {
    println!("{item}");
    println!("\n{item:#?}");

    // Parse input as a string literal
    let constant_value = parse_macro_input!(item as LitStr);

    // Create a new `Ident` (identifier) from the passed string value.
    // This is going to be the name of the constant variable.
    let constant_value_name = Ident::new(&constant_value.value(), Span::call_site());

    // Generate the code for declaring the constant variable.
    let out = quote!(pub const #constant_value_name: &str = #constant_value;);
    println!("\n{out}");
    println!("\n{out:#?}");
    out.into()
}
