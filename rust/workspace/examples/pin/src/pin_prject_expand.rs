#![feature(prelude_import)]
#![allow(dead_code)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
mod pin_project {
    use pin_project::pin_project;
    struct Foo {
        f1: i32,
        f2: i32,
    }
    struct Bar {
        b1: i32,
        b2: i32,
    }
    #[pin(__private())]
    struct Baz {
        foo: Foo,
        #[pin]
        bar: Bar,
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
        #[allow(dead_code, clippy::missing_docs_in_private_items, clippy::mut_mut)]
        struct __BazProjection<'pin>
        where
            Baz: 'pin,
        {
            foo: &'pin mut (Foo),
            bar: ::pin_project::__private::Pin<&'pin mut (Bar)>,
        }
        #[allow(
            dead_code,
            clippy::missing_docs_in_private_items,
            clippy::ref_option_ref
        )]
        struct __BazProjectionRef<'pin>
        where
            Baz: 'pin,
        {
            foo: &'pin (Foo),
            bar: ::pin_project::__private::Pin<&'pin (Bar)>,
        }
        impl Baz {
            #[allow(dead_code)]
            #[inline]
            fn project<'pin>(
                self: _pin_project::__private::Pin<&'pin mut Self>,
            ) -> __BazProjection<'pin> {
                unsafe {
                    let Self { foo, bar } = self.get_unchecked_mut();
                    __BazProjection {
                        foo,
                        bar: _pin_project::__private::Pin::new_unchecked(bar),
                    }
                }
            }
            #[allow(dead_code)]
            #[inline]
            fn project_ref<'pin>(
                self: _pin_project::__private::Pin<&'pin Self>,
            ) -> __BazProjectionRef<'pin> {
                unsafe {
                    let Self { foo, bar } = self.get_ref();
                    __BazProjectionRef {
                        foo,
                        bar: _pin_project::__private::Pin::new_unchecked(bar),
                    }
                }
            }
        }
        #[forbid(unaligned_references, safe_packed_borrows)]
        fn __assert_not_repr_packed(this: &Baz) {
            let _ = &this.foo;
            let _ = &this.bar;
        }
        #[allow(missing_debug_implementations, unnameable_types)]
        struct __Baz<'pin> {
            __pin_project_use_generics: _pin_project::__private::AlwaysUnpin<'pin, ()>,
            __field0: Bar,
        }
        impl<'pin> _pin_project::__private::Unpin for Baz
        where
            _pin_project::__private::PinnedFieldsOf<
                __Baz<'pin>,
            >: _pin_project::__private::Unpin,
        {}
        #[doc(hidden)]
        unsafe impl<'pin> _pin_project::UnsafeUnpin for Baz
        where
            _pin_project::__private::PinnedFieldsOf<
                __Baz<'pin>,
            >: _pin_project::__private::Unpin,
        {}
        trait BazMustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: _pin_project::__private::Drop> BazMustNotImplDrop for T {}
        impl BazMustNotImplDrop for Baz {}
        #[doc(hidden)]
        impl _pin_project::__private::PinnedDrop for Baz {
            unsafe fn drop(self: _pin_project::__private::Pin<&mut Self>) {}
        }
    };
    pub fn test() {}
}
use pin_project as test;
fn main() {
    test::test();
}
