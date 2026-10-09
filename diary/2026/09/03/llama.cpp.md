# llama.cpp 完整详解

> 
> llama.cpp：**纯 C/C++、零 Python 依赖的大模型离线推理引擎**，作者 ggerganov，MIT 协议；目标：把大模型跑在普通 PC、笔记本、安卓手机、边缘设备，是 Ollama、LM‑Studio、大量端侧应用的底层底座。
> 核心组件：**ggml（底层张量计算库） + llama 高层逻辑 + GGUF 模型格式 + 多硬件后端调度**。

## 一、整体分层架构

```
上层应用：main CLI / llama‑server / Python绑定 / Rust绑定 / Ollama
        ↓
llama 层（libllama）
    · Transformer模型逻辑、tokenizer、KV‑Cache、采样、对话循环
        ↓
ggml 张量计算库（核心底层）
    · 张量、计算图、算子、量化内核、内存管理
        ↓
ggml‑backend 硬件调度层
    · CUDA / Metal / Vulkan / HIP / SYCL / CPU‑SIMD / Hexagon NPU
```

1. **ggml**：只做推理，完全裁剪训练算子；静态计算图机制，算子融合、张量复用全部在内核完成，没有 Python 开销。
2. **libllama**：把 Transformer 拆解成 ggml 算子图；处理 tokenize、KV 缓存、采样、LoRA、CFG、Grammar 约束输出。
3. **ggml‑backend**：一套计算图，分发到不同硬件执行；支持**层卸载（layer offload）**：部分层放 GPU 显存，剩余放内存，显存不够也能跑大模型。

## 二、GGUF 模型格式（llama.cpp 唯一标准格式）

GGUF = GGML Universal Format，替代旧 GGML 格式，**单二进制文件打包全部资源**。
文件内部三部分：

1. **文件头魔数**：识别 GGUF 版本；
2. **元数据区**：模型架构、参数量、上下文窗口、量化类型、分组大小、分词器、chat_template；
3. **张量权重区**：量化后的权重，按页对齐，支持`mmap`内存映射加载。

### 关键特性

1. **mmap 支持**：权重不需要一次性读入内存；操作系统按需 page‑in，内存不足时系统自动换出；手机 / 小内存设备的核心能力。
2. 原生支持各类量化：Q2_K / Q3_K_M / Q4_K_M / Q5_K_M / Q6_K / Q8_0；AWQ/GPTQ 量化权重可转为 GGUF。
3. 可携带 LoRA 适配器、多模态投影权重。

> 
> 工作流：HuggingFace safetensors → `convert.py` → GGUF；再用`quantize`工具做二次量化压缩。

## 三、核心关键技术

### 1. 后训练量化 PTQ

支持 FP16 / Q8_0 / Q4_K / Q3_K 等；推理时**运行时反量化**：权重以低比特存储，计算时实时转回 FP16/F32，不占用额外存储，消耗少量 CPU/GPU 算力。

- Q4_K_M：7B 模型≈3.8GB，PC / 手机端最常用档位；
- 量化不是越小组越好：Q2_K 速度快，但语义能力明显下降。

### 2. mmap 权重加载

> 
> 这是 llama.cpp 能在小内存设备跑 7B 模型的基石。

- 不把整个 GGUF 读进内存；进程拿到虚拟地址；访问哪一块权重，操作系统才从磁盘读入物理页；
- 内存紧张时 OS 自动回收冷页面；多进程可以共享同一份模型权重；
- 缺点：磁盘 IO 速度会影响冷 token 生成速度。

### 3. KV‑Cache 完整实现

- 预分配 KV 缓存张量；Prefill 阶段一次性填充；Decode 阶段只追加新 token，不再重复计算历史 prompt；
- 支持 KV‑Cache 量化、滑动窗口、分页式 KV（模仿 vLLM PagedAttention 思想）；
- 多轮对话不需要重新处理整个历史，直接复用缓存，大幅降低开销。

### 4. 静态计算图 + 算子融合

ggml 采用**先构建图，再一次性执行**的静态图模式：

1. 根据输入 token 构建完整算子 DAG；
2. 自动算子融合：把`matmul + add + rms_norm + silu`等连续小算子合并成一个内核；减少内存读写、减少内核调度开销；
3. 张量内存复用：多个中间张量复用同一块内存，显著降低内存占用。

### 5. 硬件后端 & 层卸载 offloading

`‑‑n‑gpu‑layers N`：把前 N 层 Transformer 放到 GPU，剩下的层跑 CPU。

