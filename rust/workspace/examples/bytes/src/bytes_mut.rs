#[cfg(test)]
mod tests {
    use bytes::{BufMut, BytesMut};

    #[test]
    fn split_off() {
        let mut bm = BytesMut::with_capacity(64);
        assert_eq!(bm.len(), 0);
        assert_eq!(bm.capacity(), 64);

        // expected `i32`, found `&[u8; 4]`
        // let i: i32 = b"0123";
        
        // 数组实现了 Deref ?
        // 不是，这叫 unsize 强制转换
        // 它利用编译期已知的数组大小，在运行时构造出“指针 + 长度”的胖指针，从而安全、零成本地转换为切片。
        // 这个特性是处理数组与切片的核心设计，让代码在静态长度和动态长度间无缝切换。

        bm.put_slice(b"0123");
        assert_eq!(bm.len(), 4);
        assert_eq!(bm.capacity(), 64);

        let mut bm2 = bm.split_off(6);
        assert_eq!(bm.len(), 4);
        assert_eq!(bm.capacity(), 6);
        assert_eq!(bm2.len(), 0);
        assert_eq!(bm2.capacity(), 58);

        bm.put_slice(b"45");
        assert_eq!(bm.len(), 6);
        assert_eq!(bm.capacity(), 6);

        bm2.put_slice(b"6789");
        assert_eq!(bm2.len(), 4);
        assert_eq!(bm2.capacity(), 58);

        assert_eq!(bm, "012345");
        assert_eq!(bm2, "6789");

        bm.unsplit(bm2);
        assert_eq!(bm.len(), 10);
        assert_eq!(bm.capacity(), 64);
        assert_eq!(bm, "0123456789");
    }
}
