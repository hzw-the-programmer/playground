use std::sync::atomic::{AtomicI64, Ordering};

/*
# 设计一个线程安全的计数器（支持并发增减、读取）

## 题目要求
* 实现一个 AtomicCounter 结构体，支持：
  1. 原子化增加 / 减少计数（inc()/dec()）；
  2. 原子化读取当前计数（get()）；
  3. 线程安全（多线程并发调用无数据竞争）；
  4. 禁止使用 std::sync::Mutex（考察原子类型）。

## 设计思路
* 核心：使用 Rust 原子类型 std::sync::atomic::AtomicI64（无锁原子操作）；
* 内存顺序：使用 Ordering::SeqCst（顺序一致性，简单且安全）；
* 封装：将原子类型封装在结构体中，对外暴露简洁接口。

## 关键考点
* 原子类型（Atomic*）的使用；
* 内存顺序（Ordering）的选择；
* 线程安全的封装（Send/Sync 自动推导）。
*/

pub struct AtomicCounter {
    count: AtomicI64,
}

impl AtomicCounter {
    pub fn new(initial: i64) -> Self {
        Self {
            count: AtomicI64::new(initial),
        }
    }

    pub fn inc(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn dec(&self) {
        self.count.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn get(&self) -> i64 {
        self.count.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests;
