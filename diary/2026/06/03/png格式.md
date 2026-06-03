# 一、整体文件结构（固定：8 字节签名 + 连续 Chunk 块）

## 1. 文件魔数（固定头部 8 字节，用来识别 PNG）

十六进制：`89 50 4E 47 0D 0A 1A 0A`

- `50 4E 47` = ASCII `PNG`
- `89`：防止文本编辑器误改二进制；`0D 0A` Windows 换行、`1A` DOS 结束符、`0A` Unix 换行，用来规避跨系统传输损坏。

## 2. 通用 Chunk 块统一格式（所有块结构完全一致，大端 Big-Endian）

每个 Chunk 由 4 段组成，Rust png crate 就是按此结构体逐条解析：

字段	长度	说明
Length	4B	后面 Data 域字节数（不含自身、Type、CRC）
Type	4B	块类型 (IHDR/IDAT/IEND 等)，大小写位区分关键 / 辅助块
Data	N 字节	块实际数据，长度由 Length 决定
CRC	4B	循环冗余校验，校验 Type+Data，损坏直接解码失败

块分类：关键块 (Critical) + 辅助块 (Ancillary)

- 关键块（必须存在，解码器必须识别）：`IHDR` → [`PLTE`] → `IDAT` (多块连续) → `IEND`（顺序固定不可乱）
- 辅助块（可选，小写开头，识别不了可跳过）：`tEXt`/`gAMA`/`sRGB`/`iCCP`/`tRNS` 等，存注释、伽马、色域、透明索引

# 二、四大关键 Chunk 详细解析（Rust 解码核心）

## 1. `IHDR`（文件头块，第一个块，唯一、不可缺，13 字节固定 Data）

Data 字段排布（共 13 字节）：

```plaintext
[4B宽][4B高][1B位深bit_depth][1B颜色类型color_type][1B压缩方法][1B滤波方法][1B隔行方式]
```

1. 宽、高 (u32 大端)：图像分辨率
2. bit_depth 位深：1/2/4/8/16 bit / 采样（每个颜色样本占用 bit）
3. color_type 颜色类型（核心，决定像素格式，5 种枚举）：

   | 数值 | 类型             | 通俗叫法     | 通道             | Alpha       |
   | ---- | ---------------- | ------------ | ---------------- | ----------- |
   | 0    | 灰度 (Gray)      | PNG-Gray8/16 | 单通道           | ❌           |
   | 2    | 真彩 RGB         | PNG24        | R/G/B 3 通道     | ❌           |
   | 3    | 索引色 (Palette) | PNG8         | 索引→PLTE 调色板 | 可选 (tRNS) |
   | 4    | 灰度 + Alpha     | GrayA        | 灰度 + Alpha     | ✅           |
   | 6    | RGBA 真彩        | PNG32        | R/G/B/A 4 通道   | ✅           |

   Rust image 库 Luma/Rgb/LumaA/Rgba 像素类型直接对应上面 5 种。
4. 压缩方法：固定 0=DEFLATE (zlib)，仅此一种标准算法
5. 滤波方法：固定 0 = 自适应 5 种行滤波，标准唯一
6. 隔行：0 = 不隔行 (逐行存储)；1=Adam7 隔行（7 遍分步载入，渐进预览）

## 2. `PLTE`（调色板块，仅 color_type=3 索引色时存在）

- 每 3 字节 = 1 组 RGB (红、绿、蓝)，最多 256 组 → 最大 768 字节，PNG8 的 256 色来源。
- 像素存调色板索引值（1/2/4/8bit），大幅缩小体积，图标首选 PNG8。

## 3. `IDAT`（图像数据块，可多个连续拼接，原图压缩数据本体）

- 所有原始像素经过：行滤波预处理 → zlib+DEFLATE 压缩 → 存入 `IDAT`，多个 `IDAT` 是为了流式分段写入文件。
- Rust 解码顺序：`IDAT` 拼接→zlib 解压→逆滤波→原始像素。

## 4. `IEND`（结束块，最后一个，无 Data，Length=0）

标志 PNG 文件终止，固定 Type=`IEND`，文件末尾必备。

# 三、辅助常用 Chunk（拓展信息，image-rs 会读取解析）

