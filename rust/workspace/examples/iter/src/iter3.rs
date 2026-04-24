trait MyFnOnce {
    type Output;
    fn call_once(self) -> Self::Output;
}

trait MyFnMut: MyFnOnce {
    fn call_mut(&mut self) -> Self::Output;
}

trait MyFn: MyFnMut {
    fn call(&self) -> Self::Output;
}

struct MyRef<'a> {
    f1: &'a i32,
}

impl<'a> MyFnOnce for MyRef<'a> {
    type Output = ();

    // fn call_once(self) -> Self::Output {
    //     println!("{}", self.f1);
    // }

    fn call_once(mut self) -> Self::Output {
        self.f1 = &20;
        println!("{}", self.f1);
    }
}

fn test1() {
    let i = 10;

    let my_ref = MyRef { f1: &i };
    my_ref.call_once();
}

struct Bar {
    f1: i32,
    f2: i32,
}

struct MyRefBar<'a> {
    f1: &'a Bar,
}

impl<'a> MyFnOnce for MyRefBar<'a> {
    type Output = ();

    // cannot assign to `self.f1.f1`, which is behind a `&` reference
    // fn call_once(self) -> Self::Output {
    //     self.f1.f1 = 10;
    //     println!("{} {}", self.f1.f1, self.f1.f2);
    // }

    // variable does not need to be mutable
    // cannot assign to `self.f1.f1`, which is behind a `&` reference
    // fn call_once(mut self) -> Self::Output {
    //     self.f1.f1 = 10;
    //     println!("{} {}", self.f1.f1, self.f1.f2);
    // }

    // cannot assign to `self.f1`, as `self` is not declared as mutable
    // fn call_once(self) -> Self::Output {
    //     self.f1 = &Bar { f1: 10, f2: 20 };
    //     println!("{} {}", self.f1.f1, self.f1.f2);
    // }

    fn call_once(mut self) -> Self::Output {
        self.f1 = &Bar { f1: 10, f2: 20 };
        println!("{} {}", self.f1.f1, self.f1.f2);
    }
}

fn test2() {
    let bar = Bar { f1: 1, f2: 2 };
    let my_ref_bar = MyRefBar { f1: &bar };

    my_ref_bar.call_once();
}

struct MyRefMutBar<'a> {
    f1: &'a mut Bar,
}

impl<'a> MyFnOnce for MyRefMutBar<'a> {
    type Output = ();

    fn call_once(self) -> Self::Output {
        self.f1.f1 = 10;
        println!("{} {}", self.f1.f1, self.f1.f2);
    }

    // variable does not need to be mutable
    // fn call_once(mut self) -> Self::Output {
    //     self.f1.f1 = 10;
    //     println!("{} {}", self.f1.f1, self.f1.f2);
    // }

    // cannot assign to `self.f1`, as `self` is not declared as mutable
    // fn call_once(self) -> Self::Output {
    //     self.f1 = &mut Bar { f1: 10, f2: 20 };
    //     println!("{} {}", self.f1.f1, self.f1.f2);
    // }

    // temporary value dropped while borrowed
    // fn call_once(mut self) -> Self::Output {
    //     self.f1 = &mut Bar { f1: 10, f2: 20 };
    //     println!("{} {}", self.f1.f1, self.f1.f2);
    // }
}

fn test3() {
    let mut bar = Bar { f1: 1, f2: 2 };
    let my_ref_mut_bar = MyRefMutBar { f1: &mut bar };

    my_ref_mut_bar.call_once();
}

pub fn test() {
    test3();
}
