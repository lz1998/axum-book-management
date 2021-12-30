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