//! `image::open` 返回 `DynamicImage`，它是一个枚举，涵盖了 `Rgba8`, `Rgb8`, `Luma8` 等多种像素格式。
//! `save` 会根据文件扩展名自动选择编码器。
//! `grayscale()` 等转换方法内部使用 `match` 分发到具体格式的实现——这就是“运行时多态”的经典案例。

use image::{DynamicImage, GenericImageView};

fn main() -> Result<(), image::ImageError> {
    // 打开图片，自动识别格式
    let img: DynamicImage = image::open("input.jpg")?;
    println!("尺寸: {:?}, 颜色类型: {:?}", img.dimensions(), img.color());

    // 保存为 PNG
    img.save("output.png")?;

    // 直接转换为 8 位灰度图并保存
    img.grayscale().save("gray.png")?;
    Ok(())
}
