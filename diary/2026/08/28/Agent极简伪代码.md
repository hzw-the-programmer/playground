# Agent 极简伪代码（核心循环：感知‑规划‑行动‑反思）

```python
# Agent 主循环：大模型作为大脑，循环直到任务完成
def agent_run(goal: str):
    memory = ShortLongMemory()       # 短期上下文 + 长期记忆
    tools = ToolRegistry()           # 注册可用工具：搜索、代码执行、邮件、数据库等

    while True:
        # 1. 感知：读取当前记忆、历史执行结果、工具返回信息
        context = memory.get_context()

        # 2. 规划：交给大模型思考，输出下一步动作，不是直接输出答案
        thought = llm_reason(
            prompt=f"""
总目标：{goal}
当前记忆与历史结果：{context}
可用工具列表：{tools.describe_all()}

输出格式：
<thought>思考现在该做什么，缺什么信息</thought>
<action>选择工具 + 参数，或者 finish(结果) 代表任务结束
"""
        )

        # 3. 判断：任务完成则退出循环
        if thought.is_finish():
            return thought.final_result

        # 4. 执行动作：调用外部工具（真正做事）
        observation = tools.execute(thought.action)

        # 5. 反思记忆：把思考、动作、工具返回结果写入记忆，进入下一轮
        memory.append(
            thought=thought,
            action=thought.action,
            observation=observation
        )
```

## 和普通生成式 AI 的代码对比

普通生成式 AI，没有 while 循环，执行一次就结束：

```python
def generative_ai(prompt: str):
    return llm(prompt)
# 输入→生成，直接返回，不会自动反复调用工具
```

## 关键要点对应 Vera Rubin 的硬件压力

1. Agent 每一轮循环，**都要跑一次大模型推理**，产生大量 token；
2. `tools.execute()` 是 CPU 工作：调度 API、网络 IO、解析返回数据、管理记忆存储；
3. 如果 CPU 慢，工具调用阻塞，GPU 就在空等，这就是黄仁勋提到的 **GPU idle（GPU 闲置浪费）**；
4. Vera CPU 就是专门加速这套循环里规划、调度、IO、记忆管理这部分负载，让 GPU 尽量不等待。

> 
> 现实工程中还会增加：失败重试、工具超时控制、记忆压缩、多 Agent 并行、权限沙箱，伪代码只保留最核心闭环逻辑。
