//! Localization tool - i18n and l10n guidance
//!
//! This tool provides internationalization and localization guidance for AvaloniaUI applications.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Localization tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocalizationParams {
    /// Target locales (e.g., "en-US", "es-ES", "fr-FR")
    pub locales: Option<Vec<String>>,
    /// Include code examples
    pub include_examples: Option<bool>,
}

/// Culture formatting parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CultureFormattingParams {
    /// Culture code (e.g., 'en-US', 'de-DE', 'ja-JP')
    pub culture_code: Option<String>,
    /// Format type: 'date', 'number', 'currency', 'percent'
    pub format_type: Option<String>,
}

/// Localization tool for i18n guidance
#[derive(Debug, Clone, Default)]
pub struct LocalizationTool;

impl LocalizationTool {
    pub fn new() -> Self { Self }

    #[tool(description = "Generate localization patterns and guidance for AvaloniaUI applications. Covers i18n setup, resource files, pluralization, and RTL support.")]
    pub async fn generate_localization(
        &self,
        params: LocalizationParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_examples = params.include_examples.unwrap_or(true);
        let locales = params.locales.unwrap_or_default();

        let output = self.generate_localization_guide(&locales, include_examples);
        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    fn generate_localization_guide(&self, locales: &[String], include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Localization Guide")
            .paragraph("Internationalization (i18n) and localization (l10n) for AvaloniaUI applications.")
            .heading(2, "Setup Steps")
            .numbered_list(vec![
                "Create resource files for each locale",
                "Implement localization service",
                "Bind UI elements to localized strings",
                "Handle RTL languages if needed",
                "Test with different locales",
            ]);

        if include_examples {
            builder = builder
                .heading(2, "Resource File Structure")
                .code_block("text", r#"/Resources
    /Strings
        Strings.resx (default)
        Strings.es.resx (Spanish)
        Strings.fr.resx (French)
        Strings.de.resx (German)
        Strings.ja.resx (Japanese)
        Strings.ar.resx (Arabic - RTL)"#)
                .heading(2, "Localization Service")
                .code_block("csharp", r#"public class LocalizationService : ReactiveObject
{
    private CultureInfo _currentCulture = CultureInfo.CurrentUICulture;
    
    public CultureInfo CurrentCulture
    {
        get => _currentCulture;
        set
        {
            this.RaiseAndSetIfChanged(ref _currentCulture, value);
            Thread.CurrentThread.CurrentUICulture = value;
            Thread.CurrentThread.CurrentCulture = value;
            OnCultureChanged?.Invoke(this, value);
        }
    }
    
    public event EventHandler<CultureInfo> OnCultureChanged;
    
    public string GetLocalizedString(string key)
    {
        return Resources.Strings.ResourceManager.GetString(key) ?? key;
    }
    
    public IEnumerable<CultureInfo> GetSupportedCultures()
    {
        return new[]
        {
            new CultureInfo("en-US"),
            new CultureInfo("es-ES"),
            new CultureInfo("fr-FR"),
            new CultureInfo("de-DE"),
            new CultureInfo("ja-JP"),
            new CultureInfo("ar-SA"), // RTL
        };
    }
}"#)
                .heading(2, "XAML Bindings")
                .code_block("xml", r#"<!-- Using binding extension -->
<TextBlock Text="{x:Static strings:Strings.WelcomeMessage}"/>

<!-- Or with view model -->
<TextBlock Text="{Binding LocalizedStrings[WelcomeMessage]}"/>

<!-- Dynamic culture change -->
<TextBlock>
    <TextBlock.Text>
        <MultiBinding Converter="{StaticResource LocalizationConverter}">
            <Binding Path="CurrentCulture"/>
            <Binding Source="{x:Static strings:Strings.ResourceManager}"/>
            <Binding Source="WelcomeMessage"/>
        </MultiBinding>
    </TextBlock.Text>
</TextBlock>"#)
                .heading(2, "Pluralization")
                .code_block("csharp", r#"public static class PluralizationHelper
{
    public static string GetPluralized(int count, string singular, string plural)
    {
        return count == 1 ? singular : plural;
    }
    
    // Usage in XAML via converter
    // {Binding ItemCount, Converter={StaticResource PluralizationConverter}, 
    //          ConverterParameter='item;items'}
}"#);
        }

        if !locales.is_empty() || locales.contains(&"ar-SA".to_string()) || locales.contains(&"he-IL".to_string()) {
            builder = builder
                .heading(2, "RTL Support")
                .task_list(vec![
                    (true, "Set FlowDirection property"),
                    (true, "Mirror layouts for RTL"),
                    (true, "Test with Arabic/Hebrew"),
                    (false, "Handle bidirectional text"),
                ])
                .code_block("xml", r#"<!-- RTL Support -->
<StackPanel FlowDirection="{Binding CurrentFlowDirection}">
    <!-- Content automatically mirrors for RTL -->
</StackPanel>"#);
        }

        builder
            .heading(2, "Best Practices")
            .task_list(vec![
                (true, "Never hardcode strings in UI"),
                (true, "Use resource files for all text"),
                (true, "Test with pseudo-locales"),
                (true, "Consider text expansion (30% rule)"),
                (false, "Implement locale-specific formatting"),
            ])
            .build()
    }

    #[tool(description = "Creates culture-specific formatting and validation utilities for AvaloniaUI applications")]
    pub async fn generate_culture_formatting(&self, params: CultureFormattingParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let culture = params.culture_code.as_deref().unwrap_or("en-US");
        let format_type = params.format_type.as_deref().unwrap_or("date").to_lowercase();

        let format_code = match format_type.as_str() {
            "date" => format!(
                r#"// Date formatting for {culture}
var cultureInfo = new CultureInfo("{culture}");
var date = DateTime.Now;

// Short date pattern
var shortDate = date.ToString("d", cultureInfo);

// Long date pattern
var longDate = date.ToString("D", cultureInfo);

// Custom format
var custom = date.ToString("yyyy-MM-dd", cultureInfo);"#
            ),
            "number" => format!(
                r#"// Number formatting for {culture}
var cultureInfo = new CultureInfo("{culture}");
var number = 1234567.89;

var formatted = number.ToString("N", cultureInfo);"#
            ),
            "currency" => format!(
                r#"// Currency formatting for {culture}
var cultureInfo = new CultureInfo("{culture}");
var amount = 1234.56m;

var formatted = amount.ToString("C", cultureInfo);"#
            ),
            "percent" => format!(
                r#"// Percentage formatting for {culture}
var cultureInfo = new CultureInfo("{culture}");
var percentage = 0.85;

var formatted = percentage.ToString("P", cultureInfo);"#
            ),
            _ => "// Unknown format type. Use: date, number, currency, or percent".to_string(),
        };

        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Culture-Specific Formatting: {}", culture))
            .heading(2, "Configuration")
            .task_list(vec![
                (true, format!("Culture: {}", culture)),
                (true, format!("Format Type: {}", format_type)),
            ])
            .heading(2, "Formatting Code")
            .code_block("csharp", &format_code)
            .heading(2, "XAML Binding with Culture")
            .code_block("xml", r#"<TextBlock Text="{Binding Date, StringFormat='{}{0:d}'}" />
<TextBlock Text="{Binding Amount, StringFormat='{}{0:C}'}" />"#)
            .heading(2, "Common Cultures")
            .list(&[
                "en-US - English (United States)",
                "en-GB - English (United Kingdom)",
                "de-DE - German (Germany)",
                "fr-FR - French (France)",
                "ja-JP - Japanese (Japan)",
                "zh-CN - Chinese (Simplified, China)",
            ])
            .heading(2, "Validation Utilities")
            .code_block("csharp", r#"// Culture-aware number parsing
public static bool TryParseNumber(string input, out double result)
{
    return double.TryParse(input, NumberStyles.Any, CultureInfo.CurrentCulture, out result);
}

// Culture-aware email validation
public static bool IsValidEmail(string email)
{
    try {
        var addr = new MailAddress(email);
        return addr.Address == email;
    } catch { return false; }
}"#);

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_localization() {
        let tool = LocalizationTool::new();
        let params = LocalizationParams {
            locales: Some(vec!["en-US".to_string(), "es-ES".to_string()]),
            include_examples: Some(true),
        };

        let result = tool.generate_localization(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_culture_formatting_date() {
        let tool = LocalizationTool::new();
        let params = CultureFormattingParams {
            culture_code: Some("en-US".to_string()),
            format_type: Some("date".to_string()),
        };

        let result = tool.generate_culture_formatting(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_culture_formatting_currency() {
        let tool = LocalizationTool::new();
        let params = CultureFormattingParams {
            culture_code: Some("de-DE".to_string()),
            format_type: Some("currency".to_string()),
        };

        let result = tool.generate_culture_formatting(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_culture_formatting_defaults() {
        let tool = LocalizationTool::new();
        let params = CultureFormattingParams {
            culture_code: None,
            format_type: None,
        };

        let result = tool.generate_culture_formatting(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_culture_formatting_unknown_type() {
        let tool = LocalizationTool::new();
        let params = CultureFormattingParams {
            culture_code: Some("ja-JP".to_string()),
            format_type: Some("unknown".to_string()),
        };

        let result = tool.generate_culture_formatting(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
