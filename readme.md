# 🚀 Rust Secure API

A high-performance, secure, and fully async RESTful API built with [Axum](https://docs.rs/axum), [SQLx](https://docs.rs/sqlx), and [Tokio](https://tokio.rs/). Features authentication with JWT, password hashing with Argon2, IP/user-based rate limiting, and OpenAPI documentation via Swagger.

## 📦 Tech Stack

- **Rust** – memory-safe systems programming language.
- **Axum** – web framework powered by Tokio.
- **SQLx** – async PostgreSQL driver (no macros).
- **Tokio** – async runtime.
- **argon2** – password hashing.
- **jsonwebtoken** – JWT authentication.
- **tower** – middleware for rate limiting and request handling.
- **utoipa** – auto-generated Swagger/OpenAPI docs.
- **Docker & GitHub Actions** – containerization and CI/CD ready.

## ✨ Features

- ✅ JWT-based authentication (`/auth/login`, `/auth/register`)
- ✅ Password hashing with Argon2
- ✅ Role-ready user model
- ✅ Middleware-authenticated private routes (`/user/me`)
- ✅ Rate limiting (per IP and per user via JWT `sub`)
- ✅ OpenAPI documentation at `/docs`
- ✅ Modular, scalable project structure
- ✅ Database migrations (optional)
- ✅ Dockerized for production

## 📁 Project Structure

```
src/
├── auth/            # JWT + password hashing logic
├── db/              # DB connection and initialization
├── middleware/      # Rate limiters and auth guards
├── models/          # User, request/response types
├── routes/          # Auth and protected routes
├── docs/            # Swagger integration
├── utils/           # Misc helpers
└── main.rs          # Entry point
```

## 🚀 Getting Started

### 🔧 Prerequisites

- Rust (1.70+)
- PostgreSQL
- `sqlx-cli` (for migrations): `cargo install sqlx-cli`
- Docker (optional)

### 🛠️ Run Locally

```bash
# 1. Clone repo
git clone https://github.com/yourname/rust-secure-api.git
cd rust-secure-api

# 2. Set environment variables
cp .env.example .env
# Update your DATABASE_URL in .env

# 3. Run migrations (optional)
sqlx database setup

# 4. Run
cargo run
```

Visit: [http://localhost:3000/docs](http://localhost:3000/docs)

### 🐳 Run with Docker

```bash
docker build -t rust-secure-api .
docker run -p 3000:3000 --env-file .env rust-secure-api
```

## 🔒 Authentication

- **Register:** `POST /auth/register`
- **Login:** `POST /auth/login` → returns JWT
- **Protected route:** `GET /user/me` with header:

```http
Authorization: Bearer <JWT_TOKEN>
```

## 📊 API Documentation

Available at:  
**[http://localhost:3000/docs](http://localhost:3000/docs)**  
Generated with [utoipa + Swagger UI](https://docs.rs/utoipa).

## ⏱ Rate Limiting

- Global IP-based limit (`/auth/*`) → 1 request every 5 seconds
- Per-user JWT limit (`/user/*`) → 1 request every 3 seconds
- Returns `429 TOO MANY REQUESTS` with cooldown info

## 🧪 Testing

Coming soon: integration tests with `httpc-test` or `reqwest`.

## 📦 Build for Production

```bash
cargo build --release
```

## 🧰 Tooling

- Formatter: `cargo fmt`
- Linter: `cargo clippy`
- Audit: `cargo audit`
- Benchmarks: `cargo bench`

## 🤝 Contributing

Pull requests and issues are welcome. Please open a discussion if you plan a major change.

## 📄 License

MIT License © 2024 [Your Name]
