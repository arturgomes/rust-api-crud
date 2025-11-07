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

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
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
        match e.as_database_error() {
            Some(db_err) if db_err.is_unique_violation() => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}

// ============================================================================
// GET USER - GET /users/:id
// ============================================================================

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM users 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?  
    .ok_or(StatusCode::NOT_FOUND)?; 
    
    Ok(Json(user))
}

// ============================================================================
// LIST USERS - GET /users?page=1&per_page=10
// ============================================================================

pub async fn list_users(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<UserListResponse>, StatusCode> {
    let offset = (pagination.page - 1) * pagination.per_page;
    let total_result = sqlx::query!(
        "SELECT COUNT(*) as count FROM users"
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = total_result.count.unwrap_or(0); 

    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, created_at, updated_at FROM users
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        pagination.per_page,
        offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_pages = (total + pagination.per_page - 1) / pagination.per_page;

    Ok(Json(UserListResponse {
        users, 
        total, 
        page: pagination.page, 
        per_page: pagination.per_page, 
        total_pages}))
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
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET
            name = COALESCE($1, name),
            email = COALESCE($2, email),
            updated_at = NOW()
        WHERE id = $3
        RETURNING id, name, email, created_at, updated_at",
        payload.name,
        payload.email,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

// ============================================================================
// DELETE USER - DELETE /users/:id
// ============================================================================

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
   
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
