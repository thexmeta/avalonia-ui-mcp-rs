//! Custom Control Generator tool - Custom control scaffolding
use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CustomControlGeneratorParams {
    pub control_type: Option<String>,
    pub control_name: Option<String>,
    pub include_styles: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ControlTemplateParams {
    pub target_control: String,
    pub template_name: String,
    pub visual_states: Option<String>,
    pub include_animations: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AttachedPropertyParams {
    pub property_name: String,
    pub property_type: Option<String>,
    pub target_controls: Option<String>,
    pub include_handler: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayoutPanelParams {
    pub panel_name: String,
    pub orientation: Option<String>,
    pub include_spacing: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct CustomControlGenerator;

impl CustomControlGenerator {
    pub fn new() -> Self { Self }

    #[tool(description = "Generate custom control templates for AvaloniaUI applications. Covers templated controls, composite controls, and attached properties.")]
    pub async fn generate_custom_control(
        &self,
        params: CustomControlGeneratorParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_styles = params.include_styles.unwrap_or(true);
        let control_type = params.control_type.as_deref().unwrap_or("templated");
        let control_name = params.control_name.as_deref().unwrap_or("CustomControl");

        let output = match control_type {
            "templated" => self.generate_templated_control(control_name, include_styles),
            "composite" => self.generate_composite_control(control_name, include_styles),
            "attached" => self.generate_attached_properties(control_name, include_styles),
            _ => self.generate_templated_control(control_name, include_styles),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    fn generate_templated_control(&self, name: &str, include_styles: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("{} - Templated Control", name))
            .paragraph("Custom templated control with theme support.")
            .heading(2, "Control Class")
            .code_block("csharp", &format!(r#"public class {} : TemplatedControl
{{
    public static readonly StyledProperty<string> LabelProperty =
        AvaloniaProperty.Register<{}, string>(nameof(Label));
    
    public static readonly StyledProperty<bool> IsCheckedProperty =
        AvaloniaProperty.Register<{}, bool>(nameof(IsChecked));
    
    public static readonly RoutedEvent<RoutedEventArgs> ClickedEvent =
        RoutedEvent.Register<{}, RoutedEventArgs>(nameof(Clicked), RoutingStrategies.Direct);
    
    static {}()
    {{
        PseudoClass.Register<{}>(\":checked\");
    }}
    
    public string Label
    {{
        get => GetValue(LabelProperty);
        set => SetValue(LabelProperty, value);
    }}
    
    public bool IsChecked
    {{
        get => GetValue(IsCheckedProperty);
        set => SetValue(IsCheckedProperty, value);
    }}
    
    public event EventHandler<RoutedEventArgs> Clicked
    {{
        add => AddHandler(ClickedEvent, value);
        remove => RemoveHandler(ClickedEvent, value);
    }}
    
    protected override void OnPropertyChanged(AvaloniaPropertyChangedEventArgs change)
    {{
        base.OnPropertyChanged(change);
        
        if (change.Property == IsCheckedProperty)
        {{
            PseudoClasses.Set(\":checked\", change.GetNewValue<bool>());
        }}
    }}
    
    protected override void OnPointerPressed(PointerPressedEventArgs e)
    {{
        base.OnPointerPressed(e);
        RaiseEvent(new RoutedEventArgs(ClickedEvent));
    }}
}}"#, name, name, name, name, name, name));

        if include_styles {
            builder = builder
                .heading(2, "Default Theme")
                .code_block("xml", &format!(r#"<!-- Themes/Generic.xaml -->
<Styles xmlns="https://github.com/avaloniaui"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml">
    <Style Selector="|{}\">
        <Setter Property=\"Background\" Value=\"{{DynamicResource SystemAccentColor}}\"/>
        <Setter Property=\"Foreground\" Value=\"White\"/>
        <Setter Property=\"Padding\" Value=\"8\"/>
        <Setter Property=\"Template\">
            <ControlTemplate>
                <Border Background=\"{{TemplateBinding Background}}\"
                        BorderBrush=\"{{TemplateBinding BorderBrush}}\"
                        BorderThickness=\"{{TemplateBinding BorderThickness}}\"
                        CornerRadius=\"4\"
                        Padding=\"{{TemplateBinding Padding}}\">
                    <StackPanel Orientation=\"Horizontal\">
                        <CheckBox IsChecked=\"{{TemplateBinding IsChecked}}\"/>
                        <TextBlock Text=\"{{TemplateBinding Label}}\"
                                   Margin=\"8,0,0,0\"/>
                    </StackPanel>
                </Border>
            </ControlTemplate>
        </Setter>
    </Style>
</Styles>"#, name));
        }

        builder.heading(2, "Usage")
            .code_block("xml", &format!(r#"<local:{} Label=\"Click Me\" IsChecked=\"{{Binding IsChecked}}\"
              Clicked=\"OnCustomControlClicked\"/>"#, name))
            .heading(2, "Best Practices")
            .task_list(vec![(true, "Use styled properties"), (true, "Support theming"), (true, "Implement proper events"), (true, "Add pseudo-classes for states"), (false, "Provide default style")])
            .build()
    }

    fn generate_composite_control(&self, name: &str, _include_styles: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, &format!("{} - Composite Control", name))
            .paragraph("Composite control combining existing controls.")
            .heading(2, "Control Class")
            .code_block("csharp", &format!(r#"public class {} : UserControl
{{
    public {}()
    {{
        InitializeComponent();
        this.AttachDevTools();
    }}
    
    private void InitializeComponent()
    {{
        var textBox = new TextBox
        {{
            [!TextBox.TextProperty] = this[!TextProperty]
        }};
        
        var button = new Button
        {{
            Content = \"Search\",
            [!Button.CommandProperty] = this[!SearchCommandProperty]
        }};
        
        var panel = new StackPanel
        {{
            Orientation = Orientation.Horizontal,
            Children = {{ textBox, button }}
        }};
        
        Content = panel;
    }}
    
    public static readonly StyledProperty<string> TextProperty =
        AvaloniaProperty.Register<{}, string>(nameof(Text));
    
    public string Text
    {{
        get => GetValue(TextProperty);
        set => SetValue(TextProperty, value);
    }}
    
    public static readonly StyledProperty<ICommand> SearchCommandProperty =
        AvaloniaProperty.Register<{}, ICommand>(nameof(SearchCommand));
    
    public ICommand SearchCommand
    {{
        get => GetValue(SearchCommandProperty);
        set => SetValue(SearchCommandProperty, value);
    }}
}}"#, name, name, name, name))
            .build()
    }

    fn generate_attached_properties(&self, _name: &str, _include_styles: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Attached Properties")
            .paragraph("Attached properties for extending control behavior.")
            .heading(2, "Implementation")
            .code_block("csharp", r#"public static class Behavior
{
    // Hover Behavior
    public static readonly AttachedProperty<bool> IsHoveredProperty =
        AvaloniaProperty.RegisterAttached<Control, bool>(\"IsHovered\");
    
    public static bool GetIsHovered(Control control) =>
        control.GetValue(IsHoveredProperty);
    
    public static void SetIsHovered(Control control, bool value) =>
        control.SetValue(IsHoveredProperty, value);
    
    // Auto-resize behavior
    public static readonly AttachedProperty<bool> AutoResizeProperty =
        AvaloniaProperty.RegisterAttached<Control, bool>(\"AutoResize\");
    
    static Behavior()
    {
        AutoResizeProperty.Changed.AddClassHandler<Control>(OnAutoResizeChanged);
    }
    
    private static void OnAutoResizeChanged(Control control, AvaloniaPropertyChangedEventArgs e)
    {
        if (e.GetNewValue<bool>())
        {
            control.GetObservable(Window.WindowStateProperty)
                .Subscribe(_ => control.InvalidateMeasure());
        }
    }
}"#)
            .heading(2, "Usage")
            .code_block("xml", r#"<StackPanel>
    <TextBlock Text=\"Hello\" local:Behavior.IsHovered=\"True\"/>
    <TextBox local:Behavior.AutoResize=\"True\"/>
</StackPanel>"#)
            .build()
    }

    #[tool(description = "Creates complex control templates with visual states and triggers for AvaloniaUI controls")]
    pub async fn generate_control_template(&self, params: ControlTemplateParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.target_control.is_empty() {
            return Err(AvaloniaMcpError::validation("Target control type cannot be empty"));
        }
        if params.template_name.is_empty() {
            return Err(AvaloniaMcpError::validation("Template name cannot be empty"));
        }
        let states_str = params.visual_states.as_deref().unwrap_or("Normal,PointerOver,Pressed,Disabled");
        let states: Vec<&str> = states_str.split(',').map(|s| s.trim()).collect();
        let include_anim = params.include_animations.unwrap_or(true);
        let states_xaml = states.iter().map(|s| {
            let anim = if include_anim { "\n                <Storyboard>\n                    <DoubleAnimation Storyboard.TargetProperty=\"Opacity\" To=\"1\" Duration=\"0:0:0.2\" />\n                </Storyboard>" } else { "" };
            format!("            <VisualState x:Name=\"{}\">{}\n            </VisualState>", s, anim)
        }).collect::<Vec<_>>().join("\n");
        let template = format!("<Style Selector=\"{target}\">\n    <Setter Property=\"Template\">\n        <ControlTemplate>\n            <Border x:Name=\"PART_Border\"\n                    Background=\"{{{{TemplateBinding Background}}}}\"\n                    BorderBrush=\"{{{{TemplateBinding BorderBrush}}}}\">\n                <ContentPresenter x:Name=\"PART_ContentPresenter\"\n                                  Content=\"{{{{TemplateBinding Content}}}}\"/>\n            </Border>\n        </ControlTemplate>\n    </Setter>\n</Style>\n\n<!-- Visual States -->\n<VisualStateGroup x:Name=\"CommonStates\">\n{states_xaml}\n</VisualStateGroup>", target = params.target_control, states_xaml = states_xaml);
        let state_descs: Vec<String> = states.iter().map(|s| {
            let d = match *s { "Normal" => "Default state", "PointerOver" => "Mouse hover", "Pressed" => "Clicked", "Disabled" => "Disabled", _ => "Custom" };
            format!("{}: {}", s, d)
        }).collect();
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Control Template: {} for {}", params.template_name, params.target_control))
            .heading(2, "Template").code_block("xml", &template)
            .heading(2, "States").list(&state_descs)
            .heading(2, "Parts").list(&["PART_Border", "PART_ContentPresenter"])
            .heading(2, "Usage").code_block("xml", &format!("<{t} Classes=\"{n}\" />", t = params.target_control, n = params.template_name));
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Generates attached properties for extending existing AvaloniaUI controls")]
    pub async fn generate_attached_property(&self, params: AttachedPropertyParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.property_name.is_empty() {
            return Err(AvaloniaMcpError::validation("Property name cannot be empty"));
        }
        let prop_type = params.property_type.as_deref().unwrap_or("bool");
        let include_handler = params.include_handler.unwrap_or(true);
        let targets = params.target_controls.as_deref().unwrap_or("Control");
        let rust_type = match prop_type { "string" => "string", "int" => "int", "double" => "double", "bool" => "bool", _ => "object" };
        let default_val = match prop_type { "string" => "\"\"", "int" => "0", "double" => "0.0", "bool" => "false", _ => "null" };
        let handler = if include_handler {
            format!("\n\n    private static void On{p}Changed(AvaloniaObject d, AvaloniaPropertyChangedEventArgs e)\n    {{\n        // Handle change\n    }}", p = params.property_name)
        } else { String::new() };
        let code = format!("public static class {p}Extensions\n{{\n    public static readonly AttachedProperty<{t}> {p}Property =\n        AvaloniaProperty.RegisterAttached<{targets}, {t}>(\n            \"{p}\", defaultValue: {default});\n\n    public static {t} Get{p}({targets} element) =>\n        element.GetValue({p}Property);\n\n    public static void Set{p}({targets} element, {t} value) =>\n        element.SetValue({p}Property, value);{handler}\n}}", p = params.property_name, t = rust_type, default = default_val, handler = handler);
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Attached Property: {}", params.property_name))
            .heading(2, "Definition").code_block("csharp", &code)
            .heading(2, "Usage").code_block("xml", &format!("<Button local:{p}=\"true\" />", p = params.property_name));
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Creates custom layout panels with arrangement logic for AvaloniaUI")]
    pub async fn generate_layout_panel(&self, params: LayoutPanelParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.panel_name.is_empty() {
            return Err(AvaloniaMcpError::validation("Panel name cannot be empty"));
        }
        let orientation = params.orientation.as_deref().unwrap_or("horizontal").to_lowercase();
        let include_spacing = params.include_spacing.unwrap_or(true);
        let orient_enum = match orientation.as_str() { "horizontal" => "Horizontal", "vertical" => "Vertical", "wrap" => "Wrap", _ => "Horizontal" };
        let spacing_prop = if include_spacing { "\n\n    public static readonly StyledProperty<double> SpacingProperty =\n        AvaloniaProperty.Register<PanelName, double>(nameof(Spacing), defaultValue: 4.0);\n    public double Spacing {\n        get => GetValue(SpacingProperty);\n        set => SetValue(SpacingProperty, value);\n    }" } else { "" };
        let code = format!("public class {panel} : Panel\n{{\n    public static readonly StyledProperty<Orientation> OrientationProperty =\n        AvaloniaProperty.Register<{panel}, Orientation>(\n            nameof(Orientation), defaultValue: Orientation.{orient});\n\n    public Orientation Orientation {{\n        get => GetValue(OrientationProperty);\n        set => SetValue(OrientationProperty, value);\n    }}{spacing}\n\n    protected override Size MeasureOverride(Size availableSize)\n    {{\n        double total = 0, max = 0;\n        foreach (var child in Children)\n        {{\n            child.Measure(availableSize);\n            if (Orientation == Orientation.Horizontal)\n            {{\n                total += child.DesiredSize.Width;\n                max = Math.Max(max, child.DesiredSize.Height);\n            }}\n            else\n            {{\n                total += child.DesiredSize.Height;\n                max = Math.Max(max, child.DesiredSize.Width);\n            }}\n        }}\n        return new Size(total, max);\n    }}\n\n    protected override Size ArrangeOverride(Size finalSize)\n    {{\n        var offset = 0.0;\n        foreach (var child in Children)\n        {{\n            if (Orientation == Orientation.Horizontal)\n            {{\n                child.Arrange(new Rect(offset, 0, child.DesiredSize.Width, finalSize.Height));\n                offset += child.DesiredSize.Width + Spacing;\n            }}\n            else\n            {{\n                child.Arrange(new Rect(0, offset, finalSize.Width, child.DesiredSize.Height));\n                offset += child.DesiredSize.Height + Spacing;\n            }}\n        }}\n        return finalSize;\n    }}\n}}", panel = params.panel_name, orient = orient_enum, spacing = spacing_prop);
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Custom Layout Panel: {}", params.panel_name))
            .heading(2, "Config").task_list(vec![(true, format!("Orientation: {}", orientation)), (true, format!("Spacing: {}", include_spacing))])
            .heading(2, "Code").code_block("csharp", &code)
            .heading(2, "Usage").code_block("xml", &format!("<local:{panel} Spacing=\"8\">\n    <Button Content=\"1\" />\n    <Button Content=\"2\" />\n</local:{panel}>", panel = params.panel_name));
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_generate_custom_control() {
        let tool = CustomControlGenerator::new();
        let result = tool.generate_custom_control(CustomControlGeneratorParams { control_type: None, control_name: None, include_styles: Some(true) }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_control_template() {
        let tool = CustomControlGenerator::new();
        let params = ControlTemplateParams {
            target_control: "Button".to_string(),
            template_name: "CustomButtonTemplate".to_string(),
            visual_states: None,
            include_animations: Some(true),
        };
        let result = tool.generate_control_template(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_attached_property() {
        let tool = CustomControlGenerator::new();
        let params = AttachedPropertyParams {
            property_name: "IsHighlightable".to_string(),
            property_type: Some("bool".to_string()),
            target_controls: Some("Button".to_string()),
            include_handler: Some(true),
        };
        let result = tool.generate_attached_property(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_layout_panel() {
        let tool = CustomControlGenerator::new();
        let params = LayoutPanelParams {
            panel_name: "CustomStackPanel".to_string(),
            orientation: Some("horizontal".to_string()),
            include_spacing: Some(true),
        };
        let result = tool.generate_layout_panel(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
