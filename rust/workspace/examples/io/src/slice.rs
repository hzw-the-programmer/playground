#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::Read;

    #[test]
    fn f1() {
        // no method named `read` found for reference `&[{integer}; 5]` in the current scope
        // let data = &[1, 2, 3, 4, 5];
        let data = &[1, 2, 3, 4, 5][..];
        let mut scroll = data;

        assert_eq!(scroll.len(), 5);
        assert_eq!(scroll, &[1, 2, 3, 4, 5]);

        let mut buf = [0; 3];
        let n = scroll.read(&mut buf).unwrap();
        assert_eq!(n, 3);
        assert_eq!(buf, [1, 2, 3]);
        assert_eq!(scroll.len(), 2);
        assert_eq!(scroll, &[4, 5]);

        let n = scroll.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(buf, [4, 5, 3]);
        assert_eq!(buf[..n], [4, 5]);
        assert_eq!(scroll.len(), 0);
        assert_eq!(scroll, &[]);

        assert_eq!(data.len(), 5);
        assert_eq!(data, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn f2() {
        let data = &[1, 2, 3, 4];
        let mut a = &data[..];
        let b = a;

        let mut buf = [0; 2];
        let n = a.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(a, [3, 4]);
        assert_eq!(b, [1, 2, 3, 4]);
    }

    #[test]
    fn f3() {
        let data = &[1, 2, 3, 4, 5];
        let mut cursor = Cursor::new(data);
        let mut buf = [0; 3];
        let n = cursor.read(&mut buf).unwrap();
        assert_eq!(n, 3);
        let n = cursor.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(data, &[1, 2, 3, 4, 5]);
    }
}
