# 图书管理系统

[English](README.md) | 中文

## 环境
- Rust
- MySQL 5.7

## 使用方法
1. 执行 `init.sql` 创建数据库表。
2. 在 `.env` 设置环境变量 `DATABASE_URL` 和 `JWT_SECRET`。
3. 执行 `run.sh`。


## API

### 用户
- /user/register
- /user/login

### 图书
> 需要在 header 中设置 JWT `Authorization: Bearer <JWT>`
- /book/create
- /book/search
- /book/update
- /book/delete

## 练习
### 用自定义错误替换 HTTP 状态码
1. 用 [thiserror](https://github.com/dtolnay/thiserror) 定义 `CustomError`。
2. 为 `CustomError` 实现 `IntoResponse`。 例子: [AuthError](https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs#L142)。
3. 在 handler 中，用 `Result<Json<T>, CustomError>` 替换 `Result<Json<T>, StatusCode>`。
4. 在 `map_err` 中，把错误转换成 `CustomError`。

### 全局 404 处理器
1. 在 `src/bin/server.rs` 中，添加 [Global-404-handler](https://github.com/tokio-rs/axum/tree/main/examples/global-404-handler)。
