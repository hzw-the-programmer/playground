新 Range API 由 RFC 3550 定义，旨在用一套更安全、统一的全新类型取代自 Rust 1.0 起就存在的“旧”范围类型。

# 🏗️ 核心理念的变化：从“既是...又是...”到“各司其职”

新旧 API 最根本的区别在于类型职责的分离。

- 旧 `Range<Idx>` 类型：身兼多职。它既是一个数据结构（代表一个范围），本身又直接实现了 Iterator trait。这种方式带来了一些长期困扰 Rust 社区的严重问题，例如旧范围类型为了直接作为迭代器使用，必须排除 Copy trait，且其庞大的 API 表面与其他标准库迭代器不一致。

特性	旧 Range<Idx>	新 Range<Idx> (你的代码)
Iterator 实现	直接实现 (impl Iterator for Range<Idx>)	不再直接实现，通过 .into_iter() 转换为迭代器
Copy 实现	未实现 (与 Iterator 冲突)	派生 (Derive) 了 Copy
公共字段	start 和 end	start 和 end (与旧版一致)

这种“各司其职”的设计，清晰地解决了旧有模式的问题：

- 旧有问题的解决：新范围类型不再直接实现 `Iterator`，因此可以安全地实现 `Copy`。

- `IntoIterator` 的实现：新范围类型转而实现了 `IntoIterator` trait，允许你通过显式的 `.into_iter()` 调用，将它们按值消耗（move）并转化为一个专门的迭代器类型，例如 `IterRange<Idx>`。

- 提供精炼的 API：为了保持旧代码中类似 `(0..5).rev()` 的便利性，新类型会以固有方法（inherent methods）的形式，精选并直接提供少量常用的迭代器方法（如 `.rev()`）。这既提供了方便，又避免了直接暴露整个 `Iterator` trait 的复杂性，保持了 API 的纯净。

# ✨ 新 API 一览

新设计的 API 由三部分组成：类型、迭代器和转换。

## 核心范围类型

新的范围类型定义在 core::range 模块下，结构更清晰。

## 专门的迭代器类型

当你在新范围类型上调用 .into_iter() 时，会得到以下专门的迭代器：

- Range → IterRange<Idx>

- RangeInclusive → IterRangeInclusive<Idx>

- RangeFrom → IterRangeFrom<Idx>



