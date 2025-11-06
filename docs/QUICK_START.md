# ⚡ Quick Start - Learn by Doing

**For developers who prefer to learn by coding immediately with minimal theory.**

Skip the long explanations. Just code, break things, fix them, learn.

---

## 🚀 Get Running in 5 Minutes

```bash
# 1. Clone and enter
cd rust-api-crud

# 2. Start database
docker-compose up -d

# 3. Run the calculator
cargo run
```

Test it:
```bash
curl "http://localhost:3000/calculate?a=10&b=5&op=add"
```

**It works?** ✅ You're ready to learn Rust by doing.

---

## 📝 Your Learning Path (Action-First)

### Step 1: Modify the Calculator (30 min)

Open [src/main.rs](../src/main.rs) and:

**Exercise 1**: Add modulo operation
```rust
"modulo" => params.a % params.b,
```

**Exercise 2**: Add power operation (use `.powf()`)

**Exercise 3**: Add a health check endpoint
```rust
async fn health() -> &'static str {
    "OK"
}

// Add to router:
.route("/health", get(health))
```

**Exercise 4**: Change error response format
```rust
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: String,  // Add this
}
```

**💡 Pro Tip**: The compiler will tell you exactly what's wrong. Read the errors!

---

### Step 2: Add Database (1 hour)

**Create database module** - [src/db/mod.rs](../src/db/mod.rs):

```rust
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
```

**Create migration**:
```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate add create_users_table
```

Edit `migrations/XXXXX_create_users_table.sql`:
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

Run it:
```bash
sqlx migrate run
```

**Test connection** - Add to main.rs:
```rust
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create pool");

    println!("✅ Connected to database!");

    // ... rest of your code
}
```

---

### Step 3: Create User Model (30 min)

**Create** [src/models/user.rs](../src/models/user.rs):

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}
```

**Create** [src/models/mod.rs](../src/models/mod.rs):
```rust
pub mod user;
pub use user::*;
```

**Add to** [src/main.rs](../src/main.rs):
```rust
mod models;
mod db;
```

---

### Step 4: CREATE Endpoint (1 hour)

**Create** [src/handlers/user_handlers.rs](../src/handlers/user_handlers.rs):

```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::{User, CreateUserRequest};

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    // TODO: Implement this!
    // Hint: Use sqlx::query_as!
    // sqlx::query_as!(User,
    //     "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
    //     payload.name, payload.email
    // )
    // .fetch_one(&pool)
    // .await

    Err(StatusCode::NOT_IMPLEMENTED)
}
```

**Create** [src/handlers/mod.rs](../src/handlers/mod.rs):
```rust
pub mod user_handlers;
pub use user_handlers::*;
```

**Update main.rs**:
```rust
mod handlers;

// In main():
let app = Router::new()
    .route("/calculate", get(calculate))
    .route("/users", post(create_user))
    .with_state(pool);
```

**Your Task**: Implement `create_user`!

**Test**:
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'
```

---

### Step 5: READ Endpoint (30 min)

**Add to** [src/handlers/user_handlers.rs](../src/handlers/user_handlers.rs):

```rust
use axum::extract::Path;
use uuid::Uuid;

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    // TODO: Implement this!
    // Query: SELECT * FROM users WHERE id = $1

    Err(StatusCode::NOT_IMPLEMENTED)
}
```

**Update main.rs**:
```rust
.route("/users/:id", get(get_user))
```

**Your Task**: Implement `get_user`!

**Hint**: Handle not found case:
```rust
.fetch_optional(&pool)
.await
.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
.ok_or(StatusCode::NOT_FOUND)?
```

**Test**:
```bash
USER_ID="<paste-id-from-create>"
curl http://localhost:3000/users/$USER_ID
```

---

### Step 6: LIST Endpoint (45 min)

**Add to user_handlers.rs**:

