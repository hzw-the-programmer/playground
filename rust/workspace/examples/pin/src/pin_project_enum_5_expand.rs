#![feature(prelude_import)]
#![allow(dead_code)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
mod pin_project_enum_5 {
    use pin_project::pin_project;
    /// Internal Map future
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[pin(__private(project = MapProj, project_replace = MapProjReplace))]
    pub enum Map<Fut, F> {
        Incomplete { #[pin] future: Fut, f: F },
        Complete,
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
    pub(crate) enum MapProj<'pin, Fut, F>
    where
        Map<Fut, F>: 'pin,
    {
        Incomplete {
            future: ::pin_project::__private::Pin<&'pin mut (Fut)>,
            f: &'pin mut (F),
        },
        Complete,
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
        variant_size_differences,
        clippy::large_enum_variant,
        clippy::missing_docs_in_private_items
    )]
    pub(crate) enum MapProjReplace<Fut, F> {
        Incomplete { future: ::pin_project::__private::PhantomData<Fut>, f: F },
        Complete,
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
        impl<Fut, F> Map<Fut, F> {
            #[allow(dead_code)]
            #[inline]
            pub(crate) fn project<'pin>(
                self: _pin_project::__private::Pin<&'pin mut Self>,
            ) -> MapProj<'pin, Fut, F> {
                unsafe {
                    match self.get_unchecked_mut() {
                        Self::Incomplete { future, f } => {
                            MapProj::Incomplete {
                                future: _pin_project::__private::Pin::new_unchecked(future),
                                f,
                            }
                        }
                        Self::Complete => MapProj::Complete,
                    }
                }
            }
            #[allow(dead_code)]
            #[inline]
            pub(crate) fn project_replace(
                self: _pin_project::__private::Pin<&mut Self>,
                __replacement: Self,
            ) -> MapProjReplace<Fut, F> {
                unsafe {
                    let __self_ptr: *mut Self = self.get_unchecked_mut();
                    let __guard = _pin_project::__private::UnsafeOverwriteGuard::new(
                        __self_ptr,
                        __replacement,
                    );
                    match &mut *__self_ptr {
                        Self::Incomplete { future, f } => {
                            let __result = MapProjReplace::Incomplete {
                                future: _pin_project::__private::PhantomData,
                                f: _pin_project::__private::ptr::read(f),
                            };
                            {
                                let __guard = _pin_project::__private::UnsafeDropInPlaceGuard::new(
                                    future,
                                );
                            }
                            __result
                        }
                        Self::Complete => {
                            let __result = MapProjReplace::Complete;
                            {}
                            __result
                        }
                    }
                }
            }
        }
        #[allow(missing_debug_implementations, unnameable_types)]
        pub struct __Map<'pin, Fut, F> {
            __pin_project_use_generics: _pin_project::__private::AlwaysUnpin<
                'pin,
                (
                    _pin_project::__private::PhantomData<Fut>,
                    _pin_project::__private::PhantomData<F>,
                ),
            >,
            __field0: Fut,
        }
        impl<'pin, Fut, F> _pin_project::__private::Unpin for Map<Fut, F>
        where
            _pin_project::__private::PinnedFieldsOf<
                __Map<'pin, Fut, F>,
            >: _pin_project::__private::Unpin,
        {}
        #[doc(hidden)]
        unsafe impl<'pin, Fut, F> _pin_project::UnsafeUnpin for Map<Fut, F>
        where
            _pin_project::__private::PinnedFieldsOf<
                __Map<'pin, Fut, F>,
            >: _pin_project::__private::Unpin,
        {}
        trait MapMustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: _pin_project::__private::Drop> MapMustNotImplDrop for T {}
        impl<Fut, F> MapMustNotImplDrop for Map<Fut, F> {}
        #[doc(hidden)]
        impl<Fut, F> _pin_project::__private::PinnedDrop for Map<Fut, F> {
            unsafe fn drop(self: _pin_project::__private::Pin<&mut Self>) {}
        }
    };
    pub fn test() {}
}
use pin_project_enum_5 as test;
fn main() {
    test::test();
}
