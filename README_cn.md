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
### 用 Redis 作为缓存
1. 添加 [redis](https://github.com/redis-rs/redis-rs) 到 Cargo.toml，需要开启 feature `tokio-comp`。
> 必须使用 `async`，因为如果不用 `async`，**系统线程**会在这一行代码执行时阻塞，在执行完成前不会执行其他任务。
2. 在 src/error.rs 添加 `redis::RedisError` 。
```rust
    #[error("redis_error: {0}")]
    Redis(#[from] redis::RedisError),
```
> 如果加了 `#[from]`，[thiserror](https://github.com/dtolnay/thiserror) 会自动生成 `impl From<redis::RedisError> for CustomError`。在错误类型是`CustomResult<T>`时，你可以用 `?` 或 `.map_err(Into::into)?` 返回错误。  
> 如果不加 `#[from]`，你需要自己转换错误类型 `.map_err(|e| CustomError::Redis(e))` 或 `.map_err(CustomError::Redis)`。
3. 阅读代码 [redis/examples](https://github.com/redis-rs/redis-rs/blob/main/redis/examples/async-await.rs)。
4. 编写缓存代码。

### 全局 404 处理器
1. 在 `src/bin/server.rs` 中，添加 [Global-404-handler](https://github.com/tokio-rs/axum/tree/main/examples/global-404-handler)。
