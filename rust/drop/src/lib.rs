#[derive(Debug)]
pub struct Object {
    pub id: i32,
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("object {} dropped", self.id);
    }
}