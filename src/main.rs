// Phase 0: Calculator API - A warm-up to learn Rust + Axum basics
// This is a COMPLETE working example. Read it, run it, understand it.
// Then you'll build the CRUD API using similar patterns.

use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// TypeScript equivalent:
// interface CalculatorRequest {
//   a: number;
//   b: number;
//   op: string;
// }
#[derive(Deserialize)]
struct CalculatorRequest {
    a: f64,
    b: f64,
    op: String,
}

// TypeScript equivalent:
// interface CalculatorResponse {
//   result: number;
// }
#[derive(Serialize)]
struct CalculatorResponse {
    result: f64,
}

// TypeScript equivalent:
// interface ErrorResponse {
//   error: string;
// }
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Handler function
// TypeScript equivalent:
// async function calculate(req: CalculatorRequest): Promise<CalculatorResponse | ErrorResponse>
async fn calculate(
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
        _ => {
            return Json(serde_json::json!(ErrorResponse {
                error: format!("Unknown operation: {}", params.op)
            }));
        }
    };

    Json(serde_json::json!(CalculatorResponse { result }))
}

// Main function - async like TypeScript async function
// TypeScript equivalent:
// async function main() { ... }
#[tokio::main]
async fn main() {
    // Initialize tracing (like console.log but better)
    tracing_subscriber::fmt::init();

    // Build the router - like Express app routing
    // TypeScript equivalent:
    // const app = express();
    // app.get('/calculate', calculate);
    let app = Router::new()
        .route("/calculate", get(calculate));

    // Set the address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!("🚀 Calculator API listening on http://{}", addr);
    tracing::info!("📝 Try: http://localhost:3000/calculate?a=5&b=3&op=add");

    // Start the server
    // TypeScript equivalent:
    // app.listen(3000)
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

// 🎓 Learning Notes:
//
// 1. #[derive(...)] - Automatic trait implementation (like decorators in TS)
// 2. struct - Like TypeScript interfaces but for data structures
// 3. match - Pattern matching (super-powered switch statement)
// 4. async/await - Same concept as TypeScript, different runtime (Tokio vs Node)
// 5. Result<T, E> - Not used here, but you'll see it in Phase 1
// 6. Option<T> - Not used here, but replaces null/undefined in Rust
//
// 🧪 Test it:
// cargo run
// curl "http://localhost:3000/calculate?a=10&b=5&op=add"
// curl "http://localhost:3000/calculate?a=10&b=5&op=divide"
// curl "http://localhost:3000/calculate?a=10&b=0&op=divide"
