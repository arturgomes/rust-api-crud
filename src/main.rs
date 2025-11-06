// Phase 0: Calculator API - A warm-up to learn Rust + Axum basics
// Phase 1: Database Integration - Connect to PostgreSQL with SQLx

use std::net::SocketAddr;
use rust_api_crud::create_app;

// Main function - async like TypeScript async function
// TypeScript equivalent:
// async function main() { ... }
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Initialize tracing (like console.log but better)
    tracing_subscriber::fmt::init();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    tracing::info!("📊 Connecting to database...");

    let pool = rust_api_crud::db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("✅ Database pool created");

    // Build the router - like Express app routing
    // TypeScript equivalent:
    // const app = express();
    // app.get('/calculate', calculate);
    let app = create_app(pool);

    // Set the address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!("🚀 User CRUD API listening on http://{}", addr);
    // tracing::info!("📝 Try: http://localhost:3000/calculate?a=5&b=3&op=add");

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
