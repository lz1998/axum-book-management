# Book management

English | [中文](README_cn.md)

## Required
- Rust
- MySQL 5.7

## Usage
1. Execute `init.sql` to create tables.
2. Set environment variable `DATABASE_URL` and `JWT_SECRET` in `.env`.
3. Execute `run.sh`.


## API

### user
- /user/register
- /user/login

### book
> JWT should be provided in header. `Authorization: Bearer <JWT>`
- /book/create
- /book/search
- /book/update
- /book/delete

## Practice
### Use Redis as cache
1. Add [redis](https://github.com/redis-rs/redis-rs) with feature `tokio-comp` to Cargo.toml
> `async` is necessary, because if you don't use `async`, the **system thread** will block when the command is executing, and it will not handle other tasks.
2. Add `redis::RedisError` in src/error.rs
```rust
    #[error("redis_error: {0}")]
    Redis(#[from] redis::RedisError),
```
> With `#[from]`, [thiserror](https://github.com/dtolnay/thiserror) will generate `impl From<redis::RedisError> for CustomError` automatically, and you can return error with `?` or `.map_err(Into::into)?` when the return type is `CustomResult<T>`.   
> Without `#[from]`, you need to convert error by yourself `.map_err(|e| CustomError::Redis(e))` or `.map_err(CustomError::Redis)`
3. Read code in [redis/examples](https://github.com/redis-rs/redis-rs/blob/main/redis/examples/async-await.rs).
4. Write your cache code.

### Global 404 handler
1. Add [Global-404-handler](https://github.com/tokio-rs/axum/tree/main/examples/global-404-handler) in `src/bin/server.rs`.
