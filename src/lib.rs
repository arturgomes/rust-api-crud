// Module declarations
pub mod db;
pub mod handlers;
pub mod models;

// Imports
use sqlx::PgPool;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use tower_http::trace::TraceLayer;
use serde::{Deserialize, Serialize};
use axum::{
    extract::Query,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use tracing::info_span;
use handlers::user_handlers;

// TypeScript equivalent:
// interface CalculatorRequest {
//   a: number;
//   b: number;
//   op: string;
// }
#[derive(Deserialize)]
pub struct CalculatorRequest {
    a: f64,
    b: f64,
    op: String,
}

// TypeScript equivalent:
// interface CalculatorResponse {
//   result: number;
// }
#[derive(Serialize,Deserialize)]
pub struct CalculatorResponse {
    pub result: f64,
    pub operation: String,
}

// TypeScript equivalent:
// interface ErrorResponse {
//   error: string;
// }
#[derive(Serialize,Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Handler function
// TypeScript equivalent:
// async function calculate(req: CalculatorRequest): Promise<CalculatorResponse | ErrorResponse>
pub async fn calculate(
    Query(params): Query<CalculatorRequest>,
) -> Json<serde_json::Value> {
    // Pattern matching - like switch on steroids
    // Much more powerful than TypeScript's switch statement
    let result = match params.op.as_str() {
        "add" => params.a + params.b,
        "subtract" => params.a - params.b,
        "multiply" => params.a * params.b,
        "divide" => {
            if params.b == 0.0 {
                return Json(serde_json::json!(ErrorResponse {
                    error: "Division by zero".to_string()
                }));
            }
            params.a / params.b
        }
        "modulo" => params.a % params.b,
        "power" => {
            if params.b < 0.0 {
                return Json(serde_json::json!(ErrorResponse {
                    error: "Power operation requires a positive exponent".to_string()
                }));
            }
            params.a.powf(params.b)
        },
        "double" => params.a * 2.0,
        _ => {
            return Json(serde_json::json!(ErrorResponse {
                error: format!("Unknown operation: {}", params.op)
            }));
        }
    };

    Json(serde_json::json!(CalculatorResponse { result, operation: params.op }))
}
// Basic health check endpoint (no database required)
// TypeScript equivalent:
// async function health() { return { status: "ok" }; }
pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

// Database health check endpoint
// TypeScript equivalent:
// async function dbHealth(pool: Pool) {
//   try {
//     await pool.query('SELECT 1');
//     return { status: "ok" };
//   } catch(err) {
//     return { status: 503, error: "Database unavailable" };
//   }
// }
pub async fn db_health(State(pool): State<PgPool>) -> Result<Json<serde_json::Value>, StatusCode> {
    match crate::db::health_check(&pool).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "ok",
            "message": "Database connection is healthy"
        }))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/health/db", get(db_health))
        .route("/calculate", get(calculate))
        .route("/users", post(user_handlers::create_user))
        .route("/users/:id", get(user_handlers::get_user))
        .route("/users", get(user_handlers::list_users))
        .route("/users/:id", put(user_handlers::update_user))
        .route("/users/:id", delete(user_handlers::delete_user))
        .with_state(pool)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Create tracing span for each HTTP request
                    // TypeScript equivalent: console.log(`${method} ${uri}`)
                    info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
        )
}