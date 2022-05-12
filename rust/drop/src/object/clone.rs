use crate::object::Object;

impl Clone for Object {
    fn clone(&self) -> Self {
        Object { id: self.id + 1 }
    }
}
