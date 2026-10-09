# RESTful API 设计辅助工具分类

分为 **API 设计建模、文档生成、Mock 模拟、API 测试、规范校验 (OpenAPI/Swagger)、协作评审** 几大类；现在行业主流基本都基于 **OpenAPI(Swagger) 3.x** 作为 API 描述文件。

## 一、在线可视化设计（写 OpenAPI YAML，团队协作首选）

### 1. Swagger Editor（开源免费）

- 官网：editor.swagger.io
- 功能：在线编辑 OpenAPI yaml/json，实时预览文档；可以直接生成服务端 / 客户端骨架代码；做语法校验。
- 优点：无需安装；官方编辑器；即时报错提示；可导出 yaml 文件。
- 缺点：仅编辑器，没有数据库模型映射、版本管理。

### 2. Apifox（国产，非常流行⭐）

> 
> 集：**API 设计 + Mock + 文档 + 测试 + 自动化** 一体化

- 核心工作流：先在可视化界面设计 REST API（无需手写 YAML），自动生成 OpenAPI；一键生成 Mock 接口；写完直接做接口调试；支持团队协作、版本管理。
- 支持导入导出 OpenAPI；后端前端都在用；桌面客户端 + web 版。

> 
> 对比老工具 Postman：Apifox 强化了**先设计后开发**（API‑First），Postman 更偏向调试测试。

### 3. Postman

- 新版 Postman 同样支持 API‑First，可以新建 API 项目，维护 OpenAPI 规范；
- 强项是请求调试、集合测试；设计能力弱于 Apifox；适合已经有接口之后维护。

### 4. Stoplight Studio

- 专门面向 API‑First 设计；可视化拖拽编辑 OpenAPI，不用手写 YAML；
- 本地桌面版；支持模型复用；风格偏向专业 API 架构师；免费版有功能限制。

## 二、从数据库模型逆向生成 API 规范（DB → OpenAPI）

> 
> 如果你已经有数据库表结构，可以反向导出 API schema，加速 REST 设计

- `prisma`（Node）：Prisma Schema，可以基于模型生成 OpenAPI；搭配 prisma‑openapi 插件
- `sql‑to‑openapi`：SQL DDL 转 OpenAPI yaml；适合快速原型

> 
> ⚠️注意：逆向生成仅做初稿，**不要直接上线，需要人工修正 REST 资源命名、分页、错误返回**

## 三、代码优先（Code‑First：写代码注解自动生成 OpenAPI）

> 
> 后端代码写注解，自动产出 API 文档 OpenAPI 文件，后端常用

- SpringBoot：springdoc‑openapi（替代旧 swagger2）
- Python FastAPI：**内置自动生成 OpenAPI，非常方便**；访问 /docs
- Node(NestJS)：@nestjs/swagger
- Go(Gin)：swag + swaggo

> 
> 缺点：Code‑First 容易出现 “先写业务代码再补 API 文档”，偏离 API‑First 思想；适合中小项目。

## 四、绘图工具（画 REST 资源关系，前期架构草图）

前期还没写 OpenAPI 的时候梳理资源关系：

1. Excalidraw（在线手绘草图）：画资源实体、URL 关系，团队头脑风暴
2. Draw.io / [diagrams.net](https://diagrams.net)：画 ER 图、API 调用流程图；梳理资源之间关联
3. Mermaid（markdown 内嵌）：可以直接在文档写资源 ER 图，不需要额外软件

## 五、API 规范校验 / 风格检查（Lint）

保证团队 REST 风格统一，防止 API 写的乱七八糟

1. **Spectral（Stoplight）⭐** OpenAPI Linter
   - 对 yaml 做静态检查；自定义团队 REST 规则（URL 命名小写蛇形、HTTP 方法约束、统一错误返回格式）；可以集成 CI 流水线，提交 OpenAPI 就自动检查。
2. OpenAPI‑lint：另一个 lint 工具

## ✨两种开发范式对比

表格

| 范式                                         | 说明                                             | 推荐工具                                            |
| -------------------------------------------- | ------------------------------------------------ | --------------------------------------------------- |
| API‑First（优先设计 API 文档，再前后端开发） | 先写 OpenAPI，Mock，再写前后端代码；适合团队协作 | Apifox / Stoplight / Swagger‑Editor + Spectral Lint |
| Code‑First（后端代码注解生成文档）           | 先写后端业务代码，导出文档；适合个人快速开发     | FastAPI / SpringDoc / Nest swagger                  |

## 📌简单选型建议

1. 国内团队协作做 REST API 设计：**优先 Apifox**，一站式，可视化，降低手写 yaml 门槛
2. 自己手写 OpenAPI YAML 做规范：Swagger Editor + Spectral 做 lint 校验
3. Python 后端快速开发：直接 FastAPI（code‑first）
4. 想要 CI 流水线校验 API 风格：Spectral 集成 Gitlab/Github Action

> 
> 💡补充小提示：工具只是辅助；RESTful 本身的资源命名、无状态、合理使用 HTTP Method 这些原则还是要自己把控；工具不能自动帮你设计出符合 REST 理念的 API。
