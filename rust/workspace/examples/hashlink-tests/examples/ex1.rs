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
