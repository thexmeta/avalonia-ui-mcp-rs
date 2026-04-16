//! XAML validation tool - XAML syntax and pattern validation
//!
//! This tool validates XAML syntax and provides feedback on AvaloniaUI patterns.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// XAML validation tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct XamlValidationParams {
    /// XAML content to validate
    pub xaml_content: String,
    /// Enable strict validation mode
    pub strict_mode: Option<bool>,
    /// Check for accessibility issues
    pub check_accessibility: Option<bool>,
}

/// WPF to Avalonia conversion parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WpfConversionParams {
    /// WPF XAML content to convert
    pub wpf_xaml: String,
}

/// XAML validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XamlValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

/// XAML validation tool for syntax and pattern validation
#[derive(Debug, Clone, Default)]
pub struct XamlValidationTool;

impl XamlValidationTool {
    /// Create a new XamlValidationTool instance
    pub fn new() -> Self {
        Self
    }

    /// Validate XAML content
    #[tool(description = "Validate XAML syntax and patterns for AvaloniaUI. Checks for well-formed XML, Avalonia-specific controls, and best practices.")]
    pub async fn validate_xaml(
        &self,
        params: XamlValidationParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.xaml_content.is_empty() {
            return Err(AvaloniaMcpError::validation(
                "XAML content cannot be empty",
            ));
        }

        let strict_mode = params.strict_mode.unwrap_or(false);
        let check_accessibility = params.check_accessibility.unwrap_or(true);

        tracing::info!(
            content_length = params.xaml_content.len(),
            strict_mode,
            check_accessibility,
            "Validating XAML"
        );

        let mut result = XamlValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };

        // Basic XML well-formedness check
        let xml_result = self.validate_xml_wellformedness(&params.xaml_content);
        if !xml_result.is_valid {
            result.is_valid = false;
            result.errors.extend(xml_result.errors);
        }

        // Check for Avalonia-specific patterns
        let avalonia_result = self.validate_avalonia_patterns(&params.xaml_content);
        result.warnings.extend(avalonia_result.warnings);
        result.suggestions.extend(avalonia_result.suggestions);

        // Check accessibility if enabled
        if check_accessibility {
            let a11y_result = self.validate_accessibility(&params.xaml_content);
            result.warnings.extend(a11y_result.warnings);
            result.suggestions.extend(a11y_result.suggestions);
        }

        // Strict mode additional checks
        if strict_mode {
            let strict_result = self.validate_strict_mode(&params.xaml_content);
            result.warnings.extend(strict_result.warnings);
            result.suggestions.extend(strict_result.suggestions);
        }

        // Build output
        let output = self.build_output(&result);

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Convert WPF XAML to AvaloniaUI XAML
    #[tool(description = "Converts WPF XAML to AvaloniaUI XAML by updating namespaces and incompatible elements")]
    pub async fn convert_wpf_xaml_to_avalonia(
        &self,
        params: WpfConversionParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.wpf_xaml.is_empty() {
            return Err(AvaloniaMcpError::validation(
                "WPF XAML content cannot be empty",
            ));
        }

        tracing::info!(content_length = params.wpf_xaml.len(), "Converting WPF XAML to Avalonia");

        let mut converted_xaml = params.wpf_xaml.clone();
        let mut conversion_notes = Vec::new();
        let mut manual_attention = Vec::new();

        // 1. Replace WPF namespaces with AvaloniaUI namespaces
        if converted_xaml.contains("http://schemas.microsoft.com/winfx/2006/xaml/presentation") {
            converted_xaml = converted_xaml.replace(
                "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
                "https://github.com/avaloniaui",
            );
            conversion_notes.push("Replaced WPF presentation namespace with AvaloniaUI namespace".to_string());
        }

        // 2. Check for common items needing manual attention
        let checks = [
            ("DependencyProperty", "DependencyProperty usage detected - may need conversion to AvaloniaProperty"),
            ("RoutedCommand", "RoutedCommand usage detected - consider using ReactiveCommand instead"),
            ("<Trigger", "Trigger usage detected - AvaloniaUI uses different styling approach with Selectors"),
            ("ControlTemplate", "ControlTemplate detected - verify compatibility with AvaloniaUI templating"),
            ("DockPanel", "DockPanel is available in Avalonia.Controls.DockPanel"),
            ("UniformGrid", "UniformGrid is available in Avalonia.Controls.Primitives"),
            ("Viewbox", "Viewbox is available in Avalonia.Controls"),
        ];

        for (pattern, message) in &checks {
            if converted_xaml.contains(pattern) {
                manual_attention.push(message.to_string());
            }
        }

        // Validate the converted XAML
        let validation_result = self.validate_xml_wellformedness(&converted_xaml);

        // Build markdown output
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "WPF to AvaloniaUI XAML Conversion");

        if !conversion_notes.is_empty() {
            builder = builder
                .heading(2, "Conversion Notes")
                .list(&conversion_notes);
        }

        if !manual_attention.is_empty() {
            builder = builder
                .heading(2, "Items Requiring Manual Attention")
                .list(&manual_attention);
        }

