// User model - Database representation and request/response types

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Main User struct - matches database table
// FromRow: Allows SQLx to convert database rows to this struct
// Serialize: Allows converting to JSON for responses
// Deserialize: Allows parsing from JSON (though we usually use request types)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request type for creating a user
// Only includes fields the client should provide
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

// Request type for updating a user
// All fields optional - allows partial updates
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

// Response type for listing users with pagination
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<User>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

// Pagination query parameters
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

// Default values for pagination
fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    10
}
