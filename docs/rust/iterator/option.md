D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\option.rs
```Rust
impl<T> IntoIterator for Option<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { inner: Item { opt: self } }
    }
}

impl<'a, T> IntoIterator for &'a Option<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Option<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T> Option<T> {
    pub const fn iter(&self) -> Iter<'_, T> {
        Iter { inner: Item { opt: self.as_ref() } }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { inner: Item { opt: self.as_mut() } }
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter<A> {
    inner: Item<A>,
}

impl<A> Iterator for IntoIter<A> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<A> DoubleEndedIterator for IntoIter<A> {
    fn next_back(&mut self) -> Option<A> {
        self.inner.next_back()
    }
}

impl<A> ExactSizeIterator for IntoIter<A> {}
impl<A> FusedIterator for IntoIter<A> {}
unsafe impl<A> TrustedLen for IntoIter<A> {}

#[derive(Debug)]
pub struct Iter<'a, A: 'a> {
    inner: Item<&'a A>,
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;
    fn next(&mut self) -> Option<&'a A> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, A> DoubleEndedIterator for Iter<'a, A> {
    fn next_back(&mut self) -> Option<&'a A> {
        self.inner.next_back()
    }
}

impl<A> ExactSizeIterator for Iter<'_, A> {}
impl<A> FusedIterator for Iter<'_, A> {}
unsafe impl<A> TrustedLen for Iter<'_, A> {}
impl<A> Clone for Iter<'_, A> {
    fn clone(&self) -> Self {
        Iter { inner: self.inner.clone() }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, A: 'a> {
    inner: Item<&'a mut A>,
}

impl<'a, A> Iterator for IterMut<'a, A> {
    type Item = &'a mut A;
    fn next(&mut self) -> Option<&'a mut A> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, A> DoubleEndedIterator for IterMut<'a, A> {
    fn next_back(&mut self) -> Option<&'a mut A> {
        self.inner.next_back()
    }
}

impl<A> ExactSizeIterator for IterMut<'_, A> {}
impl<A> FusedIterator for IterMut<'_, A> {}
unsafe impl<A> TrustedLen for IterMut<'_, A> {}

#[derive(Clone, Debug)]
struct Item<A> {
    opt: Option<A>,
}

impl<A> Iterator for Item<A> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        self.opt.take()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.opt {
            Some(_) => (1, Some(1)),
            None => (0, Some(0)),
        }
    }
}

impl<A> DoubleEndedIterator for Item<A> {
    fn next_back(&mut self) -> Option<A> {
        self.opt.take()
    }
}

impl<A> ExactSizeIterator for Item<A> {}
impl<A> FusedIterator for Item<A> {}
unsafe impl<A> TrustedLen for Item<A> {}
```
