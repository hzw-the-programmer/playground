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
            let mut implementation = quote! {
                let mut map = std::collections::HashMap::<String, String>::new();
            };

            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                implementation.extend(quote!{
                    map.insert(stringify!(#identifier).to_string(), String::from(value.#identifier));
                });
            }

            quote! {
                #[automatically_derived]
                impl From<#struct_identifier> for std::collections::HashMap<String, String> {
                    fn from(value: #struct_identifier) -> Self {
                        #implementation

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