表格

| 后端    | 适用硬件                    | 备注                                    |
| ------- | --------------------------- | --------------------------------------- |
| CUDA    | NVIDIA 显卡                 | 性能最优，支持 FlashAttention2、FP4/FP8 |
| Metal   | Apple Silicon               | 统一内存，CPU/GPU 零拷贝                |
| HIP     | AMD Linux                   | ROCm 环境                               |
| Vulkan  | AMD Windows、Intel、安卓    | 跨平台通用后端                          |
| SYCL    | Intel Arc / Intel AI PC NPU | Intel 显卡 / NPU                        |
| CPU     | x86‑64 / ARM64              | AVX2 / NEON / AVX‑512 SIMD 优化         |
| Hexagon | 骁龙 NPU                    | 安卓端侧大模型后端                      |

> 
> 典型场景：8GB 显存显卡跑 13B Q4_K_M：设置`‑‑n‑gpu‑layers 20`，一部分层放显存，剩余走内存。

### 6. 采样与高级能力

完整实现：temperature、top‑p、top‑k、min‑p、重复惩罚；
额外特色：

- **Grammar 约束输出**：强制输出 JSON、CSV 等结构化格式；
- 投机采样 Speculative Decoding：小模型预生成，大模型校验，提升 token 生成速度；
- LoRA 热加载，不需要重加载主模型。

## 四、完整推理流程（从输入文本到输出 token）

1. **初始化后端**：探测硬件，加载 CUDA/Metal/Vulkan 等后端；
2. **加载 GGUF 模型**：mmap 映射权重，初始化 KV‑Cache 缓冲区；
3. **Tokenize**：用户文本转为 token id 序列；
4. **Prefill 阶段**：输入 prompt，构建 ggml 计算图，执行前向，填充 KV‑Cache；输出第一个 token 前的 logits；
5. **Decode 循环（自回归）**
   1. 用上一步输出的 token，构建增量计算图；
   2. 执行推理，更新 KV‑Cache；
   3. logits 经过采样器，选出下一个 token；
   4. token 转回文本输出；
   5. 循环直到输出终止符；
6. 释放资源，可保留 KV‑Cache 继续多轮对话。

> 
> 两个关键性能指标：
> 
> 
> - **PP(prefill tok/s)**：prompt 处理速度；
> - **TG(decode tok/s)**：逐 token 生成速度，用户直观感受到的速度。

## 五、生态与工具链

llama.cpp 仓库自带工具：

1. `main`：CLI 对话程序；
2. `llama‑server`：启动 HTTP OpenAI 兼容接口，其他程序可以像调用 OpenAI 一样调用本地模型；
3. `quantize`：GGUF 模型再量化；
4. `convert.py`：HuggingFace 模型转 GGUF；
5. `llama‑bench`：性能基准测试。

上层封装：

- Ollama：基于 llama.cpp，简化模型管理；
- LM‑Studio：GUI，底层调用 llama.cpp；
- Python 绑定：`llama‑cpp‑python`；
- Rust 绑定：`llama‑cpp‑rs`，Rust 生态本地推理常用。

## 六、优势、局限

✅ **优势**

1. 无 Python 依赖，二进制体积小，适合嵌入式、手机、PC 本地部署；
2. 硬件覆盖极广，老旧硬件也可以跑；
3. mmap + 层卸载，内存 / 显存紧张场景生存能力强；
4. GGUF 生态成熟，HuggingFace 海量 GGUF 模型；
5. MIT 协议，商业产品可以直接使用。

⚠️ **局限**

1. 静态图，动态 shape 灵活性弱于 PyTorch；
2. 大 batch 高并发不是强项，定位是**单机本地推理**，不是云端大并发服务；
3. Vulkan、Hexagon NPU 后端部分算子兼容性不如 CUDA；
4. 不支持训练，仅推理。

## 七、和端侧大模型手机、本地大模型 PC 的关系

1. **本地大模型 PC**：Ollama、LM‑Studio 底层就是 llama.cpp；PC 端使用 CUDA/Metal/Vulkan 后端，加载 GGUF；
2. **安卓手机端侧**：llama.cpp + Vulkan/Hexagon 后端，mmap 加载 GGUF；受限于手机内存，一般跑 3B‑7B 4bit 模型；
3. 对比 ExecuTorch：Meta ExecuTorch 面向手机 NPU 深度适配；llama.cpp 优势是跨平台，一套代码跑 PC、手机、嵌入式。
