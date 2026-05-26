`axum::Router` 是 axum 框架的核心类型，负责将 HTTP 请求路由到对应的处理函数（handler），并组合中间件、共享状态与嵌套子路由。它基于 `tower::Service` 构建，可无缝融入 Tower 生态。

---

# 1. 创建与基本路由

通过 `Router::new()` 创建一个空路由，再调用 `.route()` 添加路径和方法。

```rust
use axum::{Router, routing::get, routing::post};

async fn root() -> &'static str {
    "Hello, world!"
}

async fn create_user() -> &'static str {
    "User created"
}

let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user));
```

- `get(handler)`、`post(handler)` 等返回 `MethodRouter`，它同样实现了 `Service`。

- 同一路径可链式绑定多个方法：`.route("/items", get(list_items).post(add_item))`

---

# 2. 路径匹配

axum 使用 `matchit` 进行路径匹配，支持以下形式：

模式	示例路径	匹配说明
静态	`/about`	完全匹配
单段参数	`/users/:id`	匹配 `/users/5`，提取 `id`
多段通配符	`/files/*rest`	匹配 `/files/a/b/c`，`rest = a/b/c`
无键捕获	`/static/*`	只能访问，不能直接提取参数

在 handler 中通过提取器获取路径参数：

```rust
use axum::extract::Path;

async fn get_user(Path(id): Path<u64>) -> String {
    format!("User ID: {}", id)
}
```

通配符参数：

```rust
async fn file(Path(path): Path<String>) -> String {
    format!("Requested file: {}", path)
}

let app = Router::new().route("/files/*path", get(file));
// /files/images/logo.png  → path = "images/logo.png"
```

---

# 3. 嵌套路由

使用 `.nest()` 将某个路径前缀下的请求分派到另一个 `Router`，前缀会被自动剥离。

```rust
let user_routes = Router::new()
    .route("/", get(list_users))
    .route("/:id", get(get_user));

let app = Router::new()
    .nest("/users", user_routes);
// 匹配 /users、/users/42
```

若需嵌套一个非 `Router` 的 `Service`（如 `tower_http::services::ServeDir`），使用 `.nest_service()`：

```rust
use tower_http::services::ServeDir;

let app = Router::new()
    .nest_service("/static", ServeDir::new("assets"));
```

`.nest_service()` 同样会剥离匹配前缀，再把剩余路径交给内部服务。

---

# 4. 合并路由（模块化）

`.merge()` 可将另一个 `Router` 的所有路由合并进来，适用于按功能拆分路由模块。

```rust
mod admin;
mod public;

let app = public::routes()
    .merge(admin::routes());
```

合并时会进行冲突检查：若存在完全相同的路径和方法组合，后合并的会覆盖前者，并在 debug 模式下触发 panic（可通过 `Router::merge_fallible` 处理错误）。

---

# 5. 共享状态

`.with_state()` 为路由树注入全局状态，`handler` 通过 `State` 提取器获取。

```rust
use std::sync::Arc;
use axum::extract::State;

#[derive(Clone)]
struct AppState {
    db: DbPool,
}

async fn dashboard(State(state): State<Arc<AppState>>) -> String {
    // 使用 state.db
    String::from("dashboard")
}

let state = Arc::new(AppState { db: pool });
let app = Router::new()
    .route("/dashboard", get(dashboard))
    .with_state(state);
```

要点：

- 状态需要实现 `Clone`，通常包装在 `Arc` 中。

- 只要调用了 `with_state`，所有需要 `State` 的 `handler` 都必须使用它；若某些 `handler` 不需要，可不写 `State` 参数。

- `Router` 的类型参数 `S` 会变为你的状态类型。若没调用 `with_state`，默认为 `()`。

---

# 6. 中间件

## 6.1 全局中间件（对整个 Router）

使用 `.layer()` 添加任意 Tower 中间件：

```rust
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;

let app = Router::new()
    .route("/", get(root))
    .layer(TraceLayer::new_for_http());
```

`ServiceBuilder` 可组合多层中间件并应用。

---

## 6.2 单路由中间件

`.route_layer()` 只对紧邻的上一条 `.route()` 生效：

```rust
use axum::middleware;

async fn require_auth() -> ... { ... }

let app = Router::new()
    .route("/secret", get(secret))
    .route_layer(middleware::from_fn(require_auth));
// 只有 /secret 会经过 require_auth
```

