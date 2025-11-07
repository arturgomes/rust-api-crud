# Rust CRUD API - TDD Learning Journey

A comprehensive TDD-driven tutorial for TypeScript engineers learning Rust through building a production-ready REST API.

## 🎯 What You'll Build

A complete CRUD API with:
- **Phase 0**: Calculator API (warm-up)
- **Phase 1**: PostgreSQL infrastructure with Docker
- **Phase 2**: Full User CRUD operations (Create, Read, List, Update, Delete)

**Tech Stack**: Axum + SQLx + PostgreSQL + Docker + Tokio

## If you want to start from scratch, you should checkout the [`starting-point`](https://github.com/arturgomes/rust-api-crud/tree/starting-point) branch.

```bash
git clone https://github.com/arturgomes/rust-api-crud/
git checkout starting-point
```

## 🚀 Quick Start (3 Commands)

```bash
# 1. Start the database
docker-compose up -d

# 2. Run migrations
cargo install sqlx-cli
sqlx migrate run

# 3. Run the server
cargo run
```

## ✅ Prerequisites

- [ ] Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- [ ] Docker Desktop running
- [ ] PostgreSQL client (optional, for manual queries)
- [ ] TypeScript/JavaScript experience (assumed)

**Verification Commands:**
```bash
rustc --version  # Should show 1.70+
docker --version # Should show 20.10+
cargo --version  # Should show 1.70+
```

## 📚 Learning Path

Start here → **[LEARNING_PATH.md](docs/LEARNING_PATH.md)**

This is your step-by-step guide through the entire journey with:
- Clear checkpoints and validation commands
- "You are here" markers
- Troubleshooting help at each stage

## 🧠 Concept Reference

New to Rust from TypeScript? Check **[RUST_CONCEPTS.md](docs/RUST_CONCEPTS.md)** for:
- TypeScript → Rust comparison tables
- Ownership/borrowing explained for JS developers
- Common pitfalls and solutions
- When to use what pattern

## 📖 Additional Resources

- **[TYPESCRIPT_TO_RUST.md](docs/TYPESCRIPT_TO_RUST.md)** - Deep dive concept comparisons
- **[QUICK_START.md](docs/QUICK_START.md)** - Learn by doing (minimal theory)
- **[TDD_GUIDE.md](docs/TDD_GUIDE.md)** - Red-Green-Refactor workflow

## 🎓 Learning Approach

### TDD Red-Green-Refactor Cycle

```
1. 🔴 RED: Run tests → See failures
2. 💡 UNDERSTAND: Read test requirements
3. 🟢 GREEN: Write code to pass tests
4. 🔄 REFACTOR: Clean up and improve
5. ✅ VERIFY: All tests pass → Move forward
```

Every operation follows this cycle. Tests are pre-written to guide your implementation.

## 📁 Project Structure

```
rust-api-crud/
├── docs/                    # Learning guides
│   ├── LEARNING_PATH.md    # Start here!
│   ├── RUST_CONCEPTS.md    # Quick reference
│   └── ...
├── src/
│   ├── main.rs             # Application entry
│   ├── routes/             # API endpoints
│   ├── handlers/           # Request handlers
│   ├── models/             # Data models
│   └── db/                 # Database utilities
├── tests/
│   ├── calculator_tests.rs # Phase 0 tests
│   └── user_tests.rs       # Phase 2 tests
├── migrations/             # SQL migrations
├── docker-compose.yml      # PostgreSQL setup
└── Cargo.toml             # Dependencies
```

## 🧪 Running Tests

```bash
# Run all tests
cargo test

# Run specific phase tests
cargo test calculator  # Phase 0
cargo test user        # Phase 2

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*'
```

## 🐛 Troubleshooting

### Database connection errors
```bash
# Check if PostgreSQL is running
docker ps

# Reset database
docker-compose down -v
docker-compose up -d
sqlx migrate run
```

### Rust compilation errors
```bash
# Update toolchain
rustup update

# Clean build
cargo clean
cargo build
```

### Port already in use
```bash
# Change port in .env file
PORT=8080  # Try different port
```

## 🎯 Success Checkpoints

### Phase 0: Calculator ✅
```bash
cargo run
curl http://localhost:3000/calculate?a=5&b=3&op=add
# Should return: {"result":8}
```

### Phase 1: Database ✅
```bash
cargo test db_connection
# All tests pass
```

### Phase 2: CRUD ✅
```bash
# Create user
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'

# Get user
curl http://localhost:3000/users/1
```

## 📝 Your Learning Journey

Track your progress:

- [x] Phase 0: Calculator API working ✅
- [x] Phase 1: Database infrastructure setup ✅
- [x] Phase 2.1: CREATE user endpoint ✅
- [x] Phase 2.2: READ user endpoint ✅
- [x] Phase 2.3: LIST users with pagination ✅
- [x] Phase 2.4: UPDATE user endpoint ✅
- [x] Phase 2.5: DELETE user endpoint ✅
- [x] All tests passing (10/10) ✅
- [ ] Documentation complete

## 🤝 Learning Tips

1. **Compiler is your teacher** - Rust errors are incredibly helpful
2. **Tests guide you** - Read failing tests to understand requirements
3. **Iterate quickly** - Red → Green → Refactor
4. **Compare to TypeScript** - Use the concept guides when confused
5. **Experiment freely** - Tests provide a safety net

## 📚 Next Steps

Ready to start? Open **[docs/LEARNING_PATH.md](docs/LEARNING_PATH.md)** and begin your journey!

Have questions? Check **[docs/RUST_CONCEPTS.md](docs/RUST_CONCEPTS.md)** for Rust/TypeScript comparisons.

Want to dive in immediately? See **[docs/QUICK_START.md](docs/QUICK_START.md)**.

---

**Built with ❤️ for TypeScript engineers learning Rust**
