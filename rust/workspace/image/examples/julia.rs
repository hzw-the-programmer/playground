use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{
    draw_antialiased_line_segment_mut, draw_filled_circle_mut, draw_filled_rect_mut,
};
use imageproc::rect::Rect;
use rand::RngExt;
use std::f64::consts::PI;

/// 简单的 alpha 混合（前景色叠加到背景色）
fn blend(bg: Rgb<u8>, fg: Rgb<u8>, alpha: f32) -> Rgb<u8> {
    let inv = 1.0 - alpha;
    Rgb([
        (bg[0] as f32 * inv + fg[0] as f32 * alpha) as u8,
        (bg[1] as f32 * inv + fg[1] as f32 * alpha) as u8,
        (bg[2] as f32 * inv + fg[2] as f32 * alpha) as u8,
    ])
}

// ---------- 1. Julia 集 ----------
fn julia_set(width: u32, height: u32) -> RgbImage {
    let c_re = -0.7;
    let c_im = 0.27015;
    let max_iter = 300;
    let x_min = -1.5;
    let x_max = 1.5;
    let y_min = -1.5;
    let y_max = 1.5;

    let mut img = RgbImage::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let zx0 = x_min + (x as f64 / width as f64) * (x_max - x_min);
        let zy0 = y_min + (y as f64 / height as f64) * (y_max - y_min);
        let mut zx = zx0;
        let mut zy = zy0;
        let mut i = 0;
        while zx * zx + zy * zy < 4.0 && i < max_iter {
            let tmp = zx * zx - zy * zy + c_re;
            zy = 2.0 * zx * zy + c_im;
            zx = tmp;
            i += 1;
        }
        // 颜色映射：逃逸快的为暖色，未逃逸为黑
        let color = if i == max_iter {
            Rgb([0, 0, 0])
        } else {
            let t = i as f64 / max_iter as f64;
            let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
            let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
            let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
            Rgb([r, g, b])
        };
        *pixel = color;
    }
    img
}

// ---------- 2. 谢尔宾斯基地毯 (递归矩形) ----------
fn sierpinski_carpet(level: u32, size: u32) -> RgbImage {
    let mut img = RgbImage::from_pixel(size, size, Rgb([255, 255, 255]));
    draw_carpet(&mut img, 0, 0, size, level);
    img
}

fn draw_carpet(img: &mut RgbImage, x: u32, y: u32, size: u32, level: u32) {
    if level == 0 {
        return;
    }
    let sub = size / 3;
    // 中心方块涂黑
    draw_filled_rect_mut(
        img,
        Rect::at(x as i32 + sub as i32, y as i32 + sub as i32).of_size(sub, sub),
        Rgb([0, 0, 0]),
    );
    // 递归处理8个外围方块
    for dy in 0..3u32 {
        for dx in 0..3u32 {
            if dx == 1 && dy == 1 {
                continue; // 中心已涂黑
            }
            draw_carpet(img, x + dx * sub, y + dy * sub, sub, level - 1);
        }
    }
}

// ---------- 3. 彩色螺旋 ----------
fn colorful_spiral(width: u32, height: u32) -> RgbImage {
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;
    let max_radius = cx.min(cy);
    let mut img = RgbImage::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = x as f64 - cx;
        let dy = y as f64 - cy;
        let r = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx);
        // 螺旋参数：半径决定亮度，角度 + 半径/20 决定色相
        let t = (r / max_radius).min(1.0);
        let hue = (angle.to_degrees() + r * 0.3) % 360.0;
        let (r_col, g_col, b_col) = hsl_to_rgb(hue, 0.9, 0.6 - 0.3 * t);
        *pixel = Rgb([
            (r_col * 255.0) as u8,
            (g_col * 255.0) as u8,
            (b_col * 255.0) as u8,
        ]);
    }
    img
}

/// HSL -> RGB 辅助函数
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
    (r1 + m, g1 + m, b1 + m)
}

// ---------- 4. 分形树 (递归线段) ----------
fn fractal_tree(img: &mut RgbImage, x1: f64, y1: f64, angle: f64, length: f64, depth: u32) {
    if depth == 0 {
        return;
    }
    let x2 = x1 + length * angle.cos();
    let y2 = y1 + length * angle.sin();
    // 树干颜色随深度变浅
    let t = depth as f64 / 12.0;
    let color = Rgb([(100.0 + 80.0 * t) as u8, (50.0 + 100.0 * t) as u8, 20]);
    draw_antialiased_line_segment_mut(
        img,
        (x1 as i32, y1 as i32),
        (x2 as i32, y2 as i32),
        color,
        blend,
    );
    let mut rng = rand::rng();
    // 左分枝
    fractal_tree(
        img,
        x2,
        y2,
        angle - 0.45 + rng.random_range(-0.1..0.1),
        length * 0.7,
        depth - 1,
    );
    // 右分枝
    fractal_tree(
        img,
        x2,
        y2,
        angle + 0.45 + rng.random_range(-0.1..0.1),
        length * 0.7,
        depth - 1,
    );
}

fn generate_tree_image(width: u32, height: u32) -> RgbImage {
    let mut img = RgbImage::from_pixel(width, height, Rgb([240, 240, 255]));
    // 树干从底部中间开始
    fractal_tree(
        &mut img,
        width as f64 / 2.0,
        height as f64 - 20.0,
        -std::f64::consts::FRAC_PI_2, // 向上
        150.0,
        12,
    );
    img
}

// ---------- main ----------
fn main() {
    // 1. Julia 集
    let julia = julia_set(1024, 768);
    julia.save("julia_set.png").expect("save julia");

    // 2. 谢尔宾斯基地毯 (5 层)
    let carpet = sierpinski_carpet(5, 729); // 3^5 = 243? 3^6=729，用5层 3^5=243
    carpet.save("sierpinski_carpet.png").expect("save carpet");

    // 3. 彩色螺旋
    let spiral = colorful_spiral(1024, 1024);
    spiral.save("colorful_spiral.png").expect("save spiral");

    // 4. 分形树
    let tree = generate_tree_image(1024, 768);
    tree.save("fractal_tree.png").expect("save tree");

    println!("All images generated successfully!");
}
