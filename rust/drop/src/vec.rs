#[cfg(test)]
mod tests {
    use crate::Object;

    #[test]
    fn iter() {
        let v = vec![
            Object { id: 0 },
            Object { id: 1 },
            Object { id: 2 },
            Object { id: 3 },
            Object { id: 4 },
        ];
        let mut i = v.iter();
        println!("eq 0");
        assert_eq!(Some(&Object { id: 0 }), i.next());
        println!("eq 1");
        assert_eq!(Some(&Object { id: 1 }), i.next());
        println!("finish");
    }
}
