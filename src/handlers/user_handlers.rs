// User handlers - HTTP request handlers for user CRUD operations

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateUserRequest, Pagination, UpdateUserRequest, User, UserListResponse};

// ============================================================================
// CREATE USER - POST /users
// ============================================================================
// TODO (Phase 2.1): Implement this handler
//
// Requirements:
// - Accept CreateUserRequest JSON in request body
// - Insert user into database using SQLx
// - Return created user with 201 Created status
//
// SQL query hint:
// INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *
//
// Error handling:
// - Return 500 Internal Server Error for database errors
// - Consider handling duplicate email (409 Conflict)
//
// TypeScript equivalent:
// app.post('/users', async (req, res) => {
//   const { name, email } = req.body;
//   const user = await db.query('INSERT INTO users...');
//   res.status(201).json(user);
// });
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    // TODO: Implement user creation
    // 1. Use sqlx::query_as! macro to insert and return the user
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email) 
        VALUES ($1, $2) 
         RETURNING id, name, email, created_at, updated_at
        "#,
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return StatusCode::CONFLICT;
            }
        }
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 2. Handle errors appropriately
    // 3. Return (StatusCode::CREATED, Json(user))
    Ok((StatusCode::CREATED, Json(user)))
}

// ============================================================================
// GET USER - GET /users/:id
// ============================================================================
// TODO (Phase 2.2): Implement this handler
//
// Requirements:
// - Extract user ID from path parameter
// - Query database for user by ID
// - Return user if found, 404 if not found
//
// SQL query hint:
// SELECT * FROM users WHERE id = $1
//
// Error handling:
// - Return 404 Not Found if user doesn't exist
// - Return 500 Internal Server Error for database errors
//
// Tip: Use .fetch_optional() then .ok_or(StatusCode::NOT_FOUND)?
//
// TypeScript equivalent:
// app.get('/users/:id', async (req, res) => {
//   const user = await db.query('SELECT * FROM users WHERE id = $1', [req.params.id]);
//   if (!user) return res.status(404).json({ error: 'Not found' });
//   res.json(user);
// });
pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    // TODO: Implement user retrieval
    // 1. Query database for user with the given ID
    // 2. Return 404 if not found
    // 3. Return user if found

    Err(StatusCode::NOT_IMPLEMENTED)
}

// ============================================================================
// LIST USERS - GET /users?page=1&per_page=10
// ============================================================================
// TODO (Phase 2.3): Implement this handler
//
// Requirements:
// - Extract pagination parameters from query string
// - Query total count of users
// - Query paginated list of users
// - Return UserListResponse with users, total, page info
//
// SQL queries needed:
// 1. SELECT COUNT(*) FROM users
// 2. SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2
//
// Pagination math:
// - offset = (page - 1) * per_page
// - total_pages = (total + per_page - 1) / per_page
//
// TypeScript equivalent:
// app.get('/users', async (req, res) => {
//   const { page = 1, per_page = 10 } = req.query;
//   const total = await db.query('SELECT COUNT(*) FROM users');
//   const users = await db.query('SELECT * FROM users LIMIT $1 OFFSET $2', [per_page, offset]);
//   res.json({ users, total, page, per_page, total_pages });
// });
pub async fn list_users(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<UserListResponse>, StatusCode> {
    // TODO: Implement user listing with pagination
    // 1. Calculate offset: (page - 1) * per_page
    // 2. Query total count
    // 3. Query paginated users
    // 4. Calculate total_pages
    // 5. Return UserListResponse

    Err(StatusCode::NOT_IMPLEMENTED)
}

// ============================================================================
// UPDATE USER - PUT /users/:id
// ============================================================================
// TODO (Phase 2.4): Implement this handler
//
// Requirements:
// - Extract user ID from path
// - Accept UpdateUserRequest with optional fields
// - Update only provided fields (partial update)
// - Return updated user or 404 if not found
//
// SQL query hint:
// UPDATE users SET
//   name = COALESCE($1, name),
//   email = COALESCE($2, email),
//   updated_at = NOW()
// WHERE id = $3
// RETURNING *
//
// COALESCE($1, name) means: use $1 if not null, otherwise keep current value
//
// TypeScript equivalent:
// app.put('/users/:id', async (req, res) => {
//   const { name, email } = req.body;
//   const user = await db.query('UPDATE users SET name = COALESCE($1, name)... WHERE id = $2', [name, email, req.params.id]);
//   if (!user) return res.status(404).json({ error: 'Not found' });
//   res.json(user);
// });
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    // TODO: Implement user update
    // 1. Use UPDATE query with COALESCE for optional fields
    // 2. Return 404 if user doesn't exist
    // 3. Return updated user

    Err(StatusCode::NOT_IMPLEMENTED)
}

// ============================================================================
// DELETE USER - DELETE /users/:id
// ============================================================================
// TODO (Phase 2.5): Implement this handler
//
// Requirements:
// - Extract user ID from path
// - Delete user from database
// - Return 204 No Content if successful
// - Return 404 if user doesn't exist
//
// SQL query:
// DELETE FROM users WHERE id = $1
//
// Tip: Check rows_affected() to determine if user existed
//
// TypeScript equivalent:
// app.delete('/users/:id', async (req, res) => {
//   const result = await db.query('DELETE FROM users WHERE id = $1', [req.params.id]);
//   if (result.rowCount === 0) return res.status(404).send();
//   res.status(204).send();
// });
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    // TODO: Implement user deletion
    // 1. Execute DELETE query
    // 2. Check rows_affected()
    // 3. Return 204 No Content if deleted, 404 if not found
    let result = sqlx::query!(
        r#"
        DELETE FROM users WHERE id = $1
        "#,
        id,
    )
    .execute(&pool)
    .await;
    
    match result {
        Ok(query_result) => {
            if query_result.rows_affected() == 0 {
                return StatusCode::NOT_FOUND;
            } else {
                return StatusCode::NO_CONTENT;
            }
        }
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
   
}

// ============================================================================
// BONUS: Add input validation, better error handling, logging
// ============================================================================
// Ideas for improvement:
// - Validate email format before inserting
// - Add proper logging with tracing
// - Create custom error types
// - Add request/response logging middleware
// - Add rate limiting
// - Add authentication
