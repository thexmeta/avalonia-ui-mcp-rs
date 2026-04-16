//! UI/UX Design tool - Design patterns and guidance
use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UIUXDesignParams {
    pub design_aspect: Option<String>,
    pub include_examples: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ResponsiveDesignParams {
    /// Layout type: 'adaptive', 'fluid', 'hybrid'
    pub layout_type: Option<String>,
    /// Target devices: comma-separated
    pub target_devices: Option<String>,
    /// Include touch gesture support
    pub include_touch: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UXPatternsParams { pub pattern_type: Option<String>, pub include_examples: Option<bool> }
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DesignSystemParams { pub project_name: String, pub include_tokens: Option<bool> }

#[derive(Debug, Clone, Default)]
pub struct UIUXDesignTool;

impl UIUXDesignTool {
    pub fn new() -> Self { Self }

    #[tool(description = "Provide UI/UX design guidance for AvaloniaUI applications. Covers design principles, layout patterns, color theory, and user experience best practices.")]
    pub async fn provide_uiux_design(
        &self,
        params: UIUXDesignParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_examples = params.include_examples.unwrap_or(true);
        let design_aspect = params.design_aspect.as_deref().unwrap_or("principles");

        let output = match design_aspect {
            "layout" => self.design_layout_patterns(include_examples),
            "color" => self.design_color_theory(include_examples),
            "typography" => self.design_typography(include_examples),
            "accessibility" => self.design_accessibility(include_examples),
            _ => self.design_principles(include_examples),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    fn design_principles(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "UI/UX Design Principles")
            .paragraph("Core design principles for AvaloniaUI applications.")
            .heading(2, "Key Principles")
            .list(vec![
                "Consistency - Use consistent patterns throughout",
                "Hierarchy - Establish clear visual hierarchy",
                "Feedback - Provide immediate user feedback",
                "Forgiveness - Allow undo and error recovery",
                "Efficiency - Minimize user effort",
            ]);

        if include_examples {
            builder = builder
                .heading(2, "Layout Best Practices")
                .code_block("xml", r#"<!-- Use Grid for complex layouts -->
<Grid RowDefinitions=\"Auto,*,Auto\" ColumnDefinitions=\"*,*\">
    <!-- Header -->
    <Border Grid.Row=\"0\" Grid.ColumnSpan=\"2\" Background=\"Accent\">
        <TextBlock Text=\"Application Title\" Classes=\"h1\"/>
    </Border>
    
    <!-- Navigation -->
    <StackPanel Grid.Row=\"1\" Grid.Column=\"0\" Margin=\"8\">
        <Button Content=\"Home\" Classes=\"nav-button\"/>
        <Button Content=\"Settings\" Classes=\"nav-button\"/>
    </StackPanel>
    
    <!-- Content -->
    <ScrollViewer Grid.Row=\"1\" Grid.Column=\"1\">
        <!-- Main content here -->
    </ScrollViewer>
    
    <!-- Footer -->
    <Border Grid.Row=\"2\" Grid.ColumnSpan=\"2\" Classes=\"footer\">
        <TextBlock Text=\"© 2024 Company\"/>
    </Border>
</Grid>"#)
                .heading(2, "Spacing System")
                .code_block("xml", r#"<!-- Use consistent spacing (8px grid) -->
<StackPanel Margin=\"16\">
    <TextBlock Text=\"Title\" Margin=\"0,0,0,8\"/>
    <TextBlock Text=\"Content\" Margin=\"0,0,0,16\"/>
    <Button Content=\"Action\" Padding=\"16,8\"/>
</StackPanel>

<!-- Spacing values: 4, 8, 16, 24, 32, 48, 64 -->"#);
        }

        builder.heading(2, "Design Checklist")
            .task_list(vec![
                (true, "Consistent color scheme"),
                (true, "Clear typography hierarchy"),
                (true, "Adequate spacing"),
                (true, "Accessible contrast ratios"),
                (false, "Responsive layout"),
            ])
            .build()
    }

    fn design_layout_patterns(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Layout Patterns")
            .paragraph("Common layout patterns for desktop applications.")
            .heading(2, "Master-Detail")
            .code_block("xml", r#"<Grid ColumnDefinitions=\"250,*\">
    <!-- Master List -->
    <ListBox Grid.Column=\"0\" ItemsSource=\"{Binding Items}\"
             SelectedItem=\"{Binding SelectedItem}\"/>
    
    <!-- Detail View -->
    <ContentControl Grid.Column=\"1\" Content=\"{Binding SelectedItem}\"/>
</Grid>"#)
            .heading(2, "Dashboard")
            .code_block("xml", r#"<WrapPanel>
    <Card Title=\"Users\" Value=\"{Binding UserCount}\"/>
    <Card Title=\"Orders\" Value=\"{Binding OrderCount}\"/>
    <Card Title=\"Revenue\" Value=\"{Binding Revenue}\"/>
</WrapPanel>"#)
            .heading(2, "Wizard")
            .code_block("xml", r#"<Grid RowDefinitions=\"Auto,*,Auto\">
    <!-- Steps -->
    <StepsControl Steps=\"{Binding Steps}\" CurrentStep=\"{Binding CurrentStep}\"/>
    
    <!-- Content -->
    <ContentControl Content=\"{Binding CurrentView}\"/>
    
    <!-- Navigation -->
    <StackPanel Orientation=\"Horizontal\" HorizontalAlignment=\"Right\">
        <Button Content=\"Back\" Command=\"{Binding BackCommand}\"/>
        <Button Content=\"Next\" Command=\"{Binding NextCommand}\"/>
    </StackPanel>
</Grid>"#)
            .build()
    }

    fn design_color_theory(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Color Theory")
            .paragraph("Color principles for UI design.")
            .heading(2, "60-30-10 Rule")
            .list(vec!["60% Primary (background)", "30% Secondary (surfaces)", "10% Accent (calls-to-action)"])
            .heading(2, "Color Palette")
            .code_block("xml", r#"<!-- Define in App.axaml -->
<Application.Styles>
    <StyleInclude Source=\"avares://YourApp/Colors.xaml\"/>
</Application.Styles>

<!-- Colors.xaml -->
<ResourceDictionary>
    <Color x:Key=\"PrimaryColor\">#1976D2</Color>
    <Color x:Key=\"SecondaryColor\">#424242</Color>
    <Color x:Key=\"AccentColor\">#FF5722</Color>
    <Color x:Key=\"SuccessColor\">#4CAF50</Color>
    <Color x:Key=\"ErrorColor\">#F44336</Color>
</ResourceDictionary>"#)
            .build()
    }

    fn design_typography(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Typography")
            .paragraph("Typography best practices for desktop apps.")
            .heading(2, "Font Hierarchy")
            .list(vec!["H1: 32px - Page titles", "H2: 24px - Section headers", "H3: 20px - Subsections", "Body: 14px - Main content", "Caption: 12px - Secondary text"])
            .heading(2, "Font Families")
            .code_block("xml", r#"<!-- Recommended fonts -->
<Application.Styles>
    <Style Selector=\"TextBlock.h1\">
        <Setter Property=\"FontSize\" Value=\"32\"/>
        <Setter Property=\"FontWeight\" Value=\"Bold\"/>
    </Style>
    <Style Selector=\"TextBlock.body\">
        <Setter Property=\"FontSize\" Value=\"14\"/>
        <Setter Property=\"FontFamily\" Value=\"Segoe UI, Roboto\"/>
    </Style>
</Application.Styles>"#)
            .build()
    }

    fn design_accessibility(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Accessible Design")
            .paragraph("Design for all users.")
            .heading(2, "WCAG Guidelines")
            .list(vec!["Contrast ratio 4.5:1 minimum", "Don't rely on color alone", "Provide keyboard navigation", "Support screen readers", "Allow text scaling"])
            .heading(2, "Implementation")
            .code_block("xml", r#"<!-- Accessible controls -->
<Button Content=\"Submit\"
        AutomationProperties.Name=\"Submit form\"
        ToolTip.Tip=\"Click to submit\"/>

<Image Source=\"logo.png\"
       AutomationProperties.Name=\"Company Logo\"/>

<TextBox Watermark=\"Enter email\"
         AutomationProperties.Name=\"Email Address\"/>"#)
            .build()
    }

    #[tool(description = "Generates responsive design patterns with adaptive layouts and breakpoints for AvaloniaUI applications")]
    pub async fn generate_responsive_design(&self, params: ResponsiveDesignParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let layout_type = params.layout_type.as_deref().unwrap_or("adaptive").to_lowercase();
        let devices = params.target_devices.as_deref().unwrap_or("mobile,tablet,desktop");
        let include_touch = params.include_touch.unwrap_or(true);
        let _device_list: Vec<&str> = devices.split(',').map(|d| d.trim()).collect();

        let breakpoints_xaml = match layout_type.as_str() {
            "adaptive" => r#"<!-- Adaptive Layout with VisualStateGroups -->
<Window>
    <VisualStateManager.VisualStateGroups>
        <VisualStateGroup x:Name="SizeStates">
            <VisualState x:Name="Compact">
                <VisualState.StateTriggers>
                    <AdaptiveTrigger MinWindowWidth="0"/>
                </VisualState.StateTriggers>
                <VisualState.Setters>
                    <Setter Target="MainPanel.Orientation" Value="Vertical"/>
                </VisualState.Setters>
            </VisualState>
            <VisualState x:Name="Medium">
                <VisualState.StateTriggers>
                    <AdaptiveTrigger MinWindowWidth="768"/>
                </VisualState.StateTriggers>
                <VisualState.Setters>
                    <Setter Target="MainPanel.Orientation" Value="Horizontal"/>
                </VisualState.Setters>
            </VisualState>
            <VisualState x:Name="Wide">
                <VisualState.StateTriggers>
                    <AdaptiveTrigger MinWindowWidth="1200"/>
                </VisualState.StateTriggers>
            </VisualState>
        </VisualStateGroup>
    </VisualStateManager.VisualStateGroups>
</Window>"#,
            "fluid" => r#"<!-- Fluid Layout with proportional sizing -->
<Grid RowDefinitions="Auto,*,Auto" ColumnDefinitions="*,2*,*">
    <Border Grid.ColumnSpan="3" Classes="header"/>
    <NavigationPanel Grid.Row="1" Grid.Column="0" MinWidth="200" MaxWidth="300"/>
    <ContentArea Grid.Row="1" Grid.Column="1"/>
    <Sidebar Grid.Row="1" Grid.Column="2" IsVisible="{Binding ShowSidebar}"/>
</Grid>"#,
            _ => r#"<!-- Hybrid: Combine adaptive triggers with fluid grids -->"#
        };

        let _touch_section = if include_touch {
            "\n## Touch Support\n```xml\n<!-- Touch-friendly sizing -->\n<Button MinWidth=\"48\" MinHeight=\"48\" Padding=\"16,12\"/>\n<!-- Pointer gesture support -->\n<Panel ManipulationMode=\"TranslateX,TranslateY,Scale\"/>"
        } else { "" };

        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Responsive Design: {}", layout_type))
            .heading(2, "Configuration")
            .task_list(vec![
                (true, format!("Layout: {}", layout_type)),
                (true, format!("Devices: {}", devices)),
                (true, format!("Touch Support: {}", include_touch)),
            ])
            .heading(2, "Breakpoints")
            .list(&[
                "Compact: 0-767px (mobile, single column)",
                "Medium: 768-1199px (tablet, two columns)",
                "Wide: 1200px+ (desktop, full layout)",
            ])
            .heading(2, "Implementation")
            .code_block("xml", breakpoints_xaml)
            .heading(2, "Avalonia Responsive Features")
            .list(&[
                "VisualStateManager for state-based layouts",
                "AdaptiveTrigger for breakpoint-based changes",
                "RelativePanel for flexible positioning",
                "Grid with star sizing for proportional layouts",
            ]);
        let builder = if include_touch { builder.heading(2, "Touch Support").list(&["Min 48x48 touch targets", "Manipulation gestures for pan/zoom", "PointerPressed/Released for custom gestures"]) } else { builder };
        let builder = builder.heading(2, "Best Practices").list(&["Test on actual target devices", "Use em/rem for scalable typography", "Avoid fixed widths where possible", "Design mobile-first, enhance for larger screens"]);
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Generates UX patterns for improved user experience and interaction design in AvaloniaUI applications")]
    pub async fn generate_ux_patterns(&self, params: UXPatternsParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let pattern = params.pattern_type.as_deref().unwrap_or("loading").to_lowercase();
        let include_examples = params.include_examples.unwrap_or(true);
        let code = match pattern.as_str() {
            "loading" => "<Panel>\n    <TextBlock Text=\"Loading...\" HorizontalAlignment=\"Center\" />\n    <ProgressBar IsIndeterminate=\"True\" Width=\"200\" />\n</Panel>",
            "empty_state" => "<StackPanel HorizontalAlignment=\"Center\">\n    <TextBlock Text=\"No items found\" />\n    <Button Content=\"Add Item\" />\n</StackPanel>",
            "confirmation" => "var result = await MessageBox.ShowDialog(\"Are you sure?\", \"Cannot be undone.\", MessageBoxType.YesNo);",
            _ => "// Unknown",
        };
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("UX Pattern: {}", pattern))
            .heading(2, "Code").code_block(if pattern == "confirmation" { "csharp" } else { "xml" }, code);
        if include_examples {
            builder = builder.heading(2, "Common Patterns").list(&["Loading states - Show progress", "Empty states - Guide users", "Confirmation dialogs - Prevent accidents", "Error handling - Clear messages"]);
        }
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Creates design systems with consistent visual components and guidelines for AvaloniaUI")]
    pub async fn generate_design_system(&self, params: DesignSystemParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.project_name.is_empty() { return Err(AvaloniaMcpError::validation("Project name cannot be empty")); }
        let p = &params.project_name;
        let include_tokens = params.include_tokens.unwrap_or(true);
        let tokens = if include_tokens { "\n## Tokens\n```xml\n<ResourceDictionary>\n    <Color x:Key=\"PrimaryColor\">#007ACC</Color>\n    <x:Double x:Key=\"FontSizeMedium\">14</x:Double>\n    <x:Double x:Key=\"SpacingMedium\">8</x:Double>\n</ResourceDictionary>\n```" } else { "" };
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Design System: {}", p))
            .heading(2, "Overview").paragraph(&format!("Design system for {}.", p));
        let builder = if include_tokens { builder.heading(2, "Tokens").code_block("xml", &tokens.replace("\n## Tokens\n```xml\n", "").replace("\n```", "")) } else { builder };
        let builder = builder.heading(2, "Guidelines").list(&["Use tokens for colors/spacing/typography", "Define light and dark variants", "Document component usage", "Maintain contrast ratios"])
            .heading(2, "Steps").list(&["1. Define tokens in App.axaml", "2. Create base styles", "3. Build component library", "4. Document guidelines"]);
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_provide_uiux_design() {
        let tool = UIUXDesignTool::new();
        let result = tool.provide_uiux_design(UIUXDesignParams { design_aspect: None, include_examples: Some(true) }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_ux_patterns() {
        let tool = UIUXDesignTool::new();
        let params = UXPatternsParams {
            pattern_type: Some("loading".to_string()),
            include_examples: Some(true),
        };
        let result = tool.generate_ux_patterns(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_design_system() {
        let tool = UIUXDesignTool::new();
        let params = DesignSystemParams {
            project_name: "MyApp".to_string(),
            include_tokens: Some(true),
        };
        let result = tool.generate_design_system(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_responsive_design() {
        let tool = UIUXDesignTool::new();
        let params = ResponsiveDesignParams {
            layout_type: Some("adaptive".to_string()),
            target_devices: Some("mobile,tablet,desktop".to_string()),
            include_touch: Some(true),
        };
        let result = tool.generate_responsive_design(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_responsive_design_defaults() {
        let tool = UIUXDesignTool::new();
        let params = ResponsiveDesignParams {
            layout_type: None,
            target_devices: None,
            include_touch: None,
        };
        let result = tool.generate_responsive_design(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
