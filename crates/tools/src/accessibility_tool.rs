//! Accessibility tool - WCAG compliance checks and guidance
//!
//! This tool provides accessibility compliance checks and guidance for AvaloniaUI applications.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Accessibility tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccessibilityParams {
    /// XAML content to check
    pub xaml_content: Option<String>,
    /// WCAG level to check against (A, AA, AAA)
    pub wcag_level: Option<String>,
    /// Include remediation guidance
    pub include_guidance: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccessibleComponentParams { pub component_type: Option<String>, pub include_keyboard_nav: Option<bool> }

/// Accessibility tool for WCAG compliance checks
#[derive(Debug, Clone, Default)]
pub struct AccessibilityTool;

impl AccessibilityTool {
    /// Create a new AccessibilityTool instance
    pub fn new() -> Self {
        Self
    }

    /// Check accessibility compliance
    #[tool(description = "Check accessibility compliance for AvaloniaUI applications. Provides WCAG 2.2 guidance, automated checks, and remediation recommendations.")]
    pub async fn check_accessibility(
        &self,
        params: AccessibilityParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let wcag_level = params.wcag_level.as_deref().unwrap_or("AA");
        let include_guidance = params.include_guidance.unwrap_or(true);

        tracing::info!(wcag_level, "Checking accessibility compliance");

        let output = if let Some(xaml) = params.xaml_content.as_deref() {
            let xaml_results = self.check_xaml_accessibility(xaml);
            self.append_xaml_results(xaml_results, include_guidance)
        } else {
            self.provide_general_guidance(MarkdownOutputBuilder::new(), wcag_level, include_guidance)
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Check XAML content for accessibility issues
    fn check_xaml_accessibility(&self, xaml: &str) -> Vec<AccessibilityIssue> {
        let mut issues = Vec::new();

        // Check for images without AutomationProperties.Name
        if xaml.contains("<Image") && !xaml.contains("AutomationProperties.Name") {
            issues.push(AccessibilityIssue {
                severity: "Warning".to_string(),
                rule: "Images must have alternative text".to_string(),
                wcag_criterion: "1.1.1 Non-text Content".to_string(),
                description: "Image control found without AutomationProperties.Name for screen readers".to_string(),
                suggestion: "Add AutomationProperties.Name to describe the image".to_string(),
            });
        }

        // Check for buttons without Content or automation name
        if xaml.contains("<Button") {
            let has_content = xaml.contains("Content=") || xaml.contains(">");
            let has_automation = xaml.contains("AutomationProperties.Name");
            
            if !has_content && !has_automation {
                issues.push(AccessibilityIssue {
                    severity: "Error".to_string(),
                    rule: "Buttons must have accessible names".to_string(),
                    wcag_criterion: "4.1.2 Name, Role, Value".to_string(),
                    description: "Button found without accessible name".to_string(),
                    suggestion: "Add Content property or AutomationProperties.Name".to_string(),
                });
            }
        }

        // Check for TextBox without Watermark or label
        if xaml.contains("<TextBox") && !xaml.contains("Watermark=") {
            issues.push(AccessibilityIssue {
                severity: "Suggestion".to_string(),
                rule: "Input fields should have labels".to_string(),
                wcag_criterion: "1.3.1 Info and Relationships".to_string(),
                description: "TextBox found without Watermark or associated label".to_string(),
                suggestion: "Add Watermark property or associate with a Label".to_string(),
            });
        }

        // Check for color-only information indicators
        if xaml.contains("Foreground=") || xaml.contains("Background=") {
            issues.push(AccessibilityIssue {
                severity: "Suggestion".to_string(),
                rule: "Don't use color alone to convey information".to_string(),
                wcag_criterion: "1.4.1 Use of Color".to_string(),
                description: "Color properties detected - ensure information is not conveyed by color alone".to_string(),
                suggestion: "Add text labels or icons in addition to color indicators".to_string(),
            });
        }

        // Check for focus indicators
        if !xaml.contains("Focusable") && !xaml.contains("FocusVisualStyle") {
            issues.push(AccessibilityIssue {
                severity: "Suggestion".to_string(),
                rule: "Ensure visible focus indicators".to_string(),
                wcag_criterion: "2.4.7 Focus Visible".to_string(),
                description: "No explicit focus styling detected".to_string(),
                suggestion: "Ensure controls have visible focus indicators through styles".to_string(),
            });
        }

        issues
    }

    /// Append XAML check results to output
    fn append_xaml_results(
        &self,
        issues: Vec<AccessibilityIssue>,
        include_guidance: bool,
    ) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Accessibility Compliance Report")
            .paragraph("XAML Accessibility Assessment");
        let error_count = issues.iter().filter(|i| i.severity == "Error").count();
        let warning_count = issues.iter().filter(|i| i.severity == "Warning").count();
        let suggestion_count = issues.iter().filter(|i| i.severity == "Suggestion").count();

        builder = builder
            .heading(2, "Summary")
            .task_list(vec![
                (error_count == 0, format!("{} Errors found", error_count)),
                (warning_count == 0, format!("{} Warnings found", warning_count)),
                (true, format!("{} Suggestions", suggestion_count)),
            ]);

        if !issues.is_empty() {
            builder = builder.heading(2, "Issues Found");
            
            for issue in issues {
                builder = builder
                    .heading(3, &format!("{}: {}", issue.severity, issue.rule))
                    .paragraph(&issue.description)
                    .paragraph(&format!("**WCAG Criterion**: {}", issue.wcag_criterion));

                if include_guidance {
                    builder = builder
                        .paragraph(&format!("**Suggestion**: {}", issue.suggestion));
                }
            }
        } else {
            builder = builder.paragraph("✅ No accessibility issues detected!");
        }

        builder.build()
    }

    /// Provide general accessibility guidance
    fn provide_general_guidance(
        &self,
        builder: MarkdownOutputBuilder,
        _wcag_level: &str,
        include_guidance: bool,
    ) -> String {
        let mut builder = builder
            .heading(2, "WCAG Guidelines Overview")
            .paragraph("The Web Content Accessibility Guidelines (WCAG) 2.2 provide recommendations for making web content more accessible.")
            .heading(2, "Key Principles (POUR)")
            .list(vec![
                "Perceivable - Information must be presentable to users in ways they can perceive",
                "Operable - User interface components must be operable",
                "Understandable - Information and operation must be understandable",
                "Robust - Content must be robust enough to be interpreted by assistive technologies",
            ])
            .heading(2, "AvaloniaUI Accessibility Features")
            .list(vec![
                "AutomationProperties for screen reader support",
                "Keyboard navigation with Tab and arrow keys",
                "Focus management and visible focus indicators",
                "High contrast theme support",
                "Font size scaling",
            ]);

        if include_guidance {
            builder = builder
                .heading(2, "Implementation Checklist")
                .task_list(vec![
                    (false, "Add AutomationProperties.Name to all images"),
                    (false, "Ensure all buttons have accessible names"),
                    (false, "Provide labels for all input fields"),
                    (false, "Implement keyboard navigation"),
                    (false, "Test with screen readers (NVDA, Narrator)"),
                    (false, "Verify color contrast ratios (4.5:1 for AA)"),
                    (false, "Ensure focus indicators are visible"),
                    (false, "Support font scaling up to 200%"),
                ])
                .heading(2, "Code Examples")
                .heading(3, "Accessible Image")
                .code_block("xml", r#"<Image Source="logo.png" 
       AutomationProperties.Name="Company Logo"
       AutomationProperties.HelpText="Company logo image"/>"#)
                .heading(3, "Accessible Button")
                .code_block("xml", r#"<Button Content="Submit" 
        Click="Submit_Click"
        AutomationProperties.Name="Submit form"
        ToolTip.Tip="Click to submit the form"/>"#)
                .heading(3, "Accessible Input")
                .code_block("xml", r#"<StackPanel>
    <TextBlock Text="Email Address" 
               AutomationProperties.LabeledBy="{Binding $self}"/>
    <TextBox x:Name="emailTextBox" 
             Watermark="Enter your email"
             AutomationProperties.Name="Email Address"/>
</StackPanel>"#);
        }

        builder
            .heading(2, "Testing Tools")
            .list(vec![
                "Narrator (Windows built-in screen reader)",
                "NVDA (Free screen reader for Windows)",
                "VoiceOver (macOS screen reader)",
                "Accessibility Insights (Automated testing)",
                "Manual keyboard navigation testing",
            ])
            .build()
    }

    #[tool(description = "Generates WCAG compliant accessible UI components with proper ARIA labels and keyboard support")]
    pub async fn generate_accessible_component(&self, params: AccessibleComponentParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let component_type = params.component_type.as_deref().unwrap_or("button").to_lowercase();
        let include_keyboard = params.include_keyboard_nav.unwrap_or(true);
        let keyboard = if include_keyboard { "\n## Keyboard\n```csharp\nprotected override void OnKeyDown(KeyEventArgs e)\n{\n    base.OnKeyDown(e);\n    if (e.Key == Key.Enter || e.Key == Key.Space)\n    {\n        RaiseEvent(new RoutedEventArgs(Button.ClickEvent));\n        e.Handled = true;\n    }\n}\n```" } else { "" };
        let component = match component_type.as_str() {
            "button" => "<Button Content=\"Submit\"\n        AutomationProperties.Name=\"Submit form\"\n        AutomationProperties.HelpText=\"Click to submit\"\n        ToolTip.Tip=\"Submit the form\"/>",
            "input" => "<StackPanel>\n    <TextBlock Text=\"Email\" />\n    <TextBox x:Name=\"EmailBox\"\n             Watermark=\"Enter email\"\n             AutomationProperties.Name=\"Email Address\"/>\n</StackPanel>",
            "dialog" => "<Window AutomationProperties.Name=\"Confirmation Dialog\">\n    <Panel>\n        <TextBlock Text=\"Are you sure?\" FontSize=\"18\"/>\n        <Button Content=\"Yes\" IsDefault=\"True\"/>\n        <Button Content=\"No\" IsCancel=\"True\"/>\n    </Panel>\n</Window>",
            _ => "<!-- Unknown -->",
        };
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Accessible Component: {}", component_type))
            .heading(2, "Implementation").code_block("xml", component)
            .heading(2, "WCAG").list(&["1.1.1 Non-text Content", "1.3.1 Info and Relationships", "2.1.1 Keyboard", "2.4.3 Focus Order", "4.1.2 Name, Role, Value"]);
        let builder = if include_keyboard { builder.heading(2, "Keyboard").code_block("csharp", &keyboard.replace("\n## Keyboard\n```csharp\n", "").replace("\n```", "")) } else { builder };
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

/// Accessibility issue structure
#[derive(Debug, Clone)]
struct AccessibilityIssue {
    severity: String,
    rule: String,
    wcag_criterion: String,
    description: String,
    suggestion: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_accessibility_with_xaml() {
        let tool = AccessibilityTool::new();
        let params = AccessibilityParams {
            xaml_content: Some("<Image Source=\"test.png\"/><Button Content=\"OK\"/>".to_string()),
            wcag_level: Some("AA".to_string()),
            include_guidance: Some(true),
        };

        let result = tool.check_accessibility(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_check_accessibility_general() {
        let tool = AccessibilityTool::new();
        let params = AccessibilityParams {
            xaml_content: None,
            wcag_level: Some("AAA".to_string()),
            include_guidance: Some(true),
        };

        let result = tool.check_accessibility(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_accessible_component() {
        let tool = AccessibilityTool::new();
        let params = AccessibleComponentParams {
            component_type: Some("button".to_string()),
            include_keyboard_nav: Some(true),
        };
        let result = tool.generate_accessible_component(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
