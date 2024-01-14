D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\serde-1.0.178\src\ser\impls.rs
21行，为primitive类型实现了Serialize trait
每个primitive类型，在序列化时，都会调用serialize函数
serialize函数接收Serializer trait，json有Serializer实现

D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\serde_json-1.0.103\src\ser.rs
13行，用json格式实现Serializer trait