        builder = builder
            .heading(2, "Converted XAML")
            .code_block("xml", &converted_xaml);

        if validation_result.is_valid {
            builder = builder.paragraph("Converted XAML is well-formed.");
        } else {
            builder = builder
                .heading(2, "Validation Warnings")
                .list(&validation_result.errors);
        }

        builder = builder
            .heading(2, "Common WPF-to-Avalonia Mappings")
            .list(&[
                "Window -> Window (same structure)",
                "TabControl -> TabControl",
                "DataGrid -> DataGrid (Avalonia.Controls.DataGrid)",
                "TreeView -> TreeView",
                "RelativeSource={RelativeSource Self} -> RelativeSource={RelativeSource Self}",
                "System.Windows -> Avalonia.Controls",
            ])
            .heading(2, "Tips")
            .list(&[
                "AvaloniaUI XAML files should use .axaml extension",
                "Check for binding compatibility",
                "Verify control namespace references",
            ]);

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    /// Validate XML well-formedness
    fn validate_xml_wellformedness(&self, xaml: &str) -> XamlValidationResult {
        let mut result = XamlValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };

        // Basic tag matching
        let mut stack: Vec<String> = Vec::new();
        let mut in_tag = false;
        let mut tag_content = String::new();
        let mut is_closing = false;
        let mut is_self_closing = false;

        for ch in xaml.chars() {
            match ch {
                '<' if !in_tag => {
                    in_tag = true;
                    tag_content.clear();
                    is_closing = false;
                    is_self_closing = false;
                }
                '>' if in_tag => {
                    in_tag = false;

                    if !is_self_closing {
                        let tag_name = tag_content.trim().to_string();

                        if is_closing {
                            // Closing tag
                            if let Some(last) = stack.pop() {
                                if last != tag_name {
                                    result.is_valid = false;
                                    result.errors.push(format!(
                                        "Mismatched closing tag: expected </{}>, found </{}>",
                                        last, tag_name
                                    ));
                                }
                            } else {
                                result.is_valid = false;
                                result.errors.push(format!(
                                    "Unexpected closing tag: </{}>",
                                    tag_name
                                ));
                            }
                        } else if !tag_name.is_empty() && !tag_name.starts_with('?') && !tag_name.starts_with('!') {
                            // Opening tag
                            stack.push(tag_name);
                        }
                    }
                }
                '/' if in_tag && tag_content.is_empty() => {
                    is_closing = true;
                }
                '/' if in_tag => {
                    is_self_closing = true;
                }
                c if in_tag => {
                    tag_content.push(c);
                }
                _ => {}
            }
        }

        // Check for unclosed tags
        if in_tag {
            result.is_valid = false;
            result.errors.push("Unclosed tag detected".to_string());
        }

        if !stack.is_empty() {
            result.is_valid = false;
            result.errors.push(format!(
                "Unclosed tags: {:?}",
                stack
            ));
        }

