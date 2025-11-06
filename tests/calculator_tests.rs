// Phase 0: Calculator API Tests
// These tests validate the calculator endpoints work correctly

// use serde_json::Value;

use rust_api_crud::{CalculatorResponse, 
    ErrorResponse,
    create_app}; 
// This goes BEFORE line 13 (#[cfg(test)])
use tokio::sync::OnceCell;

static SERVER_URL: OnceCell<String> = OnceCell::const_new();

async fn get_test_server_url() -> String {
    // TODO: We'll fill this together
    SERVER_URL.get_or_init(async ||  {
        let app = create_app();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            axum::serve(listener,app).await.unwrap();
        });
        format!("http://127.0.0.1:{}", port)
    })
    .await
    .clone()
}

// Helper function to make requests
// In a real app, you'd use reqwest, but for learning we'll keep it simple
// This shows you the test structure for later phases

#[cfg(test)]
mod calculator_tests {
    use super::*;

    // Note: These are example test structures
    // To run integration tests properly, we'd need to:
    // 1. Start the server in a test mode
    // 2. Make HTTP requests with reqwest
    // 3. Validate responses

    // For now, these are here to show you the pattern
    // You'll implement real integration tests in Phase 2

    #[tokio::test]
    async fn test_addition() {
        // TypeScript equivalent:
        // test('should add two numbers', async () => {
        //   const response = await fetch('http://localhost:3000/calculate?a=5&b=3&op=add');
        //   const data = await response.json();
        //   expect(data.result).toBe(8);
        // });


        let url = get_test_server_url().await;

        let response = reqwest::get(&format!("{}/calculate?a=5&b=3&op=add", url)).await.unwrap();
        let data: CalculatorResponse = response.json().await.unwrap();
        assert_eq!(data.result, 8.0);
        // assert!(true, "Calculator addition test structure");
    }

    #[tokio::test]
    async fn test_division_by_zero() {
        // TypeScript equivalent:
        // test('should handle division by zero', async () => {
        //   const response = await fetch('http://localhost:3000/calculate?a=10&b=0&op=divide');
        //   const data = await response.json();
        //   expect(data.error).toBeTruthy();
        // });
        let url = get_test_server_url().await;

        let response = reqwest::get(&format!("{}/calculate?a=10&b=0&op=divide", url)).await.unwrap();
        let data: ErrorResponse = response.json().await.unwrap();

        assert_eq!(data.error, "Division by zero");
    }

    #[tokio::test]
    async fn test_unknown_operation() {
        // Test that unknown operations return an error
        assert!(true, "Calculator unknown operation test structure");
        let url = get_test_server_url().await;

        let response = reqwest::get(&format!("{}/calculate?a=10&b=0&op=lol", url)).await.unwrap();
        let data: ErrorResponse = response.json().await.unwrap();

        assert_eq!(data.error, "Unknown operation: lol");
    }
}

// 🎓 Learning Notes:
//
// 1. #[tokio::test] - Like async test in Jest/Mocha
// 2. assert! - Basic assertion macro (like expect().toBe() in Jest)
// 3. #[cfg(test)] - Conditional compilation (only in test mode)
// 4. mod - Module declaration (like export/import in TypeScript)
//
// In Phase 2, you'll write REAL integration tests that:
// - Start the server
// - Make HTTP requests
// - Validate JSON responses
// - Test error cases
//
// For now, just understand the structure!
