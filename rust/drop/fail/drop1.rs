struct Field {
    name: String,
}

impl Drop for Field {
    fn drop(&mut self) {
        println!("{} droped", self.name);
    }
}

struct Struct {
    field1: Field,
    field2: Field,
}

impl Drop for Struct {
    fn drop(&mut self) {
        println!("struct droped");
    }
}

fn consume(f: Field) {}

fn main() {
    let mut s;
    println!("before block");
    {
        let field1 = Field {
            name: String::from("filed 1"),
        };
        let field2 = Field {
            name: String::from("field 2"),
        };
        s = Struct { field1, field2 };
    }
    println!("end block");

    s.field1 = Field {
        name: String::from("filed 1 new"),
    };
    consume(s.field1);
    println!("main end");
}
