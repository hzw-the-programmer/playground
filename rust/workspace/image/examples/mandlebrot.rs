use image::{ImageBuffer, Rgb};

/// 复平面上的点 (x + iy)
struct Complex {
    re: f64,
    im: f64,
}

/// Mandelbrot 迭代：z_{n+1} = z_n^2 + c
/// 返回逃逸所需的迭代次数；如果未逃逸，返回 max_iters
fn mandelbrot(c: &Complex, max_iters: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max_iters {
        // z = z^2 + c
        let re2 = z.re * z.re;
        let im2 = z.im * z.im;
        if re2 + im2 > 4.0 {
            return i; // 逃逸，返回迭代次数
        }
        z.im = 2.0 * z.re * z.im + c.im;
        z.re = re2 - im2 + c.re;
    }
    max_iters // 未逃逸，属于集合内部
}

/// 将迭代次数映射为 RGB 颜色（平滑渐变）
fn iter_to_color(iter: u32, max_iters: u32) -> Rgb<u8> {
    if iter == max_iters {
        // 集合内部：黑色
        return Rgb([0, 0, 0]);
    }
    // 使用 HSL 颜色模型：色相随迭代次数变化，饱和度和亮度固定
    let t = iter as f64 / max_iters as f64;
    // 色相从深蓝(240°) 转到 红(0°) 再回到深蓝，形成循环渐变
    let hue = (240.0 + 360.0 * t) % 360.0;
    let (r, g, b) = hsl_to_rgb(hue, 0.8, 0.5);
    Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8])
}

/// 简单的 HSL -> RGB 转换（输入：色相 0..360, 饱和度/亮度 0..1）
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    ((r1 + m), (g1 + m), (b1 + m))
}

fn main() {
    // 图像尺寸
    let width = 3840; // 4K 分辨率
    let height = 2160;
    let max_iters = 500;

    // 复平面区域（展示 Mandelbrot 集经典全貌）
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.0;
    let y_max = 1.0;

    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // 遍历每个像素
    for (px, py, pixel) in img.enumerate_pixels_mut() {
        // 将像素坐标映射到复平面
        let re = x_min + (px as f64 / width as f64) * (x_max - x_min);
        let im = y_min + (py as f64 / height as f64) * (y_max - y_min);
        let c = Complex { re, im };

        let iter = mandelbrot(&c, max_iters);
        *pixel = iter_to_color(iter, max_iters);
    }

    // 保存图片
    img.save("mandelbrot_4k.png").expect("Failed to save image");
    println!("Mandelbrot image saved as mandelbrot_4k.png");
}
