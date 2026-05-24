Arithmetic Operator（算术运算符）

中文：算术运算符

就是用来做数学计算的基础运算符。

---

一、Rust 里的算术运算符有哪些？

就是这 5 个：

运算符	作用	示例
+	加法	a + b
-	减法	a - b
*	乘法	a * b
/	除法	a / b
%	取模（求余）	a % b

这些就叫 arithmetic operators。

---

二、关键危险点：溢出（Overflow）

Rust 的 原生算术运算符 有一个重要行为：

- Debug 模式：溢出直接 panic
- Release 模式：溢出会 环绕（wrapping）

这就是为什么 Rust 官方强烈建议：

不确定会不会溢出时，不要直接用原生 arithmetic operator！

---

三、安全版本

为了安全，Rust 给每个算术运算符都提供了 安全方法：

原生运算符	安全检查版	饱和版	环绕版
+	checked_add	saturating_add	wrapping_add
-	checked_sub	saturating_sub	wrapping_sub
*	checked_mul	saturating_mul	wrapping_mul

---

四、最简单记忆

- arithmetic operator = + - * / %
- 直接用不安全
- 工程里推荐：
  + 计数 / 长度 → saturating_*
  + 需要判断 → checked_*

---

一句话总结

Arithmetic operator = 基础数学运算符，但直接用可能溢出崩溃，Rust 提供了 saturating /checked/wrapping 三种安全版本。
