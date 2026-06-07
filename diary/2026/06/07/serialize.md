类型通过调用 `Serializer` 的方法，将自己映射到 `serde` 数据模型。

# `i32`

`i32` 实现 `Serialize`。通过调用 `Serializer` 的 `serialize_i32` 将自己映射到 `i32` 数据模型。

```rust
primitive_impl!(i32, serialize_i32);

macro_rules! primitive_impl {
    ($ty:ident, $method:ident $($cast:tt)*) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.$method(*self $($cast)*)
            }
        }
    }
}
```

# `[T; N]`

`[T; N]` 实现 `Serialize`。通过调用 `Serializer` 的 `serialize_tuple` 将自己映射到 `tuple` 数据模型。

```rust
array_impls! {
    01 02 03 04 05 06 07 08 09 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}

macro_rules! array_impls {
    ($($len:tt)+) => {
        $(
            impl<T> Serialize for [T; $len]
            where
                T: Serialize,
            {
                #[inline]
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let mut seq = tri!(serializer.serialize_tuple($len));
                    for e in self {
                        tri!(seq.serialize_element(e));
                    }
                    seq.end()
                }
            }
        )+
    }
}
```

# `(T₁, T₂, …, Tₙ)`

`(T₁, T₂, …, Tₙ)` 实现 `Serialize`。通过调用 `Serializer` 的 `serialize_tuple` 方法，将自己映射到 tuple 数据模型。

```rust
impl<T> Serialize for (T,)
where
    T: Serialize,
{
    tuple_impl_body!(1 => (0));
}

tuple_impls! {
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            #[cfg_attr(docsrs, doc(hidden))]
            impl<$($name),+> Serialize for ($($name,)+)
            where
                $($name: Serialize,)+
            {
                tuple_impl_body!($len => ($($n)+));
            }
        )+
    };
}

macro_rules! tuple_impl_body {
    ($len:expr => ($($n:tt)+)) => {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = tri!(serializer.serialize_tuple($len));
            $(
                tri!(tuple.serialize_element(&self.$n));
            )+
            tuple.end()
        }
    };
}
```

# `Vec<T>`

`Vec<T>` 实现 `Serialize`。通过调用 `Serializer` 的 `collect_seq` 方法（再调用 `serialize_seq`），将自己映射到 `seq` 数据模型。

```rust
seq_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    Vec<T>
}

macro_rules! seq_impl {
    (
        $(#[$attr:meta])*
        $ty:ident <T $(, $typaram:ident : $bound:ident)*>
    ) => {
        $(#[$attr])*
        impl<T $(, $typaram)*> Serialize for $ty<T $(, $typaram)*>
        where
            T: Serialize,
        {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.collect_seq(self)
            }
        }
    }
}

pub trait Serializer: Sized {
    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        let mut iter = iter.into_iter();
        let mut serializer = tri!(self.serialize_seq(iterator_len_hint(&iter)));
        tri!(iter.try_for_each(|item| serializer.serialize_element(&item)));
        serializer.end()
    }
}
```

# `[T]`

`[T]` 实现 `Serialize`。通过调用 `Serializer` 的 `collect_seq`（再调用 `serialize_seq`）,将自己映射到 seq 数据模型。

```rust
impl<T> Serialize for [T]
where
    T: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self)
    }
}
```