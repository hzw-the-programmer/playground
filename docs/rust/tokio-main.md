C:\Users\Admin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-macros-2.2.0\src\entry.rs
```
fn parse_knobs(mut input: ItemFn, is_test: bool, config: FinalConfig) -> TokenStream {
    let mut rt = match config.flavor {
        RuntimeFlavor::CurrentThread => quote_spanned! {last_stmt_start_span=>
            #crate_path::runtime::Builder::new_current_thread()
        },
        RuntimeFlavor::Threaded => quote_spanned! {last_stmt_start_span=>
            #crate_path::runtime::Builder::new_multi_thread()
        },
    };

    let last_block = quote_spanned! {last_stmt_end_span=>
        #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
        {
            return #rt
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(#body_ident);
        }
    };
}
```
