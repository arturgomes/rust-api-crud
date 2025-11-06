# 🧠 Rust Concepts for TypeScript Engineers

Quick reference guide for TypeScript developers learning Rust. This maps familiar TS concepts to their Rust equivalents.

---

## 📋 Quick Comparison Table

| Concept | TypeScript | Rust | Key Difference |
|---------|-----------|------|----------------|
| **Null safety** | `null \| undefined` | `Option<T>` | Explicit, compile-time checked |
| **Error handling** | `try/catch` | `Result<T, E>` | Errors are values, not exceptions |
| **Type definition** | `interface` / `type` | `struct` / `enum` | Structs own data, traits define behavior |
| **Polymorphism** | `interface` | `trait` | Similar concept, different syntax |
| **Async** | `async/await` | `async/await` | Same syntax, different runtime |
| **Collections** | `Array<T>` | `Vec<T>` | Owned, stack/heap aware |
| **Strings** | `string` | `String` / `&str` | Owned vs borrowed |
| **Memory** | Garbage collected | Ownership/borrowing | Manual but compile-time checked |
| **Mutability** | Default mutable | Default immutable | Explicit with `mut` |

---

## 1. Type Definitions

### TypeScript: Interfaces & Types
```typescript
interface User {
  id: string;
  name: string;
  email: string;
  age?: number;  // Optional
}

type Status = 'active' | 'inactive';
```

### Rust: Structs & Enums
```rust
// Struct - like interface for data
struct User {
    id: String,
    name: String,
    email: String,
    age: Option<u32>,  // Optional - explicit!
}

// Enum - like union type but more powerful
enum Status {
    Active,
    Inactive,
}
```

**Key Differences**:
- No `?` for optional fields - use `Option<T>`
- Structs own their data (more on this below)
- Enums can hold data: `enum Result<T, E> { Ok(T), Err(E) }`

---

## 2. Null/Undefined Safety

### TypeScript: Nullable Types
```typescript
function getUser(id: string): User | null {
  // Might return null
}

const user = getUser('123');
if (user) {
  console.log(user.name);  // Runtime check
}
```

### Rust: Option<T>
```rust
fn get_user(id: String) -> Option<User> {
    // Might return None
}

let user = get_user(String::from("123"));
match user {
    Some(u) => println!("{}", u.name),  // Compile-time enforced!
    None => println!("Not found"),
}

// Or shorter:
if let Some(u) = user {
    println!("{}", u.name);
}
```

**Why It's Better**:
- Compiler forces you to handle `None` case
- No `null` or `undefined` - explicit optionality
- Pattern matching makes it ergonomic

**Common Patterns**:
```rust
// Unwrap (careful - panics if None!)
let user = get_user(id).unwrap();

// Unwrap with default
let user = get_user(id).unwrap_or_default();

// Map over Option
let name = get_user(id).map(|u| u.name);

// Early return if None (the ? operator)
let user = get_user(id)?;  // Returns None if None
```

---

## 3. Error Handling

### TypeScript: Try/Catch
```typescript
async function createUser(data: UserData): Promise<User> {
  try {
    const user = await db.insert(data);
    return user;
  } catch (error) {
    console.error('Failed:', error);
    throw error;
  }
}
```

### Rust: Result<T, E>
```rust
async fn create_user(data: UserData) -> Result<User, sqlx::Error> {
    // sqlx returns Result automatically
    let user = sqlx::query_as!(User, "INSERT INTO ...")
        .fetch_one(&pool)
        .await?;  // ? propagates errors up

    Ok(user)
}
```

**Key Concepts**:
- `Result<T, E>` - Success type T, Error type E
- `Ok(value)` - Success case
- `Err(error)` - Error case
- `?` operator - Propagate errors up (like throw)

**Pattern Matching**:
```rust
match create_user(data).await {
    Ok(user) => println!("Created: {}", user.id),
    Err(e) => eprintln!("Error: {}", e),
}
```

**The ? Operator** (Most Common):
```rust
async fn create_and_email_user(data: UserData) -> Result<(), Error> {
    let user = create_user(data).await?;  // Return error if fails
    send_email(&user.email).await?;       // Return error if fails
    Ok(())  // All good!
}
```

**Converting to HTTP Responses**:
```rust
async fn handler(
    Json(data): Json<UserData>,
) -> Result<Json<User>, StatusCode> {
    let user = create_user(data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}
```

---

## 4. Ownership & Borrowing (The Big One!)

**This is the most important concept.** It replaces garbage collection.

