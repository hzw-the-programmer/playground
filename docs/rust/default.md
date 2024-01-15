rust中实现Default trait可以像go一样有默认值

go语言基础数据类型都有默认值，而且鼓励默认值可用，比如bool的默认值是false
```
var b bool
fmt.Println(b) // false
```

rust的基础数据类型也有默认值，参见如下文件157行
$(RUSTUP_HOME)\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\default.rs:157
```
default_impl! { bool, false, "Returns the default value of `false`" }
```

```
let b: bool = Defaut::default();
println!("{:?}", b); // false

let b = bool::default();
println!("{:?}", b);
```

如果结构体所有field都实现了Default trait，那么结构体可以derive Default trait
```
#[derive(Default)]
struct A {
    f1: i32,
    f2: u8,
}
```
我们在初始化A的时候，如果希望f1为1，其他使用默认值
```
let a = A {
    f1: 1,
    ..Default::default()
};
```