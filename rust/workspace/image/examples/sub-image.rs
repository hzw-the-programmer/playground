//! `view` 返回 `SubImage<&I>`，它借用原图像数据，仅存储偏移量和尺寸，不复制像素。
//! 可变版本 `sub_image` 返回 `SubImage<&mut I>`，修改它会直接影响原缓冲区。
//! `SubImage` 也实现了 `GenericImageView` 和 `GenericImage`，所以你可以像操作普通图像一样操作它。

use image::{DynamicImage, GenericImage, GenericImageView};

fn main() {
    let mut img: DynamicImage = image::open("input.jpg").unwrap();

    // 获取一个不可变的子视图（不复制数据）
    let view = img.view(50, 50, 100, 100);
    // 子视图也可以使用 GenericImageView 的所有方法
    println!("子视图尺寸: {:?}", view.dimensions());
    let cropped = DynamicImage::from(view.to_image());
    cropped.save("crop.png").unwrap();

    // 获取可变子视图，在原图上画一个红色方块
    let mut sub = img.sub_image(100, 100, 50, 50);
    for y in 0..sub.height() {
        for x in 0..sub.width() {
            sub.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
        }
    }
    // 修改会直接作用在原图 img 上
    img.save("with_red_block.png").unwrap();
}
