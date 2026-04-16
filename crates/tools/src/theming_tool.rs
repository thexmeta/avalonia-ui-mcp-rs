//! Theming tool - Theme generation and customization
//!
//! This tool provides theme generation and customization guidance for AvaloniaUI applications.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Theming tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ThemingParams {
    /// Theme type (e.g., "light", "dark", "custom")
    pub theme_type: Option<String>,
    /// Include color palette
    pub include_palette: Option<bool>,
    /// Application type (e.g., "enterprise", "consumer")
    pub app_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectorsParams { pub selector_type: Option<String>, pub target_control: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ColorSchemeParams { pub base_color: Option<String>, pub format: Option<String> }

/// Theming tool for generating theme patterns
#[derive(Debug, Clone, Default)]
pub struct ThemingTool;

impl ThemingTool {
    /// Create a new ThemingTool instance
    pub fn new() -> Self {
        Self
    }

    /// Generate theme patterns
    #[tool(description = "Generate theme patterns and XAML styles for AvaloniaUI applications. Covers light/dark themes, custom color palettes, and Fluent/Material design systems.")]
    pub async fn generate_theme(
        &self,
        params: ThemingParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let theme_type = params.theme_type.as_deref().unwrap_or("all");
        let include_palette = params.include_palette.unwrap_or(true);

        tracing::info!(theme_type, "Generating theme patterns");

        let output = match theme_type {
            "light" => self.generate_light_theme(include_palette),
            "dark" => self.generate_dark_theme(include_palette),
            "fluent" => self.generate_fluent_theme(include_palette),
            "material" => self.generate_material_theme(include_palette),
            _ => self.generate_complete_theme_guide(include_palette),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Generate light theme
    fn generate_light_theme(&self, include_palette: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Light Theme")
            .paragraph("Clean, professional light theme for AvaloniaUI applications.")
            .heading(2, "Characteristics")
            .list(vec![
                "High contrast for readability",
                "Professional appearance",
                "Better for bright environments",
                "Traditional business aesthetic",
            ])
            .heading(2, "Implementation");

        if include_palette {
            builder = builder
                .heading(3, "Color Palette")
                .code_block("xml", r#"<!-- Light Theme Resources -->
<ResourceDictionary>
    <!-- Primary Colors -->
    <Color x:Key="PrimaryColor">#1976D2</Color>
    <Color x:Key="PrimaryLightColor">#BBDEFB</Color>
    <Color x:Key="PrimaryDarkColor">#0D47A1</Color>
    
    <!-- Background Colors -->
    <Color x:Key="BackgroundColor">#FFFFFF</Color>
    <Color x:Key="SurfaceColor">#F5F5F5</Color>
    <Color x:Key="CardColor">#FFFFFF</Color>
    
    <!-- Text Colors -->
    <Color x:Key="PrimaryTextColor">#212121</Color>
    <Color x:Key="SecondaryTextColor">#757575</Color>
    <Color x:Key="DisabledTextColor">#BDBDBD</Color>
    
    <!-- Border Colors -->
    <Color x:Key="BorderColor">#E0E0E0</Color>
    <Color x:Key="DividerColor">#EEEEEE</Color>
    
    <!-- Status Colors -->
    <Color x:Key="SuccessColor">#4CAF50</Color>
    <Color x:Key="WarningColor">#FF9800</Color>
    <Color x:Key="ErrorColor">#F44336</Color>
    <Color x:Key="InfoColor">#2196F3</Color>
</ResourceDictionary>"#)
                .heading(3, "Theme Styles")
                .code_block("xml", r#"<Application.Styles>
    <!-- Light Theme Style -->
    <Style Selector="Window.light-theme">
        <Setter Property="Background" Value="{StaticResource BackgroundColor}"/>
        <Setter Property="Foreground" Value="{StaticResource PrimaryTextColor}"/>
    </Style>
    
    <Style Selector="Card.light">
        <Setter Property="Background" Value="{StaticResource CardColor}"/>
        <Setter Property="BorderBrush" Value="{StaticResource BorderColor}"/>
        <Setter Property="BorderThickness" Value="1"/>
        <Setter Property="BoxShadow" Value="0 1 3px rgba(0,0,0,0.12)"/>
    </Style>
    
    <Style Selector="Button.primary">
        <Setter Property="Background" Value="{StaticResource PrimaryColor}"/>
        <Setter Property="Foreground" Value="White"/>
        <Setter Property="BorderThickness" Value="0"/>
        <Setter Property="Padding" Value="16,8"/>
    </Style>
</Application.Styles>"#);
        }

        builder
            .heading(2, "Best Practices")
            .task_list(vec![
                (true, "Maintain 4.5:1 contrast ratio"),
                (true, "Use subtle shadows for depth"),
                (true, "Provide hover states"),
                (false, "Add high contrast mode"),
            ])
            .build()
    }

    /// Generate dark theme
    fn generate_dark_theme(&self, include_palette: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Dark Theme")
            .paragraph("Modern dark theme for reduced eye strain and battery savings.")
            .heading(2, "Characteristics")
            .list(vec![
                "Reduced eye strain in low light",
                "Battery savings on OLED displays",
                "Modern, sleek appearance",
                "Better for media-focused apps",
            ])
            .heading(2, "Implementation");

        if include_palette {
            builder = builder
                .heading(3, "Color Palette")
                .code_block("xml", r#"<!-- Dark Theme Resources -->
<ResourceDictionary>
    <!-- Primary Colors -->
    <Color x:Key="PrimaryColor">#90CAF9</Color>
    <Color x:Key="PrimaryLightColor">#E3F2FD</Color>
    <Color x:Key="PrimaryDarkColor">#42A5F5</Color>
    
    <!-- Background Colors -->
    <Color x:Key="BackgroundColor">#121212</Color>
    <Color x:Key="SurfaceColor">#1E1E1E</Color>
    <Color x:Key="CardColor">#2D2D2D</Color>
    
    <!-- Text Colors -->
    <Color x:Key="PrimaryTextColor">#FFFFFF</Color>
    <Color x:Key="SecondaryTextColor">#B0B0B0</Color>
    <Color x:Key="DisabledTextColor">#6B6B6B</Color>
    
    <!-- Border Colors -->
    <Color x:Key="BorderColor">#404040</Color>
    <Color x:Key="DividerColor">#383838</Color>
    
    <!-- Status Colors (Adjusted for Dark) -->
    <Color x:Key="SuccessColor">#81C784</Color>
    <Color x:Key="WarningColor">#FFB74D</Color>
    <Color x:Key="ErrorColor">#E57373</Color>
    <Color x:Key="InfoColor">#64B5F6</Color>
</ResourceDictionary>"#)
                .heading(3, "Dark Theme Styles")
                .code_block("xml", r#"<Application.Styles>
    <!-- Dark Theme Style -->
    <Style Selector="Window.dark-theme">
        <Setter Property="Background" Value="{StaticResource BackgroundColor}"/>
        <Setter Property="Foreground" Value="{StaticResource PrimaryTextColor}"/>
    </Style>
    
    <Style Selector="Card.dark">
        <Setter Property="Background" Value="{StaticResource CardColor}"/>
        <Setter Property="BorderBrush" Value="{StaticResource BorderColor}"/>
        <Setter Property="BorderThickness" Value="1"/>
        <Setter Property="BoxShadow" Value="0 2 4px rgba(0,0,0,0.3)"/>
    </Style>
    
    <!-- Adjust controls for dark theme -->
    <Style Selector="TextBox">
        <Setter Property="Background" Value="{StaticResource SurfaceColor}"/>
        <Setter Property="Foreground" Value="{StaticResource PrimaryTextColor}"/>
        <Setter Property="BorderBrush" Value="{StaticResource BorderColor}"/>
    </Style>
</Application.Styles>"#);
        }

        builder
            .heading(2, "Dark Theme Best Practices")
            .task_list(vec![
                (true, "Use #121212 for backgrounds (not pure black)"),
                (true, "Reduce saturation for dark mode colors"),
                (true, "Increase text size slightly"),
                (true, "Test in actual dark environment"),
            ])
            .build()
    }

    /// Generate Fluent theme
    fn generate_fluent_theme(&self, include_palette: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Fluent Design Theme")
            .paragraph("Microsoft Fluent Design System implementation for AvaloniaUI.")
            .heading(2, "Fluent Principles")
            .list(vec![
                "Light - Illuminates important elements",
                "Depth - Layered z-axis positioning",
                "Motion - Purposeful transitions",
                "Material - Acrylic and mica effects",
                "Scale - Responsive design",
            ])
            .heading(2, "Implementation");

        if include_palette {
            builder = builder
                .heading(3, "Fluent Color Palette")
                .code_block("xml", r#"<!-- Fluent Design Resources -->
<ResourceDictionary>
    <!-- Fluent Primary Colors -->
    <Color x:Key="FluentPrimaryColor">#0078D4</Color>
    <Color x:Key="FluentPrimaryHoverColor">#106EBE</Color>
    <Color x:Key="FluentPrimaryPressedColor">#005A9E</Color>
    
    <!-- Acrylic Background -->
    <Color x:Key="AcrylicBackgroundColor">#F3F3F3</Color>
    <Color x:Key="AcrylicDarkBackgroundColor">#202020</Color>
    
    <!-- Elevation Shadows -->
    <BoxShadow x:Key="Elevation4">0 2 4px rgba(0,0,0,0.1)</BoxShadow>
    <BoxShadow x:Key="Elevation8">0 4 8px rgba(0,0,0,0.12)</BoxShadow>
    <BoxShadow x:Key="Elevation16">0 8 16px rgba(0,0,0,0.14)</BoxShadow>
    <BoxShadow x:Key="Elevation32">0 16 32px rgba(0,0,0,0.16)</BoxShadow>
    
    <!-- Reveal Highlight -->
    <Color x:Key="RevealHighlightColor">#FFFFFF</Color>
    <Color x:Key="RevealDarkHighlightColor">#FFFFFF</Color>
</ResourceDictionary>"#)
                .heading(3, "Fluent Control Styles")
                .code_block("xml", r#"<!-- Fluent Button Style -->
<Style Selector="Button.fluent">
    <Setter Property="Background" Value="Transparent"/>
    <Setter Property="Foreground" Value="{StaticResource FluentPrimaryColor}"/>
    <Setter Property="BorderBrush" Value="{StaticResource FluentPrimaryColor}"/>
    <Setter Property="BorderThickness" Value="2"/>
    <Setter Property="Padding" Value="16,8"/>
    <Setter Property="CornerRadius" Value="4"/>
    <Setter Property="FontWeight" Value="SemiBold"/>
</Style>

<!-- Fluent Card with Elevation -->
<Style Selector="Card.fluent">
    <Setter Property="Background" Value="{StaticResource CardColor}"/>
    <Setter Property="CornerRadius" Value="8"/>
    <Setter Property="BoxShadow" Value="{StaticResource Elevation8}"/>
    <Setter Property="Padding" Value="16"/>
</Style>"#);
        }

        builder
            .heading(2, "Fluent Features")
            .task_list(vec![
                (true, "Use Segoe UI Variable font"),
                (true, "Apply acrylic materials"),
                (true, "Implement reveal effects"),
                (true, "Add depth with shadows"),
            ])
            .build()
    }

    /// Generate Material theme
    fn generate_material_theme(&self, include_palette: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Material Design Theme")
            .paragraph("Google Material Design 3 implementation for AvaloniaUI.")
            .heading(2, "Material Principles")
            .list(vec![
                "Material is the metaphor",
                "Bold, graphic, intentional",
                "Motion provides meaning",
                "Adaptive and responsive",
            ])
            .heading(2, "Implementation");

        if include_palette {
            builder = builder
                .heading(3, "Material Color System")
                .code_block("xml", r#"<!-- Material Design 3 Resources -->
<ResourceDictionary>
    <!-- Material Primary Palette -->
    <Color x:Key="MaterialPrimary">#6750A4</Color>
    <Color x:Key="MaterialOnPrimary">#FFFFFF</Color>
    <Color x:Key="MaterialPrimaryContainer">#EADDFF</Color>
    <Color x:Key="MaterialOnPrimaryContainer">#21005D</Color>
    
    <!-- Material Surface Colors -->
    <Color x:Key="MaterialSurface">#FFFBFE</Color>
    <Color x:Key="MaterialSurface1">#F7F2FA</Color>
    <Color x:Key="MaterialSurface2">#F3EDF7</Color>
    <Color x:Key="MaterialSurface3">#ECE6F0</Color>
    <Color x:Key="MaterialSurface4">#E7E0E9</Color>
    
    <!-- Material Elevation -->
    <BoxShadow x:Key="MaterialShadow1">0 1px 2px rgba(0,0,0,0.3)</BoxShadow>
    <BoxShadow x:Key="MaterialShadow2">0 1px 3px rgba(0,0,0,0.15)</BoxShadow>
    <BoxShadow x:Key="MaterialShadow3">0 4px 8px rgba(0,0,0,0.15)</BoxShadow>
    
    <!-- Material Shapes -->
    <CornerRadius x:Key="MaterialSmall">4</CornerRadius>
    <CornerRadius x:Key="MaterialMedium">8</CornerRadius>
    <CornerRadius x:Key="MaterialLarge">16</CornerRadius>
    <CornerRadius x:Key="MaterialExtraLarge">28</CornerRadius>
</ResourceDictionary>"#)
                .heading(3, "Material Component Styles")
                .code_block("xml", r#"<!-- Material Filled Button -->
<Style Selector="Button.material-filled">
    <Setter Property="Background" Value="{StaticResource MaterialPrimary}"/>
    <Setter Property="Foreground" Value="{StaticResource MaterialOnPrimary}"/>
    <Setter Property="CornerRadius" Value="{StaticResource MaterialMedium}"/>
    <Setter Property="Padding" Value="24,10"/>
    <Setter Property="FontWeight" Value="Medium"/>
</Style>

<!-- Material Outlined Button -->
<Style Selector="Button.material-outlined">
    <Setter Property="Background" Value="Transparent"/>
    <Setter Property="Foreground" Value="{StaticResource MaterialPrimary}"/>
    <Setter Property="BorderBrush" Value="{StaticResource MaterialPrimary}"/>
    <Setter Property="BorderThickness" Value="1"/>
    <Setter Property="CornerRadius" Value="{StaticResource MaterialMedium}"/>
</Style>

<!-- Material Card -->
<Style Selector="Card.material">
    <Setter Property="Background" Value="{StaticResource MaterialSurface}"/>
    <Setter Property="CornerRadius" Value="{StaticResource MaterialLarge}"/>
    <Setter Property="BoxShadow" Value="{StaticResource MaterialShadow2}"/>
</Style>"#);
        }

        builder
            .heading(2, "Material Features")
            .task_list(vec![
                (true, "Use Roboto or Material Symbols"),
                (true, "Apply elevation system"),
                (true, "Implement state layers"),
                (true, "Use color tokens system"),
            ])
            .build()
    }

    /// Generate complete theme guide
    fn generate_complete_theme_guide(&self, include_palette: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Complete Theme Guide")
            .paragraph("Comprehensive theming guide for AvaloniaUI applications.")
            .heading(2, "Available Themes")
            .list(vec![
                "Light Theme - Traditional, professional",
                "Dark Theme - Modern, eye-friendly",
                "Fluent Design - Microsoft design language",
                "Material Design - Google design system",
            ])
            .heading(2, "Theme Switching Implementation");

        if include_palette {
            builder = builder
                .heading(3, "Theme Manager Service")
                .code_block("csharp", r#"public class ThemeManager : ReactiveObject
{
    private string _currentTheme = "Light";
    
    public string CurrentTheme
    {
        get => _currentTheme;
        set => this.RaiseAndSetIfChanged(ref _currentTheme, value);
    }
    
    public void ApplyTheme(string themeName)
    {
        var app = Application.Current;
        
        // Remove existing theme styles
        app.Styles.Remove(ThemeStyles);
        
        // Load new theme
        ThemeStyles = LoadTheme(themeName);
        app.Styles.Add(ThemeStyles);
        
        CurrentTheme = themeName;
    }
    
    private StyleInclude ThemeStyles { get; set; }
    
    private StyleInclude LoadTheme(string themeName)
    {
        return new StyleInclude(new Uri("avares://App"))
        {
            Source = new Uri($"avares://YourApp/Themes/{themeName}.xaml")
        };
    }
}"#)
                .heading(3, "Theme Toggle Control")
                .code_block("xml", r#"<StackPanel Orientation="Horizontal">
    <RadioButton Content="Light" 
                 IsChecked="{Binding IsLightTheme}"/>
    <RadioButton Content="Dark" 
                 IsChecked="{Binding IsDarkTheme}"/>
    <RadioButton Content="System" 
                 IsChecked="{Binding IsSystemTheme}"/>
</StackPanel>"#);
        }

        builder
            .heading(2, "Theme Checklist")
            .task_list(vec![
                (false, "Define color palette"),
                (false, "Create light theme"),
                (false, "Create dark theme"),
                (false, "Implement theme switching"),
                (false, "Test all controls"),
                (false, "Add system theme detection"),
            ])
            .heading(2, "Theme Files Structure")
            .code_block("text", r#"/Themes
    /Light
        Colors.xaml
        Styles.xaml
    /Dark
        Colors.xaml
        Styles.xaml
    /Fluent
        Colors.xaml
        Styles.xaml
    /Material
        Colors.xaml
        Styles.xaml
    ThemeManager.xaml"#)
            .build()
    }

    #[tool(description = "Creates CSS-like selectors for AvaloniaUI styling with specificity and inheritance patterns")]
    pub async fn generate_selectors(&self, params: SelectorsParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let st = params.selector_type.as_deref().unwrap_or("class").to_lowercase();
        let target = params.target_control.as_deref().unwrap_or("Button");
        let examples = match st.as_str() {
            "class" => ".primary { Background: #007ACC; }\n.primary:pointerover { Background: #005A9E; }".to_string(),
            "type" => format!("{target} {{ Padding: 12,8; MinHeight: 32; }}"),
            "name" => "#SubmitButton { Background: Green; }".to_string(),
            "hierarchy" => format!("DockPanel > {target} {{ Margin: 4; }}"),
            _ => "/* Unknown */".to_string(),
        };
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Selectors: {}", st))
            .heading(2, "Examples").code_block("css", &examples)
            .heading(2, "Specificity").list(&["Type: Button {}", "Class: .primary {}", "Name: #Id {}", "Pseudo: :pointerover, :pressed", "Combinator: > (child)"]);
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Converts colors between formats and generates color schemes for AvaloniaUI themes")]
    pub async fn generate_color_scheme(&self, params: ColorSchemeParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let base = params.base_color.as_deref().unwrap_or("#007ACC");
        let fmt = params.format.as_deref().unwrap_or("hex").to_lowercase();
        let builder = MarkdownOutputBuilder::new()
            .heading(1, "Color Scheme Generator")
            .heading(2, "Config").task_list(vec![(true, format!("Base: {}", base)), (true, format!("Format: {}", fmt))])
            .heading(2, "Palette").list(&[&format!("Primary: {}", base), &format!("Light: {}80", base), "Dark: #005A9E", "Accent: #FF6B35", "Success: #28A745", "Error: #DC3545"])
            .heading(2, "XAML").code_block("xml", &format!("<ResourceDictionary>\n    <SolidColorBrush x:Key=\"Primary\" Color=\"{}\" />\n</ResourceDictionary>", base))
            .heading(2, "Convert").code_block("csharp", r##"var color = Color.Parse("#007ACC");
var rgb = $"RGB({color.R}, {color.G}, {color.B})";"##);
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_theme_light() {
        let tool = ThemingTool::new();
        let params = ThemingParams {
            theme_type: Some("light".to_string()),
            include_palette: Some(true),
            app_type: None,
        };

        let result = tool.generate_theme(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_theme_complete() {
        let tool = ThemingTool::new();
        let params = ThemingParams {
            theme_type: None,
            include_palette: Some(true),
            app_type: None,
        };

        let result = tool.generate_theme(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_selectors() {
        let tool = ThemingTool::new();
        let params = SelectorsParams {
            selector_type: Some("class".to_string()),
            target_control: Some("Button".to_string()),
        };
        let result = tool.generate_selectors(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_color_scheme() {
        let tool = ThemingTool::new();
        let params = ColorSchemeParams {
            base_color: Some("#007ACC".to_string()),
            format: Some("hex".to_string()),
        };
        let result = tool.generate_color_scheme(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
