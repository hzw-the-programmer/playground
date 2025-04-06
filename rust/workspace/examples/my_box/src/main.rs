mod my_box_non_null;
mod my_box_raw_ptr;
mod my_global_alloc;

fn main() {
    my_box_raw_ptr::main::test1();
    println!("");
    my_box_non_null::main::test1();
}
