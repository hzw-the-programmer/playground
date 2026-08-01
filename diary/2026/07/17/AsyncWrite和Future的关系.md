AsyncWrite的定义如下：

```rust
pub trait AsyncWrite {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>>;

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>>;

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>>;

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        let buf = bufs
            .iter()
            .find(|b| !b.is_empty())
            .map_or(&[][..], |b| &**b);
        self.poll_write(cx, buf)
    }

    fn is_write_vectored(&self) -> bool {
        false
    }
}
```

可以看出和 Future 的关系不明显。

poll_write、poll_flush、poll_shutdown、poll_write_vectored 很像 Future 的 poll 方法。

那么 AsyncWrite 是怎么被整合进 Future 体系的呢？

通过 AsyncWriteExt trait。其定义如下：

```rust
pub trait AsyncWriteExt: AsyncWrite {
    fn write<'a>(&'a mut self, src: &'a [u8]) -> Write<'a, Self>
    where
        Self: Unpin,
    {
        write(self, src)
    }

    fn write_vectored<'a, 'b>(&'a mut self, bufs: &'a [IoSlice<'b>]) -> WriteVectored<'a, 'b, Self>
    where
        Self: Unpin,
    {
        write_vectored(self, bufs)
    }

    fn write_buf<'a, B>(&'a mut self, src: &'a mut B) -> WriteBuf<'a, Self, B>
    where
        Self: Sized + Unpin,
        B: Buf,
    {
        write_buf(self, src)
    }

    fn write_all_buf<'a, B>(&'a mut self, src: &'a mut B) -> WriteAllBuf<'a, Self, B>
    where
        Self: Sized + Unpin,
        B: Buf,
    {
        write_all_buf(self, src)
    }

    fn write_all<'a>(&'a mut self, src: &'a [u8]) -> WriteAll<'a, Self>
    where
        Self: Unpin,
    {
        write_all(self, src)
    }

    fn flush(&mut self) -> Flush<'_, Self>
    where
        Self: Unpin,
    {
        flush(self)
    }

    fn shutdown(&mut self) -> Shutdown<'_, Self>
    where
        Self: Unpin,
    {
        shutdown(self)
    }
}
```

这些方法返回的：Write、WriteVectored、WriteBuf、WriteAllBuf、WriteAll、Flush、Shutdown 都是实现了 Future。
