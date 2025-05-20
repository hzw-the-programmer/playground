// https://www.freecodecamp.org/news/procedural-macros-in-rust/

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

mod cached_fn;
mod constant_string;
mod custom_model;
mod hash_mapify;
mod log_duration;

#[proc_macro_derive(IntoStringHashMap)]
pub fn derive_into_hash_map(item: TokenStream) -> TokenStream {
    println!("{item}");

    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    println!("{input:#?}");

    let struct_identifier = input.ident;

    let out = match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let field_identifiers = fields
                .iter()
                .map(|item| item.ident.as_ref().unwrap())
                .collect::<Vec<_>>();

            quote! {
                #[automatically_derived]
                impl From<#struct_identifier> for std::collections::HashMap<String, String> {
                    fn from(value: #struct_identifier) -> Self {
                        let mut map = std::collections::HashMap::<String, String>::new();

                        #(
                            map.insert(stringify!(#field_identifiers).to_string(), String::from(value.#field_identifiers));
                        )*

                        map
                    }
                }
            }
        }
        _ => unimplemented!(),
    };

    println!("");
    println!("{out}");

    let out = out.into();
    println!("");
    println!("{out}");

    out
}

#[proc_macro_derive(DeriveCustomModel, attributes(custom_model))]
pub fn derive_custom_model(item: TokenStream) -> TokenStream {
    custom_model::derive_custom_model_impl(item)
}

#[proc_macro_attribute]
pub fn log_duration(args: TokenStream, item: TokenStream) -> TokenStream {
    log_duration::log_duration_impl(args, item)
}

#[proc_macro_attribute]
pub fn cached_fn(args: TokenStream, item: TokenStream) -> TokenStream {
    cached_fn::cached_fn_impl(args, item)
}

#[proc_macro]
pub fn constant_string(item: TokenStream) -> TokenStream {
    constant_string::constant_string_impl(item)
}

#[proc_macro]
pub fn hash_mapify(item: TokenStream) -> TokenStream {
    hash_mapify::hash_mapify_impl(item)
}
