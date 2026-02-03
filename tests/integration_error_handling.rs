//! Integration tests for error handling and custom errors.

use igdb_atlas::error::{IGDBError, Result};

#[test]
fn test_error_display_authentication_failed() {
    let err = IGDBError::AuthenticationFailed("bad credentials".to_string());
    assert!(
        err.to_string().contains("Authentication failed"),
        "Got: {}",
        err
    );
    assert!(err.to_string().contains("bad credentials"), "Got: {}", err);
}

#[test]
fn test_error_display_rate_limited() {
    let err = IGDBError::RateLimited {
        retry_after_ms: 2500,
        attempts: 5,
    };
    assert!(err.to_string().contains("5"), "Got: {}", err);
    assert!(err.to_string().contains("2500"), "Got: {}", err);
}

#[test]
fn test_error_display_api_error() {
    let err = IGDBError::ApiError {
        status: 404,
        message: "Not Found".to_string(),
    };
    assert!(err.to_string().contains("404"), "Got: {}", err);
    assert!(err.to_string().contains("Not Found"), "Got: {}", err);
}

#[test]
fn test_error_display_query_build() {
    let err = IGDBError::QueryBuildError("invalid field".to_string());
    assert!(err.to_string().contains("invalid field"), "Got: {}", err);
}

#[test]
fn test_error_display_token_expired() {
    let err = IGDBError::TokenExpired;
    assert!(err.to_string().contains("Token expired"), "Got: {}", err);
}

#[test]
fn test_error_display_invalid_configuration() {
    let err = IGDBError::InvalidConfiguration("missing client_id".to_string());
    assert!(
        err.to_string().contains("missing client_id"),
        "Got: {}",
        err
    );
}

#[test]
fn test_is_retriable_rate_limited() {
    let err = IGDBError::RateLimited {
        retry_after_ms: 1000,
        attempts: 3,
    };
    assert!(err.is_retriable());
}

#[test]
fn test_is_retriable_auth_failed() {
    let err = IGDBError::AuthenticationFailed("test".to_string());
    assert!(!err.is_retriable());
}

#[test]
fn test_is_retriable_api_error() {
    let err = IGDBError::ApiError {
        status: 500,
        message: "Server error".to_string(),
    };
    assert!(!err.is_retriable());
}

#[test]
fn test_is_retriable_token_expired() {
    let err = IGDBError::TokenExpired;
    assert!(!err.is_retriable());
}

#[test]
fn test_retry_after_ms_present() {
    let err = IGDBError::RateLimited {
        retry_after_ms: 5000,
        attempts: 2,
    };
    assert_eq!(err.retry_after_ms(), Some(5000));
}

#[test]
fn test_retry_after_ms_absent() {
    let err = IGDBError::AuthenticationFailed("test".to_string());
    assert_eq!(err.retry_after_ms(), None);
}

#[test]
fn test_custom_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
    let err = IGDBError::from_custom(io_err);

    assert!(err.to_string().contains("disk full"), "Got: {}", err);
    assert!(!err.is_retriable());
}

#[test]
fn test_custom_error_from_string() {
    let string_err = std::io::Error::new(std::io::ErrorKind::Other, "custom message");
    let err = IGDBError::from_custom(string_err);
    assert!(err.to_string().contains("custom message"), "Got: {}", err);
}

#[test]
fn test_result_type_alias() {
    fn parse(input: &str) -> Result<u64> {
        input
            .parse::<u64>()
            .map_err(|e| IGDBError::QueryBuildError(e.to_string()))
    }

    assert!(parse("42").is_ok());
    assert_eq!(parse("42").unwrap(), 42);
    assert!(parse("not_a_number").is_err());
}

#[test]
fn test_error_debug_format() {
    let err = IGDBError::ApiError {
        status: 422,
        message: "Validation error".to_string(),
    };

    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("ApiError"), "Got: {}", debug_str);
    assert!(debug_str.contains("422"), "Got: {}", debug_str);
}
