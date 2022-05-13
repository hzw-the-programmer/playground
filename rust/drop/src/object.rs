mod clone;

#[derive(Debug, PartialEq, Default)]
pub struct Object {
    pub id: i32,
}

impl Object {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn consume(self) {
        println!("Object::consume");
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("object {} dropped", self.id);
    }
}
