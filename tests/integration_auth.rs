

//! Integration tests for the authentication module using mock responses.

use serde_json;

use igdb_atlas::auth::{OAuthToken, TokenManager};

#[test]
fn test_oauth_token_deserialize() {
    let fixture = include_str!("fixtures/auth/token_response.json");
    let token: OAuthToken = serde_json::from_str(fixture).expect("Should parse token");

    assert_eq!(token.access_token, "test_access_token_abc123");
    assert_eq!(token.expires_in, 3600);
    assert_eq!(token.token_type, "bearer");
}

#[test]
fn test_oauth_token_deserialize_with_extra_fields() {
    // Twitch may return additional fields; these should be ignored
    let json = r#"{
        "access_token": "tok_xyz",
        "expires_in": 7200,
        "token_type": "bearer",
        "scope": "",
        "extra_field": "should be ignored"
    }"#;

    let token: OAuthToken = serde_json::from_str(json).expect("Should handle extra fields");
    assert_eq!(token.access_token, "tok_xyz");
    assert_eq!(token.expires_in, 7200);
}

#[test]
fn test_token_manager_construction() {
    let manager = TokenManager::new("test_id".to_string(), "test_secret".to_string());

    // Should not have a valid token before any fetch
    assert!(!manager.has_valid_token());
}

#[test]
fn test_token_manager_invalidate() {
    let manager = TokenManager::new("id".to_string(), "secret".to_string());

    // Invalidate (should be a no-op if nothing cached)
    manager.invalidate_token();
    assert!(!manager.has_valid_token());
}

#[test]
fn test_token_manager_clone_shares_state() {
    let manager = TokenManager::new("id".to_string(), "secret".to_string());
    let cloned = manager.clone();

    // Both should report no valid token
    assert!(!manager.has_valid_token());
    assert!(!cloned.has_valid_token());

    // Invalidating one should affect the other (shared Arc)
    manager.invalidate_token();
    assert!(!cloned.has_valid_token());
}

#[tokio::test]
async fn test_token_manager_fetch_fails_with_invalid_url() {
    // Use a non-existent server to test failure handling
    // The actual fetch will fail because we can't reach the Twitch endpoint
    // in tests without mocking
    let manager = TokenManager::new("fake_id".to_string(), "fake_secret".to_string());

    // get_valid_token will try to fetch and fail
    let result = manager.get_valid_token().await;
    assert!(
        result.is_err(),
        "Should fail with invalid/unreachable credentials"
    );
}

#[test]
fn test_oauth_token_clone() {
    let original = OAuthToken {
        access_token: "token_abc".to_string(),
        expires_in: 3600,
        token_type: "bearer".to_string(),
    };

    let cloned = original.clone();
    assert_eq!(original.access_token, cloned.access_token);
    assert_eq!(original.expires_in, cloned.expires_in);
}
