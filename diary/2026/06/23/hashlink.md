# 一、基础定位

`hashlink` 是 Rust 第三方库，提供 `LruCache`，是**工业级精准 LRU 缓存**，基于 `HashMap + 双向链表` 实现，对标手写 LRU，但做了大量内存、性能、安全优化，Tokio、Tonic、各类中间件广泛使用。

```toml
# Cargo.toml
hashlink = "0.9"
```

核心特性：

1. 纯标准库，无其他依赖；
2. 单线程高性能，无 Rc/RefCell 开销，内部用原生裸指针管理双向链表；
3. 严格 O (1) get/put；
4. 自动淘汰最少使用项，支持手动淘汰、容量限制；
5. 提供缓存逐出回调、迭代器、批量操作；
6. 支持 serde 序列化（开启 feature）。

# 二、核心数据结构设计（底层原理）

## 1. 两大核心组件

(1) 哈希表 `HashMap<K, *mut Entry<K, V>>`

- key 映射到链表节点裸指针，O (1) 查找；
- 不同于手写 `Rc<RefCell>`，无引用计数，内存开销极低。

(2) 双向链表（内嵌 Entry）

```rust
struct Entry<K, V> {
    key: K,
    value: V,
    prev: *mut Entry<K, V>,
    next: *mut Entry<K, V>,
}
```

- 每个 Entry 同时是哈希表 value、双向链表节点；
- 全局维护 head、tail 两个裸指针（哨兵逻辑，无虚拟节点，靠空指针判断边界）；
- head：最近使用热点；tail：待淘汰冷数据。

## 2. 内存管理关键优势（对比手写 Rc LRU）

1. 不用 Rc：无原子增减引用计数，单线程速度大幅提升；
2. 不用 RefCell：内部通过裸指针直接修改链表前后指针，无运行时借用检查开销；
3. 所有 Entry 统一在 Vec 预分配 / 管理（底层存储池），内存连续，缓存友好；
4. Drop 自动遍历链表释放所有 Entry，无内存泄漏；
5. 所有 unsafe 边界严格封装在库内部，对外 API 100% 安全。

## 3. 淘汰逻辑

- 设定 max_capacity，插入新元素后若长度超限，自动移除 tail 节点；
- 访问 get、get_mut、contains_key 都会自动将节点移到 head（更新热度）；
- peek / peek_mut：只读不更新热度，适合只查询、不提升访问权重的场景。

# 三、完整核心 API 分类讲解

## 1. 构造与容量

```rust
// 创建固定容量LRU
LruCache::new(max_capacity: usize) -> Self

// 手动设置容量，可动态扩容缩容
cache.set_max_capacity(Some(100));
cache.set_max_capacity(None); // 无限容量，不自动淘汰

// 获取当前容量限制
cache.max_capacity() -> Option<usize>
```

## 2. 插入 put /insert

```rust
/// 插入键值，key存在则覆盖旧值，返回旧值；自动更新热度
pub fn put(&mut self, k: K, v: V) -> Option<V>

/// 批量插入，批量触发淘汰
pub fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I)
```

流程：

1. key 存在：取出节点，更新 value，移到链表头部，返回旧 value；
2. key 不存在：新建 Entry，插入哈希表 + 链表头部；
3. 若长度 > max_capacity，循环弹出 tail，同步删除哈希表映射。

## 3. 查询（区分是否更新热度，核心区分点）

```rust
// 命中→移到头部（更新热度），返回引用
pub fn get(&mut self, k: &K) -> Option<&V>

// 命中→移到头部，返回可变引用（可修改value，热度同样提升）
pub fn get_mut(&mut self, k: &K) -> Option<&mut V>

// 仅查询，**不修改链表位置、不更新热度**，只读场景专用
pub fn peek(&self, k: &K) -> Option<&V>
pub fn peek_mut(&mut self, k: &K) -> Option<&mut V>

// 仅判断存在，不更新热度
pub fn contains_key(&self, k: &K) -> bool
```

业务场景区分：

- 接口正常读取：get，提升热点；
- 定时后台巡检、统计遍历：peek，避免污染缓存热度。

## 4. 手动删除、弹出淘汰项

```rust
// 手动删除指定key，返回value
pub fn remove(&mut self, k: &K) -> Option<V>

// 强制弹出最久未使用的tail项（手动淘汰）
pub fn pop_lru(&mut self) -> Option<(K, V)>

// 清空全部缓存
pub fn clear(&mut self)
```

## 5. 长度、迭代器

