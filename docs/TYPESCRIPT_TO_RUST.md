# 📘 TypeScript to Rust: Deep Dive Comparison

Comprehensive guide for TypeScript engineers learning Rust with detailed examples and mental models.

---

## Table of Contents

1. [Ownership & Borrowing Explained](#1-ownership--borrowing-explained)
2. [Memory Model Comparison](#2-memory-model-comparison)
3. [Async Runtime Differences](#3-async-runtime-differences)
4. [Error Handling Philosophy](#4-error-handling-philosophy)
5. [Type System Depth](#5-type-system-depth)
6. [Web Framework Patterns](#6-web-framework-patterns)
7. [Database Patterns](#7-database-patterns)
8. [Common Gotchas](#8-common-gotchas)

---

## 1. Ownership & Borrowing Explained

This is **THE** fundamental difference. Understanding this unlocks Rust.

### The Problem Rust Solves

**TypeScript/JavaScript** (Garbage Collected):
```typescript
const user = { name: 'Alice', email: 'alice@example.com' };
const user2 = user;  // Both reference same object
user2.name = 'Bob';

console.log(user.name);  // 'Bob' - surprise!

// Behind the scenes:
// - GC tracks references
// - Memory freed when no references remain
// - Runtime overhead for GC pauses
// - Possible memory leaks if references retained
```

**Rust** (Ownership):
```rust
let user = User { name: String::from("Alice"), email: String::from("alice@example.com") };
let user2 = user;  // Ownership MOVED (not copied)

// println!("{}", user.name);  // COMPILE ERROR: value borrowed after move

// Benefits:
// - No GC needed
// - Memory freed immediately when owner goes out of scope
// - Zero runtime overhead
// - Impossible to have memory leaks (compiler prevents)
```

### The Three Rules of Ownership

1. **Each value has exactly ONE owner**
2. **When the owner goes out of scope, value is dropped (freed)**
3. **Values can be borrowed without transferring ownership**

### Practical Example: Function Calls

**TypeScript**:
```typescript
function processUser(user: User): void {
  console.log(user.name);
  // user reference still valid after function
}

const user = { name: 'Alice' };
processUser(user);
console.log(user.name);  // Still works
```

**Rust - Ownership Transfer** (Move):
```rust
fn process_user(user: User) {  // Takes ownership
    println!("{}", user.name);
}  // user dropped here

let user = User { name: String::from("Alice") };
process_user(user);  // user moved into function
// println!("{}", user.name);  // ERROR! user no longer valid
```

**Rust - Borrowing** (The Right Way):
```rust
fn process_user(user: &User) {  // Borrows (doesn't take ownership)
    println!("{}", user.name);
}  // user reference ends, but original still valid

let user = User { name: String::from("Alice") };
process_user(&user);  // Borrow user
println!("{}", user.name);  // Still works!
```

### Mutable vs Immutable Borrowing

**Immutable Borrow** (`&T`):
```rust
fn read_user(user: &User) {
    println!("{}", user.name);
    // Can't modify user
}

let user = User { name: String::from("Alice") };
read_user(&user);  // Borrow immutably
read_user(&user);  // Can borrow multiple times!
```

**Mutable Borrow** (`&mut T`):
```rust
fn update_user(user: &mut User) {
    user.name = String::from("Bob");  // Can modify
}

let mut user = User { name: String::from("Alice") };
update_user(&mut user);  // Borrow mutably

// IMPORTANT: Only ONE mutable borrow at a time!
// let ref1 = &mut user;
// let ref2 = &mut user;  // ERROR: cannot borrow as mutable more than once
```

**The Borrowing Rules**:
- **Many immutable borrows** (`&T`, `&T`, `&T`) OR
- **One mutable borrow** (`&mut T`)
- **But never both at the same time!**

**Why?** Prevents data races at compile time!

### Real-World Example: Database Handler

**TypeScript** (Express + TypeORM):
```typescript
// Connection pool shared by reference
const pool = createPool();

app.post('/users', async (req, res) => {
  // pool used here
  const user = await pool.query('INSERT ...');
});

app.get('/users/:id', async (req, res) => {
  // pool used here too
  const user = await pool.query('SELECT ...');
});
// No explicit lifetime management needed (GC handles it)
```

**Rust** (Axum + SQLx):
```rust
// Pool is cloneable (cheap Arc reference internally)
let pool = create_pool().await?;

// Share pool via state
let app = Router::new()
    .route("/users", post(create_user))
    .route("/users/:id", get(get_user))
    .with_state(pool);  // Pool cloned for each handler

async fn create_user(
    State(pool): State<PgPool>,  // pool borrowed here
) -> Result<Json<User>, StatusCode> {
    // Use &pool to borrow
    sqlx::query_as!(User, "INSERT ...")
        .fetch_one(&pool)
        .await
}
// No GC, no manual free - compiler handles everything!
```

---

## 2. Memory Model Comparison

### Stack vs Heap

**TypeScript**: You don't think about this (hidden by GC)

**Rust**: You need to understand it

**Stack**:
- Fast allocation (just move stack pointer)
- Fixed size known at compile time
- Automatically freed when scope ends

**Heap**:
- Slower allocation (find free space)
- Dynamic size at runtime
- Must be explicitly freed (Rust does this via ownership)

```rust
// Stack allocated (fixed size)
let x: i32 = 5;
let arr: [i32; 3] = [1, 2, 3];

// Heap allocated (dynamic size)
let s: String = String::from("hello");
let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

// String internally:
// Stack: { pointer, length, capacity }
// Heap: actual character data
```

### Copy vs Move

**Types that implement Copy** (cheap to copy, on stack):
- All integer types (`i32`, `u64`, etc.)
- Floating point (`f32`, `f64`)
- Booleans (`bool`)
- Characters (`char`)
- Tuples of Copy types

```rust
let x = 5;
let y = x;  // COPIED (both valid)
println!("x: {}, y: {}", x, y);  // Works!
```

**Types that Move** (expensive to copy, on heap):
- `String`
- `Vec<T>`
- Most structs (unless they derive Copy)

```rust
let s1 = String::from("hello");
let s2 = s1;  // MOVED (s1 invalidated)
// println!("{}", s1);  // ERROR: value moved
```

**Forcing a Copy** (Clone):
```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit deep copy
println!("s1: {}, s2: {}", s1, s2);  // Both valid
```

---

## 3. Async Runtime Differences

### Node.js vs Tokio

**Node.js**:
- Single-threaded event loop
- Callbacks, Promises, async/await
- Built-in runtime
- I/O automatically non-blocking

**Tokio**:
- Multi-threaded work-stealing scheduler
- Explicit async runtime
- Zero-cost abstractions
- Must explicitly choose async operations

### Starting Async Code

**TypeScript**:
```typescript
// Built-in runtime, just use async
async function main() {
  const result = await fetchData();
}

main();  // Automatically handled
```

**Rust**:
```rust
// Need to choose a runtime
#[tokio::main]  // This macro sets up Tokio runtime
async fn main() {
    let result = fetch_data().await;
}

// Alternative: Manual runtime
// let runtime = tokio::runtime::Runtime::new()?;
// runtime.block_on(async { ... });
```

### Async Functions

**TypeScript**:
```typescript
async function fetchUser(id: string): Promise<User> {
  const response = await fetch(`/users/${id}`);
  return response.json();
}
```

**Rust**:
```rust
async fn fetch_user(id: String) -> Result<User, Error> {
    let response = reqwest::get(&format!("/users/{}", id))
        .await?;
    response.json().await
}
```

**Key Difference**: Rust's `async fn` returns a `Future` that must be `.await`ed or spawned

### Spawning Tasks

**TypeScript** (Node.js):
```typescript
// Everything runs on event loop
Promise.all([
  fetchUser('1'),
  fetchUser('2'),
  fetchUser('3'),
]);
// Concurrent but still single-threaded
```

**Rust** (Tokio):
```rust
// True parallel execution possible
let handles = vec![
    tokio::spawn(fetch_user(String::from("1"))),
    tokio::spawn(fetch_user(String::from("2"))),
    tokio::spawn(fetch_user(String::from("3"))),
];

// Wait for all
for handle in handles {
    let user = handle.await??;
}
```

### Axum vs Express

**Express**:
```typescript
const app = express();

app.get('/users/:id', async (req, res) => {
  const user = await db.findUser(req.params.id);
  res.json(user);
});

app.listen(3000);
```

**Axum**:
```rust
let app = Router::new()
    .route("/users/:id", get(get_user));

async fn get_user(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
axum::serve(listener, app).await?;
```

**Key Differences**:
- Axum uses extractors (`Path`, `State`, `Json`) - compile-time validated
- Return types are checked at compile time
- No middleware function order issues
- Type-safe throughout

---

## 4. Error Handling Philosophy

### Exceptions vs Results

**TypeScript Philosophy**: Errors are exceptional, use exceptions

**Rust Philosophy**: Errors are values, handle explicitly

### TypeScript: Try/Catch

```typescript
async function createUser(data: UserData): Promise<User> {
  try {
    // Validate
    if (!data.email.includes('@')) {
      throw new Error('Invalid email');
    }

    // Database operation
    const user = await db.insert(data);
    return user;

  } catch (error) {
    console.error('Failed to create user:', error);
    throw error;  // Propagate up
  }
}

// Caller
try {
  const user = await createUser(data);
  console.log('Created:', user.id);
} catch (error) {
  console.error('Error:', error.message);
}
```

### Rust: Result<T, E>

```rust
async fn create_user(data: UserData) -> Result<User, CreateUserError> {
    // Validate
    if !data.email.contains('@') {
        return Err(CreateUserError::InvalidEmail);
    }

    // Database operation - ? propagates errors
    let user = sqlx::query_as!(User, "INSERT INTO users ...")
        .fetch_one(&pool)
        .await
        .map_err(|e| CreateUserError::DatabaseError(e))?;

    Ok(user)
}

// Define custom error type
enum CreateUserError {
    InvalidEmail,
    DatabaseError(sqlx::Error),
    DuplicateEmail,
}

// Caller
match create_user(data).await {
    Ok(user) => println!("Created: {}", user.id),
    Err(CreateUserError::InvalidEmail) => eprintln!("Invalid email"),
    Err(CreateUserError::DuplicateEmail) => eprintln!("Email exists"),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### The ? Operator (Error Propagation)

**TypeScript**: `throw error` or `return Promise.reject(error)`

**Rust**: `?` operator

```rust
// Without ?
async fn get_user(id: Uuid) -> Result<User, Error> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => Err(e.into()),
    }
}

// With ? (equivalent, much cleaner)
async fn get_user(id: Uuid) -> Result<User, Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await?;  // If error, return it; otherwise unwrap Ok

    Ok(user)
}
```

### Converting to HTTP Responses

**Express**:
```typescript
app.get('/users/:id', async (req, res) => {
  try {
    const user = await getUser(req.params.id);
    res.json(user);
  } catch (error) {
    if (error instanceof NotFoundError) {
      res.status(404).json({ error: 'User not found' });
    } else {
      res.status(500).json({ error: 'Internal error' });
    }
  }
});
```

**Axum**:
```rust
async fn get_user(
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let user = get_user_from_db(id)
        .await
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(user))
}

// Or implement IntoResponse for your error type
impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            UserError::NotFound => (StatusCode::NOT_FOUND, "User not found"),
            UserError::InvalidInput => (StatusCode::BAD_REQUEST, "Invalid input"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

---

## 5. Type System Depth

### Structural vs Nominal Typing

**TypeScript**: Structural ("duck typing")
```typescript
interface User {
  name: string;
  email: string;
}

interface Person {
  name: string;
  email: string;
}

const user: User = { name: 'Alice', email: 'alice@example.com' };
const person: Person = user;  // OK - same structure
```

**Rust**: Nominal (name matters)
```rust
struct User {
    name: String,
    email: String,
}

struct Person {
    name: String,
    email: String,
}

let user = User { name: String::from("Alice"), email: String::from("alice@example.com") };
// let person: Person = user;  // ERROR - different types!
```

### Generics

**TypeScript**:
```typescript
function first<T>(arr: T[]): T | undefined {
  return arr[0];
}

const num = first([1, 2, 3]);  // T inferred as number
const str = first(['a', 'b']);  // T inferred as string
```

**Rust**:
```rust
fn first<T>(arr: &[T]) -> Option<&T> {
    arr.get(0)
}

let num = first(&[1, 2, 3]);  // T inferred as i32
let str = first(&["a", "b"]);  // T inferred as &str
```

### Trait Bounds (Like Interface Constraints)

**TypeScript**:
```typescript
interface Comparable {
  compare(other: this): number;
}

function max<T extends Comparable>(a: T, b: T): T {
  return a.compare(b) > 0 ? a : b;
}
```

**Rust**:
```rust
trait Comparable {
    fn compare(&self, other: &Self) -> i32;
}

fn max<T: Comparable>(a: T, b: T) -> T {
    if a.compare(&b) > 0 { a } else { b }
}

// Or with where clause (more readable for many bounds)
fn max<T>(a: T, b: T) -> T
where
    T: Comparable + Clone + Debug,
{
    if a.compare(&b) > 0 { a } else { b }
}
```

---

## 6. Web Framework Patterns

### Request Handling

**Express Middleware**:
```typescript
app.use(express.json());
app.use(cors());
app.use(logger);

app.get('/users/:id',
  authenticate,  // Middleware
  authorize('admin'),  // Middleware
  async (req, res) => {
    // Handler
  }
);
```

**Axum Extractors + Layers**:
```rust
let app = Router::new()
    .route("/users/:id", get(get_user))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive());

async fn get_user(
    Path(id): Path<Uuid>,  // Extract path param
    State(pool): State<PgPool>,  // Extract shared state
    Extension(user): Extension<AuthUser>,  // Extract from middleware
) -> Result<Json<User>, StatusCode> {
    // Handler implementation
}
```

### State Sharing

**Express**: Attach to req/res or use closures
```typescript
const db = createConnection();

app.get('/users', (req, res) => {
  // db captured in closure
  db.query('SELECT * FROM users');
});
```

**Axum**: Type-safe state with `.with_state()`
```rust
#[derive(Clone)]
struct AppState {
    pool: PgPool,
    cache: Arc<Cache>,
}

let state = AppState { pool, cache };

let app = Router::new()
    .route("/users", get(list_users))
    .with_state(state);

async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {
    // Use state.pool, state.cache
}
```

---

## 7. Database Patterns

### Query Builders

**TypeORM** (TypeScript):
```typescript
const user = await User
  .createQueryBuilder('user')
  .where('user.email = :email', { email })
  .getOne();
```

**SQLx** (Rust) - Compile-time checked!:
```rust
let user = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE email = $1",
    email
)
.fetch_one(&pool)
.await?;

// Type-checked at compile time!
// If query doesn't match User struct, won't compile
```

### Migrations

**TypeORM**:
```typescript
// Generated migration file
export class CreateUsers1234567890 implements MigrationInterface {
  async up(queryRunner: QueryRunner): Promise<void> {
    await queryRunner.query(`CREATE TABLE users ...`);
  }
}
```

**SQLx**:
```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE
);
```

```bash
# Run migrations
sqlx migrate run
```

---

## 8. Common Gotchas

### Gotcha 1: String Concatenation

**TypeScript**: Easy
```typescript
const greeting = `Hello, ${name}!`;
```

**Rust**: Multiple ways
```rust
// format! macro (like template literal)
let greeting = format!("Hello, {}!", name);

// String concatenation (ownership gotcha!)
let greeting = "Hello, ".to_string() + &name + "!";

// Or use push_str
let mut greeting = String::from("Hello, ");
greeting.push_str(&name);
greeting.push('!');
```

### Gotcha 2: Modifying While Iterating

**TypeScript**: Works (GC protects)
```typescript
const users = [user1, user2, user3];
for (const user of users) {
  users.push(newUser);  // Modifying while iterating (dangerous but allowed)
}
```

**Rust**: Compiler prevents
```rust
let mut users = vec![user1, user2, user3];
for user in &users {  // Immutable borrow
    // users.push(new_user);  // ERROR: can't mutate while borrowed
}

// Solution: Collect what to add, then add
let to_add: Vec<_> = users.iter()
    .filter(|u| u.needs_copy())
    .cloned()
    .collect();
users.extend(to_add);
```

### Gotcha 3: Optional Chaining

**TypeScript**: Built-in
```typescript
const email = user?.profile?.email;
```

**Rust**: Use combinators
```rust
let email = user
    .as_ref()
    .and_then(|u| u.profile.as_ref())
    .map(|p| &p.email);
```

### Gotcha 4: Default Values

**TypeScript**:
```typescript
const port = process.env.PORT || 3000;
```

**Rust**:
```rust
let port = std::env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(3000);
```

### Gotcha 5: Closures and Ownership

**TypeScript**: Capture by reference
```typescript
const counter = { value: 0 };
const increment = () => counter.value++;  // Captures reference
```

**Rust**: Explicit capture semantics
```rust
let mut counter = 0;

// Borrow
let increment = || counter += 1;  // ERROR if counter used elsewhere

// Move
let increment = move || counter += 1;  // Moves counter into closure
```

---

## 🎯 Mental Model Summary

**TypeScript**: "Everything is a reference, GC cleans up"

**Rust**: "Everything has an owner, compiler ensures safety"

| Aspect | TypeScript | Rust |
|--------|-----------|------|
| **Memory** | GC manages | Ownership manages |
| **Null** | null/undefined | Option<T> |
| **Errors** | Exceptions | Result<T, E> |
| **Concurrency** | Event loop | Threads + async |
| **Mutability** | Default mutable | Default immutable |
| **Types** | Structural | Nominal |
| **Performance** | Runtime overhead | Zero-cost abstractions |

---

## 🚀 Conversion Strategy

When converting TypeScript code to Rust:

1. **Find the types** - TypeScript interfaces → Rust structs
2. **Identify ownership** - Who owns each value?
3. **Convert error handling** - try/catch → Result<T, E>
4. **Handle nulls** - null/undefined → Option<T>
5. **Understand borrowing** - When to use `&` vs owned values
6. **Choose the runtime** - Tokio for async
7. **Test incrementally** - Compiler guides you

---

**Ready to build?** Continue with [LEARNING_PATH.md](LEARNING_PATH.md) to start your hands-on journey!
