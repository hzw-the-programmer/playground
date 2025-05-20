#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod map {
    pub struct Msg {
        #[prost(map = "string, int32", tag = "7")]
        pub g: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Msg {
        #[inline]
        fn clone(&self) -> Msg {
            Msg {
                g: ::core::clone::Clone::clone(&self.g),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Msg {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Msg {
        #[inline]
        fn eq(&self, other: &Msg) -> bool {
            self.g == other.g
        }
    }
    impl ::prost::Message for Msg {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            ::prost::encoding::hash_map::encode(
                ::prost::encoding::string::encode,
                ::prost::encoding::string::encoded_len,
                ::prost::encoding::int32::encode,
                ::prost::encoding::int32::encoded_len,
                7u32,
                &self.g,
                buf,
            );
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Msg";
            match tag {
                7u32 => {
                    let mut value = &mut self.g;
                    ::prost::encoding::hash_map::merge(
                            ::prost::encoding::string::merge,
                            ::prost::encoding::int32::merge,
                            &mut value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "g");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + ::prost::encoding::hash_map::encoded_len(
                    ::prost::encoding::string::encoded_len,
                    ::prost::encoding::int32::encoded_len,
                    7u32,
                    &self.g,
                )
        }
        fn clear(&mut self) {
            self.g.clear();
        }
    }
    impl ::core::default::Default for Msg {
        fn default() -> Self {
            Msg {
                g: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for Msg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Msg");
            let builder = {
                let wrapper = {
                    struct MapWrapper<'a>(
                        &'a ::std::collections::HashMap<
                            ::prost::alloc::string::String,
                            i32,
                        >,
                    );
                    impl<'a> ::core::fmt::Debug for MapWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn KeyWrapper<T>(v: T) -> T {
                                v
                            }
                            #[allow(non_snake_case)]
                            fn ValueWrapper<T>(v: T) -> T {
                                v
                            }
                            let mut builder = f.debug_map();
                            for (k, v) in self.0 {
                                builder.entry(&KeyWrapper(k), &ValueWrapper(v));
                            }
                            builder.finish()
                        }
                    }
                    MapWrapper(&self.g)
                };
                builder.field("g", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct Msg2 {
        #[prost(message, repeated, tag = "7")]
        pub g: ::prost::alloc::vec::Vec<msg2::GEntry>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Msg2 {
        #[inline]
        fn clone(&self) -> Msg2 {
            Msg2 {
                g: ::core::clone::Clone::clone(&self.g),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Msg2 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Msg2 {
        #[inline]
        fn eq(&self, other: &Msg2) -> bool {
            self.g == other.g
        }
    }
    impl ::prost::Message for Msg2 {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            for msg in &self.g {
                ::prost::encoding::message::encode(7u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Msg2";
            match tag {
                7u32 => {
                    let mut value = &mut self.g;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "g");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::message::encoded_len_repeated(7u32, &self.g)
        }
        fn clear(&mut self) {
            self.g.clear();
        }
    }
    impl ::core::default::Default for Msg2 {
        fn default() -> Self {
            Msg2 {
                g: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for Msg2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Msg2");
            let builder = {
                let wrapper = &self.g;
                builder.field("g", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `Msg2`.
    pub mod msg2 {
        pub struct GEntry {
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
            #[prost(int32, tag = "2")]
            pub value: i32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GEntry {
            #[inline]
            fn clone(&self) -> GEntry {
                GEntry {
                    key: ::core::clone::Clone::clone(&self.key),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for GEntry {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for GEntry {
            #[inline]
            fn eq(&self, other: &GEntry) -> bool {
                self.key == other.key && self.value == other.value
            }
        }
        impl ::prost::Message for GEntry {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.key != "" {
                    ::prost::encoding::string::encode(1u32, &self.key, buf);
                }
                if self.value != 0i32 {
                    ::prost::encoding::int32::encode(2u32, &self.value, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "GEntry";
                match tag {
                    1u32 => {
                        let mut value = &mut self.key;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "key");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.value;
                        ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "value");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.key != "" {
                        ::prost::encoding::string::encoded_len(1u32, &self.key)
                    } else {
                        0
                    }
                    + if self.value != 0i32 {
                        ::prost::encoding::int32::encoded_len(2u32, &self.value)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.key.clear();
                self.value = 0i32;
            }
        }
        impl ::core::default::Default for GEntry {
            fn default() -> Self {
                GEntry {
                    key: ::prost::alloc::string::String::new(),
                    value: 0i32,
                }
            }
        }
        impl ::core::fmt::Debug for GEntry {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("GEntry");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.key)
                    };
                    builder.field("key", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.value)
                    };
                    builder.field("value", &wrapper)
                };
                builder.finish()
            }
        }
    }
}
pub mod oneof {
    pub struct Msg {
        #[prost(int32, tag = "7")]
        pub f: i32,
        #[prost(oneof = "msg::A", tags = "4, 6")]
        pub a: ::core::option::Option<msg::A>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Msg {
        #[inline]
        fn clone(&self) -> Msg {
            Msg {
                f: ::core::clone::Clone::clone(&self.f),
                a: ::core::clone::Clone::clone(&self.a),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Msg {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Msg {
        #[inline]
        fn eq(&self, other: &Msg) -> bool {
            self.f == other.f && self.a == other.a
        }
    }
    impl ::prost::Message for Msg {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref oneof) = self.a {
                oneof.encode(buf)
            }
            if self.f != 0i32 {
                ::prost::encoding::int32::encode(7u32, &self.f, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Msg";
            match tag {
                4u32 | 6u32 => {
                    let mut value = &mut self.a;
                    msg::A::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "a");
                            error
                        })
                }
                7u32 => {
                    let mut value = &mut self.f;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "f");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.a.as_ref().map_or(0, msg::A::encoded_len)
                + if self.f != 0i32 {
                    ::prost::encoding::int32::encoded_len(7u32, &self.f)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.a = ::core::option::Option::None;
            self.f = 0i32;
        }
    }
    impl ::core::default::Default for Msg {
        fn default() -> Self {
            Msg {
                a: ::core::default::Default::default(),
                f: 0i32,
            }
        }
    }
    impl ::core::fmt::Debug for Msg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Msg");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.f)
                };
                builder.field("f", &wrapper)
            };
            let builder = {
                let wrapper = &self.a;
                builder.field("a", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `Msg`.
    pub mod msg {
        pub enum A {
            #[prost(string, tag = "4")]
            D(::prost::alloc::string::String),
            #[prost(int32, tag = "6")]
            E(i32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for A {
            #[inline]
            fn clone(&self) -> A {
                match self {
                    A::D(__self_0) => A::D(::core::clone::Clone::clone(__self_0)),
                    A::E(__self_0) => A::E(::core::clone::Clone::clone(__self_0)),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for A {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for A {
            #[inline]
            fn eq(&self, other: &A) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (A::D(__self_0), A::D(__arg1_0)) => __self_0 == __arg1_0,
                        (A::E(__self_0), A::E(__arg1_0)) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl A {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    A::D(ref value) => {
                        ::prost::encoding::string::encode(4u32, &*value, buf);
                    }
                    A::E(ref value) => {
                        ::prost::encoding::int32::encode(6u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<A>,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    4u32 => {
                        match field {
                            ::core::option::Option::Some(A::D(ref mut value)) => {
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(A::D(owned_value));
                                    })
                            }
                        }
                    }
                    6u32 => {
                        match field {
                            ::core::option::Option::Some(A::E(ref mut value)) => {
                                ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(A::E(owned_value));
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid A tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    A::D(ref value) => {
                        ::prost::encoding::string::encoded_len(4u32, &*value)
                    }
                    A::E(ref value) => {
                        ::prost::encoding::int32::encoded_len(6u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for A {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    A::D(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("D").field(&wrapper).finish()
                    }
                    A::E(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("E").field(&wrapper).finish()
                    }
                }
            }
        }
    }
}
pub mod repeated {
    pub struct Repeated {
        #[prost(string, tag = "4")]
        pub d: ::prost::alloc::string::String,
        #[prost(int32, repeated, tag = "6")]
        pub e: ::prost::alloc::vec::Vec<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Repeated {
        #[inline]
        fn clone(&self) -> Repeated {
            Repeated {
                d: ::core::clone::Clone::clone(&self.d),
                e: ::core::clone::Clone::clone(&self.e),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Repeated {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Repeated {
        #[inline]
        fn eq(&self, other: &Repeated) -> bool {
            self.d == other.d && self.e == other.e
        }
    }
    impl ::prost::Message for Repeated {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.d != "" {
                ::prost::encoding::string::encode(4u32, &self.d, buf);
            }
            ::prost::encoding::int32::encode_packed(6u32, &self.e, buf);
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Repeated";
            match tag {
                4u32 => {
                    let mut value = &mut self.d;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "d");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.e;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "e");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.d != "" {
                    ::prost::encoding::string::encoded_len(4u32, &self.d)
                } else {
                    0
                } + ::prost::encoding::int32::encoded_len_packed(6u32, &self.e)
        }
        fn clear(&mut self) {
            self.d.clear();
            self.e.clear();
        }
    }
    impl ::core::default::Default for Repeated {
        fn default() -> Self {
            Repeated {
                d: ::prost::alloc::string::String::new(),
                e: ::prost::alloc::vec::Vec::new(),
            }
        }
    }
    impl ::core::fmt::Debug for Repeated {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Repeated");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.d)
                };
                builder.field("d", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.e)
                };
                builder.field("e", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct Packed {
        #[prost(string, tag = "4")]
        pub d: ::prost::alloc::string::String,
        #[prost(int32, repeated, packed = "false", tag = "6")]
        pub e: ::prost::alloc::vec::Vec<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Packed {
        #[inline]
        fn clone(&self) -> Packed {
            Packed {
                d: ::core::clone::Clone::clone(&self.d),
                e: ::core::clone::Clone::clone(&self.e),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Packed {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Packed {
        #[inline]
        fn eq(&self, other: &Packed) -> bool {
            self.d == other.d && self.e == other.e
        }
    }
    impl ::prost::Message for Packed {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.d != "" {
                ::prost::encoding::string::encode(4u32, &self.d, buf);
            }
            ::prost::encoding::int32::encode_repeated(6u32, &self.e, buf);
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Packed";
            match tag {
                4u32 => {
                    let mut value = &mut self.d;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "d");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.e;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "e");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.d != "" {
                    ::prost::encoding::string::encoded_len(4u32, &self.d)
                } else {
                    0
                } + ::prost::encoding::int32::encoded_len_repeated(6u32, &self.e)
        }
        fn clear(&mut self) {
            self.d.clear();
            self.e.clear();
        }
    }
    impl ::core::default::Default for Packed {
        fn default() -> Self {
            Packed {
                d: ::prost::alloc::string::String::new(),
                e: ::prost::alloc::vec::Vec::new(),
            }
        }
    }
    impl ::core::fmt::Debug for Packed {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Packed");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.d)
                };
                builder.field("d", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.e)
                };
                builder.field("e", &wrapper)
            };
            builder.finish()
        }
    }
}
pub mod sint32 {
    pub struct Msg {
        #[prost(int32, tag = "7")]
        pub a: i32,
        #[prost(sint32, tag = "8")]
        pub b: i32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Msg {
        #[inline]
        fn clone(&self) -> Msg {
            let _: ::core::clone::AssertParamIsClone<i32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Msg {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Msg {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Msg {
        #[inline]
        fn eq(&self, other: &Msg) -> bool {
            self.a == other.a && self.b == other.b
        }
    }
    impl ::prost::Message for Msg {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.a != 0i32 {
                ::prost::encoding::int32::encode(7u32, &self.a, buf);
            }
            if self.b != 0i32 {
                ::prost::encoding::sint32::encode(8u32, &self.b, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Msg";
            match tag {
                7u32 => {
                    let mut value = &mut self.a;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "a");
                            error
                        })
                }
                8u32 => {
                    let mut value = &mut self.b;
                    ::prost::encoding::sint32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "b");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.a != 0i32 {
                    ::prost::encoding::int32::encoded_len(7u32, &self.a)
                } else {
                    0
                }
                + if self.b != 0i32 {
                    ::prost::encoding::sint32::encoded_len(8u32, &self.b)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.a = 0i32;
            self.b = 0i32;
        }
    }
    impl ::core::default::Default for Msg {
        fn default() -> Self {
            Msg { a: 0i32, b: 0i32 }
        }
    }
    impl ::core::fmt::Debug for Msg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Msg");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.a)
                };
                builder.field("a", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.b)
                };
                builder.field("b", &wrapper)
            };
            builder.finish()
        }
    }
}
pub mod submsg {
    pub struct SubMsg {
        #[prost(int32, tag = "1")]
        pub a: i32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SubMsg {
        #[inline]
        fn clone(&self) -> SubMsg {
            let _: ::core::clone::AssertParamIsClone<i32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SubMsg {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SubMsg {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SubMsg {
        #[inline]
        fn eq(&self, other: &SubMsg) -> bool {
            self.a == other.a
        }
    }
    impl ::prost::Message for SubMsg {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.a != 0i32 {
                ::prost::encoding::int32::encode(1u32, &self.a, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SubMsg";
            match tag {
                1u32 => {
                    let mut value = &mut self.a;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "a");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.a != 0i32 {
                    ::prost::encoding::int32::encoded_len(1u32, &self.a)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.a = 0i32;
        }
    }
    impl ::core::default::Default for SubMsg {
        fn default() -> Self {
            SubMsg { a: 0i32 }
        }
    }
    impl ::core::fmt::Debug for SubMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SubMsg");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.a)
                };
                builder.field("a", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct Msg {
        #[prost(message, optional, tag = "3")]
        pub c: ::core::option::Option<SubMsg>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Msg {
        #[inline]
        fn clone(&self) -> Msg {
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<SubMsg>>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Msg {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Msg {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Msg {
        #[inline]
        fn eq(&self, other: &Msg) -> bool {
            self.c == other.c
        }
    }
    impl ::prost::Message for Msg {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.c {
                ::prost::encoding::message::encode(3u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Msg";
            match tag {
                3u32 => {
                    let mut value = &mut self.c;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "c");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .c
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(3u32, msg))
        }
        fn clear(&mut self) {
            self.c = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for Msg {
        fn default() -> Self {
            Msg {
                c: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for Msg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Msg");
            let builder = {
                let wrapper = &self.c;
                builder.field("c", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct SubMsg2 {
        #[prost(int32, optional, tag = "1")]
        pub a: ::core::option::Option<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SubMsg2 {
        #[inline]
        fn clone(&self) -> SubMsg2 {
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<i32>>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SubMsg2 {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SubMsg2 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SubMsg2 {
        #[inline]
        fn eq(&self, other: &SubMsg2) -> bool {
            self.a == other.a
        }
    }
    impl ::prost::Message for SubMsg2 {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let ::core::option::Option::Some(ref value) = self.a {
                ::prost::encoding::int32::encode(1u32, value, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SubMsg2";
            match tag {
                1u32 => {
                    let mut value = &mut self.a;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "a");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .a
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(1u32, value),
                    )
        }
        fn clear(&mut self) {
            self.a = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for SubMsg2 {
        fn default() -> Self {
            SubMsg2 {
                a: ::core::option::Option::None,
            }
        }
    }
    impl ::core::fmt::Debug for SubMsg2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SubMsg2");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.a)
                };
                builder.field("a", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl SubMsg2 {
        ///Returns the value of `a`, or the default value if `a` is unset.
        pub fn a(&self) -> i32 {
            match self.a {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
    }
}
