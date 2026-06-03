//! `ImageBuffer` 提供了 `.pixels()`、`.enumerate_pixels()`（带坐标）等迭代器，它们是惰性的。
//! 你可以无缝使用 `map`、`filter`、`fold` 等标准迭代器方法。
//! `from_fn` 展示了用闭包生成图像的声明式风格，非常灵活。

use image::{ImageBuffer, Luma};

fn main() {
    // 用 from_fn 根据坐标生成渐变的灰度图
    let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(256, 256, |x, y| {
        let intensity = ((x + y) as f32 / 2.0 * 255.0 / 256.0) as u8;
        Luma([intensity])
    });
    img.save("gradient.png").unwrap();

    // 遍历所有像素，统计平均亮度
    let total: u64 = img
        .pixels() // 迭代器 Item = &Luma<u8>
        .map(|p| p.0[0] as u64)
        .sum();
    let count = img.width() as u64 * img.height() as u64;
    println!("平均亮度: {}", total / count);
}
