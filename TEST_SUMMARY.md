# Rust Developer Test - Quick Summary

## What You Need to Do

### Required Tasks (2-3 hours)

1. **Add User Creation** - `POST /api/users`
   - Validate input (`name`, `email`, `role`)
   - Generate unique ID
   - Return 201 with created user

2. **Add Task Creation** - `POST /api/tasks`
   - Validate input (`title`, `status`, `userId`)
   - Validate status enum and `userId` exists
   - Return 201 with created task

3. **Add Task Update** - `PUT /api/tasks/{id}`
   - Support partial updates
   - Validate `status` and `userId` if provided
   - Return 404 if not found

4. **Add Request Logging**
   - Log method, path, status, duration
   - Use Actix-web `Logger` middleware or the `log` crate

### Optional Tasks (2-3 hours)

- Data persistence (JSON file)
- Caching layer with expiration
- Request validation middleware
- Enhanced health check

### Code Quality (Ongoing)

- Write unit and integration tests (`actix-web::test`)
- Follow Rust best practices (ownership, `Result`, idiomatic patterns)
- Add doc comments (`///`)
- Handle errors with `Result<T, E>` and the `?` operator

## Evaluation

- **60%** - Technical correctness and code quality
- **25%** - Rust best practices and idioms
- **15%** - Problem solving and architecture

## Full Details

See [TEST_REQUIREMENTS.md](./TEST_REQUIREMENTS.md) for complete requirements.
