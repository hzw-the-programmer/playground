fn main() {
    let p = Person {
        id: 0,
        obj: Obj { id: 0 },
        obj1: Obj { id: 1 },
    };
    //let p = Person{id: 0, name: "hzw".to_string()};
    match p {
        Person { id: 0, obj, .. } => println!("{:?}", obj),
        /*
        Person{id: 0, name} => {
            //let i: i32 = name;
            println!("{}", name);
        }
        */
        _ => println!("not match"),
    }
    //println!("obj: {:?}, obj1: {:?}", p.obj, p.obj1);
    //println!("{:?}", p);
    println!("obj1: {:?}", p.obj1);
    println!("finish main");
}

#[derive(Debug)]
struct Obj {
    id: i32,
}

impl Drop for Obj {
    fn drop(&mut self) {
        println!("Obj {} dropped.", self.id);
    }
}

#[derive(Debug)]
struct Person {
    id: i32,
    obj: Obj,
    obj1: Obj,
    //name: String,
}

/*
impl Drop for Person {
    fn drop(&mut self) {
        println!("Person {} dropped.", self.id);
    }
}
*/
