#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn size() {
        assert_eq!(8, size_of::<*const u8>());
        assert_eq!(8, size_of::<usize>());

        assert_eq!(24, size_of::<String>());
        assert_eq!(24, size_of::<Vec<u8>>());

        assert_eq!(8, size_of::<&String>());
        assert_eq!(8, size_of::<&Vec<u8>>());

        assert_eq!(8, size_of::<Box<String>>());
        assert_eq!(8, size_of::<Box<Vec<u8>>>());
        assert_eq!(8, size_of::<&Box<Vec<u8>>>());

        assert_eq!(16, size_of::<&str>());
        assert_eq!(16, size_of::<&[u8]>());

        assert_eq!(16, size_of::<&dyn Drop>());
        assert_eq!(8, size_of::<&&dyn Drop>());
    }
}
