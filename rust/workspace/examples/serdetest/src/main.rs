fn main() {
    serialize_unit();
    serialize_array();
    serialize_slice();
    serialize_tuple();
    serialize_struct();
    serialize_phantomdata();
    serialize_enum();
    serialize_bound();
    serialize_result();
    serialize_option();

    serialize_cstring();
    serialize_string();
}

fn serialize_unit() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 363

    println!("serialize_unit");

    let v = ();
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_array() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 166

    println!("serialize_array");

    let v = [1; 0];
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    let v = [1; 1];
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    let v = [2; 32];
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    // let v = [0; 33];
    // let s = serde_json::to_string(&v).unwrap();
    // print!("\t{}", s);
}

fn serialize_slice() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 175

    println!("serialize_slice");

    let v = [2; 33];
    let v: &[i32] = &v;
    let s = serde_json::to_string(v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_tuple() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 409

    println!("serialize_tuple");

    let v = (1,);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    let v = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    // let v = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
    // let s = serde_json::to_string(&v).unwrap();
    // print!("\t{}\n", s);

    // impl<
    //         T0,
    //         T1,
    //         T2,
    //         T3,
    //         T4,
    //         T5,
    //         T6,
    //         T7,
    //         T8,
    //         T9,
    //         T10,
    //         T11,
    //         T12,
    //         T13,
    //         T14,
    //         T15,
    //         T16,
    //         T17,
    //         T18,
    //         T19,
    //     > serde::Serialize
    //     for (
    //         T0,
    //         T1,
    //         T2,
    //         T3,
    //         T4,
    //         T5,
    //         T6,
    //         T7,
    //         T8,
    //         T9,
    //         T10,
    //         T11,
    //         T12,
    //         T13,
    //         T14,
    //         T15,
    //         T16,
    //         T17,
    //         T18,
    //         T19,
    //     )
    // {
    //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //     where
    //         S: serde::Serializer,
    //     {
    //         let s = serializer.serialize_tuple(18)?;
    //         s.serialize_element(self.0)?;
    //         s.serialize_element(self.1)?;
    //         s.serialize_element(self.2)?;
    //         s.serialize_element(self.3)?;
    //         s.serialize_element(self.4)?;
    //         s.serialize_element(self.5)?;
    //         s.serialize_element(self.6)?;
    //         s.serialize_element(self.7)?;
    //         s.serialize_element(self.8)?;
    //         s.serialize_element(self.9)?;
    //         s.serialize_element(self.10)?;
    //         s.serialize_element(self.11)?;
    //         s.serialize_element(self.12)?;
    //         s.serialize_element(self.13)?;
    //         s.serialize_element(self.14)?;
    //         s.serialize_element(self.15)?;
    //         s.serialize_element(self.16)?;
    //         s.serialize_element(self.17)?;
    //         s.serialize_element(self.18)?;
    //         s.serialize_element(self.19)?;
    //         s.end()?;
    //     }
    // }

    // let v = (
    //     1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    // );
    // let s = serde_json::to_string(&v).unwrap();
    // print!("\t{}\n", s);
}

fn serialize_struct() {
    println!("serialize_struct");

    // D:\github\serde-rs\serde\serde\src\lib.rs
    // line: 325, 330
    #[derive(serde::Serialize)]
    struct U;
    let v = U;
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    #[derive(serde::Serialize)]
    struct N(i32);
    let v = N(1);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    // #[derive(serde::Serialize)]
    // struct T1(i32,);
    // let v = T1(1,);
    // let s = serde_json::to_string(&v).unwrap();
    // print!("\t{}\n", s);

    #[derive(serde::Serialize)]
    struct T2(i32, i32);
    let v = T2(1, 2);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    #[derive(serde::Serialize)]
    struct S<'a> {
        f1: i32,
        f2: &'a str,
    }
    let v = S { f1: 1, f2: "hello" };
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_phantomdata() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 117

    println!("serialize_phantomdata");

    let v = core::marker::PhantomData::<i32>;
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_enum() {
    println!("serialize_enum");

    #[derive(serde::Serialize)]
    enum E {
        U,
        N(i32),
        // T1(i32,),
        T2(i32, i32),
        S { fd1: i32, fd2: f32 },
    }

    let v = E::U;
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    let v = E::N(3);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    // let v = E::T1(1,);
    // let s = serde_json::to_string(&v).unwrap();
    // print!("\t{}\n", s);

    let v = E::T2(1, 2);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    let v = E::S { fd1: 1, fd2: 2.0 };
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_bound() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 341, 99

    println!("serialize_bound");

    let v = core::ops::Bound::Unbounded::<i32>;
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    let v = core::ops::Bound::Included(1);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_result() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 698
    println!("serialize_result");

    let v = Ok::<i32, i32>(1);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    let v = Err::<i32, i32>(1);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_option() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 99
    println!("serialize_opton");

    let v = None::<i32>;
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);

    let v = Some(true);
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_cstring() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 87
    println!("serialize_cstring");
    let v = std::ffi::CString::new("hello world").unwrap();
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}

fn serialize_string() {
    // D:\github\serde-rs\serde\serde\src\ser\impls.rs
    // line: 52
    println!("serialize_string");
    let v = String::from("hello world");
    let s = serde_json::to_string(&v).unwrap();
    print!("\t{}\n", s);
}