块名	作用
`tRNS`	透明索引：索引色指定某号调色板全透明；灰度 / RGB 指定单色透明
`tEXt`/`zTXt`	文本注释、作者、备注（`zTXt` 是压缩文本）
`gAMA`/`sRGB`/`iCCP`	伽马值、`sRGB` 色域、`ICC` 色彩配置文件（跨设备色彩统一）

# 四、PNG 双层无损压缩原理（重中之重：Filter 预处理 + DEFLATE 压缩）

PNG 压缩分两步：先行滤波去冗余 → 再 DEFLATE 无损压缩，缺一不可，也是比 BMP 体积小的关键。

## 第一步：行滤波 Filter（每行像素预处理，5 种预测算法，自适应选最优）

按扫描线逐行处理，每行开头 1 字节标记滤波类型 (0~4)，用「当前像素 = 原值 - 预测值」存差值，图像相邻像素近似，差值大多趋近 0，极大提升后续压缩率。

Filter 编号	名称	计算公式 (当前 Px，左 L，上 U，左上 UL)	适用场景
0	None	原值，不处理	随机噪点图
1	Sub	Px-L	横向渐变、横条纹
2	Up	Px-U	竖向渐变、竖条纹
3	Average	Px-(L+U)/2	平缓混合渐变
4	Paeth	Px - 最优预测 (Paeth 公式，L/U/UL 择优)	默认通用最优，绝大多数 PNG 选用

编码器逐行尝试 5 种滤波，选压缩后体积最小的方案（自适应滤波）。

## 第二步：DEFLATE 压缩（zlib 封装的 LZ77 + 哈夫曼，同 zip/gzip 算法）

1. LZ77：查找重复字节序列，用「偏移 + 长度」代替重复数据；
2. 霍夫曼编码：对出现频率高的短码、低频长码变长编码；
3. 外层套 zlib 头部 + 校验，最终二进制存入 IDAT 块。

# 五、日常 3 种 PNG 分类（PNG8 / PNG24 / PNG32）

## 1. PNG8（索引色，color_type=3）

- 位深 1/2/4/8bit，最多 256 色，依赖 `PLTE` 调色板；
- 透明：`tRNS` 指定单个索引全透明（无半透明）；
- 优点：体积极小，图标、logo、扁平化素材首选。

## 2. PNG24（RGB 真彩，color_type=2，24bit/px）

- R/G/B 各 8bit，1670 万色，无 Alpha 通道、不能半透明；
- 用于高质量无透明截图、照片。

## 3. PNG32（RGBA，color_type=6，32bit/px）

- R/G/B/A 各 8bit，256 级 Alpha 半透明，阴影 / 毛玻璃 / 不规则透明必备；
- UI 设计、带渐变透明素材标准格式。

拓展：16bit 高位深：Gray16/RGB48/RGBA64，医疗 / 摄影专业格式，image 库完全支持。

# 六、Adam7 隔行扫描（IHDR 隔行 = 1）

图像分 7 轮分步填充像素，加载时先模糊缩略、逐步变清晰，早期网页弱网浏览用，现在基本禁用（增加体积）。

# 七、APNG（动态 PNG，W3C 纳入新标准）

原生 PNG 不支持动图，APNG 靠 `acTL`/`fcTL`/`fdAT` 扩展块实现多帧动画，兼容普通 PNG 解码器（普通解码器只解析第一帧静态图），替代 GIF 动图，半透明优于 GIF。

# 八、Rust png crate 对应实现要点（对接 image-rs）

- 解析：按 Chunk 循环读取，先 `IHDR` 获取色彩配置，按需加载 `PLTE`，拼接所有 `IDAT`；
- 解压：zlib_inflate 还原滤波后行数据；
- 逆滤波：按每行 Filter 类型还原原始像素；
- 封装为 `ImageBuffer<P, Vec<u8>>`（Rgb/Rgba/Luma），上层 image 库封装 `DynamicImage`。

# 九、PNG 优缺点总结

✅ 优点：无损无画质损失、全 Alpha 透明、多色深、跨平台、无专利、CRC 校验防损坏、丰富元数据；
❌ 缺点：照片类素材压缩率不如 JPG，体积更大；无内置压缩率可调（固定 DEFLATE）。
