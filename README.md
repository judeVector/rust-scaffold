# rust-scaffold

A production-ready Rust REST API scaffold built with [Axum](https://github.com/tokio-rs/axum). Includes JWT authentication, bcrypt password hashing, a thread-safe in-memory store, request logging, and CORS — all wired up and ready to extend.

## Features

- **JWT auth** — register, login, and protected routes via Bearer token
- **bcrypt** — passwords hashed with `bcrypt::DEFAULT_COST` on a blocking thread
- **In-memory store** — concurrent `DashMap`-backed store; swap for a real DB without touching handlers
- **Full user CRUD** — create, read, update, delete users (all routes require auth)
- **Request logging** — method, path, status, duration, and auth status on every request
- **CORS** — permissive by default via `tower-http`; tighten in `main.rs` for production
- **Structured errors** — `AppError` maps to correct HTTP status codes with JSON bodies

## Project Structure

```
src/
├── main.rs           # App entrypoint, router, AppState
├── config.rs         # Config loaded from environment variables
├── errors.rs         # AppError enum → HTTP responses
├── auth/
│   ├── jwt.rs        # Token creation and validation
│   └── extractor.rs  # AuthUser extractor for protected routes
├── handlers/
│   ├── auth.rs       # POST /auth/register, /auth/login, GET /auth/me
│   └── users.rs      # CRUD handlers under /users
├── middleware/
│   └── logging.rs    # Request/response logger
├── models/
│   └── user.rs       # User, CreateUser, UpdateUser, LoginRequest/Response
└── store/
    └── memory.rs     # Thread-safe in-memory data store
```

## Getting Started

**Prerequisites:** Rust 1.75+ (`rustup update stable`)

```bash
git clone https://github.com/judeVector/rust-scaffold.git
cd rust-scaffold

cp .env.example .env   # edit JWT_SECRET before running in production

cargo run
```

The server starts on `http://0.0.0.0:3000` by default.

## Configuration

All configuration is read from environment variables (or a `.env` file):

| Variable           | Default                                | Description                    |
|--------------------|----------------------------------------|--------------------------------|
| `HOST`             | `0.0.0.0`                              | Bind address                   |
| `PORT`             | `3000`                                 | Bind port                      |
| `JWT_SECRET`       | `change-me-in-production-secret-key`   | Secret used to sign JWTs       |
| `JWT_EXPIRY_HOURS` | `24`                                   | Token lifetime in hours        |
| `RUST_LOG`         | `rust_scaffold=debug,info`             | Log filter (tracing-subscriber)|

## API Reference

### Auth

| Method | Path             | Auth | Description              |
|--------|------------------|------|--------------------------|
| POST   | `/auth/register` | No   | Create account           |
| POST   | `/auth/login`    | No   | Obtain JWT               |
| GET    | `/auth/me`       | Yes  | Get current user profile |

### Users

All routes below require `Authorization: Bearer <token>`.

| Method | Path           | Description          |
|--------|----------------|----------------------|
| GET    | `/users`       | List all users       |
| GET    | `/users/:id`   | Get user by UUID     |
| POST   | `/users`       | Create a user        |
| PATCH  | `/users/:id`   | Update username/email|
| DELETE | `/users/:id`   | Delete user          |

### Example: Register & Login

```bash
# Register
curl -s -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"secret123"}' | jq

# Login
curl -s -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@example.com","password":"secret123"}' | jq

# Authenticated request
curl -s http://localhost:3000/auth/me \
  -H "Authorization: Bearer <token>" | jq
```

### Error Responses

All errors return JSON:

```json
{ "error": "unauthorized" }
```

| Status | Condition                        |
|--------|----------------------------------|
| 400    | Bad request / validation failure |
| 401    | Missing or invalid JWT           |
| 404    | Resource not found               |
| 409    | Email already registered         |
| 500    | Internal server error            |

## Extending the Scaffold

- **Swap the store** — implement the same method signatures in `store/` against Postgres (sqlx), SQLite (rusqlite), or Redis without changing any handler code.
- **Add a resource** — create a model in `models/`, a handler file in `handlers/`, and register the routes in `main.rs`.
- **Tighten CORS** — replace the permissive `CorsLayer::permissive()` in `main.rs` with an explicit `allow_origin` list.

## License

MIT
