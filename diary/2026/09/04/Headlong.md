# Headlong：基于 Unix 哲学的开源 Agent 微框架

Headlong 由 Laude Institute（MIT 附属研究机构）发布，Apache‑2.0 开源协议，**核心约 1 万行 Bash 脚本**，是一套 Persistent Agency（持久自主智能体）微型 Agent 运行框架，主打 “shells all the way down（彻底基于 Shell）”，颠覆主流 Function‑Calling Agent 范式。

## 一、核心设计思想：回归 Unix 哲学

> 
> 做一件事并做好；小工具通过文本流互相组合；一切皆文本。

主流 Agent 框架：维护一份预定义工具菜单（`search_web/read_file`），LLM 只能调用注册好的函数，能力被 Schema 约束。
**Headlong 反范式：不给 Agent 预制工具列表，直接把 Bash Shell 交给大模型。**

- LLM 输入输出天然是文本，Shell 的 stdin/stdout/ 管道 / 文件全部是文本接口，结构完全对齐；
- Agent 直接编写、执行 Bash 命令完成任务：`curl`请求网络、`jq`解析 JSON、管道串联工具，不需要写任何包装层、函数 Schema；
- 复用整个 Unix 生态，系统已有的全部命令行工具开箱即用。

## 二、核心组件 shellm（Recursive Language Model，递归语言模型）

shellm 是 Headlong 的内核，Bash 实现的递归循环：

1. 喂入全部历史上下文轨迹；
2. LLM 输出一段 Bash 脚本；
3. 在沙箱中执行脚本，捕获输出；
4. 将执行结果追加到轨迹；
5. 循环迭代，直到 Agent 输出最终结果或者触发迭代上限。

Agent**思考等价于写 Shell 命令**，思考过程就是一次次执行 shellm 循环。

## 三、标志性特性：Persistent Agency 持久自主智能体

绝大多数 Agent 属于**响应式 Reactive**：收到用户消息才唤醒，回复完毕会话结束，进入休眠。
Headlong 是**持续思考模式**：

1. Agent 后台持续运行，维持一条不间断的 “思想流”；
2. 用户消息**不是会话启动器，只是一条新观测事件**，注入思想流；
3. Agent 自主决定要不要回复、什么时候回复，无人交互时也会自主规划任务、复盘、排查问题；

> 
> 内部实例 Audel 曾经在无人值守的凌晨，自主发现代码 bug，自行调试、修复、提交代码 commit。

### 多端共享同一个心智

Slack / Telegram / TUI 终端的消息全部汇入同一条思想流。团队多人可以和同一个 Agent 对话；Agent 感知所有人的工作，主动关联信息、推送提醒。

> 
> 局限：所有对话信息对团队内部互通，不做隐私隔离，不适合私密对话。

### 完整记忆与自省

- `mem/learn/recall`：跨天长期记忆；
- Agent 可读自己的历史轨迹、甚至读取自身源码，实现自我审视；
- 支持运行时动态安装技能`skills install`。

## 四、环境依赖与快速安装

**依赖**：Bash3.2+、git、curl、jq；LLM API Key（OpenAI 兼容接口）；官方强烈推荐 Docker 做执行沙箱，规避 shell 命令安全风险。

```
# 一键安装
curl -fsSL https://headlong.ai/install.sh | bash
# 启动TUI交互界面，观察Agent思考过程
ada dash
```

GitHub 仓库：`github.com/laude‑institute/headlong`

## 五、优势与短板

✅ **优势**

1. 代码体量极小，全部源码可读完，透明可审计；
2. 零工具封装成本，直接复用 Unix 海量工具链；
3. 原生支持 7×24 小时持续自主 Agent；
4. 完整可回溯的执行轨迹，便于调试、审计 Agent 推理路径。

⚠️ **短板与风险**

1. **安全风险极高**：Agent 生成执行 Shell 命令，一旦越权会修改本地文件，必须跑在 Docker 沙箱内；
2. 属于前沿研究原型，生产可用性有限；
3. 缺少用户会话隔离，多人共用 Agent 会信息互通；
4. LLM 输出非法、破坏性 shell 指令的风险需要额外防护；
5. Bash 实现，复杂业务逻辑扩展不如 Python/Rust 生态便捷。

## 六、对比主流 Agent 框架

表格

| 维度     | Headlong                           | AutoGPT/LangGraph                                |
| -------- | ---------------------------------- | ------------------------------------------------ |
| 工具调用 | 原生 Shell 管道，无预定义 Schema   | Function‑Calling，需要注册工具、定义 JSON Schema |
| 运行模式 | 持久后台自主思考，消息作为观测事件 | 响应式，收到请求启动会话，结束休眠               |
| 实现语言 | Bash ~10k 行                       | Python，数万行以上                               |
| 工具来源 | 整个 Unix 命令行生态               | 开发者手动封装注册                               |
| 适用场景 | 研究持续自主 Agent、Shell 环境任务 | 应用开发、业务工作流编排                         |

## 七、技术启示

Headlong 提出一条 Agent 的另类路线：**不把 LLM 当做函数调度器，而是把 LLM 作为 Shell 的用户**。
Unix 本身就是一套强大的组合式工具系统，LLM 擅长文本，Shell 擅长文本驱动工具，二者天然契合。
代价是安全压力全部转移到沙箱隔离层，这也是这套框架最大的工程约束。
