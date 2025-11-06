# 🧪 TDD Guide - Test-Driven Development in Rust

Learn Rust through the Red-Green-Refactor cycle.

---

## The TDD Workflow

```
🔴 RED    → Write failing test
💡 THINK  → Understand requirements
🟢 GREEN  → Write minimal code to pass
🔄 REFACTOR → Improve code quality
✅ VERIFY → All tests pass
```

Repeat for each feature.

---

## Why TDD for Learning Rust?

1. **Tests guide you** - Know exactly what to build
2. **Safety net** - Experiment without fear of breaking things
3. **Immediate feedback** - Know when you're done
4. **Learn patterns** - See Rust idioms in action
5. **Build confidence** - Proven working code

---

## Setting Up Tests

### Test File Structure

```
rust-api-crud/
├── src/
│   └── main.rs              # Application code
└── tests/
    ├── calculator_tests.rs  # Phase 0 tests
    ├── user_tests.rs        # Phase 2 tests
    └── common/
        └── mod.rs           # Shared test utilities
```

### Integration Test Template

```rust
// tests/user_tests.rs

use serde_json::json;
use uuid::Uuid;

// Test server setup (reusable helper)
async fn setup_test_server() -> String {
    "http://localhost:3000".to_string()
}

#[tokio::test]
async fn test_create_user() {
    let base_url = setup_test_server().await;
    let client = reqwest::Client::new();

    // Arrange - Set up test data
    let user_data = json!({
        "name": "Test User",
        "email": "test@example.com"
    });

    // Act - Execute the operation
    let response = client
        .post(&format!("{}/users", base_url))
        .json(&user_data)
        .send()
        .await
        .expect("Failed to send request");

    // Assert - Verify expectations
    assert_eq!(response.status(), 200);

    let user: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert_eq!(user["name"], "Test User");
    assert_eq!(user["email"], "test@example.com");
    assert!(user["id"].is_string());
}
```

---

## Phase 0: Calculator TDD Example

### Step 1: Write Failing Test (RED 🔴)

```rust
// tests/calculator_tests.rs

#[tokio::test]
async fn test_addition() {
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/calculate?a=5&b=3&op=add")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let result: serde_json::Value = response.json().await.unwrap();
    assert_eq!(result["result"], 8.0);
}
```

**Run it**:
```bash
cargo test test_addition
```

