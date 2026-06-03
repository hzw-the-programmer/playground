//! imageops 中的函数大多数接受 &I where I: GenericImageView 并返回一个新的 ImageBuffer。
//! 你可以按需组合，比如先模糊再缩放，链式处理非常清晰。

use image::{DynamicImage, imageops};

fn main() -> Result<(), image::ImageError> {
    let img: DynamicImage = image::open("input.jpg")?;

    // 缩放到 300x200
    let resized = imageops::resize(&img, 300, 200, imageops::FilterType::Lanczos3);
    resized.save("resized.png")?;

    // 高斯模糊，sigma = 2.0
    let blurred = imageops::blur(&img, 2.0);
    blurred.save("blurred.png")?;

    // 旋转 90 度
    let rotated = imageops::rotate90(&img);
    rotated.save("rotated.png")?;
    Ok(())
}
