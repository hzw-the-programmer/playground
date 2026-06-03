//! `GenericImageView` 是所有只读图像视图必须实现的 trait，其关联类型 `Pixel` 定义了像素格式。
//! 通过 trait 约束，你写出的函数可以零成本抽象处理多种图像类型，而无需枚举分发。
//! 这种设计正是 image 库的灵魂——让你体会到 Rust 静态派发的力量。

use image::{DynamicImage, GenericImageView, Pixel};

/// 统计任意图像中，所有像素的通道值总和
fn sum_channels<I>(img: &I) -> u64
where
    I: GenericImageView,
    I::Pixel: Pixel<Subpixel = u8>, // 确保通道是 u8
{
    img.pixels()
        .map(|(_, _, p)| p.channels().iter().map(|&ch| ch as u64).sum::<u64>())
        .sum()
}

fn main() {
    let img1: DynamicImage = image::open("input.jpg").unwrap();
    let img2 = img1.grayscale();
    println!("原图通道总和: {}", sum_channels(&img1));
    println!("灰度图通道总和: {}", sum_channels(&img2));
}