```rust
use axum::extract::Query;

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 { 1 }
fn default_per_page() -> i64 { 10 }

#[derive(Serialize)]
pub struct UserListResponse {
    pub users: Vec<User>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

pub async fn list_users(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<UserListResponse>, StatusCode> {
    // TODO: Implement pagination!
    // 1. Get total count: SELECT COUNT(*) FROM users
    // 2. Get page of users: SELECT * FROM users LIMIT $1 OFFSET $2
    // Offset = (page - 1) * per_page

    Err(StatusCode::NOT_IMPLEMENTED)
}
```

**Update main.rs**:
```rust
.route("/users", get(list_users))
```

**Test**:
```bash
curl "http://localhost:3000/users?page=1&per_page=10"
```

---

### Step 7: UPDATE Endpoint (30 min)

**Add to user_handlers.rs**:

```rust
#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    // TODO: Implement update!
    // UPDATE users SET
    //   name = COALESCE($1, name),
    //   email = COALESCE($2, email)
    // WHERE id = $3
    // RETURNING *

    Err(StatusCode::NOT_IMPLEMENTED)
}
```

**Update main.rs**:
```rust
.route("/users/:id", put(update_user))
```

---

### Step 8: DELETE Endpoint (20 min)

**Add to user_handlers.rs**:

```rust
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    // TODO: Implement delete!
    // DELETE FROM users WHERE id = $1
    // Return 204 NO_CONTENT if success, 404 if not found

    StatusCode::NOT_IMPLEMENTED
}
```

**Update main.rs**:
```rust
.route("/users/:id", delete(delete_user))
```

---

## 🎯 Complete Workflow Test

Once everything is implemented:

```bash
# 1. Create
USER_ID=$(curl -s -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@example.com"}' \
  | jq -r '.id')

echo "Created user: $USER_ID"

# 2. Get
curl http://localhost:3000/users/$USER_ID | jq

# 3. List
curl "http://localhost:3000/users?page=1&per_page=10" | jq

# 4. Update
curl -X PUT http://localhost:3000/users/$USER_ID \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated Name"}' | jq

# 5. Delete
curl -X DELETE http://localhost:3000/users/$USER_ID

# 6. Verify deleted
curl http://localhost:3000/users/$USER_ID
# Should return 404
```

---

## 🧪 Write Tests (Optional but Recommended)

**Create** [tests/user_tests.rs](../tests/user_tests.rs):

```rust
use reqwest;
use serde_json::json;

#[tokio::test]
async fn test_create_user() {
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/users")
        .json(&json!({
            "name": "Test User",
            "email": "test@example.com"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let user: serde_json::Value = response.json().await.unwrap();
    assert_eq!(user["name"], "Test User");
}

// Add more tests...
```

Run tests:
```bash
cargo test
```

---

## 🐛 Common Issues & Quick Fixes

### Error: "Cannot borrow as mutable"
**Fix**: Add `mut`
```rust
let mut pool = create_pool().await?;
```

### Error: "Expected `Result`, found ..."
**Fix**: Wrap in `Ok()` or add `?`
```rust
Ok(Json(user))
```

### Error: "Value moved"
**Fix**: Clone or borrow
```rust
let value = &original_value;  // Borrow
```

### Database connection fails
**Fix**: Check `.env` and Docker
```bash
docker ps  # Is PostgreSQL running?
cat .env   # Is DATABASE_URL correct?
```

### Port already in use
**Fix**: Change port in `.env`
```
PORT=8080
```

---

## 📚 When You Get Stuck

1. **Read the compiler error** - It's usually very helpful
2. **Check [RUST_CONCEPTS.md](RUST_CONCEPTS.md)** - For concept explanations
3. **Look at the calculator** - It shows all basic patterns
4. **Google the error** - Rust has great community support
5. **Experiment** - Break things, fix them, learn!

---

## 🎓 What You've Learned

By following this quick start, you've learned:

- ✅ Basic Rust syntax and types
- ✅ Async/await with Tokio
- ✅ Web framework (Axum) patterns
- ✅ Database operations with SQLx
- ✅ Error handling with Result
- ✅ JSON serialization
- ✅ Complete REST API patterns

**Next**: Dive deeper into concepts with [RUST_CONCEPTS.md](RUST_CONCEPTS.md) or explore advanced topics!

---

**Remember**: The best way to learn Rust is to write Rust. Keep coding! 🚀
