//! Security pattern tool - Security best practices generation
//!
//! This tool provides security patterns and best practices for AvaloniaUI applications.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Security pattern tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SecurityPatternParams {
    /// Security concern area (e.g., "authentication", "data-protection", "xss-prevention")
    pub area: Option<String>,
    /// Application type (e.g., "enterprise", "consumer", "kiosk")
    pub app_type: Option<String>,
    /// Include code examples
    pub include_examples: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DataSecurityParams {
    pub security_area: Option<String>,
    pub include_encryption: Option<bool>,
    pub include_audit_logging: Option<bool>,
}

/// Security pattern tool for generating security best practices
#[derive(Debug, Clone, Default)]
pub struct SecurityPatternTool;

impl SecurityPatternTool {
    /// Create a new SecurityPatternTool instance
    pub fn new() -> Self {
        Self
    }

    /// Generate security patterns and best practices
    #[tool(description = "Generate security patterns and best practices for AvaloniaUI applications. Covers authentication, data protection, XSS prevention, and secure coding practices.")]
    pub async fn generate_security_pattern(
        &self,
        params: SecurityPatternParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let area = params.area.as_deref().unwrap_or("general");
        let include_examples = params.include_examples.unwrap_or(true);

        tracing::info!(area, "Generating security patterns");

        let output = match area {
            "authentication" => self.generate_authentication_patterns(include_examples),
            "data-protection" => self.generate_data_protection_patterns(include_examples),
            "xss-prevention" => self.generate_xss_prevention_patterns(include_examples),
            "secure-coding" => self.generate_secure_coding_patterns(include_examples),
            _ => self.generate_general_security_patterns(include_examples),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(description = "Creates defensive data security patterns with proper encryption, sanitization, and audit logging for AvaloniaUI applications")]
    pub async fn generate_data_security_pattern(
        &self,
        params: DataSecurityParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let security_area = params.security_area.as_deref().unwrap_or("encryption").to_lowercase();
        let include_encryption = params.include_encryption.unwrap_or(true);
        let include_audit = params.include_audit_logging.unwrap_or(true);

        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Data Security Patterns")
            .heading(2, "Configuration")
            .task_list(vec![
                (true, format!("Area: {}", security_area)),
                (true, format!("Encryption: {}", include_encryption)),
                (true, format!("Audit Logging: {}", include_audit)),
            ])
            .heading(2, "Data Sanitization")
            .code_block(
                "csharp",
                r#"public static class InputSanitizer
{
    public static string Sanitize(string input)
    {
        if (string.IsNullOrEmpty(input)) return input;
        return WebUtility.HtmlEncode(input)
            .Replace("'", "''")  // SQL injection prevention
            .Trim();
    }
}"#,
            );

        if include_encryption {
            builder = builder
                .heading(2, "Encryption Implementation")
                .code_block(
                    "csharp",
                    r#"// AES Encryption for sensitive data
public class DataEncryptionService
{
    private readonly Aes _aes = Aes.Create();

    public string Encrypt(string plainText)
    {
        var encryptor = _aes.CreateEncryptor();
        var plainBytes = Encoding.UTF8.GetBytes(plainText);
        var cipherBytes = encryptor.TransformFinalBlock(plainBytes, 0, plainBytes.Length);
        return Convert.ToBase64String(cipherBytes);
    }

    public string Decrypt(string cipherText)
    {
        var decryptor = _aes.CreateDecryptor();
        var cipherBytes = Convert.FromBase64String(cipherText);
        var plainBytes = decryptor.TransformFinalBlock(cipherBytes, 0, cipherBytes.Length);
        return Encoding.UTF8.GetString(plainBytes);
    }
}"#,
                );
        }

        if include_audit {
            builder = builder
                .heading(2, "Audit Logging")
                .code_block(
                    "csharp",
                    r#"public class AuditLogger
{
    private readonly ILogger<AuditLogger> _logger;

    public void LogAccess(string resource, string action, string userId)
    {
        _logger.LogInformation(
            "Audit: User {UserId} performed {Action} on {Resource} at {Timestamp}",
            userId, action, resource, DateTime.UtcNow);
    }

    public void LogSecurityEvent(string eventType, string details, string severity = "info")
    {
        _logger.LogWarning(
            "Security Event: {Type} - {Details} [{Severity}]",
            eventType, details, severity);
    }
}"#,
                );
        }

        builder = builder
            .heading(2, "Security Best Practices")
            .list(vec![
                "Encrypt sensitive data at rest",
                "Use parameterized queries",
                "Implement input validation",
                "Enable audit logging",
                "Follow principle of least privilege",
                "Regular security audits",
            ]);

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    /// Generate authentication patterns
    fn generate_authentication_patterns(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Authentication Security Patterns")
            .paragraph("Best practices for implementing secure authentication in AvaloniaUI applications.")
            .heading(2, "Key Principles")
            .list(vec![
                "Never store credentials in plain text",
                "Use secure credential storage (Windows Credential Manager, Keychain, etc.)",
                "Implement multi-factor authentication for sensitive operations",
                "Use token-based authentication with short expiration times",
                "Implement account lockout after failed attempts",
            ])
            .heading(2, "Implementation Patterns");

        if include_examples {
            builder = builder
                .heading(3, "Secure Credential Storage")
                .code_block("csharp", r#"// Use platform-specific secure storage
// Windows: CredentialManager
// macOS: Keychain
// Linux: libsecret

public interface ISecureStorage
{
    Task StoreAsync(string key, string value);
    Task<string?> RetrieveAsync(string key);
    Task DeleteAsync(string key);
}

// Implementation using protected data
public class SecureStorage : ISecureStorage
{
    public async Task StoreAsync(string key, string value)
    {
        var protectedData = ProtectedData.Protect(
            Encoding.UTF8.GetBytes(value),
            null,
            DataProtectionScope.CurrentUser);
        
        await SaveToFileAsync(key, protectedData);
    }
}"#)
                .heading(3, "Token-Based Authentication")
                .code_block("csharp", r#"// Implement token refresh mechanism
public class AuthenticationService
{
    private readonly ITokenService _tokenService;
    private DateTime _tokenExpiry;
    
    public async Task<bool> AuthenticateAsync(string username, string password)
    {
        var token = await _tokenService.RequestTokenAsync(username, password);
        _tokenExpiry = token.ExpiresAt.AddMinutes(-5); // Buffer time
        
        return true;
    }
    
    public async Task<string> GetValidTokenAsync()
    {
        if (DateTime.UtcNow >= _tokenExpiry)
        {
            await RefreshTokenAsync();
        }
        
        return _currentToken;
    }
}"#);
        }

        builder
            .heading(2, "Security Checklist")
            .task_list(vec![
                (true, "Credentials encrypted at rest"),
                (true, "Tokens have short expiration"),
                (true, "HTTPS for all network communication"),
                (true, "Account lockout implemented"),
                (false, "Multi-factor authentication enabled"),
            ])
            .build()
    }

    /// Generate data protection patterns
    fn generate_data_protection_patterns(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Data Protection Patterns")
            .paragraph("Protect sensitive data in AvaloniaUI applications using encryption and secure storage.")
            .heading(2, "Key Principles")
            .list(vec![
                "Encrypt sensitive data at rest",
                "Use platform-specific secure storage APIs",
                "Implement data classification (public, internal, confidential)",
                "Clear sensitive data from memory after use",
                "Use secure random number generation for keys",
            ])
            .heading(2, "Implementation Patterns");

        if include_examples {
            builder = builder
                .heading(3, "Data Encryption")
                .code_block("csharp", r#"// Use AES encryption for sensitive data
public class DataProtectionService
{
    private readonly Aes _aes;
    
    public DataProtectionService()
    {
        _aes = Aes.Create();
        _aes.KeySize = 256;
        _aes.Mode = CipherMode.GCM; // Authenticated encryption
    }
    
    public byte[] Encrypt(byte[] plaintext, byte[] key, byte[] iv)
    {
        using var transform = _aes.CreateEncryptor(key, iv);
        return transform.TransformFinalBlock(plaintext, 0, plaintext.Length);
    }
    
    public byte[] Decrypt(byte[] ciphertext, byte[] key, byte[] iv)
    {
        using var transform = _aes.CreateDecryptor(key, iv);
        return transform.TransformFinalBlock(ciphertext, 0, ciphertext.Length);
    }
}"#)
                .heading(3, "Secure Memory Handling")
                .code_block("csharp", r#"// Clear sensitive data from memory
public class SecureString : IDisposable
{
    private byte[] _data;
    
    public void Dispose()
    {
        if (_data != null)
        {
            // Overwrite with zeros
            Array.Clear(_data, 0, _data.Length);
            _data = null;
        }
    }
}"#);
        }

        builder
            .heading(2, "Security Checklist")
            .task_list(vec![
                (true, "Sensitive data encrypted"),
                (true, "Keys stored securely"),
                (true, "Memory cleared after use"),
                (false, "Data classification implemented"),
                (false, "Audit logging enabled"),
            ])
            .build()
    }

    /// Generate XSS prevention patterns
    fn generate_xss_prevention_patterns(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "XSS Prevention Patterns")
            .paragraph("Prevent Cross-Site Scripting attacks in AvaloniaUI applications.")
            .heading(2, "Key Principles")
            .list(vec![
                "Never trust user input",
                "Encode all output displayed to users",
                "Use Content Security Policy (CSP) headers",
                "Validate and sanitize HTML input",
                "Use parameterized queries for database access",
            ])
            .heading(2, "Implementation Patterns");

        if include_examples {
            builder = builder
                .heading(3, "Input Validation")
                .code_block("csharp", r#"// Validate and sanitize user input
public class InputSanitizer
{
    private static readonly Regex ScriptPattern = 
        new Regex("<script.*?>.*?</script>", RegexOptions.IgnoreCase | RegexOptions.Singleline);
    
    public static string Sanitize(string input)
    {
        if (string.IsNullOrEmpty(input))
            return input;
        
        // Remove script tags
        var sanitized = ScriptPattern.Replace(input, string.Empty);
        
        // HTML encode remaining content
        return System.Net.WebUtility.HtmlEncode(sanitized);
    }
}"#)
                .heading(3, "Safe Text Display")
                .code_block("csharp", r#"// Use TextBlock instead of HtmlTextBlock for user content
<StackPanel>
    <!-- Safe: TextBlock automatically escapes content -->
    <TextBlock Text="{Binding UserInput}" />
    
    <!-- Dangerous: Avoid rendering raw HTML from users -->
    <!-- <local:HtmlTextBlock Html="{Binding UserHtml}" /> -->
</StackPanel>"#);
        }

        builder
            .heading(2, "Security Checklist")
            .task_list(vec![
                (true, "All user input validated"),
                (true, "Output encoded before display"),
                (true, "No raw HTML from users"),
                (false, "CSP headers configured"),
                (false, "Security headers implemented"),
            ])
            .build()
    }

    /// Generate secure coding patterns
    fn generate_secure_coding_patterns(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Secure Coding Patterns")
            .paragraph("General secure coding practices for AvaloniaUI applications.")
            .heading(2, "Key Principles")
            .list(vec![
                "Follow principle of least privilege",
                "Implement defense in depth",
                "Use secure defaults",
                "Log security events",
                "Keep dependencies updated",
                "Perform regular security audits",
            ])
            .heading(2, "Implementation Patterns");

        if include_examples {
            builder = builder
                .heading(3, "Exception Handling")
                .code_block("csharp", r#"// Don't expose internal errors to users
public class SecureExceptionHandler
{
    public static async Task HandleAsync(Exception ex)
    {
        // Log full details internally
        Logger.LogError(ex, "An error occurred");
        
        // Show generic message to user
        await ShowUserMessageAsync(
            "An unexpected error occurred. Please try again.");
        
        // Never expose:
        // - Stack traces
        // - Connection strings
        // - Internal paths
        // - Version information
    }
}"#)
                .heading(3, "Secure File Access")
                .code_block("csharp", r#"// Validate file paths to prevent directory traversal
public class SecureFileAccess
{
    private readonly string _baseDirectory;
    
    public async Task<string> ReadFileAsync(string relativePath)
    {
        // Validate and normalize path
        var fullPath = Path.GetFullPath(
            Path.Combine(_baseDirectory, relativePath));
        
        // Ensure path is within base directory
        if (!fullPath.StartsWith(_baseDirectory))
        {
            throw new SecurityException(
                "Access denied: Path traversal detected");
        }
        
        return await File.ReadAllTextAsync(fullPath);
    }
}"#);
        }

        builder
            .heading(2, "Security Checklist")
            .task_list(vec![
                (true, "Exceptions handled securely"),
                (true, "File paths validated"),
                (true, "Dependencies updated"),
                (true, "Security events logged"),
                (false, "Penetration testing performed"),
            ])
            .build()
    }

    /// Generate general security patterns
    fn generate_general_security_patterns(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "General Security Patterns")
            .paragraph("Comprehensive security guidance for AvaloniaUI applications.")
            .heading(2, "Security Domains")
            .list(vec![
                "Authentication & Authorization",
                "Data Protection & Encryption",
                "Input Validation & Output Encoding",
                "Secure Communication",
                "Error Handling & Logging",
                "Dependency Management",
            ])
            .heading(2, "Quick Start Security Checklist")
            .task_list(vec![
                (false, "Implement secure authentication"),
                (false, "Encrypt sensitive data"),
                (false, "Validate all user input"),
                (false, "Use HTTPS for network calls"),
                (false, "Implement secure error handling"),
                (false, "Enable security logging"),
                (false, "Update all dependencies"),
                (false, "Perform security code review"),
            ])
            .heading(2, "Recommended Security Tools")
            .list(vec![
                "OWASP ZAP - Security testing",
                "SonarQube - Code quality and security",
                "Dependency-Check - Vulnerable dependency scanning",
                "dotnet-scan - .NET vulnerability scanning",
            ]);

        if include_examples {
            builder = builder
                .heading(2, "Security Configuration Template")
                .code_block("csharp", r#"// App security configuration
public class SecurityConfiguration
{
    // Authentication
    public int TokenExpirationMinutes { get; set; } = 30;
    public int MaxLoginAttempts { get; set; } = 5;
    public TimeSpan LockoutDuration { get; set; } = TimeSpan.FromMinutes(15);
    
    // Encryption
    public int EncryptionKeySize { get; set; } = 256;
    public string EncryptionAlgorithm { get; set; } = "AES-GCM";
    
    // Network
    public bool RequireHttps { get; set; } = true;
    public string[] AllowedOrigins { get; set; } = Array.Empty<string>();
    
    // Logging
    public bool LogSecurityEvents { get; set; } = true;
    public LogLevel SecurityLogLevel { get; set; } = LogLevel.Warning;
}"#);
        }

        builder
            .heading(2, "Next Steps")
            .numbered_list(vec![
                "Review authentication patterns",
                "Implement data protection",
                "Add input validation",
                "Configure secure logging",
                "Schedule security audit",
            ])
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_security_pattern_general() {
        let tool = SecurityPatternTool::new();
        let params = SecurityPatternParams {
            area: None,
            app_type: None,
            include_examples: Some(true),
        };

        let result = tool.generate_security_pattern(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
        assert!(result.content.len() > 0);
    }

    #[tokio::test]
    async fn test_generate_security_pattern_authentication() {
        let tool = SecurityPatternTool::new();
        let params = SecurityPatternParams {
            area: Some("authentication".to_string()),
            app_type: None,
            include_examples: Some(true),
        };

        let result = tool.generate_security_pattern(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_security_pattern_no_examples() {
        let tool = SecurityPatternTool::new();
        let params = SecurityPatternParams {
            area: Some("general".to_string()),
            app_type: None,
            include_examples: Some(false),
        };

        let result = tool.generate_security_pattern(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_data_security_pattern_success() {
        let tool = SecurityPatternTool::new();
        let params = DataSecurityParams {
            security_area: Some("encryption".to_string()),
            include_encryption: Some(true),
            include_audit_logging: Some(true),
        };
        let result = tool.generate_data_security_pattern(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
        assert!(result.content.len() > 0);
    }

    #[tokio::test]
    async fn test_generate_data_security_pattern_no_encryption() {
        let tool = SecurityPatternTool::new();
        let params = DataSecurityParams {
            security_area: Some("sanitization".to_string()),
            include_encryption: Some(false),
            include_audit_logging: Some(true),
        };
        let result = tool.generate_data_security_pattern(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_data_security_pattern_no_audit() {
        let tool = SecurityPatternTool::new();
        let params = DataSecurityParams {
            security_area: Some("encryption".to_string()),
            include_encryption: Some(true),
            include_audit_logging: Some(false),
        };
        let result = tool.generate_data_security_pattern(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_data_security_pattern_defaults() {
        let tool = SecurityPatternTool::new();
        let params = DataSecurityParams {
            security_area: None,
            include_encryption: None,
            include_audit_logging: None,
        };
        let result = tool.generate_data_security_pattern(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_data_security_pattern_minimal() {
        let tool = SecurityPatternTool::new();
        let params = DataSecurityParams {
            security_area: Some("general".to_string()),
            include_encryption: Some(false),
            include_audit_logging: Some(false),
        };
        let result = tool.generate_data_security_pattern(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
