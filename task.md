<task>
You are an expert Rust educator helping a senior TypeScript engineer learn Rust through Test-Driven Development (TDD). The developer is experienced with design patterns, DRY principles, and clean code.

<objective>
Create a comprehensive TDD-based learning tutorial for building a User CRUD API in Rust. The tutorial should:
- Start with pre-written failing tests that drive development
- Include a simple "Hello World" example first (non-user context) to demonstrate patterns
- Progress incrementally from simple to complex
- Result in a fully working, containerized application
- Teach Rust concepts through the red-green-refactor cycle
</objective>

<tutorial_structure>
PHASE 0: Simple Working Example (Non-User Context)
- Create a minimal "Calculator API" with 2 endpoints as a warm-up:
  * POST /calculate/add (adds two numbers)
  * GET /health (health check)
- Pre-written tests for these endpoints
- Demonstrates: basic Axum setup, routing, JSON handling, testing patterns
- Goal: Get something working quickly to understand the workflow

PHASE 1: Project Setup & Infrastructure
- Docker Compose setup (PostgreSQL + adminer/pgAdmin)
- Database connection and migrations
- Test database setup
- Pre-written infrastructure tests (DB connectivity, migrations)

PHASE 2: TDD User CRUD (one operation at a time)
Each operation follows: Test → Implement → Pass → Refactor

2.1: CREATE User
- Pre-written integration tests for POST /users
- Pre-written unit tests for validation logic
- Implement to pass tests

2.2: READ User by ID
- Pre-written tests for GET /users/:id
- Handle 404 cases
- Implement to pass tests

2.3: LIST Users (with pagination)
- Pre-written tests for GET /users
- Test pagination edge cases
- Implement to pass tests

2.4: UPDATE User
- Pre-written tests for PUT /users/:id
- Test partial updates
- Implement to pass tests

2.5: DELETE User
- Pre-written tests for DELETE /users/:id
- Test cascading effects
- Implement to pass tests
</tutorial_structure>

<requirements>
TECHNICAL STACK:
- Axum web framework
- SQLx with PostgreSQL (compile-time checked queries)
- Docker & Docker Compose
- Tokio async runtime

PROJECT STRUCTURE:
```
rust-crud-tutorial/
├── docker-compose.yml
├── Dockerfile
├── Cargo.toml
├── .env.example
├── README.md
├── LEARNING_PATH.md (step-by-step guide)
├── migrations/
│   └── 001_create_users_table.sql
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── examples/          # Phase 0: Calculator example
│   │   ├── calculator_api.rs
│   │   └── calculator_tests.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── user_handlers.rs
│   │   └── health.rs
│   ├── services/
│   │   ├── mod.rs
│   │   └── user_service.rs
│   ├── repositories/
│   │   ├── mod.rs
│   │   └── user_repository.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── errors/
│   │   ├── mod.rs
│   │   └── app_error.rs
│   └── tests/              # Pre-written tests
│       ├── integration_tests.rs
│       ├── user_create_tests.rs
│       ├── user_read_tests.rs
│       ├── user_update_tests.rs
│       └── user_delete_tests.rs
└── tests/                   # Integration tests directory
    └── api_tests.rs
```

DOCKER SETUP:
- PostgreSQL 16 container
- Test database for running tests
- Automatic migration runner
- Volume persistence
- Health checks
- Adminer for database inspection

ALL TESTS PRE-WRITTEN:
- Unit tests for each service method
- Integration tests for each endpoint
- Edge cases and error scenarios
- Database transaction tests
- All tests should FAIL initially
- Success = all tests passing (green)
</requirements>

<user_model>
User {
  id: UUID (auto-generated)
  email: String (unique, validated)
  name: String
  age: Option<i32> (nullable)
  created_at: DateTime (auto-generated)
  updated_at: DateTime (auto-updated)
}
</user_model>

<teaching_approach>
FOR EACH PHASE:
1. Explain what we're building and why
2. Show the failing tests first
3. Explain Rust concepts needed to make tests pass
4. Compare to TypeScript equivalents
5. Implement minimal code to pass tests
6. Refactor and explain improvements
7. Verify all tests pass

KEY RUST CONCEPTS TO HIGHLIGHT:
- Ownership & borrowing (vs JS garbage collection)
- Result<T, E> for error handling (vs try/catch)
- Option<T> for nullable values (vs null/undefined)
- Traits (vs interfaces)
- Pattern matching (vs switch/if-else)
- async/await differences
- Lifetime annotations when they appear
- Macro usage (#[derive], #[tokio::test], etc.)
</teaching_approach>

<deliverables>
1. LEARNING_PATH.md - Step-by-step guide with checkpoints:
   - "Start here: Run Phase 0 Calculator example"
   - "Checkpoint 1: All infrastructure tests pass"
   - "Checkpoint 2: CREATE tests pass"
   - etc.

2. README.md with:
   - Prerequisites (Rust, Docker installed)
   - Quick start commands
   - How to run tests: `cargo test`
   - How to run app: `docker-compose up`
   - How to verify: example curl commands

3. Complete working application:
   - docker-compose.yml (ready to run)
   - All migrations
   - Pre-written comprehensive tests
   - Implementation code with educational comments
   - .env.example with all config

4. RUST_CONCEPTS.md:
   - Glossary of Rust terms encountered
   - TypeScript → Rust comparison table
   - Common pitfalls and solutions
   - Resource links for deep dives

5. Example requests file (requests.http or curl scripts)

6. Makefile or justfile with useful commands:
   - `make setup` - Initial setup
   - `make test` - Run tests
   - `make dev` - Start development environment
   - `make clean` - Clean up
</deliverables>

<verification_criteria>
WORKING MEANS:
✅ `docker-compose up` starts all services
✅ Database migrations run automatically
✅ API responds to requests
✅ `cargo test` shows all tests passing
✅ Calculator example works (Phase 0)
✅ All CRUD operations work via curl/Postman
✅ Database persists data correctly
✅ Error handling returns proper status codes
✅ Validation works (invalid email fails, etc.)
</verification_criteria>

<style>
- Write idiomatic, clean Rust code
- Comprehensive comments explaining "why" not just "what"
- Tests should be self-documenting
- Use proper error handling (no unwrap() in production)
- Follow Rust naming conventions
- Make it feel like a real production codebase
- Progressive complexity: simple → advanced
</style>

<first_steps>
When generating this tutorial:
1. Create LEARNING_PATH.md first - the roadmap
2. Set up Docker infrastructure (docker-compose.yml)
3. Build Phase 0 (Calculator example) - fully working
4. Generate ALL tests for Phase 1 & 2 (they should fail)
5. Create skeleton implementations (enough to compile but not pass tests)
6. Add detailed comments guiding the learner through implementation
7. Show the final working implementation in comments or separate files
</first_steps>

<success_definition>
The tutorial is successful when:
- A TypeScript developer can clone, run `docker-compose up`, and see a working API
- They can run `cargo test` and see tests fail
- They can follow LEARNING_PATH.md to implement features
- Each implementation makes specific tests pass
- At the end, all tests are green and the API is production-ready
- They understand core Rust concepts through practical application
</success_definition>
</task>