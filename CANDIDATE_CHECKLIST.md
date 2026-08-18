# Candidate Checklist

Use this checklist to track your progress through the test.

## Phase 1: Setup (30 minutes)

- [ ] Forked/cloned the repository
- [ ] Installed Rust 1.70+ (`rustc --version`)
- [ ] Installed Node.js 16+
- [ ] Installed dependencies (`npm install` in node-backend and react-frontend)
- [ ] Built Rust backend (`cargo build` in rust-backend)
- [ ] Started Rust backend on port 8080
- [ ] Started Node.js backend on port 3000
- [ ] Started React frontend on port 5173
- [ ] Verified application works end-to-end
- [ ] Tested existing endpoints manually

## Phase 2: Core Requirements (2-3 hours)

### User Creation Endpoint
- [ ] Added `POST /api/users` endpoint
- [ ] Validates `name`, `email`, `role` fields
- [ ] Validates email format
- [ ] Generates unique ID
- [ ] Returns 201 with created user
- [ ] Returns 400 for invalid input
- [ ] New users appear in GET `/api/users`

### Task Creation Endpoint
- [ ] Added `POST /api/tasks` endpoint
- [ ] Validates `title`, `status`, `userId` fields
- [ ] Validates status enum (`pending` / `in-progress` / `completed`)
- [ ] Validates `userId` exists
- [ ] Generates unique ID
- [ ] Returns 201 with created task
- [ ] Returns 400 for invalid input
- [ ] New tasks appear in GET `/api/tasks`

### Task Update Endpoint
- [ ] Added `PUT /api/tasks/{id}` endpoint
- [ ] Supports partial updates
- [ ] Validates `status` if provided
- [ ] Validates `userId` if provided
- [ ] Returns 200 with updated task
- [ ] Returns 404 if task not found
- [ ] Returns 400 for invalid input

### Request Logging
- [ ] Added logging for all requests
- [ ] Logs HTTP method
- [ ] Logs request path
- [ ] Logs response status code
- [ ] Logs response time/duration
- [ ] Logs errors with context
- [ ] Logs are readable and consistent

## Phase 3: Advanced Requirements (Optional)

- [ ] Data persistence (JSON file)
- [ ] Caching layer with expiration
- [ ] Request validation middleware
- [ ] Enhanced health check
- [ ] Other advanced features

## Phase 4: Code Quality

### Testing
- [ ] Unit tests for data store (`#[cfg(test)]` in `data.rs`)
- [ ] Integration tests for HTTP endpoints (`actix_web::test`)
- [ ] Test error cases
- [ ] Test edge cases
- [ ] Code coverage > 70%

### Code Organization
- [ ] Follows Rust naming conventions (snake_case / CamelCase)
- [ ] Code split into logical modules
- [ ] Functions are focused and single-purpose
- [ ] Meaningful comments for complex logic
- [ ] No unused code, no compiler warnings

### Error Handling
- [ ] Uses `Result<T, E>` and `?` operator consistently
- [ ] Custom error types defined where appropriate
- [ ] Appropriate HTTP status codes returned
- [ ] Meaningful error messages
- [ ] Internal errors not exposed to clients

### Documentation
- [ ] Doc comments (`///`) on public functions and types
- [ ] API endpoint documentation
- [ ] Updated README
- [ ] Design decisions documented

## Phase 5: Bonus Tasks (Optional)

- [ ] Authentication / API keys
- [ ] Rate limiting
- [ ] Metrics / observability (Prometheus)
- [ ] Database integration (SQLite / PostgreSQL)

## Submission

- [ ] All required features implemented
- [ ] Code compiles without errors (`cargo build`)
- [ ] No compiler warnings
- [ ] All services run successfully
- [ ] Tests written and passing (`cargo test`)
- [ ] Documentation updated
- [ ] Code review notes (optional)

## Notes

Use this space to track any issues, questions, or design decisions:

```
[Your notes here]
```
