//! `ImageBuffer<P, Container>` 是 `image` 最常用的具体图像类型。
//! `P` 是像素类型（必须实现 `Pixel` trait），例如 `Rgb<u8>`、`Rgba<u8>`、`Luma<u8>` 等。
//! `Container` 默认为 `Vec<u8>`，表示在堆上分配的连续像素数据。
//! `from_pixel`、`from_fn` 等构造器让你可以纯函数式地生成图像。

use image::{ImageBuffer, Rgb};

fn main() {
    // 创建一个 200x100 的 RGB 图像，全部填充蓝色
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(200, 100, Rgb([0, 0, 255]));

    // 设置某个像素为红色
    img.put_pixel(50, 50, Rgb([255, 0, 0]));

    // 读取像素
    let pixel = img.get_pixel(50, 50);
    println!("中心像素: {:?}", pixel.0); // 输出 [255, 0, 0]

    img.save("blue_with_red_dot.png").unwrap();
}
