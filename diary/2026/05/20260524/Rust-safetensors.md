# 一、官方 Rust 库（唯一推荐）

safetensors

- 官方：huggingface/safetensors
- Rust 原生实现
- 安全、零拷贝、无 unsafe、无依赖
- 支持：读取、写入、内存映射 (mmap)

安装（Cargo.toml）

```toml
[dependencies]
safetensors = "0.7"       # 核心解析
memmap2 = "0.9"           # 可选：mmap 零拷贝加载
ndarray = "0.15"          # 可选：张量运算
```

---

# 二、最简单示例：读取 safetensors

```rust
use safetensors::SafeTensors;
use std::fs::read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 读取文件到内存
    let data = read("model.safetensors")?;

    // 2. 解析 safetensors 格式（校验头部 + 边界）
    let tensors = SafeTensors::deserialize(&data)?;

    // 3. 遍历所有张量
    for (name, tensor) in tensors.iter() {
        println!("张量名: {}", name);
        println!("  数据类型: {:?}", tensor.dtype());
        println!("  形状: {:?}", tensor.shape());
        println!("  数据长度: {} 字节", tensor.data().len());
    }

    // 4. 获取单个张量
    let wq = tensors.tensor("layers.0.wq.weight")?;
    println!("wq 数据: {:?}", &wq.data()[0..16]); // 查看前16字节

    Ok(())
}
```

---

# 三、高级：零拷贝 mmap 加载（超大模型必备）

不占内存，几毫秒加载 10GB 模型

```rust
use safetensors::SafeTensors;
use memmap2::Mmap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 内存映射文件（零拷贝）
    let file = File::open("model.safetensors")?;
    let mmap = unsafe { Mmap::map(&file)? };

    // 直接解析，不复制任何数据
    let tensors = SafeTensors::deserialize(&mmap)?;

    let tensor = tensors.tensor("embedding.weight")?;
    println!("形状: {:?}", tensor.shape());

    Ok(())
}
```

---

# 四、写入 safetensors


```rust
use safetensors::{
    SafeTensors,
    tensor::{Tensor, Dtype},
};
use std::fs::write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建张量数据
    let data1 = vec![1.0f32, 2.0, 3.0, 4.0];
    let data2 = vec![10u8, 20, 30];

    // 2. 构造张量
    let tensors = vec![
        (
            "fc1.weight".to_string(),
            Tensor::new(
                Dtype::F32,
                vec![2, 2], // shape
                &data1,
            )?,
        ),
        (
            "labels".to_string(),
            Tensor::new(
                Dtype::U8,
                vec![3],
                &data2,
            )?,
        ),
    ];

    // 3. 序列化
    let serialized = SafeTensors::serialize(tensors)?;

    // 4. 保存文件
    write("output.safetensors", serialized)?;

    Ok(())
}
```

---

# 五、支持的数据类型（Dtype）

```rust
Dtype::F16
Dtype::BF16
Dtype::F32
Dtype::F64
Dtype::I8
Dtype::U8
Dtype::I16
Dtype::U16
Dtype::I32
Dtype::U32
Dtype::I64
Dtype::U64
```

和 Hugging Face 模型完全对齐

---

# 六、配合 ndarray 使用（张量计算）

```rust
use safetensors::SafeTensors;
use ndarray::ArrayViewD;

let tensors = SafeTensors::deserialize(&data)?;
let tensor = tensors.tensor("q.weight")?;

// 直接转为 ndarray （零拷贝）
let arr = ArrayViewD::from_shape(tensor.shape(), tensor.data())?;
```

---

# 七、配合 tch-rs (PyTorch Rust) 使用

```rust
use safetensors::tch::load_multi_safetensors;
use tch::Tensor;

let tensors = load_multi_safetensors(&["file1.safetensors", "file2.safetensors"])?;
let w = tensors.get("w").unwrap();
```

---

# 八、这个库的核心优势（生产级）

- ✅ 官方 Hugging Face 维护
- ✅ 100% 安全，无 unsafe 代码
- ✅ 零拷贝、mmap 支持
- ✅ 极快：10GB 模型毫秒级打开
- ✅ 无任何第三方依赖
- ✅ 跨平台：Linux / Mac / Windows / WASM 全支持
- ✅ 严格校验：防止越界、恶意文件攻击

---

# 九、一句话总结

Rust 解析 safetensors 唯一官方库 = safetensors
简单、安全、零拷贝、生产可用。
