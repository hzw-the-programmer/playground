use std::rc::Rc;

pub fn test() {
    test1();
}

fn test1() {
    let rc = Rc::new(1);
    let rc2 = rc.clone();

    println!("{:p}", &rc);
    println!("{:p}", &rc2);

    fn f(t: &i32) {
        println!("{:p}", t);
    }
    f(&rc);
    f(&rc2);

    println!("{:p}", rc);
    println!("{:p}", rc2);
    println!("{:p}", Rc::as_ptr(&rc));
    println!("{:p}", Rc::as_ptr(&rc2));
}
