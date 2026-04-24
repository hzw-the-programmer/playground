winnow-1.0.1/src/parser.rs

```rust
impl<I, O, E, F> Parser<I, O, E> for F
where
    F: FnMut(&mut I) -> Result<O, E>,
    I: Stream,
{
    #[inline(always)]
    fn parse_next(&mut self, i: &mut I) -> Result<O, E> {
        self(i)
    }
}
```
