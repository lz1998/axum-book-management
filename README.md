# Book management

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