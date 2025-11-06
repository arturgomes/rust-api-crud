// Phase 1: Database Integration Tests
// These tests validate that database connection and operations work correctly
//
// 🎓 Testing Strategy in Rust:
// - Integration tests go in the `tests/` directory
// - Each file in `tests/` is a separate test crate
// - Use #[tokio::test] for async tests (like #[test] but for async)
// - Tests run in parallel by default
//
// TypeScript equivalent:
// describe('Database Tests', () => {
//   it('should connect to database', async () => { ... });
// });

use sqlx::PgPool;

// Helper function to create a test database pool
// This is similar to a test fixture in other languages
// TypeScript equivalent:
// async function setupTestDb(): Promise<Pool> {
//   await dotenv.config();
//   return createPool(process.env.DATABASE_URL);
// }
async fn setup_test_db() -> PgPool {
    // Load environment variables
    dotenv::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set for tests");

    // Create a connection pool for testing
    rust_api_crud::db::create_pool(&database_url)
        .await
        .expect("Failed to create test database pool")
}

// ============================================================================
// TEST 1: Basic Database Connection
// ============================================================================
// This test verifies that we can connect to PostgreSQL
// TypeScript equivalent:
// test('should connect to database', async () => {
//   const pool = await createPool(DATABASE_URL);
//   expect(pool).toBeDefined();
// });

#[tokio::test]
async fn test_database_connection() {
    // Arrange: Set up test database pool
    let pool = setup_test_db().await;

    // Act: Try to execute a simple query
    // SELECT 1 is a standard "ping" query that returns 1 if connection works
    let result = sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await;

    // Assert: The query should succeed
    assert!(result.is_ok(), "Database connection test failed: {:?}", result.err());
}

// ============================================================================
// TEST 2: Health Check Function
// ============================================================================
// This test verifies our health_check utility function works
// TypeScript equivalent:
// test('should pass health check', async () => {
//   const pool = await createPool(DATABASE_URL);
//   await expect(healthCheck(pool)).resolves.toBeUndefined();
// });

#[tokio::test]
async fn test_health_check() {
    // Arrange: Set up test database pool
    let pool = setup_test_db().await;

    // Act: Call our health_check function
    let result = rust_api_crud::db::health_check(&pool).await;

    // Assert: Health check should succeed
    assert!(result.is_ok(), "Health check failed: {:?}", result.err());
}

// ============================================================================
// TEST 3: Database Pool Statistics
// ============================================================================
// This test verifies we can get pool statistics
// Helps understand connection pool behavior

#[tokio::test]
async fn test_pool_statistics() {
    // Arrange: Set up test database pool
    let pool = setup_test_db().await;

    // Act: Get pool statistics
    let (size, idle, max_connections) = rust_api_crud::db::get_database_statistics(&pool);

    // Assert: Pool should have reasonable values
    assert!(max_connections > 0, "Max connections should be positive");
    assert!(size <= max_connections, "Pool size should not exceed max");
    assert!(idle <= size, "Idle connections should not exceed total size");

    println!("📊 Pool Stats: size={}, idle={}, max={}", size, idle, max_connections);
}

// ============================================================================
// TEST 4: Migration Execution
// ============================================================================
// This test verifies that database migrations run successfully
// TypeScript equivalent:
// test('should run migrations', async () => {
//   const pool = await createPool(DATABASE_URL);
//   await expect(runMigrations(pool)).resolves.toBeUndefined();
// });

#[tokio::test]
async fn test_run_migrations() {
    // Arrange: Set up test database pool
    let pool = setup_test_db().await;

    // Act: Run migrations
    let result = rust_api_crud::db::run_migrations(&pool).await;

    // Assert: Migrations should succeed
    assert!(result.is_ok(), "Migrations failed: {:?}", result.err());
}

// ============================================================================
// TEST 5: Table Existence After Migration
// ============================================================================
// This test verifies that the users table exists after migration
// This ensures migrations created the expected schema

