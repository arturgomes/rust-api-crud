// Database module - Connection pool and utilities

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

// Create a connection pool to the PostgreSQL database
// A pool maintains multiple database connections that can be reused
// This is much more efficient than creating a new connection for each request
//
// TypeScript equivalent:
// const pool = new Pool({ connectionString: database_url, max: 5 });
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)                    // Maximum concurrent connections
        .acquire_timeout(Duration::from_secs(3)) // Timeout waiting for connection
        .connect(database_url)
        .await
}

// TODO (Phase 1): Add any additional database utility functions here
// Examples:
// - Health check function
pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
// - Migration runner
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
// - Database statistics
pub fn get_database_statistics(pool: &PgPool) -> (u32, u32, u32) {
    (
        pool.size(),
        pool.num_idle(),
        pool.options().get_max_connections()
    )
}
