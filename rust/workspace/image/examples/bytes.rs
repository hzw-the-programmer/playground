//! `as_bytes()` / `as_mut_bytes()` 提供了对底层 `[u8]` 的访问，前提是缓冲区布局为紧密、无填充（`ImageBuffer` 保证这一点）。
//! 当需要跨行、带步长的图像数据时，可以使用 `flat` 模块的 `FlatSamples`，它通过安全 API 封装了 `unsafe` 细节。这部分内容属于进阶主题，建议阅读源码学习。

use image::{ImageBuffer, Rgb};

fn main() {
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);

    // 安全地获取连续字节的可变切片
    let bytes = img.as_mut();
    // 将所有字节设为 128（灰色）
    bytes.fill(128);

    // 验证：第一个像素现在是 [128,128,128]
    assert_eq!(img.get_pixel(0, 0), &Rgb([128, 128, 128]));
    img.save("grey.png").unwrap();

    // 更复杂的布局（如步长不为 width*3）需要用 unsafe 访问，但 ImageBuffer 内部保证了
    // 缓冲区是连续且紧密排列的，因此上述操作是安全的。
}