### The Rules
1. Each value has one owner
2. When owner goes out of scope, value is dropped
3. Values can be borrowed (referenced) without transferring ownership

### TypeScript: References Everywhere
```typescript
const user = { name: 'Alice' };
const user2 = user;  // Both reference same object
user2.name = 'Bob';
console.log(user.name);  // 'Bob' - shared reference
```

### Rust: Ownership Transfer
```rust
let user = User { name: String::from("Alice") };
let user2 = user;  // Ownership MOVED to user2
// println!("{}", user.name);  // ERROR! user is no longer valid
println!("{}", user2.name);  // OK
```

### Rust: Borrowing
```rust
let user = User { name: String::from("Alice") };
let user2 = &user;  // Borrow (reference) - ownership stays with user
println!("{}", user.name);   // OK - still owner
println!("{}", user2.name);  // OK - borrowed
```

**Mutable Borrowing**:
```rust
let mut user = User { name: String::from("Alice") };
let user_ref = &mut user;  // Mutable borrow
user_ref.name = String::from("Bob");
// Can't use `user` while `user_ref` exists!
println!("{}", user.name);  // OK after user_ref is done
```

**The Rules**:
- Many immutable borrows (`&T`) OR
- One mutable borrow (`&mut T`)
- But not both at the same time!

**Why It Matters**:
- No garbage collector overhead
- No data races (compile-time prevention!)
- Memory safety without runtime cost

**Practical Example** (Database):
```rust
// Pool is owned by the app state
let pool = create_pool().await?;

// Handlers borrow the pool
async fn create_user(
    State(pool): State<PgPool>,  // PgPool is Clone (cheap reference)
    Json(data): Json<UserData>,
) -> Result<Json<User>, StatusCode> {
    // pool is borrowed here, not moved
    sqlx::query_as!(User, "INSERT ...")
        .fetch_one(&pool)  // & = borrow
        .await
}
```

---

## 5. Strings

### TypeScript: One String Type
```typescript
const s: string = "hello";
const s2: string = s;  // Copy
```

### Rust: Two Main Types
```rust
// String - owned, heap-allocated, growable
let s: String = String::from("hello");

// &str - string slice, borrowed, fixed size
let s2: &str = "hello";  // String literal

// Convert between them
let owned: String = s2.to_string();
let borrowed: &str = &owned;
```

**When to Use**:
- `String` - When you own/modify the string (like fields in structs)
- `&str` - When you're just reading (like function parameters)

**Common Pattern**:
```rust
// Function accepts borrowed string
fn print_name(name: &str) {
    println!("{}", name);
}

// Works with both types!
let owned = String::from("Alice");
print_name(&owned);  // Borrow String as &str
print_name("Bob");   // &str literal
```

---

## 6. Collections

### TypeScript: Arrays
```typescript
const numbers: number[] = [1, 2, 3];
numbers.push(4);
```

### Rust: Vectors
```rust
// Vec<T> - growable array on heap
let mut numbers: Vec<i32> = vec![1, 2, 3];
numbers.push(4);

// Iterate
for num in &numbers {  // Borrow each element
    println!("{}", num);
}

// Map (functional style)
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect();
```

**Arrays vs Vectors**:
```rust
// Array - fixed size, stack allocated
let arr: [i32; 3] = [1, 2, 3];

// Vector - dynamic size, heap allocated
let vec: Vec<i32> = vec![1, 2, 3];
```

---

## 7. Pattern Matching

### TypeScript: Switch
```typescript
switch (operation) {
  case 'add':
    return a + b;
  case 'subtract':
    return a - b;
  default:
    throw new Error('Unknown op');
}
```

### Rust: Match (Much More Powerful!)
```rust
match operation.as_str() {
    "add" => a + b,
    "subtract" => a - b,
    _ => panic!("Unknown op"),  // _ = default
}
```

**Matching Enums**:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => eprintln!("Error: {}", e),
}
```

**Matching Option**:
```rust
match user {
    Some(u) => println!("Found: {}", u.name),
    None => println!("Not found"),
}

// Or use if let for one case
if let Some(u) = user {
    println!("Found: {}", u.name);
}
```

**Exhaustiveness**:
```rust
// Compiler ensures ALL cases handled!
match status {
    Status::Active => "active",
    // Forgot Inactive? Compiler error!
}
```

---

## 8. Async/Await

### TypeScript: Node.js Runtime
```typescript
async function fetchUser(id: string): Promise<User> {
  const response = await fetch(`/users/${id}`);
  return response.json();
}
```

### Rust: Tokio Runtime
```rust
async fn fetch_user(id: String) -> Result<User, Error> {
    let response = reqwest::get(&format!("/users/{}", id))
        .await?;
    response.json().await
}
```

**Key Differences**:
- Need a runtime: `#[tokio::main]`
- More explicit about async traits
- Zero-cost abstractions (no overhead!)

