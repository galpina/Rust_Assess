# Rust Backend

A port of the Rust backend, rewritten in Rust using [Actix-web](https://actix.rs/).  
It exposes the same REST API on the same default port (`8080`) so it is a drop-in replacement.

---

## Project Structure

```
rust-backend/
├── Cargo.toml          # Dependencies and package metadata
└── src/
    ├── main.rs         # Entry point – server setup and routing
    ├── models.rs       # Shared data types (User, Task, response structs)
    ├── data.rs         # In-memory data store (thread-safe with RwLock)
    └── handlers.rs     # One handler function per route
```

---

## Prerequisites

- **Rust 1.70 or higher** (includes `cargo`)

Install via [rustup](https://rustup.rs/):

```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows – download and run the installer from:
# https://rustup.rs/
```

Verify the installation:

```bash
rustc --version   # Should show 1.70+
cargo --version
```

---

## Setup

Install dependencies (Cargo handles this automatically on first build):

```bash
cargo build
```

---

## Running

### Development

```bash
cargo run
```

You should see:

```
Rust backend server starting on http://localhost:8080
Serving data directly from Rust backend
```

### Release (optimised binary)

```bash
cargo build --release
./target/release/rust-backend
```

### Custom Port

Set the `PORT` environment variable to override the default (`8080`):

```bash
# macOS / Linux
PORT=3001 cargo run

# Windows (PowerShell)
$env:PORT="3001"; cargo run

# Windows (CMD)
set PORT=3001 && cargo run
```

---

## Endpoints

| Method | Path               | Description                                        |
|--------|--------------------|----------------------------------------------------|
| GET    | `/health`          | Health check – returns server status               |
| GET    | `/api/users`       | List all users                                     |
| GET    | `/api/users/{id}`  | Get a single user by ID                            |
| GET    | `/api/tasks`       | List tasks (supports `?status=` and `?userId=`)    |
| GET    | `/api/stats`       | Aggregate statistics for users and tasks           |

### Example Requests

```bash
# Health check
curl http://localhost:8080/health

# All users
curl http://localhost:8080/api/users

# Single user
curl http://localhost:8080/api/users/1

# All tasks
curl http://localhost:8080/api/tasks

# Filter tasks by status
curl "http://localhost:8080/api/tasks?status=pending"

# Filter tasks by user
curl "http://localhost:8080/api/tasks?userId=2"

# Stats
curl http://localhost:8080/api/stats
```

### Example Responses

**GET /health**
```json
{
  "status": "ok",
  "message": "Rust backend is running"
}
```

**GET /api/users**
```json
{
  "users": [
    { "id": 1, "name": "John Doe",    "email": "john@example.com", "role": "developer" },
    { "id": 2, "name": "Jane Smith",  "email": "jane@example.com", "role": "designer"  },
    { "id": 3, "name": "Bob Johnson", "email": "bob@example.com",  "role": "manager"   }
  ],
  "count": 3
}
```

**GET /api/stats**
```json
{
  "users": { "total": 3 },
  "tasks": {
    "total": 3,
    "pending": 1,
    "inProgress": 1,
    "completed": 1
  }
}
```

---

## Dependencies

| Crate        | Purpose                              |
|--------------|--------------------------------------|
| `actix-web`  | HTTP server and routing              |
| `actix-cors` | CORS middleware (allow all origins)  |
| `serde`      | Serialisation / deserialisation      |
| `serde_json` | JSON support                         |
| `tokio`      | Async runtime                        |

---

## Rust → Rust Mapping

| Rust                  | Rust equivalent          |
|---------------------|--------------------------|
| `net/http`          | `actix-web`              |
| `sync.RWMutex`      | `std::sync::RwLock`      |
| `encoding/json`     | `serde` + `serde_json`   |
| Manual CORS headers | `actix-cors` middleware  |
| `os.Getenv("PORT")` | `std::env::var("PORT")`  |

---

## Common Issues

### Port Already in Use

```bash
# Find and kill the process on port 8080 (macOS / Linux)
lsof -ti:8080 | xargs kill -9

# Windows (PowerShell)
Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess | Stop-Process

# Or just use a different port
PORT=8081 cargo run
```

### Rust Not Found After Install

Close and reopen your terminal after running `rustup`, or source the environment manually:

```bash
source "$HOME/.cargo/env"
```

### Slow First Build

The first `cargo build` downloads and compiles all dependencies — this is normal and only happens once.
Subsequent builds are much faster.
