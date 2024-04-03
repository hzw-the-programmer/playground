D:\github\tokio-rs\bytes\src\buf\buf_impl.rs
```Rust
impl<T: Buf + ?Sized> Buf for &mut T {
    deref_forward_buf!();
}

pub trait Buf {
    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
        use super::BufMut;

        if self.remaining() < len {
            panic_advance(len, self.remaining());
        }

        let mut ret = crate::BytesMut::with_capacity(len);
        ret.put(self.take(len));
        ret.freeze()
    }

    fn take(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        take::new(self, limit)
    }
}
```
