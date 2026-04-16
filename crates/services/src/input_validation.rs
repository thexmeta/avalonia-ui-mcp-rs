//! Input validation service
//!
//! This module provides comprehensive parameter validation for tool inputs,
//! ensuring data integrity and providing clear error messages.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::types::ValidationResult;

/// Input validation service for tool parameters
pub struct InputValidationService;

impl InputValidationService {
    /// Validate a string is not empty
    pub fn validate_required_string(value: Option<&str>, field_name: &str) -> ValidationResult {
        match value {
            None | Some("") => ValidationResult::failure(vec![format!(
                "{} is required and cannot be empty",
                field_name
            )]),
            Some(_) => ValidationResult::success(),
        }
    }

    /// Validate a string matches a pattern (basic validation)
    pub fn validate_string_pattern(
        _value: &str,
        field_name: &str,
        pattern_name: &str,
        valid: bool,
    ) -> ValidationResult {
        if !valid {
            ValidationResult::failure(vec![format!(
                "{} does not match expected {} pattern",
                field_name, pattern_name
            )])
        } else {
            ValidationResult::success()
        }
    }

    /// Validate a file path exists (async check)
    pub async fn validate_path_exists(path: &str, field_name: &str) -> ValidationResult {
        match tokio::fs::metadata(path).await {
            Ok(_) => ValidationResult::success(),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                ValidationResult::failure(vec![format!("{} does not exist: {}", field_name, path)])
            }
            Err(e) => ValidationResult::failure(vec![format!(
                "Error accessing {}: {}",
                field_name, e
            )]),
        }
    }

    /// Validate a directory path exists
    pub async fn validate_directory_exists(path: &str, field_name: &str) -> ValidationResult {
        match tokio::fs::metadata(path).await {
            Ok(metadata) => {
                if metadata.is_dir() {
                    ValidationResult::success()
                } else {
                    ValidationResult::failure(vec![format!(
                        "{} exists but is not a directory: {}",
                        field_name, path
                    )])
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                ValidationResult::failure(vec![format!("{} does not exist: {}", field_name, path)])
            }
            Err(e) => ValidationResult::failure(vec![format!(
                "Error accessing {}: {}",
                field_name, e
            )]),
        }
    }

    /// Validate a path is writable
    pub async fn validate_path_writable(path: &str, field_name: &str) -> ValidationResult {
        // Try to get metadata to check if path is accessible
        match tokio::fs::metadata(path).await {
            Ok(metadata) => {
                // Check if readonly attribute is set (Windows)
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    const FILE_ATTRIBUTE_READONLY: u32 = 1;
                    if metadata.file_attributes() & FILE_ATTRIBUTE_READONLY != 0 {
                        return ValidationResult::failure(vec![format!(
                            "{} is read-only: {}",
                            field_name, path
                        )]);
                    }
                }

                ValidationResult::success()
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Path doesn't exist, check if parent directory is writable
                if let Some(parent) = std::path::Path::new(path).parent() {
                    return Self::validate_directory_writable(parent.to_str().unwrap(), "Parent directory").await;
                }
                ValidationResult::failure(vec![format!("{} does not exist: {}", field_name, path)])
            }
            Err(e) => ValidationResult::failure(vec![format!(
                "Error accessing {}: {}",
                field_name, e
            )]),
        }
    }

    /// Validate a directory is writable (can create files)
    pub async fn validate_directory_writable(
        path: &str,
        field_name: &str,
    ) -> ValidationResult {
        match tokio::fs::metadata(path).await {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return ValidationResult::failure(vec![format!(
                        "{} is not a directory: {}",
                        field_name, path
                    )]);
                }

                // Try to create a temporary file to test write access
                let test_file = std::path::Path::new(path).join(".write_test");
                match tokio::fs::File::create(&test_file).await {
                    Ok(_) => {
                        // Clean up test file
                        let _ = tokio::fs::remove_file(test_file).await;
                        ValidationResult::success()
                    }
                    Err(e) => ValidationResult::failure(vec![format!(
                        "{} is not writable: {}",
                        field_name, e
                    )]),
                }
            }
            Err(e) => ValidationResult::failure(vec![format!(
                "Cannot access {}: {}",
                field_name, e
            )]),
        }
    }

    /// Validate JSON value
    pub fn validate_json(value: &str, field_name: &str) -> ValidationResult {
        match serde_json::from_str::<serde_json::Value>(value) {
            Ok(_) => ValidationResult::success(),
            Err(e) => ValidationResult::failure(vec![format!(
                "{} contains invalid JSON: {}",
                field_name, e
            )]),
        }
    }

    /// Validate XAML syntax (basic XML validation)
    pub fn validate_xaml_syntax(value: &str, field_name: &str) -> ValidationResult {
        // Basic XML well-formedness check
        // In production, use a proper XML parser like quick-xml
        let mut stack: Vec<String> = Vec::new();
        let mut in_tag = false;
        let mut tag_content = String::new();

        for ch in value.chars() {
            match ch {
                '<' if !in_tag => {
                    in_tag = true;
                    tag_content.clear();
                }
                '>' if in_tag => {
                    in_tag = false;
                    // Parse the tag content
                    let tag_content = tag_content.trim();
                    
                    // Skip comments, processing instructions
                    if tag_content.starts_with('!') || tag_content.starts_with('?') {
                        continue;
                    }
                    
                    // Check if self-closing
                    let is_self_closing = tag_content.ends_with('/');
                    let tag_content = tag_content.trim_end_matches('/').trim();
                    
                    // Extract tag name (first word)
                    let parts: Vec<&str> = tag_content.split_whitespace().collect();
                    if parts.is_empty() {
                        continue;
                    }
                    
                    let tag_name = parts[0];
                    
                    if tag_name.starts_with('/') {
                        // Closing tag
                        let closing_name = tag_name.trim_start_matches('/');
                        if stack.pop().as_deref() != Some(closing_name) {
                            return ValidationResult::failure(vec![format!(
                                "{} has mismatched XML tags",
                                field_name
                            )]);
                        }
                    } else if !is_self_closing {
                        // Opening tag - push to stack
                        stack.push(tag_name.to_string());
                    }
                    // Self-closing tags don't go on the stack
                }
                c if in_tag => {
                    tag_content.push(c);
                }
                _ => {}
            }
        }

        if in_tag {
            return ValidationResult::failure(vec![format!(
                "{} has unclosed XML tag",
                field_name
            )]);
        }

        if !stack.is_empty() {
            return ValidationResult::failure(vec![format!(
                "{} has unclosed XML tags: {:?}",
                field_name, stack
            )]);
        }

        ValidationResult::success()
    }

    /// Validate numeric range
    pub fn validate_number_range<T: PartialOrd + std::fmt::Display>(
        value: T,
        field_name: &str,
        min: Option<T>,
        max: Option<T>,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(min_val) = min {
            if value < min_val {
                errors.push(format!(
                    "{} ({}) is less than minimum allowed ({})",
                    field_name, value, min_val
                ));
            }
        }

        if let Some(max_val) = max {
            if value > max_val {
                errors.push(format!(
                    "{} ({}) is greater than maximum allowed ({})",
                    field_name, value, max_val
                ));
            }
        }

        if errors.is_empty() {
            ValidationResult::success()
        } else {
            ValidationResult::failure(errors)
        }
    }

    /// Validate list is not empty
    pub fn validate_non_empty_list<T>(items: &[T], field_name: &str) -> ValidationResult {
        if items.is_empty() {
            ValidationResult::failure(vec![format!("{} cannot be empty", field_name)])
        } else {
            ValidationResult::success()
        }
    }

    /// Validate string length
    pub fn validate_string_length(
        value: &str,
        field_name: &str,
        min_len: Option<usize>,
        max_len: Option<usize>,
    ) -> ValidationResult {
        let len = value.len();
        let mut errors = Vec::new();

        if let Some(min) = min_len {
            if len < min {
                errors.push(format!(
                    "{} length ({}) is less than minimum ({})",
                    field_name, len, min
                ));
            }
        }

        if let Some(max) = max_len {
            if len > max {
                errors.push(format!(
                    "{} length ({}) exceeds maximum ({})",
                    field_name, len, max
                ));
            }
        }

        if errors.is_empty() {
            ValidationResult::success()
        } else {
            ValidationResult::failure(errors)
        }
    }

    /// Validate email format (basic)
    pub fn validate_email(email: &str, field_name: &str) -> ValidationResult {
        // Basic email validation - contains @ and has text before and after
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() || !parts[1].contains('.')
        {
            ValidationResult::failure(vec![format!("{} is not a valid email address", field_name)])
        } else {
            ValidationResult::success()
        }
    }

    /// Validate URL format (basic)
    pub fn validate_url(url: &str, field_name: &str) -> ValidationResult {
        // Basic URL validation - starts with http:// or https://
        if url.starts_with("http://") || url.starts_with("https://") {
            ValidationResult::success()
        } else {
            ValidationResult::failure(vec![format!(
                "{} must start with http:// or https://",
                field_name
            )])
        }
    }

    /// Combine multiple validation results
    pub fn combine_results(results: Vec<ValidationResult>) -> ValidationResult {
        results.into_iter().fold(
            ValidationResult::success(),
            |acc, result| acc.merge(result),
        )
    }

    /// Assert validation result or return error
    pub fn assert_valid(result: ValidationResult, context: &str) -> Result<(), AvaloniaMcpError> {
        if result.is_valid {
            Ok(())
        } else {
            Err(AvaloniaMcpError::validation(format!(
                "{}: {}",
                context,
                result.errors.join(", ")
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_required_string() {
        assert!(InputValidationService::validate_required_string(
            Some("value"),
            "field"
        )
        .is_valid);
        assert!(!InputValidationService::validate_required_string(None, "field").is_valid);
        assert!(!InputValidationService::validate_required_string(Some(""), "field").is_valid);
    }

    #[test]
    fn test_validate_json() {
        assert!(InputValidationService::validate_json(r#"{"key": "value"}"#, "field").is_valid);
        assert!(!InputValidationService::validate_json("invalid json", "field").is_valid);
    }

    #[test]
    fn test_validate_xaml_syntax() {
        assert!(
            InputValidationService::validate_xaml_syntax("<Grid></Grid>", "xaml").is_valid
        );
        assert!(
            InputValidationService::validate_xaml_syntax("<Grid><TextBlock/></Grid>", "xaml")
                .is_valid
        );
        assert!(
            !InputValidationService::validate_xaml_syntax("<Grid><TextBlock></Grid>", "xaml")
                .is_valid
        );
    }

    #[test]
    fn test_validate_number_range() {
        assert!(InputValidationService::validate_number_range(5, "value", Some(0), Some(10)).is_valid);
        assert!(!InputValidationService::validate_number_range(-1, "value", Some(0), Some(10)).is_valid);
        assert!(!InputValidationService::validate_number_range(11, "value", Some(0), Some(10)).is_valid);
    }

    #[test]
    fn test_validate_string_length() {
        assert!(InputValidationService::validate_string_length("hello", "text", Some(1), Some(10)).is_valid);
        assert!(!InputValidationService::validate_string_length("", "text", Some(1), Some(10)).is_valid);
        assert!(!InputValidationService::validate_string_length("very long text", "text", Some(1), Some(10)).is_valid);
    }

    #[test]
    fn test_validate_email() {
        assert!(InputValidationService::validate_email("test@example.com", "email").is_valid);
        assert!(!InputValidationService::validate_email("invalid", "email").is_valid);
        assert!(!InputValidationService::validate_email("test@", "email").is_valid);
    }

    #[test]
    fn test_validate_url() {
        assert!(InputValidationService::validate_url("https://example.com", "url").is_valid);
        assert!(InputValidationService::validate_url("http://example.com", "url").is_valid);
        assert!(!InputValidationService::validate_url("ftp://example.com", "url").is_valid);
        assert!(!InputValidationService::validate_url("example.com", "url").is_valid);
    }

    #[tokio::test]
    async fn test_validate_directory_exists() {
        // Test with existing directory
        let result = InputValidationService::validate_directory_exists(
            &std::env::temp_dir().to_string_lossy(),
            "temp_dir",
        )
        .await;
        assert!(result.is_valid);

        // Test with non-existent directory
        let result = InputValidationService::validate_directory_exists(
            "/nonexistent/directory/path",
            "nonexistent",
        )
        .await;
        assert!(!result.is_valid);
    }

    #[test]
    fn test_combine_results() {
        let results = vec![
            ValidationResult::success(),
            ValidationResult::failure(vec!["error1".to_string()]),
            ValidationResult::success().with_warning("warning1"),
        ];
        let combined = InputValidationService::combine_results(results);
        assert!(!combined.is_valid);
        assert_eq!(combined.errors.len(), 1);
        assert_eq!(combined.warnings.len(), 1);
    }
}