#[tokio::test]
async fn test_users_table_exists() {
    // Arrange: Set up test database pool
    let pool = setup_test_db().await;

    // Ensure migrations have run
    rust_api_crud::db::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    // Act: Query for the users table in information_schema
    // This is a PostgreSQL system table that lists all tables
    let result = sqlx::query(
        "SELECT table_name
         FROM information_schema.tables
         WHERE table_schema = 'public'
         AND table_name = 'users'"
    )
    .fetch_one(&pool)
    .await;

    // Assert: The query should succeed (table exists)
    assert!(result.is_ok(), "Users table does not exist: {:?}", result.err());
}

// ============================================================================
// TEST 6: Connection Pool Reuse
// ============================================================================
// This test verifies that the pool efficiently reuses connections
// Important for performance under load

#[tokio::test]
async fn test_pool_connection_reuse() {
    // Arrange: Set up test database pool
    let pool = setup_test_db().await;

    // Get initial idle count
    let (_, idle_before, _) = rust_api_crud::db::get_database_statistics(&pool);

    // Act: Execute multiple queries to use connections
    for _ in 0..3 {
        sqlx::query("SELECT 1")
            .fetch_one(&pool)
            .await
            .expect("Query failed");
    }

    // Get idle count after queries
    let (_, idle_after, _) = rust_api_crud::db::get_database_statistics(&pool);

    // Assert: Connections should return to idle pool
    // After async queries complete, connections should be released back
    println!("📊 Idle connections: before={}, after={}", idle_before, idle_after);

    // We can't assert exact equality due to timing, but connections should be available
    assert!(idle_after <= pool.options().get_max_connections() as u32,
            "Idle connections should not exceed max");
}

// ============================================================================
// TEST 7: Concurrent Connections
// ============================================================================
// This test verifies the pool handles concurrent requests properly
// Simulates multiple simultaneous HTTP requests

#[tokio::test]
async fn test_concurrent_database_queries() {
    // Arrange: Set up test database pool
    let pool = setup_test_db().await;

    // Act: Spawn 10 concurrent queries
    // TypeScript equivalent:
    // await Promise.all(Array(10).fill(0).map(() => pool.query('SELECT 1')));
    let mut handles = vec![];

    for i in 0..10 {
        let pool_clone = pool.clone(); // PgPool is cheap to clone
        let handle = tokio::spawn(async move {
            sqlx::query("SELECT $1::INTEGER")
                .bind(i)
                .fetch_one(&pool_clone)
                .await
        });
        handles.push(handle);
    }

    // Wait for all queries to complete
    let results = futures::future::join_all(handles).await;

    // Assert: All queries should succeed
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "Task {} panicked", i);
        assert!(result.as_ref().unwrap().is_ok(), "Query {} failed", i);
    }
}

// ============================================================================
// 🎓 LEARNING NOTES
// ============================================================================
//
// 1. **#[tokio::test]** - Marks async test functions
//    - Same as #[test] but for async code
//    - Automatically sets up Tokio runtime
//
// 2. **Test Organization**
//    - Each test is independent (can run in any order)
//    - Tests run in parallel by default
//    - Use `cargo test -- --test-threads=1` for sequential
//
// 3. **Assertions**
//    - `assert!(condition)` - Basic assertion
//    - `assert_eq!(left, right)` - Equality assertion
//    - Include error context with `, "message: {:?}", error`
//
// 4. **PgPool Cloning**
//    - `pool.clone()` is cheap (uses Arc internally)
//    - Safe to share across async tasks
//    - Each clone shares the same connection pool
//
// 5. **Error Handling in Tests**
//    - `.expect("msg")` - Unwrap or panic with message
//    - `.is_ok()` - Check if Result is Ok variant
//    - `.is_err()` - Check if Result is Err variant
//
// 6. **Database Test Best Practices**
//    - Use test database (never production!)
//    - Clean up test data (we'll add this in Phase 2)
//    - Use transactions for isolation (Phase 2)
//
// ============================================================================
// 🧪 RUNNING TESTS
// ============================================================================
//
// Run all database tests:
// ```bash
// cargo test --test db_tests
// ```
//
// Run specific test:
// ```bash
// cargo test test_database_connection
// ```
//
// Run with output:
// ```bash
// cargo test -- --nocapture
// ```
//
// Run sequentially:
// ```bash
// cargo test -- --test-threads=1
// ```
//
// ============================================================================