## 6.3 嵌套中的中间件

在嵌套的 `Router` 上应用 `.layer()`，中间件仅对该子树生效：

```rust
let admin_routes = Router::new()
    .route("/dashboard", get(admin_dashboard))
    .layer(middleware::from_fn(admin_auth));

let app = Router::new()
    .nest("/admin", admin_routes);
```

---

# 7. 回退处理（404 等）

未匹配到任何路由时，axum 默认返回空 404。可通过 `.fallback()` 自定义：

```rust
async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing here")
}

let app = Router::new()
    .route("/", get(root))
    .fallback(not_found);
```

若回退需要状态或更多逻辑，使用 `.fallback_service()` 并提供任意 `Service`。

---

# 8. 错误处理与 Router 的错误类型

`Router` 实现 `Service<Request<Body>>`，其 `Error` 类型为 `Infallible`。这意味着它永远不会产生错误（从 `Service` 契约角度），因为所有 `handler` 返回的错误（如 `Result`）会被 axum 自动转换为 HTTP 响应（如 500）。因此，当你把 `Router` 传给 `axum::serve` 时，可以安全地假设服务不会失败。

如果需要显式处理整个路由器级别的错误（极少见），可用 `tower::ServiceBuilder` 包裹，但通常不需要。

---

# 9. 将 Router 转换为服务器

使用 `axum::serve` 配合 `tokio::net::TcpListener` 运行：

```rust
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

`axum::serve` 内部会将 `Router` 转换为 `MakeService`，然后通过 `hyper` 提供服务。也可手动调用 `.into_make_service()` 适配不同的监听器。

---

# 10. 内部原理摘要

- `Service` 实现：`Router<S>` 实现了 `tower::Service<hyper::Request<hyper::Body>>`，`call()` 方法会解析请求路径，在 radix tree（由 `matchit` 提供）中查找匹配的路由，并执行对应的 handler 或嵌套服务。

- 状态注入：`with_state` 会将状态保存到 `Router` 中；在执行 handler 前，axum 会从请求扩展中设置状态，使得 `State` 提取器能工作。

- 中间件包装：`.layer()` 本质上是 Tower 的 `ServiceBuilder` 的语法糖，它会包裹整个 `Router`，每个请求经过外层中间件，再进入路由匹配。

- 路径匹配优先级：静态路径优先级高于参数段，更具体的模式优先。匹配顺序符合 `matchit` 规则，因此无需担心定义顺序导致匹配异常（但合并路由时需注意覆盖）。

---

# 11. 完整模块化示例

```rust
// main.rs
mod routes;
use std::sync::Arc;
use axum::Router;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("...").await.unwrap();
    let state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .merge(routes::public::router())
        .nest("/admin", routes::admin::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .fallback(routes::not_found);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// routes/mod.rs
pub mod admin;
pub mod public;
use axum::response::IntoResponse;

pub async fn not_found() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "Page not found")
}

// routes/public.rs
use axum::{Router, routing::get};

pub fn router() -> Router<()> {
    Router::new()
        .route("/", get(|| async { "Home" }))
        .route("/about", get(|| async { "About" }))
}

// routes/admin.rs
use axum::{Router, routing::get, middleware, extract::State};
use std::sync::Arc;
use crate::AppState;

async fn dashboard(State(_state): State<Arc<AppState>>) -> &'static str {
    "Admin Dashboard"
}

async fn auth_middleware() -> ... { /* ... */ }

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .layer(middleware::from_fn(auth_middleware))
}
```

---

# 12. 注意事项

- 路由顺序与合并：使用 `merge` 时，若存在冲突，后合并的路由优先级更高。为保持可预测性，尽量让各个子模块路由无重叠。

- 状态类型一致：`with_state` 后 `Router` 的类型变为 `Router<S>`，嵌套或合并的路由也必须兼容同一状态类型，或为 `Router<()>`（可自动提升）。

- `nest` 与 `nest_service` 剥离前缀：请求 `/users/42` 进入 `nest("/users", ...)` 后，内部路由看到的路径是 `/42`。

- 大小写敏感：默认路径匹配区分大小写。

`Router` 的设计让 axum 应用可以高度模块化，同时保持与 Tower 生态的无缝集成，是构建可维护、可扩展 HTTP 服务的基础。