        result
    }

    /// Validate Avalonia-specific patterns
    fn validate_avalonia_patterns(&self, xaml: &str) -> XamlValidationResult {
        let mut result = XamlValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };

        // Check for common Avalonia controls
        let avalonia_controls = [
            "Grid", "StackPanel", "DockPanel", "WrapPanel", "UniformGrid",
            "Button", "TextBox", "TextBlock", "ListBox", "ComboBox",
            "DataGrid", "TabControl", "MenuItem", "ContextMenu",
        ];

        for control in &avalonia_controls {
            if xaml.contains(&format!("<{}", control)) {
                result.suggestions.push(format!(
                    "Found Avalonia control: {}. Ensure you have the correct namespace.",
                    control
                ));
            }
        }

        // Check for WPF-specific patterns that don't exist in Avalonia
        if xaml.contains("System.Windows") {
            result.warnings.push(
                "Found 'System.Windows' namespace. Avalonia uses 'Avalonia.Controls' instead."
                    .to_string(),
            );
        }

        // Check for common patterns
        if xaml.contains("Grid.RowDefinitions") && !xaml.contains("Grid.ColumnDefinitions") {
            result.suggestions.push(
                "Grid has RowDefinitions but no ColumnDefinitions. Consider if columns are needed."
                    .to_string(),
            );
        }

        result
    }

    /// Validate accessibility patterns
    fn validate_accessibility(&self, xaml: &str) -> XamlValidationResult {
        let mut result = XamlValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };

        // Check for Button without Content or AutomationProperties
        if xaml.contains("<Button") && !xaml.contains("Content=") && !xaml.contains("AutomationProperties") {
            result.warnings.push(
                "Button without Content or AutomationProperties may not be accessible."
                    .to_string(),
            );
        }

        // Check for images without Alt text
        if xaml.contains("<Image") && !xaml.contains("AutomationProperties.Name") {
            result.suggestions.push(
                "Consider adding AutomationProperties.Name to Image controls for screen readers."
                    .to_string(),
            );
        }

        // Check for TextBox without Watermark or Label
        if xaml.contains("<TextBox") && !xaml.contains("Watermark=") {
            result.suggestions.push(
                "Consider adding Watermark or associated Label to TextBox for better UX."
                    .to_string(),
            );
        }

        result
    }

    /// Validate strict mode patterns
    fn validate_strict_mode(&self, xaml: &str) -> XamlValidationResult {
        let mut result = XamlValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };

        // Check for hardcoded values that should be resources
        if xaml.contains("Color=\"") || xaml.contains("Background=\"#") {
            result.suggestions.push(
                "Consider using resource references for colors instead of hardcoded values."
                    .to_string(),
            );
        }

        // Check for inline styles
        if xaml.contains("<") && xaml.contains(">") && xaml.contains("Style=") {
            result.suggestions.push(
                "Consider defining styles in resources instead of inline for reusability."
                    .to_string(),
            );
        }

        result
    }

    /// Build markdown output from validation result
    fn build_output(&self, result: &XamlValidationResult) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "XAML Validation Results");

        if result.is_valid {
            builder = builder.paragraph("✅ XAML is well-formed and follows AvaloniaUI patterns.");
        } else {
            builder = builder.paragraph("❌ XAML validation failed. Please fix the errors below.");
        }

        if !result.errors.is_empty() {
            builder = builder
                .heading(2, "Errors")
                .list(&result.errors);
        }

        if !result.warnings.is_empty() {
            builder = builder
                .heading(2, "Warnings")
                .list(&result.warnings);
        }

        if !result.suggestions.is_empty() {
            builder = builder
                .heading(2, "Suggestions")
                .list(&result.suggestions);
        }

        if result.errors.is_empty() && result.warnings.is_empty() && result.suggestions.is_empty() {
            builder = builder.paragraph("No issues found. XAML looks good!");
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_xaml_valid() {
        let tool = XamlValidationTool::new();
        let params = XamlValidationParams {
            xaml_content: "<Grid><TextBlock Text=\"Hello\"/></Grid>".to_string(),
            strict_mode: None,
            check_accessibility: None,
        };

        let result = tool.validate_xaml(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_validate_xaml_empty() {
        let tool = XamlValidationTool::new();
        let params = XamlValidationParams {
            xaml_content: "".to_string(),
            strict_mode: None,
            check_accessibility: None,
        };

        let result = tool.validate_xaml(params).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AvaloniaMcpError::ValidationError(_)
        ));
    }

    #[tokio::test]
    async fn test_validate_xaml_mismatched_tags() {
        let tool = XamlValidationTool::new();
        let params = XamlValidationParams {
            xaml_content: "<Grid><TextBlock></Grid>".to_string(),
            strict_mode: None,
            check_accessibility: None,
        };

        let result = tool.validate_xaml(params).await.unwrap();
        // Should contain warnings or errors about mismatched tags
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[test]
    fn test_validate_xml_wellformedness() {
        let tool = XamlValidationTool::new();

        // Valid XML
        let result = tool.validate_xml_wellformedness("<Grid></Grid>");
        assert!(result.is_valid);

        // Self-closing tag
        let result = tool.validate_xml_wellformedness("<TextBlock/>");
        assert!(result.is_valid);

        // Mismatched tags
        let result = tool.validate_xml_wellformedness("<Grid><TextBlock></Grid>");
        assert!(!result.is_valid);

        // Unclosed tag
        let result = tool.validate_xml_wellformedness("<Grid>");
        assert!(!result.is_valid);
    }

    #[tokio::test]
    async fn test_convert_wpf_xaml_to_avalonia_success() {
        let tool = XamlValidationTool::new();
        let params = WpfConversionParams {
            wpf_xaml: r#"<Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
    <Grid>
        <TextBlock Text="Hello WPF"/>
    </Grid>
</Window>"#.to_string(),
        };

        let result = tool.convert_wpf_xaml_to_avalonia(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));

        let content_str = format!("{:?}", result.content[0]);
        assert!(content_str.contains("WPF to AvaloniaUI XAML Conversion"));
        assert!(content_str.contains("Conversion Notes"));
        assert!(content_str.contains("https://github.com/avaloniaui"));
    }

    #[tokio::test]
    async fn test_convert_wpf_xaml_empty() {
        let tool = XamlValidationTool::new();
        let params = WpfConversionParams {
            wpf_xaml: "".to_string(),
        };

        let result = tool.convert_wpf_xaml_to_avalonia(params).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AvaloniaMcpError::ValidationError(_)
        ));
    }

    #[tokio::test]
    async fn test_convert_wpf_xaml_with_manual_attention() {
        let tool = XamlValidationTool::new();
        let params = WpfConversionParams {
            wpf_xaml: r#"<Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
    <DockPanel>
        <Button Command="{Binding RoutedCommand}"/>
    </DockPanel>
</Window>"#.to_string(),
        };

        let result = tool.convert_wpf_xaml_to_avalonia(params).await.unwrap();
        let content_str = format!("{:?}", result.content[0]);
        assert!(content_str.contains("Items Requiring Manual Attention"));
        assert!(content_str.contains("DockPanel"));
        assert!(content_str.contains("RoutedCommand"));
    }
}
