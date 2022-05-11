mod object;
mod size;
mod string;
mod vec;
pub use object::*;

#[derive(Debug)]
pub struct S {
    pub f1: Object,
    pub f2: Object,
}

impl Drop for S {
    fn drop(&mut self) {
        println!("S dropped");
    }
}

pub fn tests(tests: &[fn()]) {
    for (i, test) in tests.iter().enumerate() {
        println!("/*** test {} ***/", i);
        test();
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let o = Object { id: 1 };
        let ao = Object { id: 1 };
        assert_eq!(o, ao);
        println!("finish");
    }
}
