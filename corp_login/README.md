# Corp Login Server

A Rust-based authentication server using Axum and SQLite for the Corp One project.

## Running the Server

### Database Migration

```bash
cargo run --bin migrate
```

### Start Server

```bash
cargo run --bin corp_login
```

The server will start on `http://127.0.0.1:25560`

## API Examples

All users use the password `password123` for testing.

### Register User

```bash
curl -X POST http://127.0.0.1:25560/register \
  -H "Content-Type: application/json" \
  -d '{"username": "newuser", "email": "user@example.com", "password": "password123"}'
```

### Login

```bash
curl -X POST http://127.0.0.1:25560/login \
  -H "Content-Type: application/json" \
  -d '{"username": "commander_shepard", "password": "password123"}'
```

### Validate User

```bash
curl -X POST http://127.0.0.1:25560/validate \
  -H "Content-Type: application/json" \
  -d '{"token": "fd42bde3-a69a-4672-835f-bbedaafd7433"}'
```

### Logout

```bash
curl -X POST http://127.0.0.1:25560/logout \
  -H "Content-Type: application/json" \
  -d '{"token": "cd858d15-702d-48c3-861c-310aa9ea8ec3"}'
```

## Dependencies

- `axum` - Web framework
- `sqlx` - Database driver
- `bcrypt` - Password hashing
- `tokio` - Async runtime
- `serde` - JSON serialization
- `tracing` - Logging

## Database File

The SQLite database is stored as `corp_login.db` in the project root with WAL mode enabled for better performance.