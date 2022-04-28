#[derive(Debug)]
pub struct Object {
    pub id: i32,
}

impl Object {
    pub fn Id(&self) -> i32 {
        self.id
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("object {} dropped", self.id);
    }
}

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
