// Integration tests for user CRUD operations
//
// These tests will guide your implementation through TDD.
// Run each test, see it fail, implement the handler, see it pass!
//
// PREREQUISITES:
// 1. Docker PostgreSQL running: docker-compose up -d
// 2. Migrations applied: sqlx migrate run
// 3. Server running: cargo run (in another terminal)
//
// To run: cargo test
// To run specific test: cargo test test_create_user
// To see output: cargo test -- --nocapture

use serde_json::json;
use uuid::Uuid;
use rust_api_crud::models::{User, UserListResponse, ErrorResponse};

const BASE_URL: &str = "http://localhost:3000";

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

#[allow(dead_code)]
async fn cleanup_all_users() {
    let client = client();

    if let Ok(response) = client.get(&format!("{}/users", BASE_URL)).send().await {
        if let Ok(result) = response.json::<UserListResponse>().await {
            for user in &result.users {
                let user_id = user.id.to_string();
                let _ = client
                    .delete(&format!("{}/users/{}", BASE_URL, user_id))
                    .send()
                    .await;
            }
        }
    }
}

async fn cleanup_user_by_email(email: &str) {
    let client = client();

    let Ok(response) = client.get(&format!("{}/users", BASE_URL)).send().await else {
        return;
    };

    if !response.status().is_success() {
        return;
    }

    let Ok(result) = response.json::<UserListResponse>().await else {
        return;
    };

    for user in &result.users {
        if user.email == email {
            let user_id = user.id.to_string();
            let _ = client
                .delete(&format!("{}/users/{}", BASE_URL, user_id))
                .send()
                .await;
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

// ============================================================================
// Phase 2.1: CREATE USER Tests
// ============================================================================

#[tokio::test]
async fn test_create_user_success() {
    cleanup_user_by_email("alice@example.com").await;

    let client = client();

    let response = client
        .post(&format!("{}/users", BASE_URL))
        .json(&json!({
            "name": "Alice",
            "email": "alice@example.com"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 201);

    let user: User = response
        .json()
        .await
        .expect("Failed to parse response");

    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");

    let user_id = user.id.to_string();
    let _ = client
        .delete(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await;
}


#[tokio::test]
async fn test_create_duplicated_user() {
    cleanup_user_by_email("alice@example.com").await;

    let client = client();

    let response = client
        .post(&format!("{}/users", BASE_URL))
        .json(&json!({
            "name": "Alice",
            "email": "alice@example.com"
        }))
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), 201);
    let first_user: User = response.json().await.expect("Failed to parse first user");

    let response1 = client
        .post(&format!("{}/users", BASE_URL))
        .json(&json!({
            "name": "Alice",
            "email": "alice@example.com"
        }))
        .send()
        .await
        .expect("Failed to send request");
    tracing::info!("error status {}", response1.status());

    assert_eq!(response1.status(), 409);

    let user_id = first_user.id.to_string();
    let _ = client
        .delete(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await;
}

// TODO: Add test for invalid email format (should return 400)
// TODO: Add test for missing fields (should return 400)

// ============================================================================
// Phase 2.2: GET USER Tests
// ============================================================================

#[tokio::test]
async fn test_get_user_success() {
    // Cleanup any existing test user with this email
    cleanup_user_by_email("bob@example.com").await;

    let client = client();

    // First create a user
    let create_response = client
        .post(&format!("{}/users", BASE_URL))
        .json(&json!({
            "name": "Bob",
            "email": "bob@example.com"
        }))
        .send()
        .await
        .unwrap();

    let created_user: User = create_response.json().await.unwrap();
    let user_id = created_user.id.to_string();

    // Now get the user
    let response = client
        .get(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let user: User = response.json().await.unwrap();
    assert_eq!(user.name, "Bob");
    assert_eq!(user.email, "bob@example.com");

    // Cleanup
    let _ = client
        .delete(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await;
}

#[tokio::test]
async fn test_get_user_not_found() {
    let client = client();

    // Try to get a non-existent user
    let fake_id = Uuid::new_v4();
    let response = client
        .get(&format!("{}/users/{}", BASE_URL, fake_id))
        .send()
        .await
        .unwrap();

    // Should return 404
    assert_eq!(response.status(), 404);
}

// ============================================================================
// Phase 2.3: LIST USERS Tests
// ============================================================================

#[tokio::test]
async fn test_list_users_default_pagination() {
    let client = client();

    // Create a few test users
    for i in 1..=3 {
        let _ = client
            .post(&format!("{}/users", BASE_URL))
            .json(&json!({
                "name": format!("User {}", i),
                "email": format!("user{}@example.com", i)
            }))
            .send()
            .await;
    }

    // List users
    let response = client.get(&format!("{}/users", BASE_URL)).send().await.unwrap();

    assert_eq!(response.status(), 200);

    let result: UserListResponse = response.json().await.unwrap();

    assert!(result.total >= 0); 
    assert_eq!(result.page, 1);
    assert_eq!(result.per_page, 10);

    for user in &result.users {
        let user_id = user.id.to_string();
        let _ = client
            .delete(&format!("{}/users/{}", BASE_URL, user_id))
            .send()
            .await;
    }
}

// TODO: Add test for pagination with page and per_page parameters
// TODO: Add test for empty list
// TODO: Add test for pagination edge cases

// ============================================================================
// Phase 2.4: UPDATE USER Tests
// ============================================================================

#[tokio::test]
async fn test_update_user_success() {
    cleanup_user_by_email("original@example.com").await;

    let client = client();

    let create_response = client
        .post(&format!("{}/users", BASE_URL))
        .json(&json!({
            "name": "Original Name",
            "email": "original@example.com"
        }))
        .send()
        .await
        .unwrap();

    let created_user: User = create_response.json().await.unwrap();
    let user_id = created_user.id.to_string();

    let response = client
        .put(&format!("{}/users/{}", BASE_URL, user_id))
        .json(&json!({
            "name": "Updated Name"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let updated_user: User = response.json().await.unwrap();
    assert_eq!(updated_user.name, "Updated Name");
    assert_eq!(updated_user.email, "original@example.com"); 

    let _ = client
        .delete(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await;
}

#[tokio::test]
async fn test_update_user_not_found() {
    let client = client();

    let fake_id = Uuid::new_v4();
    let response = client
        .put(&format!("{}/users/{}", BASE_URL, fake_id))
        .json(&json!({
            "name": "New Name"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

// TODO: Add test for partial update (only email)
// TODO: Add test for updating to duplicate email

// ============================================================================
// Phase 2.5: DELETE USER Tests
// ============================================================================

#[tokio::test]
async fn test_delete_user_success() {
    cleanup_user_by_email("delete@example.com").await;

    let client = client();

    let create_response = client
        .post(&format!("{}/users", BASE_URL))
        .json(&json!({
            "name": "To Delete",
            "email": "delete@example.com"
        }))
        .send()
        .await
        .unwrap();

    let created_user: User = create_response.json().await.unwrap();
    let user_id = created_user.id.to_string();

    let response = client
        .delete(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 204);

    let get_response = client
        .get(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .unwrap();

    assert_eq!(get_response.status(), 404);
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let client = client();

    let fake_id = Uuid::new_v4();
    let response = client
        .delete(&format!("{}/users/{}", BASE_URL, fake_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

// ============================================================================
// Phase 2.6: Integration Tests
// ============================================================================

#[tokio::test]
async fn test_complete_user_lifecycle() {
    cleanup_user_by_email("lifecycle@example.com").await;

    let client = client();

    let create_response = client
        .post(&format!("{}/users", BASE_URL))
        .json(&json!({
            "name": "Lifecycle Test",
            "email": "lifecycle@example.com"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(create_response.status(), 201);
    let user: User = create_response.json().await.unwrap();
    let user_id = user.id.to_string();

    let get_response = client
        .get(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .unwrap();
    assert_eq!(get_response.status(), 200);

    let update_response = client
        .put(&format!("{}/users/{}", BASE_URL, user_id))
        .json(&json!({
            "name": "Updated Lifecycle"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(update_response.status(), 200);

    let list_response = client
        .get(&format!("{}/users", BASE_URL))
        .send()
        .await
        .unwrap();
    assert_eq!(list_response.status(), 200);

    let delete_response = client
        .delete(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .unwrap();
    assert_eq!(delete_response.status(), 204);

    let final_get = client
        .get(&format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .unwrap();
    assert_eq!(final_get.status(), 404);
}

// ============================================================================
// NOTE: To run these tests, you need:
// 1. Docker PostgreSQL running: docker-compose up -d
// 2. Migrations applied: sqlx migrate run
// 3. Server running: cargo run (in another terminal)
// 4. Then: cargo test
//
// For test-driven development:
// 1. Run one test: cargo test test_create_user
// 2. See it fail (Red)
// 3. Implement the handler
// 4. See it pass (Green)
// 5. Refactor if needed
// 6. Move to next test
// ============================================================================