**Starting Async Code**:
```rust
#[tokio::main]
async fn main() {
    let result = fetch_user(String::from("123")).await;
}
```

---

## 9. Traits (Like Interfaces)

### TypeScript: Interfaces
```typescript
interface Serializable {
  toJSON(): string;
}

class User implements Serializable {
  toJSON(): string {
    return JSON.stringify(this);
  }
}
```

### Rust: Traits
```rust
trait Serializable {
    fn to_json(&self) -> String;
}

impl Serializable for User {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
```

**Common Traits**:
- `Clone` - Can be cloned
- `Debug` - Can be printed with `{:?}`
- `Serialize`/`Deserialize` - JSON conversion (serde)

**Derive Macro** (Auto-implement):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}
// Now User automatically has these traits!
```

---

## 10. Mutability

### TypeScript: Default Mutable
```typescript
let x = 5;
x = 6;  // OK

const y = 5;
// y = 6;  // Error - const
```

### Rust: Default Immutable
```rust
let x = 5;
// x = 6;  // Error - immutable by default

let mut x = 5;
x = 6;  // OK - explicitly mutable
```

**Why It Matters**:
- Forces you to think about mutability
- Helps prevent bugs
- Enables compiler optimizations

**Mutable References**:
```rust
fn increment(x: &mut i32) {
    *x += 1;  // * = dereference
}

let mut num = 5;
increment(&mut num);  // Pass mutable reference
println!("{}", num);  // 6
```

---

## 11. Common Patterns

### Map/Filter/Reduce (Iterators)

**TypeScript**:
```typescript
const doubled = numbers.map(x => x * 2);
const evens = numbers.filter(x => x % 2 === 0);
const sum = numbers.reduce((a, b) => a + b, 0);
```

**Rust**:
```rust
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect();

let evens: Vec<i32> = numbers.iter()
    .filter(|x| *x % 2 == 0)
    .cloned()
    .collect();

let sum: i32 = numbers.iter().sum();
```

### Destructuring

**TypeScript**:
```typescript
const { name, email } = user;
const [first, second] = array;
```

**Rust**:
```rust
let User { name, email, .. } = user;  // .. = ignore rest
let [first, second] = array;

// More common in match
match point {
    Point { x: 0, y } => println!("On y-axis at {}", y),
    Point { x, y: 0 } => println!("On x-axis at {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

---

## 🎯 Key Takeaways

1. **Option<T>** replaces null/undefined - compiler-enforced handling
2. **Result<T, E>** replaces try/catch - errors are values
3. **Ownership** replaces GC - one owner, borrowing for access
4. **match** replaces switch - exhaustive, powerful pattern matching
5. **Immutable by default** - explicit `mut` for changes
6. **Traits** replace interfaces - behavior definitions
7. **String vs &str** - owned vs borrowed strings
8. **Vec<T>** - growable arrays, like Array<T> but ownership-aware

---

## 🚀 Quick Mental Model

**Coming from TypeScript, think of Rust as**:
- TypeScript's type safety × 100
- Manual memory management (but compiler-checked!)
- No null/undefined (use Option)
- No exceptions (use Result)
- Explicit about ownership and borrowing
- Performance-first (zero-cost abstractions)

**The Rust Compiler is Your Friend**:
- Errors are detailed and helpful
- It prevents entire classes of bugs at compile-time
- No data races, no null pointer errors, no use-after-free
- If it compiles, it usually works!

---

## 📚 When You're Stuck

1. **Read the error message** - Rust errors are incredibly helpful
2. **Check the docs** - https://doc.rust-lang.org/
3. **Pattern match everything** - When in doubt, use `match`
4. **Use `?` for errors** - Propagate errors up automatically
5. **Borrow by default** - Use `&` unless you need ownership
6. **Make it compile first** - Optimize later

---

## 🎓 Next Steps

- **Practice**: The more you write, the more natural it becomes
- **Read**: Check [The Rust Book](https://doc.rust-lang.org/book/)
- **Experiment**: Break things and learn from compiler errors
- **Build**: This CRUD API is your hands-on learning project

Remember: **The compiler is teaching you.** Rust errors are designed to be educational. Read them carefully!
