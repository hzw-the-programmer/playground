# LRU 缓存完整详解（原理、实现、优缺点、工程变种）

LRU = **Least Recently Used**，最近最少使用淘汰算法。

核心规则：缓存容量满时，**删掉最久没被访问过**的数据，保留近期高频访问数据。

# 一、核心设计目标

缓存容量有限，无法存放全部数据；

假设业务访问有**局部性**（热点数据短时间重复访问），LRU 保证：

1. 刚读写过的数据不容易被淘汰；

2. 长期无人访问的数据优先被清理。

# 二、理想数据结构：哈希表 + 双向链表

单独链表 / 单独哈希表都无法同时满足 O (1) 复杂度，二者组合：

1. 双向链表

   - 头部：最近刚使用（热点）
   - 尾部：最久未使用（淘汰候选）
   - 支持：头部插入、尾部删除、任意节点移动到头部 O (1)
   - 双向指针，删除中间节点不需要遍历查找前驱

2. 哈希表（HashMap / Dict）

   - key → 链表节点指针 / 引用
   - O (1) 快速定位任意 key 对应的链表节点

整体操作流程

1. 查询 get (key)

   1. 哈希表查不到 key → 返回 None（缓存未命中 miss）
   2. 查到节点：
      - 将该节点从链表原位置摘除
      - 移动到链表头部（标记为最近使用）
      - 返回节点 value（缓存命中 hit）

2. 写入 put (key, value)

分两种情况

情况 A：key 已存在

- 更新节点 value
- 节点移到链表头部

情况 B：key 不存在

1. 新建节点插入链表头部，哈希表新增映射
2. 判断缓存容量是否超限：
   - 未超限：直接结束
   - 已满：
     + 移除链表尾部节点（最久未用）
     + 哈希表同步删除该 key

复杂度

- get /put 均为 O(1)，无遍历操作。

# 三、极简伪代码实现

```python
# 双向链表节点
class Node:
    def __init__(self, k, v):
        self.key = k
        self.val = v
        self.prev = None
        self.next = None

class LRUCache:
    def __init__(self, capacity):
        self.cap = capacity
        self.cache = dict()
        # 虚拟头尾哨兵，避免判空边界
        self.head = Node(0,0)
        self.tail = Node(0,0)
        self.head.next = self.tail
        self.tail.prev = self.head

    # 把节点移到头部
    def _move_head(self, node):
        self._remove(node)
        self._add_head(node)

    # 头部新增节点
    def _add_head(self, node):
        node.prev = self.head
        node.next = self.head.next
        self.head.next.prev = node
        self.head.next = node

    # 删除任意节点
    def _remove(self, node):
        p = node.prev
        n = node.next
        p.next = n
        n.prev = p

    # 删除尾节点并返回
    def _pop_tail(self):
        del_node = self.tail.prev
        self._remove(del_node)
        return del_node

    def get(self, key):
        if key not in self.cache:
            return -1
        node = self.cache[key]
        self._move_head(node)
        return node.val

    def put(self, key, value):
        if key in self.cache:
            node = self.cache[key]
            node.val = value
            self._move_head(node)
        else:
            new_node = Node(key, value)
            self.cache[key] = new_node
            self._add_head(new_node)
            # 容量溢出，淘汰尾部
            if len(self.cache) > self.cap:
                tail_node = self._pop_tail()
                del self.cache[tail_node.key]
```

# 四、工程中的 LRU 实现方案

方案 1：语言内置封装（快速开发）

1. Python：functools.lru_cache 装饰器（基于有序字典 OrderedDict）
2. Java：LinkedHashMap，重写 removeEldestEntry 开启 LRU
3. C++：std::list + std::unordered_map 手动组合

方案 2：高性能无锁 LRU（Rust/Tokio、Redis）

- 分段 LRU（Sharded LRU）：拆分多个独立 LRU，降低锁竞争
- Tokio 内部缓存、内存池常用分段 LRU

方案 3：Redis LRU（近似 LRU，非精准）

Redis 不会维护完整双向链表，节省内存：

1. 每个 key 存储访问时间戳；
2. 淘汰时随机采样一批 key，删掉其中最旧时间戳；
3. 采样越多越接近标准 LRU，牺牲精度换极低内存开销。

# 五、LRU 优缺点

优点

1. 逻辑简单，易于实现；
2. 贴合「时间局部性」热点访问场景；
3. 读写 O (1)，性能优秀。

致命缺点（业务踩坑高频）

1. 缓存污染问题（一次性批量冷数据击穿）
   
   一次性读取大量从未访问的数据，直接把热点全部挤走，缓存失效。
   
   例：批量遍历数据库全表，大量冷数据涌入 LRU，热点被淘汰。

2. 对稀疏低频但长期有用的数据不友好

   数据隔很久访问一次，会被频繁淘汰，重复加载。

3. 精准 LRU 内存开销高

   需要存储双向链表指针，海量 key 场景内存占用明显上升。

# 六、LRU 经典优化变种（解决缺陷）

1. LRU-K

记录每个 key 最近 K 次访问时间，只有访问次数 ≥K 才放入缓存，过滤一次性冷数据。
常用 K=2（LRU2），抵御批量扫描污染。

2. LFU（最不常使用）

淘汰访问次数最少的数据，而非时间；
适合长期热点稳定场景，但新数据初始计数低，容易刚存入就被淘汰（冷启动问题）。

3. ARC（自适应替换缓存）

LRU + LFU 结合，自动切换策略，同时兼顾时间、访问频次，数据库、存储系统广泛使用。

4. SLRU（分段 LRU）

缓存分两部分：

- 保护区：长期热点，很难被淘汰

- probation 试用期：新数据，容易清理
  
  新数据先进试用期，多次访问晋升保护区，大幅缓解批量冷数据污染。

5. 2Q（Two Queue）

两个队列：近期访问队列、历史队列，和 SLRU 思路接近。

# 七、典型使用场景

1. 接口内存缓存、本地页面缓存；
2. 操作系统页缓存、文件系统缓存；
3. Redis 内存淘汰策略；
4. 数据库缓冲池（Buffer Pool）；
5. Rust/Tokio 运行时内部任务 / 资源缓存。

# 八、面试核心考点总结

1. 标准 LRU 底层结构：哈希表 + 双向链表，O (1) 读写；
2. 哨兵虚拟头尾节点作用：消除头尾空判断，简化代码；
3. LRU 缺陷：批量冷数据污染、冷数据易淘汰；
4. 优化方案：LRU-K / SLRU / ARC；
5. Redis 是近似 LRU，随机采样淘汰，不维护完整链表。
