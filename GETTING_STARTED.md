# Getting Started - Rust Developer Test

Welcome! This guide will help you get started with the test project.

## Step 1: Read the Requirements

1. **Start here**: Read [TEST_REQUIREMENTS.md](./TEST_REQUIREMENTS.md) for complete requirements
2. **Quick reference**: Check [TEST_SUMMARY.md](./TEST_SUMMARY.md) for a quick overview
3. **Track progress**: Use [CANDIDATE_CHECKLIST.md](./CANDIDATE_CHECKLIST.md) to track your work

## Step 2: Set Up Your Environment

### Prerequisites

- **Rust 1.70 or higher** (includes `cargo`)
  ```bash
  rustc --version   # Should show 1.70+
  cargo --version
  ```
  Install via https://rustup.rs/ if not present.

- **Node.js 16 or higher**
  ```bash
  node --version  # Should show v16+
  npm --version
  ```

### Install Dependencies

1. **Rust Backend** (Cargo downloads dependencies automatically)
   ```bash
   cd rust-backend
   cargo build
   ```

2. **Node.js Backend**
   ```bash
   cd node-backend
   npm install
   ```

3. **React Frontend**
   ```bash
   cd react-frontend
   npm install
   ```

## Step 3: Start the Services

**Important**: Start services in this order:

### Terminal 1: Rust Backend
```bash
cd rust-backend
cargo run
```
You should see: `Rust backend server starting on http://localhost:8080`

### Terminal 2: Node.js Backend
```bash
cd node-backend
npm start
```
You should see: `Node.js backend server running on http://localhost:3000`

### Terminal 3: React Frontend
```bash
cd react-frontend
npm run dev
```
You should see: `Local: http://localhost:5173`

## Step 4: Verify Everything Works

1. **Open the frontend**: http://localhost:5173
   - You should see users and tasks displayed
   - Health status should show "ok"

2. **Test Rust backend directly**:
   ```bash
   curl http://localhost:8080/health
   curl http://localhost:8080/api/users
   curl http://localhost:8080/api/tasks
   curl http://localhost:8080/api/stats
   ```

3. **Test Node.js backend**:
   ```bash
   curl http://localhost:3000/health
   curl http://localhost:3000/api/users
   ```

## Step 5: Understand the Codebase

### Rust Backend Structure

```
rust-backend/
├── Cargo.toml       # Dependencies and package metadata
└── src/
    ├── main.rs      # Entry point – server setup and routing
    ├── models.rs    # Shared data types (User, Task, response structs)
    ├── data.rs      # In-memory data store (thread-safe with RwLock)
    └── handlers.rs  # One handler function per route
```

**Key Files to Review**:
- `src/data.rs` — See how data is stored and accessed with `RwLock`
- `src/handlers.rs` — See how HTTP endpoints are handled with Actix-web
- `src/models.rs` — See how types are defined with `serde`
- `src/main.rs` — See how the server is configured and started

### Current Endpoints

**GET endpoints** (already implemented):
- `GET /health` — Health check
- `GET /api/users` — Get all users
- `GET /api/users/{id}` — Get user by ID
- `GET /api/tasks` — Get all tasks (supports `?status=` and `?userId=` query params)
- `GET /api/stats` — Get statistics

**POST/PUT endpoints** (you need to implement):
- `POST /api/users` — Create new user
- `POST /api/tasks` — Create new task
- `PUT /api/tasks/{id}` — Update existing task

## Step 6: Start Implementing

### Recommended Order

1. **Start with User Creation** (`POST /api/users`)
   - Simplest endpoint to add
   - Good for understanding the handler and data store patterns

2. **Then Task Creation** (`POST /api/tasks`)
   - Similar to user creation
   - Adds validation complexity (`userId` must exist)

3. **Then Task Update** (`PUT /api/tasks/{id}`)
   - More complex (partial updates with `Option` fields)
   - Good practice for handling edge cases

4. **Finally Logging**
   - Actix-web's `Logger` middleware can be added in `main.rs`
   - Add throughout the process to help with debugging

### Tips

- **Read the existing code first** — Understand patterns before adding new code
- **Trust the compiler** — Rust error messages are detailed and helpful
- **Test as you go** — Use `curl` or Postman to test endpoints
- **Write tests early** — Don't wait until the end
- **Check the checklist** — Use `CANDIDATE_CHECKLIST.md` to track progress

## Step 7: Testing Your Code

### Manual Testing

```bash
# Create a user
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@example.com","role":"developer"}'

# Create a task
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"Test Task","status":"pending","userId":1}'

# Update a task
curl -X PUT http://localhost:8080/api/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"status":"completed"}'
```

### Writing Tests

Add test modules directly in the source files:

```rust
// In src/data.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_by_id_found() {
        let store = DataStore::new();
        assert!(store.get_user_by_id(1).is_some());
    }

    #[test]
    fn test_get_user_by_id_not_found() {
        let store = DataStore::new();
        assert!(store.get_user_by_id(999).is_none());
    }
}
```

Run tests:
```bash
cd rust-backend
cargo test                  # Run all tests
cargo test -- --nocapture   # Show println! output
cargo test data             # Run only tests in the data module
```

## Common Issues

### Port Already in Use
```bash
# macOS / Linux — find and kill process on port 8080
lsof -ti:8080 | xargs kill -9

# Windows (PowerShell)
Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess | Stop-Process

# Or use a different port
PORT=8081 cargo run
```

### Rust Not Found After Install
Close and reopen your terminal, or source the environment manually:
```bash
source "$HOME/.cargo/env"
```

### Slow First Build
The first `cargo build` downloads and compiles all dependencies — this is normal and only happens once. Subsequent builds are much faster.

### Compiler Errors
Read the error message carefully — Rust errors include the file, line, and often a suggested fix. Run `cargo check` for fast feedback without a full build.

### Node.js Issues
```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

## Getting Help

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Actix-web Docs**: https://actix.rs/docs/
- **docs.rs**: https://docs.rs/ (crate documentation)
- **Review existing code** — The codebase has examples of patterns to follow
- **Ask questions** — If requirements are unclear, ask for clarification

## Next Steps

1. ✅ Read [TEST_REQUIREMENTS.md](./TEST_REQUIREMENTS.md)
2. ✅ Set up your environment
3. ✅ Start all services
4. ✅ Verify everything works
5. ✅ Review the codebase
6. ✅ Start implementing required features
7. ✅ Write tests
8. ✅ Submit your work

**Good luck!** 🚀
