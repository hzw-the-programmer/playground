在core\src\slice\iter\macros.rs看到一段代码：
```rust
macro_rules! if_zst {
    (mut $this:ident, $len:ident => $zst_body:expr, $end:ident => $other_body:expr,) => {{
        #![allow(unused_unsafe)] // we're sometimes used within an unsafe block

        if T::IS_ZST {
            // SAFETY: for ZSTs, the pointer is storing a provenance-free length,
            // so consuming and updating it as a `usize` is fine.
            let $len = unsafe { &mut *ptr::addr_of_mut!($this.end_or_len).cast::<usize>() };
            $zst_body
        } else {
            // SAFETY: for non-ZSTs, the type invariant ensures it cannot be null
            let $end = unsafe { &mut *ptr::addr_of_mut!($this.end_or_len).cast::<NonNull<T>>() };
            $other_body
        }
    }};
    ($this:ident, $len:ident => $zst_body:expr, $end:ident => $other_body:expr,) => {{
        #![allow(unused_unsafe)] // we're sometimes used within an unsafe block

        if T::IS_ZST {
            let $len = $this.end_or_len.addr();
            $zst_body
        } else {
            // SAFETY: for non-ZSTs, the type invariant ensures it cannot be null
            let $end = unsafe { *ptr::addr_of!($this.end_or_len).cast::<NonNull<T>>() };
            $other_body
        }
    }};
}
```

end_or_len的定义：
core\src\slice\iter.rs
```rust
pub struct Iter<'a, T: 'a> {
    /// The pointer to the next element to return, or the past-the-end location
    /// if the iterator is empty.
    ///
    /// This address will be used for all ZST elements, never changed.
    ptr: NonNull<T>,
    /// For non-ZSTs, the non-null pointer to the past-the-end element.
    ///
    /// For ZSTs, this is `ptr::invalid(len)`.
    end_or_len: *const T,
    _marker: PhantomData<&'a T>,
}
```

其中的这段代码没看懂：
```rust
let $end = unsafe { &mut *ptr::addr_of_mut!($this.end_or_len).cast::<NonNull<T>>() };
```

先确定优先级：
https://doc.rust-lang.org/reference/expressions.html#:~:text=The%20precedence%20of%20Rust%20operators%20and%20expressions%20is,grouped%20in%20the%20order%20given%20by%20their%20associativity.
Paths	
Method calls	
Field expressions, left to right
Function calls, array indexing	
?
Unary - * ! & &mut
as, left to right
* / %, left to right
+ -, left to right
<< >>, left to right
&, left to right
^, left to right
|, left to right
== != < > <= >=	Require parentheses
&&, left to right
||, left to right
.. ..=, Require parentheses
= += -= *= /= %=
&= |= ^= <<= >>=, right to left
return break closures

方法调用.的优先级大于*
那么 ptr::addr_of_mut!($this.end_or_len) 的类型就是 *mut *const T
cast::<NonNull<T>>() 后就是 *mut NonNull<T>
再*，就是NonNull<T>
再&mut，就是&mut NonNull<T>

cast的定义：
core\src\ptr\mut_ptr.rs
```rust
impl<T: ?Sized> *mut T {
    pub const fn cast<U>(self) -> *mut U {
        self as _
    }
}
```

ptr::addr_of_mut!($this.end_or_len) 类型是 *mut *const T
ptr::addr_of_mut!($this.end_or_len).cast::<usize>() 类型是 *mut usize
*ptr::addr_of_mut!($this.end_or_len).cast::<usize>() 类型是 usize
&mut *ptr::addr_of_mut!($this.end_or_len).cast::<usize>() 类型是 &mut usize

例子：rust\workspace\examples\ptr\src\test2.rs
