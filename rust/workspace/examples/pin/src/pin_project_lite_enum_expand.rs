#![feature(prelude_import)]
#![allow(dead_code)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
mod pin_project_lite_enum {
    use pin_project_lite::pin_project;
    pub enum Flatten<Fut1, Fut2> {
        First { f: Fut1 },
        Second { f: Fut2 },
        Empty,
    }
    #[automatically_derived]
    impl<Fut1: ::core::fmt::Debug, Fut2: ::core::fmt::Debug> ::core::fmt::Debug
    for Flatten<Fut1, Fut2> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Flatten::First { f: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "First",
                        "f",
                        &__self_0,
                    )
                }
                Flatten::Second { f: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Second",
                        "f",
                        &__self_0,
                    )
                }
                Flatten::Empty => ::core::fmt::Formatter::write_str(f, "Empty"),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        dead_code,
        single_use_lifetimes,
        clippy::unknown_clippy_lints,
        clippy::absolute_paths,
        clippy::min_ident_chars,
        clippy::mut_mut,
        clippy::redundant_pub_crate,
        clippy::ref_option_ref,
        clippy::single_char_lifetime_names,
        clippy::type_repetition_in_bounds
    )]
    pub(crate) enum FlattenProj<'__pin, Fut1, Fut2>
    where
        Flatten<Fut1, Fut2>: '__pin,
    {
        First { f: ::pin_project_lite::__private::Pin<&'__pin mut (Fut1)> },
        Second { f: ::pin_project_lite::__private::Pin<&'__pin mut (Fut2)> },
        Empty,
    }
    #[allow(
        single_use_lifetimes,
        clippy::unknown_clippy_lints,
        clippy::absolute_paths,
        clippy::min_ident_chars,
        clippy::single_char_lifetime_names,
        clippy::used_underscore_binding
    )]
    const _: () = {
        impl<Fut1, Fut2> Flatten<Fut1, Fut2> {
            #[doc(hidden)]
            #[inline]
            pub(crate) fn project<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
            ) -> FlattenProj<'__pin, Fut1, Fut2> {
                unsafe {
                    match self.get_unchecked_mut() {
                        Self::First { f } => {
                            FlattenProj::First {
                                f: ::pin_project_lite::__private::Pin::new_unchecked(f),
                            }
                        }
                        Self::Second { f } => {
                            FlattenProj::Second {
                                f: ::pin_project_lite::__private::Pin::new_unchecked(f),
                            }
                        }
                        Self::Empty => FlattenProj::Empty,
                    }
                }
            }
        }
        #[allow(non_snake_case)]
        pub struct __Origin<'__pin, Fut1, Fut2> {
            __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
            First: (Fut1),
            Second: (Fut2),
            Empty: (),
        }
        impl<'__pin, Fut1, Fut2> ::pin_project_lite::__private::Unpin
        for Flatten<Fut1, Fut2>
        where
            ::pin_project_lite::__private::PinnedFieldsOf<
                __Origin<'__pin, Fut1, Fut2>,
            >: ::pin_project_lite::__private::Unpin,
        {}
        trait MustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
        impl<Fut1, Fut2> MustNotImplDrop for Flatten<Fut1, Fut2> {}
    };
    pub fn test() {}
}
use pin_project_lite_enum as test;
fn main() {
    test::test();
}
