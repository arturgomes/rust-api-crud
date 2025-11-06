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
// - Migration runner
// - Database statistics
