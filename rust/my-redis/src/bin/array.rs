use std::io::{Read, Result};

fn main() -> Result<()> {
    test1()?;
    println!("");
    test2();
    println!("");
    test3();
    println!("");
    test4();
    println!("");
    test5();
    println!("");
    test6();
    println!("");
    test7();
    Ok(())
}

fn test1() -> Result<()> {
    let mut rd: &[u8] = b"hello world!";
    let mut buf = [0; 10];
    let n = rd.read(&mut buf)?;
    println!("{:?}", &buf[..n]);
    Ok(())
}

fn test2() {
    let ar = b"hello world!";
    //let _: i32 = ar;
    func1(ar);
    let a = [1u8, 2u8, 3u8];
    //let _: i32 = a;
    //let _: i32 = &a;
    func1(&a);
}

fn func1(s: &[u8]) {}

fn test3() {
    let a = [
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    {
        let [x, y, ..] = a;
        //println!("{:?}", a);
        //println!("{:?}", a[0]);
        //println!("{:?}", a[2]);
    }
    println!("finish test3");
}

fn test4() {
    let a = [
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    for i in a {
        println!("{:?}", i);
    }
    println!("finish test4");
}

fn test5() {
    let a = [
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    for i in &a {
        println!("{:?}", i);
    }
    println!("finish test5");
}

fn test6() {
    let a = [
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    for i in a {
        if i.id == 2 {
            break;
        }
        println!("{:?}", i);
    }
    println!("finish test6");
}

fn test7() {
    let mut a = [1u8, 2u8, 3u8];
    let mut buf = [0; 128];
    //a.read(&mut buf);
    let mut ar = &a;
    //let _: i32 = ar;
    //ar.read(&mut buf);
    let mut b = &a[..];
    //let _: i32 = b;
    b.read(&mut buf);
}

#[derive(Debug)]
struct Object {
    id: i32,
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("Object {} dropped", self.id);
    }
}