**Expected**: ❌ Test fails (endpoint doesn't exist)

### Step 2: Write Minimal Code (GREEN 🟢)

```rust
// src/main.rs

#[derive(Deserialize)]
struct CalculatorRequest {
    a: f64,
    b: f64,
    op: String,
}

#[derive(Serialize)]
struct CalculatorResponse {
    result: f64,
}

async fn calculate(Query(params): Query<CalculatorRequest>) -> Json<serde_json::Value> {
    let result = match params.op.as_str() {
        "add" => params.a + params.b,
        _ => 0.0,
    };
    Json(serde_json::json!(CalculatorResponse { result }))
}

let app = Router::new().route("/calculate", get(calculate));
```

**Run it**:
```bash
cargo test test_addition
```

**Expected**: ✅ Test passes!

### Step 3: Refactor (REFACTOR 🔄)

- Add error handling
- Add more operations
- Improve code organization

### Step 4: Add More Tests (Repeat)

```rust
#[tokio::test]
async fn test_division_by_zero() {
    // Test error case
}

#[tokio::test]
async fn test_unknown_operation() {
    // Test error case
}
```

---

## Phase 2: CRUD TDD

### CREATE User - Red-Green-Refactor

#### Red 🔴: Failing Test

```rust
#[tokio::test]
async fn test_create_user_success() {
    let base_url = "http://localhost:3000";
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/users", base_url))
        .json(&json!({
            "name": "Alice",
            "email": "alice@example.com"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);

    let user: serde_json::Value = response.json().await.unwrap();
    assert_eq!(user["name"], "Alice");
    assert_eq!(user["email"], "alice@example.com");
    assert!(user["id"].is_string());

    // Cleanup
    let user_id = user["id"].as_str().unwrap();
    cleanup_user(user_id).await;
}
```

#### Green 🟢: Minimal Implementation

```rust
// src/handlers/user_handlers.rs

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}
```

#### Refactor 🔄: Improve

- Add input validation
- Add better error handling
- Add duplicate email check

```rust
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    // Validate email
    if !payload.email.contains('@') {
        return Err(AppError::InvalidInput("Invalid email format".into()));
    }

    // Insert with better error handling
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.constraint() == Some("users_email_key") {
                return AppError::DuplicateEmail;
            }
        }
        AppError::DatabaseError(e)
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}
```

#### Add More Test Cases

```rust
#[tokio::test]
async fn test_create_user_invalid_email() {
    // Should return 400 Bad Request
}

#[tokio::test]
async fn test_create_user_duplicate_email() {
    // Create user
    // Try to create same email again
    // Should return 409 Conflict
}
```

---

## Test Organization

### Shared Test Utilities

```rust
// tests/common/mod.rs

use serde_json::json;
use uuid::Uuid;

pub struct TestContext {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl TestContext {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn create_test_user(&self, name: &str, email: &str) -> Uuid {
        let response = self.client
            .post(&format!("{}/users", self.base_url))
            .json(&json!({ "name": name, "email": email }))
            .send()
            .await
            .unwrap();

        let user: serde_json::Value = response.json().await.unwrap();
        Uuid::parse_str(user["id"].as_str().unwrap()).unwrap()
    }

    pub async fn cleanup_user(&self, id: Uuid) {
        let _ = self.client
            .delete(&format!("{}/users/{}", self.base_url, id))
            .send()
            .await;
    }
}
```

**Use in tests**:

```rust
// tests/user_tests.rs

mod common;
use common::TestContext;

#[tokio::test]
async fn test_get_user() {
    let ctx = TestContext::new();

    // Create test user
    let user_id = ctx.create_test_user("Test", "test@example.com").await;

    // Test get user
    let response = ctx.client
        .get(&format!("{}/users/{}", ctx.base_url, user_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    // Cleanup
    ctx.cleanup_user(user_id).await;
}
```

---

## Test-Driven Learning Strategy

### For Each Endpoint

1. **Read the test** - Understand what's expected
2. **Run the test** - See it fail (Red 🔴)
3. **Think about requirements** - What data structures? What queries?
4. **Implement minimal solution** - Just enough to pass
5. **Run test again** - See it pass (Green 🟢)
6. **Refactor** - Improve code quality
7. **Add edge case tests** - Error handling, validation
8. **Repeat refactor until satisfied** (Refactor 🔄)

### Example: UPDATE User Endpoint

#### 1. Read Pre-Written Test

```rust
#[tokio::test]
async fn test_update_user() {
    let ctx = TestContext::new();
    let user_id = ctx.create_test_user("Original", "original@example.com").await;

    // Update user
    let response = ctx.client
        .put(&format!("{}/users/{}", ctx.base_url, user_id))
        .json(&json!({ "name": "Updated" }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let user: serde_json::Value = response.json().await.unwrap();
    assert_eq!(user["name"], "Updated");
    assert_eq!(user["email"], "original@example.com");

    ctx.cleanup_user(user_id).await;
}
```

#### 2. Run Test (Should Fail)

```bash
cargo test test_update_user
```

❌ Fails: "No route found for PUT /users/:id"

#### 3. Implement Handler

```rust
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET
            name = COALESCE($1, name),
            email = COALESCE($2, email)
        WHERE id = $3
        RETURNING *",
        payload.name,
        payload.email,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}
```

#### 4. Run Test Again

```bash
cargo test test_update_user
```

✅ Passes!

#### 5. Add Edge Cases

```rust
#[tokio::test]
async fn test_update_user_not_found() {
    // Should return 404
}

#[tokio::test]
async fn test_update_user_partial() {
    // Update only name, email stays same
}
```

---

## Running Tests

### All Tests
```bash
cargo test
```

### Specific Test
```bash
cargo test test_create_user
```

### Show Output
```bash
cargo test -- --nocapture
```

### Run Sequentially (for database tests)
```bash
cargo test -- --test-threads=1
```

### Watch Mode (requires cargo-watch)
```bash
cargo install cargo-watch
cargo watch -x test
```

---

## Test Database Strategy

### Option 1: Cleanup After Each Test
```rust
#[tokio::test]
async fn test_something() {
    // Arrange
    let user_id = create_test_user().await;

    // Act
    test_something_with_user(user_id).await;

    // Cleanup
    delete_test_user(user_id).await;
}
```

### Option 2: Transaction Rollback
```rust
#[tokio::test]
async fn test_something() {
    let mut tx = pool.begin().await.unwrap();

    // All operations on &mut tx
    create_user_in_transaction(&mut tx, user_data).await;

    // Transaction automatically rolled back at end of test
}
```

### Option 3: Separate Test Database
```bash
# .env.test
DATABASE_URL=postgres://rustuser:rustpass@localhost:5432/rustcrud_test
```

---

## Debugging Tests

### Print in Tests
```rust
#[tokio::test]
async fn test_something() {
    println!("Debug: {:?}", value);
    eprintln!("Error: {:?}", error);

    // Run with --nocapture to see output
}
```

### Use dbg! Macro
```rust
let result = expensive_computation();
dbg!(&result);  // Prints with file and line number
```

### Test-Only Code
```rust
#[cfg(test)]
impl User {
    pub fn test_user() -> Self {
        User {
            id: Uuid::new_v4(),
            name: "Test".into(),
            email: "test@example.com".into(),
            created_at: Utc::now(),
        }
    }
}
```

---

## TDD Best Practices

1. **Write test first** - Forces you to think about interface
2. **One thing at a time** - Test one behavior per test
3. **Descriptive names** - `test_create_user_with_invalid_email`
4. **Arrange-Act-Assert** - Clear test structure
5. **Fast tests** - Keep integration tests fast
6. **Independent tests** - No test depends on another
7. **Cleanup** - Always clean up test data
8. **Read errors** - Rust test errors are helpful

---

## Example: Complete TDD Cycle

### Feature: Delete User

#### Test 1: Successful Delete
```rust
#[tokio::test]
async fn test_delete_user_success() {
    let ctx = TestContext::new();
    let user_id = ctx.create_test_user("ToDelete", "delete@example.com").await;

    let response = ctx.client
        .delete(&format!("{}/users/{}", ctx.base_url, user_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 204);

    // Verify user is gone
    let get_response = ctx.client
        .get(&format!("{}/users/{}", ctx.base_url, user_id))
        .send()
        .await
        .unwrap();

    assert_eq!(get_response.status(), 404);
}
```

**Run**: ❌ Fails

**Implement**:
```rust
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
```

**Run**: ✅ Passes

#### Test 2: Delete Non-Existent User
```rust
#[tokio::test]
async fn test_delete_user_not_found() {
    let ctx = TestContext::new();
    let fake_id = Uuid::new_v4();

    let response = ctx.client
        .delete(&format!("{}/users/{}", ctx.base_url, fake_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}
```

**Run**: ✅ Passes (already handled!)

---

## 🎯 TDD Checklist for Each Feature

- [ ] Write failing test (Red 🔴)
- [ ] Understand requirements from test
- [ ] Write minimal implementation
- [ ] Test passes (Green 🟢)
- [ ] Refactor code
- [ ] Test still passes
- [ ] Add edge case tests
- [ ] All tests pass
- [ ] Code is clean
- [ ] Move to next feature

---

**Remember**: TDD isn't about perfect tests. It's about having a clear goal and knowing when you're done. Let the tests guide your learning! 🚀