```rust
// 当前缓存元素总数
pub fn len(&self) -> usize
pub fn is_empty(&self) -> bool

// 迭代：从热点head → 冷tail遍历（按访问热度从新到旧）
cache.iter() -> Iter<'_, K, V>
cache.iter_mut() -> IterMut<'_, K, V>

// 反向迭代：从冷到热
cache.iter().rev()
```

## 6. 逐出回调（非常实用的工程特性）

创建缓存时绑定回调，元素被自动淘汰 / 手动删除时触发，常用于资源释放、持久化落盘、监控统计：

```rust
// 构造时指定回调
let mut cache = LruCache::new_unbounded_with_cap_and_callback(100, |k, v| {
    // k,v 是被逐出的键值对
    println!("evict key={:?}", k);
    // 释放文件句柄、网络连接、写回数据库等
});
```

两种触发场景：

1. 容量超限自动 pop_lru；
2. 用户手动调用 remove。

## 7. 其他工具方法

```rust
// 保留最近N个热点，删除全部冷数据
cache.retain(50);

// 重置所有元素热度，等价于全部视为同等旧数据
cache.clear_lru_order();
```

# 四、极简可运行示例

```rust
use hashlink::LruCache;

fn main() {
    // 最大容量3
    let mut cache = LruCache::new(3);

    // 插入数据
    cache.insert(1, "a");
    cache.insert(2, "b");
    cache.insert(3, "c");
    println!("len:{}", cache.len()); // 3

    // get 提升热度
    cache.get(&2);
    // 插入4，容量满，淘汰最久未使用 1
    cache.insert(4, "d");

    println!("1 exists? {}", cache.contains_key(&1)); // false
    println!("{}", cache.get(&2).unwrap()); // b

    // peek 不改变热度
    cache.peek(&3);
    cache.insert(5, "e"); // 淘汰3

    // 手动弹出LRU项
    let (k, v) = cache.remove_lru().unwrap();
    println!("pop lru: {}={}", k, v);

    // 迭代，从热到冷
    for (k, v) in cache.iter() {
        println!("{}:{}", k, v);
    }
}
```

# 五、底层安全与 Unsafe 设计

库内部大量使用裸指针，但对外完全安全：

1. 所有 Entry 生命周期由 LruCache 独占管理，外部无法持有裸指针；
2. 所有链表操作、指针修改全部封装在内部方法；
3. Drop 实现完整遍历双向链表，逐个释放 Entry，无内存泄漏；
4. 迭代器生命周期绑定缓存，避免悬垂引用；
5. 禁止外部构造 Entry，所有内存分配由缓存统一控制。

对比手写 `Rc<RefCell>` LRU 性能优势：

1. 无原子操作，单线程吞吐量提升 30%~100%；
2. 内存占用更低，无双重引用计数内存；
3. 连续内存存储池，CPU 缓存命中率更高。

# 六、局限与多线程方案

1. 原生限制

LruCache 仅实现 Send、未实现 Sync，只能单线程使用，多线程并发读写会数据竞争 UB。

2. 多线程标准封装方案

方案 1：单锁全局 LRU（简单，高并发锁冲突严重）

```rust
use hashlink::LruCache;
use std::sync::Mutex;

struct SharedCache<K, V>(Mutex<LruCache<K, V>>);
```

方案 2：分片 Sharded LRU（生产最优，Tokio 运行时同款思路）

将 key 哈希取模，拆分 N 个独立 LruCache，每个分片单独一把锁，大幅降低锁竞争：

```rust
pub struct ShardedLru<K, V> {
    shards: Vec<Mutex<LruCache<K, V>>>,
    shard_count: usize,
}
```

高并发内存缓存、连接池普遍采用该模式。

# 七、对比其他 LRU 实现

实现	底层	性能	特性	适用场景
`hashlink::LruCache`	HashMap + 裸指针双向链表	极高	逐出回调、peek、迭代、动态容量	单线程内存缓存、异步运行时本地缓存
手写 `Rc<RefCell>` LRU	Rc+RefCell 节点	低	基础 get/put，无高级特性	教学演示，不推荐生产
`std::collections::BTreeMap`	有序树	极低 O (logn)	无原生 LRU	不用于缓存
Redis 近似 LRU	时间戳随机采样	极低内存开销	非精准	分布式缓存

# 八、工程使用最佳实践

1. 异步 Tokio 单 Worker 本地缓存：直接使用 hashlink::LruCache，每个 Worker 独立实例，无锁；
2. 全局多线程缓存：分片 + Mutex 封装；
3. 资源缓存（文件句柄、连接）：使用逐出回调释放资源；
4. 后台巡检统计一律使用 peek，避免打乱访问热度；
5. 热点长期稳定场景搭配 retain() 定期清理冷数据，控制内存上限。
