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
### Replace StatusCode with CustomError
1. Define `CustomError` enum with [thiserror](https://github.com/dtolnay/thiserror).
2. Implement `IntoResponse` for `CustomError`. Example: [AuthError](https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs#L142).
3. Replace `Result<Json<T>, StatusCode>` with `Result<Json<T>, CustomError>` in handlers.
4. Convert errors to `CustomError` in `map_err`.

### Global 404 handler
1. Add [Global-404-handler](https://github.com/tokio-rs/axum/tree/main/examples/global-404-handler) in `src/bin/server.rs`.
