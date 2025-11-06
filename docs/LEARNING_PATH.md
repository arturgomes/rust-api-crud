# 🗺️ Learning Path - Your Step-by-Step Guide

**Start here!** This is your navigation guide through the Rust CRUD API learning journey.

## 📍 You Are Here: Getting Started

Before you begin, make sure you've completed the prerequisites from [README.md](../README.md).

---

## 🎯 Phase 0: Calculator API (Week 1)

**Goal**: Get comfortable with Rust syntax and Axum basics through a working example.

### What You'll Learn
- Basic Rust syntax and types
- Axum routing and handlers
- Serialization with Serde
- Pattern matching
- Async/await in Rust

### Steps

#### 1. Environment Setup (Day 1) ⏱️ 2 hours

```bash
# Copy environment file
cp .env.example .env

# Verify Rust installation
rustc --version
cargo --version

# Install dependencies
cargo build
```

**✅ Checkpoint**: `cargo build` completes successfully - DONE

**🚨 Stuck?**
- Rust not found? Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Build errors? Check Rust version is 1.70+

#### 2. Read and Understand Calculator Code (Day 1-2) ⏱️ 3 hours

**Your Task**: Open [src/main.rs](../src/main.rs) and read through the entire file.

**Focus On**:
- `struct` definitions (like TypeScript interfaces)
- `#[derive]` macros (automatic trait implementation)
- `async fn` functions (same as TS async functions)
- `match` statements (super-powered switch)
- The `Router` setup (like Express.js routing)

**TypeScript Comparison**:
```typescript
// TypeScript
interface CalculatorRequest {
  a: number;
  b: number;
  op: string;
}

// Rust equivalent
#[derive(Deserialize)]
struct CalculatorRequest {
    a: f64,
    b: f64,
    op: String,
}
```

**💡 Learning Tip**: The calculator uses the same patterns you'll use for CRUD operations, just simpler.

**✅ Checkpoint**: Can you explain what each section does?

#### 3. Run the Calculator (Day 2) ⏱️ 30 minutes

```bash
# Start the server
cargo run
```

You should see:
```
🚀 Calculator API listening on http://0.0.0.0:3000
📝 Try: http://localhost:3000/calculate?a=5&b=3&op=add
```

**Test it**:
```bash
# Addition
curl "http://localhost:3000/calculate?a=10&b=5&op=add"
# Should return: {"result":15.0}

# Division
curl "http://localhost:3000/calculate?a=10&b=2&op=divide"
# Should return: {"result":5.0}

# Error case - division by zero
curl "http://localhost:3000/calculate?a=10&b=0&op=divide"
# Should return: {"error":"Division by zero"}

# Error case - unknown operation
curl "http://localhost:3000/calculate?a=5&b=3&op=power"
# Should return: {"error":"Unknown operation: power"}
```

**✅ Checkpoint**: All curl commands work correctly

**🚨 Stuck?**
- Port 3000 in use? Change `PORT` in `.env` file
- Connection refused? Make sure `cargo run` is still running

#### 4. Experiment and Modify (Day 3-4) ⏱️ 4 hours

Now that it works, experiment! This is the best way to learn.

**Experiments to Try**:

1. DONE - **Add a new operation** (e.g., modulo, power)
   ```rust
   "modulo" => params.a % params.b,
   ```

2. DONE - **Change the response format**
   ```rust
   #[derive(Serialize)]
   struct CalculatorResponse {
       result: f64,
       operation: String,  // Add this
   }
   ```

3. DONE - **Add input validation** (reject negative numbers for certain ops)

4. DONE - **Add a health check endpoint**
   ```rust
   .route("/health", get(health_check))
   ```

**💡 Learning Tip**: Break things! The compiler will teach you.

**✅ Checkpoint**: Successfully added at least one modification

#### 5. Understand the Tests (Day 4-5) ⏱️ 2 hours

Read [tests/calculator_tests.rs](../tests/calculator_tests.rs).

**Note**: These are test *structures* for learning. In Phase 2, you'll write real integration tests.

```bash
# Run tests
cargo test

# Run with output
cargo test -- --nocapture
```

