http-body-1.0.0\src\lib.rs
```
pub trait Body {
    type Data: Buf;
    type Error;
    
    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>;

    fn is_end_stream(&self) -> bool {
        false
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }
}
```
Poll::Pending
Poll::Ready(None)
Poll::Ready(Some(Ok(frame)))
Poll::Ready(Some(Err(err)))

futures-core-0.3.30\src\stream.rs
```
pub trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
```
Poll::Pending
Poll::Ready(None)
Poll::Ready(Some(Self::Item))
