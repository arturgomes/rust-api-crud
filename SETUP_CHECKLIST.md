# ✅ Setup Checklist - Verify Your Learning Environment

Use this checklist to ensure everything is properly set up before starting.

---

## 📋 Prerequisites Check

### 1. Install Rust
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

**Expected Output**:
```
rustc 1.70.0 (or higher)
cargo 1.70.0 (or higher)
```

- [ ] Rust installed
- [ ] Version 1.70 or higher

---

### 2. Install Docker Desktop
Download from: https://www.docker.com/products/docker-desktop

```bash
# Verify installation
docker --version
docker-compose --version
```

**Expected Output**:
```
Docker version 20.10.x or higher
docker-compose version 1.29.x or higher
```

- [ ] Docker installed
- [ ] Docker Desktop running

---

### 3. Optional Tools

```bash
# Install SQLx CLI (for migrations)
cargo install sqlx-cli --no-default-features --features postgres

# Install cargo-watch (for auto-reload during development)
cargo install cargo-watch

# Install jq (for parsing JSON in curl commands)
# macOS:
brew install jq
# Linux:
sudo apt install jq
```

- [ ] sqlx-cli installed (required for Phase 1)
- [ ] cargo-watch installed (optional, nice to have)
- [ ] jq installed (optional, for testing)

---

## 🚀 Project Setup

### 1. Navigate to Project
```bash
cd rust-api-crud
```

- [ ] In project directory

### 2. Build Project
```bash
cargo build
```

**Expected**: Downloads dependencies and compiles successfully (may take 5-10 minutes first time)

**If errors**: Check that Cargo.toml exists and is properly formatted

- [ ] Project builds successfully

### 3. Start Database
```bash
docker-compose up -d
```

**Expected**:
```
Creating rust_crud_db ... done
```

**Verify it's running**:
```bash
docker ps
```

Should see a container named `rust_crud_db` running

- [ ] PostgreSQL container running

### 4. Copy Environment File
```bash
cp .env.example .env
```

**Verify**:
```bash
cat .env
```

Should see DATABASE_URL, PORT, etc.

- [ ] .env file exists with correct values

---

## 🧪 Phase 0: Calculator Verification

### 1. Run the Calculator
```bash
cargo run
```

**Expected Output**:
```
🚀 Calculator API listening on http://0.0.0.0:3000
📝 Try: http://localhost:3000/calculate?a=5&b=3&op=add
```

- [ ] Server starts without errors

### 2. Test Calculator (in new terminal)
```bash
# Test addition
curl "http://localhost:3000/calculate?a=10&b=5&op=add"
```

**Expected**: `{"result":15.0}`

```bash
# Test division
curl "http://localhost:3000/calculate?a=10&b=2&op=divide"
```

**Expected**: `{"result":5.0}`

```bash
# Test error case
curl "http://localhost:3000/calculate?a=10&b=0&op=divide"
```

**Expected**: `{"error":"Division by zero"}`

- [ ] Addition works
- [ ] Division works
- [ ] Error handling works

### 3. Stop Server
Press `Ctrl+C` in the terminal running the server

- [ ] Server stops cleanly

---

## 🗄️ Phase 1: Database Verification

### 1. Run Migrations
```bash
sqlx migrate run
```

**Expected**:
```
Applied 001/001_create_users_table
```

- [ ] Migrations applied successfully

### 2. Verify Database Connection
```bash
# Connect to PostgreSQL
docker exec -it rust_crud_db psql -U rustuser -d rustcrud

# Inside psql:
\dt          # List tables (should see 'users' table)
\d users     # Describe users table
\q           # Quit
```

**Expected**: See users table with id, name, email, created_at, updated_at columns

- [ ] Can connect to database
- [ ] Users table exists
- [ ] Table has correct schema

---

## 📚 Documentation Check

Verify all documentation files exist:

```bash
ls -la docs/
```

**Expected files**:
- [ ] LEARNING_PATH.md
- [ ] RUST_CONCEPTS.md
- [ ] TYPESCRIPT_TO_RUST.md
- [ ] QUICK_START.md
- [ ] TDD_GUIDE.md
- [ ] PROJECT_SUMMARY.md

```bash
ls -la *.md
```

- [ ] README.md exists
- [ ] task.md exists (original requirements)
- [ ] SETUP_CHECKLIST.md exists (this file)

---

## 🧪 Test Infrastructure Check

```bash
# Check test files exist
ls -la tests/
```

**Expected files**:
- [ ] calculator_tests.rs
- [ ] user_tests.rs

```bash
# Run tests (will show structure tests)
cargo test
```

**Expected**: Tests compile (some may be placeholders)

- [ ] Tests compile successfully

---

## 📁 Project Structure Verification

```bash
tree -L 2 -I target
```

**Expected structure**:
```
.
├── Cargo.toml
├── README.md
├── docker-compose.yml
├── docs/
│   ├── LEARNING_PATH.md
│   ├── RUST_CONCEPTS.md
│   ├── TYPESCRIPT_TO_RUST.md
│   ├── QUICK_START.md
│   ├── TDD_GUIDE.md
│   └── PROJECT_SUMMARY.md
├── migrations/
│   └── 001_create_users_table.sql
├── src/
│   ├── main.rs
│   ├── db/
│   ├── models/
│   └── handlers/
└── tests/
    ├── calculator_tests.rs
    └── user_tests.rs
```

- [ ] Project structure matches

---

## 🎯 Ready to Start!

If all checkboxes are checked, you're ready to begin!

### Choose Your Learning Path:

**Option 1: Structured Learning (Recommended)**
→ Open [docs/LEARNING_PATH.md](docs/LEARNING_PATH.md) and follow Phase 0

**Option 2: Conceptual First**
→ Read [docs/TYPESCRIPT_TO_RUST.md](docs/TYPESCRIPT_TO_RUST.md), then LEARNING_PATH

**Option 3: Learn by Doing**
→ Jump into [docs/QUICK_START.md](docs/QUICK_START.md)

---

## 🆘 Troubleshooting

### Rust not found after installation
```bash
# Add to PATH (restart terminal after)
source $HOME/.cargo/env
```

### Docker permission denied
```bash
# Linux: Add user to docker group
sudo usermod -aG docker $USER
# Then log out and back in
```

### Port 3000 already in use
Edit `.env`:
```
PORT=8080
```

### Database connection fails
```bash
# Reset database
docker-compose down -v
docker-compose up -d
sqlx migrate run
```

### Build errors
```bash
# Clean and rebuild
cargo clean
cargo build
```

---

## 📞 Getting Help

**Rust compiler errors**: Read them carefully! They're incredibly helpful.

**Documentation**: All guides are in the `docs/` folder

**Stuck?**: Check [docs/RUST_CONCEPTS.md](docs/RUST_CONCEPTS.md) for concept explanations

---

**Everything working?** Time to start learning! 🚀

Head to [docs/LEARNING_PATH.md](docs/LEARNING_PATH.md) to begin your journey.
