```
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

```
#[link(name = "mylib", kind = "static")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    println!("{}", unsafe { add(1, 1) });
}
```
mylib.lib放到crate根目录可行