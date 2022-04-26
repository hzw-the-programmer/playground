#[derive(Debug)]
pub struct Object {
    pub id: i32,
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