**✅ Checkpoint**: Tests run (even if they're just placeholders)

### 🎉 Phase 0 Complete!

You've learned:
- ✅ Basic Rust syntax
- ✅ Structs and serialization
- ✅ Async functions
- ✅ Pattern matching
- ✅ Axum routing
- ✅ Running and testing

**Next**: Phase 1 - Database Infrastructure

---

## 🗄️ Phase 1: Database Infrastructure (Week 2)

**Goal**: Set up PostgreSQL, migrations, and database connection patterns.

### What You'll Learn
- Docker Compose for PostgreSQL
- SQLx for type-safe database queries
- Database migrations
- Connection pooling
- Environment configuration
- Error handling with Result<T, E>

### Steps

#### 1. Start PostgreSQL (Day 1) ⏱️ 1 hour

```bash
# Start PostgreSQL container
docker-compose up -d

# Verify it's running
docker ps

# Check logs
docker-compose logs postgres
```

**✅ Checkpoint**: PostgreSQL container is running

**🚨 Stuck?**
- Docker not running? Start Docker Desktop
- Port 5432 in use? Change port in `docker-compose.yml`

#### 2. Install SQLx CLI (Day 1) ⏱️ 30 minutes

```bash
# Install SQLx command-line tool
cargo install sqlx-cli --no-default-features --features postgres

# Verify installation
sqlx --version
```

**✅ Checkpoint**: `sqlx --version` works

#### 3. Create First Migration (Day 1-2) ⏱️ 2 hours

```bash
# Create users table migration
sqlx migrate add create_users_table
```

This creates a file in `migrations/`. Edit it:

```sql
-- migrations/XXXXXX_create_users_table.sql
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```

**Run the migration**:
```bash
sqlx migrate run
```

**✅ Checkpoint**: Migration runs successfully

**Verify in psql** (optional):
```bash
docker exec -it rust_crud_db psql -U rustuser -d rustcrud
\dt  # List tables
\d users  # Describe users table
\q  # Quit
```

#### 4. Set Up Database Module (Day 2-3) ⏱️ 3 hours

Create [src/db/mod.rs](../src/db/mod.rs):

```rust
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

// Create database connection pool
// TypeScript equivalent: const pool = new Pool({ ... })
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}
```

**💡 Key Concepts**:
- `PgPool` - Connection pool (like pg Pool in Node.js)
- `Result<T, E>` - Rust's error handling (replaces try/catch)
- `.await` - Same as TypeScript await

**✅ Checkpoint**: Code compiles

#### 5. Update main.rs for Database (Day 3-4) ⏱️ 3 hours

You'll modify `main.rs` to:
1. Load environment variables
2. Create database pool
3. Pass pool to handlers
4. Handle errors properly

**This is YOUR first implementation task!**

**Hints**:
- Use `dotenv::dotenv()` to load `.env`
- Use `std::env::var()` to read `DATABASE_URL`
- Use `with_state()` to share the pool

See [RUST_CONCEPTS.md](RUST_CONCEPTS.md) for Result<T, E> examples.

**✅ Checkpoint**: Server starts and connects to database

#### 6. Write Database Tests (Day 4-5) ⏱️ 3 hours

Create [tests/db_tests.rs](../tests/db_tests.rs):

```rust
#[tokio::test]
async fn test_database_connection() {
    // Load env
    // Create pool
    // Execute simple query: SELECT 1
    // Assert success
}
```

**Run tests**:
```bash
cargo test test_database_connection
```

**✅ Checkpoint**: Database connection test passes

### 🎉 Phase 1 Complete!

You've learned:
- ✅ Docker Compose for services
- ✅ Database migrations
- ✅ Connection pooling with SQLx
- ✅ Result<T, E> error handling
- ✅ Sharing state in Axum
- ✅ Integration testing with databases

**Next**: Phase 2 - CRUD Operations

---

## 📝 Phase 2: CRUD Operations (Weeks 3-5)

**Goal**: Build complete Create, Read, List, Update, Delete operations with TDD.

### What You'll Learn
- RESTful API design
- JSON request/response handling
- Database queries with SQLx
- Error handling patterns
- Pagination
- Input validation
- Integration testing

### Overview

Each operation follows the **TDD Red-Green-Refactor** cycle:

```
1. 🔴 RED: Read pre-written failing test
2. 💡 UNDERSTAND: What does the test expect?
3. 🟢 GREEN: Write minimal code to pass
4. 🔄 REFACTOR: Clean up and improve
5. ✅ VERIFY: All tests pass
```

### 2.1: CREATE User (Week 3, Day 1-3)

**Endpoint**: `POST /users`

**Request**:
```json
{
  "name": "Alice",
  "email": "alice@example.com"
}
```

**Response**:
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Alice",
  "email": "alice@example.com",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

**Files to Create**:
1. `src/models/user.rs` - User struct
2. `src/handlers/user_handlers.rs` - create_user function
3. `src/routes/user_routes.rs` - Route setup
4. `tests/user_tests.rs` - Integration tests

**Test First** (in `tests/user_tests.rs`):
```rust
#[tokio::test]
async fn test_create_user() {
    // Setup: Start test server
    // Action: POST /users with JSON
    // Assert: Status 201, valid user returned
    // Cleanup: Delete test user
}
```

**Implementation Pattern**:
```rust
// TypeScript: app.post('/users', async (req, res) => { ... })
// Rust equivalent:
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    // Insert into database with sqlx
    // Return created user
}
```

**✅ Checkpoint**:
- POST /users creates a user
- Test passes
- Returns proper JSON

**⏱️ Time**: 6-8 hours

---

### 2.2: READ User (Week 3, Day 4-5)

**Endpoint**: `GET /users/:id`

**Response**:
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Alice",
  "email": "alice@example.com",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

**New Concepts**:
- Path parameters with `Path<Uuid>`
- Option<T> for not-found cases
- 404 error handling

**Test First**:
```rust
#[tokio::test]
async fn test_get_user() {
    // Setup: Create test user
    // Action: GET /users/{id}
    // Assert: Returns correct user
}

#[tokio::test]
async fn test_get_user_not_found() {
    // Action: GET /users/{invalid_id}
    // Assert: Returns 404
}
```

**✅ Checkpoint**:
- GET /users/:id returns user
- Returns 404 for non-existent users
- Tests pass

**⏱️ Time**: 4-6 hours

---

### 2.3: LIST Users (Week 4, Day 1-3)

**Endpoint**: `GET /users?page=1&per_page=10`

**Response**:
```json
{
  "users": [...],
  "total": 42,
  "page": 1,
  "per_page": 10,
  "total_pages": 5
}
```

**New Concepts**:
- Query parameters
- Pagination with LIMIT/OFFSET
- Multiple database queries
- Aggregate queries (COUNT)

**Test First**:
```rust
#[tokio::test]
async fn test_list_users_pagination() {
    // Setup: Create 15 test users
    // Action: GET /users?page=1&per_page=10
    // Assert: Returns 10 users, correct total
    // Action: GET /users?page=2&per_page=10
    // Assert: Returns 5 users
}
```

**✅ Checkpoint**:
- GET /users returns paginated results
- Pagination math is correct
- Default values work
- Tests pass

**⏱️ Time**: 6-8 hours

---

### 2.4: UPDATE User (Week 4, Day 4-5)

**Endpoint**: `PUT /users/:id`

**Request**:
```json
{
  "name": "Alice Smith",
  "email": "alice.smith@example.com"
}
```

**New Concepts**:
- Optional fields (partial updates)
- updated_at timestamp update
- Returning updated record

**Test First**:
```rust
#[tokio::test]
async fn test_update_user() {
    // Setup: Create test user
    // Action: PUT /users/{id} with changes
    // Assert: User updated, updated_at changed
}

#[tokio::test]
async fn test_update_user_not_found() {
    // Action: PUT /users/{invalid_id}
    // Assert: Returns 404
}
```

**✅ Checkpoint**:
- PUT /users/:id updates user
- updated_at timestamp changes
- Returns 404 for non-existent users
- Tests pass

**⏱️ Time**: 4-6 hours

---

### 2.5: DELETE User (Week 5, Day 1-3)

**Endpoint**: `DELETE /users/:id`

**Response**: 204 No Content

**Test First**:
```rust
#[tokio::test]
async fn test_delete_user() {
    // Setup: Create test user
    // Action: DELETE /users/{id}
    // Assert: Status 204
    // Action: GET /users/{id}
    // Assert: Returns 404 (user gone)
}

#[tokio::test]
async fn test_delete_user_not_found() {
    // Action: DELETE /users/{invalid_id}
    // Assert: Returns 404
}
```

**✅ Checkpoint**:
- DELETE /users/:id removes user
- Returns 204 on success
- Returns 404 for non-existent users
- User actually deleted from database
- Tests pass

**⏱️ Time**: 3-4 hours

---

### 2.6: Integration & Refactoring (Week 5, Day 4-5)

**Goal**: Complete system works together, code is clean.

**Tasks**:
1. Run ALL tests: `cargo test`
2. Test full workflow manually
3. Refactor common patterns
4. Add error handling improvements
5. Add logging
6. Update documentation

**Complete Workflow Test**:
```bash
# 1. Create user
USER_ID=$(curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Test","email":"test@example.com"}' \
  | jq -r '.id')

# 2. Get user
curl http://localhost:3000/users/$USER_ID

# 3. List users
curl http://localhost:3000/users

# 4. Update user
curl -X PUT http://localhost:3000/users/$USER_ID \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated","email":"updated@example.com"}'

# 5. Delete user
curl -X DELETE http://localhost:3000/users/$USER_ID

# 6. Verify deletion
curl http://localhost:3000/users/$USER_ID
# Should return 404
```

**✅ Checkpoint**:
- All tests pass
- Complete workflow works
- Code is clean and organized
- No compiler warnings
- Documentation updated

**⏱️ Time**: 4-6 hours

### 🎉 Phase 2 Complete!

You've built:
- ✅ Complete CRUD API
- ✅ RESTful endpoints
- ✅ Database operations
- ✅ Error handling
- ✅ Pagination
- ✅ Integration tests
- ✅ Production-ready code

---

## 🎓 Week 6: Mastery & Next Steps

**Goal**: Consolidate learning and explore advanced topics.

### Consolidation Activities

1. **Code Review** (Day 1)
   - Read all your code
   - Refactor anything unclear
   - Add comments for future you

2. **Concept Review** (Day 2)
   - Review [RUST_CONCEPTS.md](RUST_CONCEPTS.md)
   - Review [TYPESCRIPT_TO_RUST.md](TYPESCRIPT_TO_RUST.md)
   - Fill knowledge gaps

3. **Documentation** (Day 3)
   - API documentation
   - Setup instructions
   - Architecture overview

4. **Testing** (Day 4)
   - Add missing test cases
   - Test edge cases
   - Load testing (optional)

5. **Enhancement** (Day 5)
   - Add feature of your choice
   - Experiment with new patterns
   - Explore Rust ecosystem

### Optional Advanced Topics

- **Authentication**: JWT, sessions
- **Validation**: More robust input validation
- **Logging**: Structured logging with tracing
- **Error Handling**: Custom error types
- **Performance**: Database optimization
- **Deployment**: Docker multi-stage builds
- **Testing**: Property-based testing

---

## 📊 Progress Tracker

Track your journey:

- [ ] Phase 0: Calculator working
- [ ] Phase 1: Database connected
- [ ] Phase 2.1: CREATE user
- [ ] Phase 2.2: READ user
- [ ] Phase 2.3: LIST users
- [ ] Phase 2.4: UPDATE user
- [ ] Phase 2.5: DELETE user
- [ ] Phase 2.6: Integration complete
- [ ] All tests passing
- [ ] Documentation complete
- [ ] Understanding solid

---

## 🆘 Getting Help

**Stuck on Rust concepts?** → [RUST_CONCEPTS.md](RUST_CONCEPTS.md)

**Want TypeScript comparisons?** → [TYPESCRIPT_TO_RUST.md](TYPESCRIPT_TO_RUST.md)

**Need quick examples?** → [QUICK_START.md](QUICK_START.md)

**Compiler errors?** → Read them carefully! Rust errors are incredibly helpful.

**Still stuck?** → Check the Rust documentation at https://doc.rust-lang.org/

---

**🎉 Congratulations on starting your Rust journey!**

Remember: The compiler is your teacher. Read error messages carefully - they're designed to help you learn.

Ready? Let's build! 🚀
