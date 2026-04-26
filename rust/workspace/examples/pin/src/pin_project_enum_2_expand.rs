#![feature(prelude_import)]
#![allow(dead_code)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
mod pin_project_enum_2 {
    use pin_project::pin_project;
    #[pin(__private(project = FlattenProj))]
    pub enum Flatten<Fut1, Fut2> {
        First { #[pin] f: Fut1 },
        Second { #[pin] f: Fut2 },
        Empty,
    }
    #[allow(
        dead_code,
        deprecated,
        explicit_outlives_requirements,
        single_use_lifetimes,
        unreachable_pub,
        unused_tuple_struct_fields,
        clippy::unknown_clippy_lints,
        clippy::absolute_paths,
        clippy::min_ident_chars,
        clippy::pattern_type_mismatch,
        clippy::pub_with_shorthand,
        clippy::redundant_pub_crate,
        clippy::single_char_lifetime_names,
        clippy::type_repetition_in_bounds,
        clippy::missing_docs_in_private_items,
        clippy::mut_mut
    )]
    pub(crate) enum FlattenProj<'pin, Fut1, Fut2>
    where
        Flatten<Fut1, Fut2>: 'pin,
    {
        First { f: ::pin_project::__private::Pin<&'pin mut (Fut1)> },
        Second { f: ::pin_project::__private::Pin<&'pin mut (Fut2)> },
        Empty,
    }
    #[allow(
        unused_qualifications,
        deprecated,
        explicit_outlives_requirements,
        single_use_lifetimes,
        unreachable_pub,
        unused_tuple_struct_fields,
        clippy::unknown_clippy_lints,
        clippy::absolute_paths,
        clippy::min_ident_chars,
        clippy::pattern_type_mismatch,
        clippy::pub_with_shorthand,
        clippy::redundant_pub_crate,
        clippy::single_char_lifetime_names,
        clippy::type_repetition_in_bounds,
        clippy::elidable_lifetime_names,
        clippy::missing_const_for_fn,
        clippy::needless_lifetimes,
        clippy::semicolon_if_nothing_returned,
        clippy::use_self,
        clippy::used_underscore_binding
    )]
    const _: () = {
        #[allow(unused_extern_crates)]
        extern crate pin_project as _pin_project;
        impl<Fut1, Fut2> Flatten<Fut1, Fut2> {
            #[allow(dead_code)]
            #[inline]
            pub(crate) fn project<'pin>(
                self: _pin_project::__private::Pin<&'pin mut Self>,
            ) -> FlattenProj<'pin, Fut1, Fut2> {
                unsafe {
                    match self.get_unchecked_mut() {
                        Self::First { f } => {
                            FlattenProj::First {
                                f: _pin_project::__private::Pin::new_unchecked(f),
                            }
                        }
                        Self::Second { f } => {
                            FlattenProj::Second {
                                f: _pin_project::__private::Pin::new_unchecked(f),
                            }
                        }
                        Self::Empty => FlattenProj::Empty,
                    }
                }
            }
        }
        #[allow(missing_debug_implementations, unnameable_types)]
        pub struct __Flatten<'pin, Fut1, Fut2> {
            __pin_project_use_generics: _pin_project::__private::AlwaysUnpin<
                'pin,
                (
                    _pin_project::__private::PhantomData<Fut1>,
                    _pin_project::__private::PhantomData<Fut2>,
                ),
            >,
            __field0: Fut1,
            __field1: Fut2,
        }
        impl<'pin, Fut1, Fut2> _pin_project::__private::Unpin for Flatten<Fut1, Fut2>
        where
            _pin_project::__private::PinnedFieldsOf<
                __Flatten<'pin, Fut1, Fut2>,
            >: _pin_project::__private::Unpin,
        {}
        #[doc(hidden)]
        unsafe impl<'pin, Fut1, Fut2> _pin_project::UnsafeUnpin for Flatten<Fut1, Fut2>
        where
            _pin_project::__private::PinnedFieldsOf<
                __Flatten<'pin, Fut1, Fut2>,
            >: _pin_project::__private::Unpin,
        {}
        trait FlattenMustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: _pin_project::__private::Drop> FlattenMustNotImplDrop for T {}
        impl<Fut1, Fut2> FlattenMustNotImplDrop for Flatten<Fut1, Fut2> {}
        #[doc(hidden)]
        impl<Fut1, Fut2> _pin_project::__private::PinnedDrop for Flatten<Fut1, Fut2> {
            unsafe fn drop(self: _pin_project::__private::Pin<&mut Self>) {}
        }
    };
    pub fn test() {}
}
use pin_project_enum_2 as test;
fn main() {
    test::test();
}
