PNG 在 IHDR 中用 color_type(1字节) + bit_depth(1字节) 组合唯一确定色彩模式，一共 5 种标准 ColorType（0/2/3/4/6），无 1、5 取值；再配合位深 (1/2/4/8/16)、可选 PLTE 调色板、可选 tRNS 透明块，衍生出日常 PNG8/PNG24/PNG32/Gray16/RGBA64 等格式。

公式：

单像素字节 = 通道数 × bit_depth / 8

# 一、总表速览（color_type 定义）

color_type	名称	通道构成	是否依赖 PLTE	原生 Alpha	Rust image 对应类型
0	灰度（Gray）	单亮度通道 Y	❌	❌	Luma<u8/u16>
2	RGB 真彩色	R+G+B 三通道	❌	❌	Rgb<u8/u16>
3	索引调色板（Palette）	索引值→查表 RGB	✅必带 PLTE	可选 tRNS	靠 PLTE 映射 Rgb
4	灰度 + Alpha (GrayA)	Y+A 双通道	❌	✅原生	LumaA<u8/u16>
6	RGBA 真彩色	R+G+B+A 四通道	❌	✅原生	Rgba<u8/u16>

关键区分：
- 原生 Alpha（type4/6）：每个像素自带 Alpha 值，支持 0~255 全范围半透明（渐变透明、阴影）；
- 透明仅靠 tRNS（type0/2/3）：只能单一色全透明，无半透明。

# 二、逐个拆解 5 种色彩模式

## 1. ColorType=0：Gray 灰度图（单通道）

规则

- 只有亮度 1 个采样，无 RGB、无内置 Alpha；
- bit_depth ∈ {1,2,4,8,16}
- 无 `PLTE`；如需透明靠可选辅助块 `tRNS` 指定某一个灰度值为全透明。

常见细分

- Gray1（1bit）：黑白二值，0 黑 1 白，图标线稿
- Gray2/Gray4：低灰度阶
- Gray8（8bit）：0~255 共 256 级灰度，最常用
- Gray16（16bit）：0~65535 高精度灰度，医疗图像 / RAW

透明规则

写入 `tRNS` 块：存一个 16bit 灰度值，图像中等于该值的像素整体透明，不能局部半透明。

## 2. ColorType=2：RGB 真彩（PNG24）

规则

- 顺序：R G B 三个独立采样，无 Alpha 通道；
- bit_depth：8 / 16（标准无 1/2/4）
- 8bit → 每像素 3Byte = PNG24
- 16bit → 每像素 6Byte = RGB48（专业高位深）
- 无 `PLTE`；透明可选 `tRNS` ：存储一组 R/G/B，匹配这个颜色则全透明。
- PNG24：8bit-RGB，网页截图、相片无透明场景主流。

## 3. ColorType=3：Palette 索引色（PNG8，最容易混淆）

PNG 独有调色板模式，体积最优，图标首选

核心原理

- 像素不存 RGB，只存调色板下标索引，真正 RGB 存在 `PLTE` 块。
- `PLTE`：N×3 字节，N∈[1,256]，每组R G B；
- 像素值 = 索引 ID，取值范围由 bit_depth 决定：
- bit1 → 0~1、bit2→0~3、bit4→0~15、bit8→0~255（最多 256 色 = PNG8）
- 透明由 `tRNS` 控制：
  + `tRNS` 内是一张透明映射表：第 N 字节 = 0 则 PLTE [N] 完全透明，非 0 不透明；
  + 只能整索引全透明，没有单像素半透明（GIF 同款透明逻辑）。

典型 PNG8

bit_depth=8 + PLTE≤256 色 + 可选 tRNS 透明，扁平化图标、表情包体积远小于 PNG32。

限制：最多 256 种颜色，色彩丰富照片不适合。

## 4. ColorType=4：Gray+Alpha（LA，灰度带透明）

- 双通道：Y(亮度)+A(透明度)，原生 Alpha
- bit_depth=8 → LA8：每像素 2 字节（1 灰 + 1 透明）
- bit_depth=16 → LA32：每像素 4 字节
- 不需要 `tRNS`：A 通道 0 = 完全透明，A = 最大值不透明，中间值渐变半透明。

灰度透明图标专用。

## 5. ColorType=6：RGBA（PNG32）全彩带透明

- 四通道：R G B A，原生 Alpha
- bit_depth=8：每像素 4Byte → PNG32（UI 透明、阴影、渐变标配）
- bit_depth=16：每像素 8Byte → RGBA64，印刷 / 影视高位深素材
- Alpha 逐像素独立：0 全透→255 不透明，任意半透明渐变，是 PNG 最强透明能力。

# 三、bit_depth（位深）对每种模式的约束

不是所有位深全可用，PNG 标准强制约束：

- Type0 (灰度)：1/2/4/8/16 ✅全支持
- Type2 (RGB)：仅 8/16
- Type3 (索引)：1/2/4/8
- Type4 (GrayA)：仅 8/16
- Type6 (RGBA)：仅 8/16

1/2/4bit 基本只用在灰度、索引色，真彩不用低位深。

# 四、`tRNS` 透明补充：三种透明实现方式（重中之重）

PNG 三种透明实现，对应三类色彩模式：

- 原生 Alpha（Type4/6）：像素自带 A，全阶半透明（最优）
- 单色透明 `tRNS` (Type0/2)：固定某一个色值全局透明，无半透明
- 索引透明 `tRNS` (Type3)：指定若干调色板索引全透明，无半透明

结论：想要渐变半透明，只能 Type4 / Type6（GrayA/RGBA），PNG8/PNG24 做不出半透明。

# 五、日常叫法对应标准色彩模式

俗称	标准类型	color_type	bit_depth
PNG8	索引调色板	3	8
PNG24	RGB 真彩	2	8
PNG32	RGBA 真彩带 Alpha	6	8
Gray8	8 位灰度	0	8
RGBA64	16 位高位深 RGBA	6	16

# 六、Rust png/image 库映射关系

png crate 从 IHDR 读出(color_type, bit_depth) → 映射到 image 固定 Pixel：

```plaintext
type0+8 → Luma<u8>
type0+16 → Luma<u16>
type2+8 → Rgb<u8>
type6+8 → Rgba<u8>
type4+8 → LumaA<u8>
type3 → 读取PLTE，运行时转Rgb
```

# 七、拓展：`sRGB`/`gAMA`/`iCCP` 色彩配置（不改变色彩模式）

sRGB/gAMA/iCCP 只是色彩空间描述块，不修改像素存储格式：

- `sRGB`：标记遵循 `sRGB` 色域（绝大多数屏幕默认）
- `gAMA`：伽马校正值
- `iCCP`：自定义 ICC 色彩配置文件

读取后仅做色彩空间转换，不会变更 color_type 与位深。

# 八、快速选型建议

- 扁平化图标、少量色彩、单色透明 → PNG8(type3)（体积最小）
- 照片、无透明图片 → PNG24(type2)
- UI 素材、阴影、渐变半透明 → PNG32(type6)
- 黑白透明图标 → GrayA (type4)
