use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

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
