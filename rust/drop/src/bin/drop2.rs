#[derive(Debug)]
struct Item(&'static str);

impl Drop for Item {
    fn drop(&mut self) {
        println!("{} droped", self.0);
    }
}

fn main() {
    let i = Item("item 1");
    let v = vec![Some(i), Some(Item("item 2"))];
    //for i in v {
    for i in &v {
        println!("{:?}", i);
    }
    match &v[0] {
        //Some(ref i) => println!("{:?}", i),
        Some(i) => println!("{:?}", i),
        None => println!("None"),
    }
    println!("end");
}
