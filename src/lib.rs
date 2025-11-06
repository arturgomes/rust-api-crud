use serde::{Deserialize, Serialize};
use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};

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
// Health check endpoint
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "Calculator API is working well"
    }))
}
pub fn create_app() -> Router {
    Router::new()
        .route("/calculate", get(calculate))
        .route("/health", get(health_check))
}