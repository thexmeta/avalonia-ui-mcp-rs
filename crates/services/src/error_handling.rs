//! Async-safe error handling service
//!
//! This module provides a centralized error handling service with tracing
//! integration for consistent error management across the server.

use anyhow::Result;
use avalonia_mcp_core::error::AvaloniaMcpError;
use tracing::{error, info_span, instrument, warn, Instrument};

/// Async-safe error handling service
pub struct ErrorHandlingService;

impl ErrorHandlingService {
    /// Execute an async operation with error handling and tracing
    #[instrument(skip(f), fields(operation = %operation))]
    pub async fn safe_execute<T, F, Fut>(operation: &'static str, f: F) -> Result<T, AvaloniaMcpError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AvaloniaMcpError>>,
    {
        let span = info_span!("safe_execute", operation);
        async move {
            match f().await {
                Ok(result) => Ok(result),
                Err(err) => {
                    error!(error = %err, "Operation failed: {}", operation);
                    Err(err)
                }
            }
        }
        .instrument(span)
        .await
    }

    /// Execute an async operation with retry logic
    #[instrument(skip(f), fields(operation = %operation, max_retries = %max_retries))]
    pub async fn safe_execute_with_retry<T, F, Fut>(
        operation: &'static str,
        max_retries: u32,
        f: F,
    ) -> Result<T, AvaloniaMcpError>
    where
        F: Fn() -> Fut + Clone,
        Fut: std::future::Future<Output = Result<T, AvaloniaMcpError>>,
    {
        let mut last_error = None;
        let mut attempt = 0;

        while attempt <= max_retries {
            match f().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    warn!(
                        error = %err,
                        attempt = attempt + 1,
                        max_retries,
                        "Operation failed, retrying..."
                    );
                    last_error = Some(err);
                    attempt += 1;

                    if attempt <= max_retries {
                        // Exponential backoff: 100ms, 200ms, 400ms, etc.
                        let delay_ms = 100 * (2u64.pow(attempt - 1));
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AvaloniaMcpError::internal("Unknown retry error")))
    }

    /// Validate common parameters and return error if invalid
    pub fn validate_common_params(
        params: &avalonia_mcp_core::types::CommonParams,
    ) -> avalonia_mcp_core::types::ValidationResult {
        let mut errors = Vec::new();

        if let Some(ref path) = params.project_path {
            if path.is_empty() {
                errors.push("project_path cannot be empty".to_string());
            }
        }

        avalonia_mcp_core::types::ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    /// Wrap a result with additional context
    pub fn with_context<T>(
        result: Result<T, AvaloniaMcpError>,
        context: &str,
    ) -> Result<T, AvaloniaMcpError> {
        result.map_err(|err| AvaloniaMcpError::internal(format!("{}: {}", context, err)))
    }

    /// Log and convert an anyhow error to AvaloniaMcpError
    #[instrument(skip(err), fields(error = %err))]
    pub fn handle_anyhow_error(err: anyhow::Error, operation: &str) -> AvaloniaMcpError {
        error!(operation, "Anyhow error occurred");
        AvaloniaMcpError::InternalError(format!("{}: {}", operation, err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_execute_success() {
        let result = ErrorHandlingService::safe_execute("test_op", || async {
            Ok::<_, AvaloniaMcpError>("success".to_string())
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_safe_execute_error() {
        let result = ErrorHandlingService::safe_execute("test_op", || async {
            Err::<String, _>(AvaloniaMcpError::validation("test error"))
        })
        .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AvaloniaMcpError::ValidationError(_)
        ));
    }

    #[tokio::test]
    async fn test_safe_execute_with_retry_success() {
        let attempts = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let attempts_clone = attempts.clone();
        
        let result = ErrorHandlingService::safe_execute_with_retry(
            "test_retry",
            3,
            move || {
                let attempts = attempts_clone.clone();
                async move {
                    let current = attempts.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    if current < 1 {
                        Err(AvaloniaMcpError::internal("temporary error"))
                    } else {
                        Ok::<_, AvaloniaMcpError>("success after retry".to_string())
                    }
                }
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success after retry");
    }

    #[tokio::test]
    async fn test_safe_execute_with_retry_failure() {
        let result = ErrorHandlingService::safe_execute_with_retry(
            "test_retry_fail",
            2,
            || async { Err::<String, _>(AvaloniaMcpError::internal("permanent error")) },
        )
        .await;

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_common_params() {
        let valid_params = avalonia_mcp_core::types::CommonParams {
            project_path: Some("/path/to/project".to_string()),
            verbose: Some(true),
        };
        let result = ErrorHandlingService::validate_common_params(&valid_params);
        assert!(result.is_valid);

        let invalid_params = avalonia_mcp_core::types::CommonParams {
            project_path: Some("".to_string()),
            verbose: None,
        };
        let result = ErrorHandlingService::validate_common_params(&invalid_params);
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_with_context() {
        let ok_result: Result<String, AvaloniaMcpError> = Ok("value".to_string());
        let wrapped = ErrorHandlingService::with_context(ok_result, "adding context");
        assert!(wrapped.is_ok());

        let err_result: Result<String, AvaloniaMcpError> =
            Err(AvaloniaMcpError::validation("original error"));
        let wrapped = ErrorHandlingService::with_context(err_result, "adding context");
        assert!(wrapped.is_err());
        let err = wrapped.unwrap_err();
        assert!(err.to_string().contains("adding context"));
        assert!(err.to_string().contains("original error"));
    }
}
