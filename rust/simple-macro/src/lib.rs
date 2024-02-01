use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
    format!("fn foo() -> String {{ {}.to_string() }}", args)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn hashmap(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    println!("{input}");
    // let input = input.trim_end_matches(",");
    // println!("{input}");
    let insert = input
        .split(",")
        .filter(|i| i.len() != 0)
        .map(|i| {
            let kv = if i.contains("=>") {
                i.split("=>")
            } else {
                i.split(":")
            };
            let kv = kv.map(|i| i.trim()).collect::<Vec<&str>>();
            // format!("hm.insert({}, {});\n", kv[0], kv[1])
            format!("hm.insert({}, {});\n", kv[0], kv[1])
        })
        // .for_each(|i| println!("{i:?}"));
        // .collect::<String>();
        .collect::<Vec<String>>();
    println!("{insert:?}");

    format!(
        r#"
        {{
            let mut hm = std::collections::HashMap::with_capacity({});
            {}
            hm
        }}
    "#,
        insert.len(),
        insert.into_iter().collect::<String>()
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(New)]
pub fn derive_new(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:?}", input.attrs);
    println!("{:?}", input.vis);
    println!("{:?}", input.ident);
    println!("{:?}", input.generics);
    println!("{:?}", input.data);
    match input.data {
        Data::Struct(ds) => {
            // println!("{ds:?}");
            println!("{:?}", ds.struct_token);
            // println!("{:?}", ds.fields);
            match ds.fields {
                Fields::Named(fin) => {
                    // println!("{fin:?}");
                    // println!("{:?}", fin.named);
                    for f in fin.named {
                        // println!("{f:?}");
                        println!("\nfiled: {}", f.ident.unwrap());
                        println!("{:?}", f.attrs);
                        println!("{:?}", f.vis);
                        println!("{:?}", f.mutability);
                        // println!("{:?}", f.ident);
                        println!("{:?}", f.colon_token);
                        println!("{:?}", f.ty);
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    let struct_name = &input.ident;
    let new = syn::Ident::new("new", proc_macro2::Span::call_site());

    quote! {
        impl #struct_name {
            pub fn #new() -> Self {
                #struct_name { id: 1 }
            }
        }
    }
    .into()
}
