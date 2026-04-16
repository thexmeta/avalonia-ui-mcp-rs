//! Animation tool - Animation patterns and guidance
//!
//! This tool provides animation patterns and best practices for AvaloniaUI applications.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Animation tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnimationParams {
    /// Animation type (e.g., "fade", "slide", "scale", "rotate")
    pub animation_type: Option<String>,
    /// Include code examples
    pub include_examples: Option<bool>,
    /// Target platform (e.g., "desktop", "mobile")
    pub platform: Option<String>,
}

/// Page transition parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PageTransitionParams {
    /// Transition type: 'slide', 'fade', 'scale'
    pub transition_type: Option<String>,
    /// Direction: 'left', 'right', 'up', 'down'
    pub direction: Option<String>,
    /// Duration in milliseconds
    pub duration: Option<i32>,
}

/// Storyboard parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StoryboardParams {
    /// Animation sequence description
    pub sequence: String,
    /// Overall duration in milliseconds
    pub total_duration: Option<i32>,
    /// Storyboard name
    pub storyboard_name: Option<String>,
}

/// Custom animation parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CustomAnimationParams {
    /// Effect name
    pub effect_name: String,
    /// Properties to animate (comma-separated)
    pub properties: String,
    /// Animation pattern: 'wave', 'spiral', 'bounce', 'elastic', 'spring'
    pub pattern: Option<String>,
    /// Complexity: 'simple', 'moderate', 'complex'
    pub complexity: Option<String>,
}

/// Animation tool for generating animation patterns
#[derive(Debug, Clone, Default)]
pub struct AnimationTool;

impl AnimationTool {
    /// Create a new AnimationTool instance
    pub fn new() -> Self {
        Self
    }

    /// Generate animation patterns
    #[tool(description = "Generate animation patterns and XAML templates for AvaloniaUI applications. Covers fade, slide, scale, rotate animations with performance best practices.")]
    pub async fn generate_animation(
        &self,
        params: AnimationParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let animation_type = params.animation_type.as_deref().unwrap_or("all");
        let include_examples = params.include_examples.unwrap_or(true);

        tracing::info!(animation_type, "Generating animation patterns");

        let output = match animation_type {
            "fade" => self.generate_fade_animation(include_examples),
            "slide" => self.generate_slide_animation(include_examples),
            "scale" => self.generate_scale_animation(include_examples),
            "rotate" => self.generate_rotate_animation(include_examples),
            "transition" => self.generate_page_transitions(include_examples),
            _ => self.generate_all_animations(include_examples),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Generate fade animation
    fn generate_fade_animation(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Fade Animation Patterns")
            .paragraph("Smooth opacity transitions for showing/hiding elements.")
            .heading(2, "Use Cases")
            .list(vec![
                "Modal dialogs appearing/disappearing",
                "Content loading states",
                "Hover effects",
                "Notification toasts",
            ])
            .heading(2, "Implementation");

        if include_examples {
            builder = builder
                .heading(3, "Simple Fade In")
                .code_block("xml", r#"<Border Background="Blue" Opacity="0">
    <Border.Styles>
        <Style Selector="Border:hover">
            <Style.Animations>
                <Animation Duration="0:0:0.3" FillMode="Forward">
                    <KeyFrame Cue="0%">
                        <Setter Property="Opacity" Value="0"/>
                    </KeyFrame>
                    <KeyFrame Cue="100%">
                        <Setter Property="Opacity" Value="1"/>
                    </KeyFrame>
                </Animation>
            </Style.Animations>
        </Style>
    </Border.Styles>
</Border>"#)
                .heading(3, "Fade with KeyFrames")
                .code_block("xml", r#"<Window.Styles>
    <Style Selector="ContentControl.fade-in">
        <Style.Animations>
            <Animation Duration="0:0:0.5" Easing="CubicEaseOut">
                <KeyFrame Cue="0%">
                    <Setter Property="Opacity" Value="0"/>
                    <Setter Property="TranslateTransform.Y" Value="20"/>
                </KeyFrame>
                <KeyFrame Cue="70%">
                    <Setter Property="Opacity" Value="0.7"/>
                </KeyFrame>
                <KeyFrame Cue="100%">
                    <Setter Property="Opacity" Value="1"/>
                    <Setter Property="TranslateTransform.Y" Value="0"/>
                </KeyFrame>
            </Animation>
        </Style.Animations>
    </Style>
</Window.Styles>

<ContentControl Classes="fade-in" Content="Hello World"/>"#);
        }

        builder
            .heading(2, "Best Practices")
            .task_list(vec![
                (true, "Use 200-500ms duration for fades"),
                (true, "Apply easing for natural motion"),
                (true, "Combine with translate for depth"),
                (false, "Respect reduced motion settings"),
            ])
            .build()
    }

    /// Generate slide animation
    fn generate_slide_animation(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Slide Animation Patterns")
            .paragraph("Translate-based animations for smooth element movement.")
            .heading(2, "Use Cases")
            .list(vec![
                "Drawer/sliding panels",
                "List item additions/removals",
                "Page transitions",
                "Tooltip appearances",
            ])
            .heading(2, "Implementation");

        if include_examples {
            builder = builder
                .heading(3, "Slide In From Left")
                .code_block("xml", r#"<Panel>
    <Panel.Styles>
        <Style Selector="Panel.slide-in">
            <Style.Animations>
                <Animation Duration="0:0:0.4" Easing="QuadraticEaseOut">
                    <KeyFrame Cue="0%">
                        <Setter Property="TranslateTransform.X" Value="-300"/>
                        <Setter Property="Opacity" Value="0"/>
                    </KeyFrame>
                    <KeyFrame Cue="100%">
                        <Setter Property="TranslateTransform.X" Value="0"/>
                        <Setter Property="Opacity" Value="1"/>
                    </KeyFrame>
                </Animation>
            </Style.Animations>
        </Style>
    </Panel.Styles>
    
    <Panel Classes="slide-in">
        <!-- Content -->
    </Panel>
</Panel>"#)
                .heading(3, "Slide Up with Stagger")
                .code_block("xml", r#"<!-- Staggered animation for list items -->
<ItemsControl ItemsSource="{Binding Items}">
    <ItemsControl.ItemContainerTheme>
        <ResourceDictionary>
            <Style Selector="ContentPresenter">
                <Style.Animations>
                    <Animation Duration="0:0:0.3" Easing="CubicEaseOut">
                        <KeyFrame Cue="0%">
                            <Setter Property="Opacity" Value="0"/>
                            <Setter Property="TranslateTransform.Y" Value="50"/>
                        </KeyFrame>
                        <KeyFrame Cue="100%">
                            <Setter Property="Opacity" Value="1"/>
                            <Setter Property="TranslateTransform.Y" Value="0"/>
                        </KeyFrame>
                    </Animation>
                </Style.Animations>
            </Style>
        </ResourceDictionary>
    </ItemsControl.ItemContainerTheme>
</ItemsControl>"#);
        }

        builder
            .heading(2, "Best Practices")
            .task_list(vec![
                (true, "Use hardware-accelerated transforms"),
                (true, "Match slide distance to element size"),
                (true, "Combine with opacity for polish"),
                (false, "Add spring physics for bounce"),
            ])
            .build()
    }

    /// Generate scale animation
    fn generate_scale_animation(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Scale Animation Patterns")
            .paragraph("Size transformation animations for emphasis and feedback.")
            .heading(2, "Use Cases")
            .list(vec![
                "Button press feedback",
                "Card expansion",
                "Thumbnail to detail view",
                "Loading spinners",
            ])
            .heading(2, "Implementation");

        if include_examples {
            builder = builder
                .heading(3, "Button Press Scale")
                .code_block("xml", r#"<Button Content="Press Me">
    <Button.Styles>
        <Style Selector="Button:pressed">
            <Style.Animations>
                <Animation Duration="0:0:0.1">
                    <KeyFrame Cue="0%">
                        <Setter Property="ScaleTransform.ScaleX" Value="1"/>
                        <Setter Property="ScaleTransform.ScaleY" Value="1"/>
                    </KeyFrame>
                    <KeyFrame Cue="100%">
                        <Setter Property="ScaleTransform.ScaleX" Value="0.95"/>
                        <Setter Property="ScaleTransform.ScaleY" Value="0.95"/>
                    </KeyFrame>
                </Animation>
            </Style.Animations>
        </Style>
    </Button.Styles>
</Button>"#)
                .heading(3, "Pop-in Scale Effect")
                .code_block("xml", r#"<Border Background="White" BoxShadow="0 4 8px rgba(0,0,0,0.2)">
    <Border.Styles>
        <Style Selector="Border.pop-in">
            <Style.Animations>
                <Animation Duration="0:0:0.4" Easing="BackEaseOut">
                    <KeyFrame Cue="0%">
                        <Setter Property="ScaleTransform.ScaleX" Value="0"/>
                        <Setter Property="ScaleTransform.ScaleY" Value="0"/>
                        <Setter Property="Opacity" Value="0"/>
                    </KeyFrame>
                    <KeyFrame Cue="100%">
                        <Setter Property="ScaleTransform.ScaleX" Value="1"/>
                        <Setter Property="ScaleTransform.ScaleY" Value="1"/>
                        <Setter Property="Opacity" Value="1"/>
                    </KeyFrame>
                </Animation>
            </Style.Animations>
        </Style>
    </Border.Styles>
</Border>"#);
        }

        builder
            .heading(2, "Best Practices")
            .task_list(vec![
                (true, "Use BackEaseOut for playful pop"),
                (true, "Scale between 0.9-1.1 for subtlety"),
                (true, "Combine with opacity changes"),
                (false, "Add sound effects for feedback"),
            ])
            .build()
    }

    /// Generate rotate animation
    fn generate_rotate_animation(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Rotate Animation Patterns")
            .paragraph("Rotation animations for loading states and transitions.")
            .heading(2, "Use Cases")
            .list(vec![
                "Loading spinners",
                "Refresh indicators",
                "Expand/collapse indicators",
                "Directional cues",
            ])
            .heading(2, "Implementation");

        if include_examples {
            builder = builder
                .heading(3, "Loading Spinner")
                .code_block("xml", r#"<Panel Width="40" Height="40">
    <Panel.Styles>
        <Style Selector="Ellipse.spinner">
            <Style.Animations>
                <Animation Duration="0:0:1" IterationCount="Infinite">
                    <KeyFrame Cue="0%">
                        <Setter Property="RotateTransform.Angle" Value="0"/>
                    </KeyFrame>
                    <KeyFrame Cue="100%">
                        <Setter Property="RotateTransform.Angle" Value="360"/>
                    </KeyFrame>
                </Animation>
            </Style.Animations>
        </Style>
    </Panel.Styles>
    
    <Ellipse Classes="spinner" 
             Stroke="Blue" 
             StrokeThickness="4"
             RenderTransformOrigin="50% 50%"/>
</Panel>"#)
                .heading(3, "Expand Arrow Rotation")
                .code_block("xml", r#"<StackPanel Orientation="Horizontal">
    <StackPanel.Styles>
        <Style Selector="PathIcon.expanded">
            <Style.Animations>
                <Animation Duration="0:0:0.2" Easing="QuadraticEaseOut">
                    <KeyFrame Cue="0%">
                        <Setter Property="RotateTransform.Angle" Value="0"/>
                    </KeyFrame>
                    <KeyFrame Cue="100%">
                        <Setter Property="RotateTransform.Angle" Value="180"/>
                    </KeyFrame>
                </Animation>
            </Style.Animations>
        </Style>
    </StackPanel.Styles>
    
    <TextBlock Text="Details"/>
    <PathIcon Classes="expanded" Data="M0,0 L10,10 L20,0"/>
</StackPanel>"#);
        }

        builder
            .heading(2, "Best Practices")
            .task_list(vec![
                (true, "Use 1-2 second duration for spinners"),
                (true, "Apply easing for start/stop"),
                (true, "Set RenderTransformOrigin correctly"),
                (false, "Provide reduced motion alternative"),
            ])
            .build()
    }

    /// Generate page transitions
    fn generate_page_transitions(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Page Transition Patterns")
            .paragraph("Smooth transitions between pages and views.")
            .heading(2, "Transition Types")
            .list(vec![
                "Fade transition",
                "Slide transition",
                "Flip transition",
                "Zoom transition",
            ])
            .heading(2, "Implementation");

        if include_examples {
            builder = builder
                .heading(3, "Cross-fade Transition")
                .code_block("csharp", r#"// Implement in your view model or code-behind
public async Task NavigateWithFadeAsync(UserControl newPage)
{
    // Fade out current
    await this.FadeOutAsync(300);
    
    // Swap content
    ContentControl.Content = newPage;
    
    // Fade in new
    await this.FadeInAsync(300);
}

// Extension methods
public static class AnimationExtensions
{
    public static async Task FadeOutAsync(this Visual visual, int durationMs)
    {
        var animation = new Animation
        {
            Duration = TimeSpan.FromMilliseconds(durationMs),
            FillMode = FillMode.Forward,
            Children =
            {
                new KeyFrame
                {
                    Cue = new Cue(0),
                    Setters = { new Setter(OpacityProperty, 1) }
                },
                new KeyFrame
                {
                    Cue = new Cue(100),
                    Setters = { new Setter(OpacityProperty, 0) }
                }
            }
        };
        
        await animation.RunAsync(visual);
    }
}"#)
                .heading(3, "Slide Page Transition")
                .code_block("xml", r#"<!-- Define in App.xaml or Window resources -->
<TransitioningContentControl>
    <TransitioningContentControl.PageTransition>
        <SlidePageTransition Direction="Left"/>
    </TransitioningContentControl.PageTransition>
    
    <!-- Content changes will animate -->
    <ContentControl Content="{Binding CurrentPage}"/>
</TransitioningContentControl>"#);
        }

        builder
            .heading(2, "Best Practices")
            .task_list(vec![
                (true, "Keep transitions under 500ms"),
                (true, "Match transition to navigation context"),
                (true, "Maintain consistent direction"),
                (false, "Add gesture support for mobile"),
            ])
            .build()
    }

    /// Generate all animations overview
    fn generate_all_animations(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Animation Patterns Overview")
            .paragraph("Comprehensive guide to animations in AvaloniaUI applications.")
            .heading(2, "Animation Categories")
            .list(vec![
                "Fade - Opacity transitions",
                "Slide - Translate movements",
                "Scale - Size transformations",
                "Rotate - Rotation animations",
                "Page Transitions - View changes",
            ])
            .heading(2, "Performance Best Practices")
            .task_list(vec![
                (true, "Use RenderTransform for animations"),
                (true, "Enable GPU acceleration"),
                (true, "Avoid animating layout properties"),
                (true, "Use Composition API for complex animations"),
                (false, "Profile animation performance"),
            ])
            .heading(2, "Accessibility")
            .task_list(vec![
                (true, "Respect Windows reduced motion setting"),
                (true, "Provide non-animated alternatives"),
                (true, "Keep animations subtle and purposeful"),
                (false, "Allow users to disable animations"),
            ]);

        if include_examples {
            builder = builder
                .heading(2, "Quick Start Template")
                .code_block("xml", r#"<Window xmlns:anim="using:Avalonia.Animation">
    <Window.Styles>
        <!-- Define reusable animation style -->
        <Style Selector="Border.animated">
            <Style.Animations>
                <Animation Duration="0:0:0.3" Easing="CubicEaseOut">
                    <KeyFrame Cue="0%">
                        <Setter Property="Opacity" Value="0"/>
                        <Setter Property="TranslateTransform.Y" Value="20"/>
                    </KeyFrame>
                    <KeyFrame Cue="100%">
                        <Setter Property="Opacity" Value="1"/>
                        <Setter Property="TranslateTransform.Y" Value="0"/>
                    </KeyFrame>
                </Animation>
            </Style.Animations>
        </Style>
    </Window.Styles>
    
    <!-- Apply animation -->
    <Border Classes="animated" Background="Blue">
        <TextBlock Text="Animated Content" Foreground="White"/>
    </Border>
</Window>"#)
                .heading(2, "Easing Functions Reference")
                .table(
                    vec!["Easing", "Use Case", "Feel"],
                    vec![
                        vec!["Linear", "Progress bars", "Mechanical"],
                        vec!["CubicEaseOut", "General UI", "Natural"],
                        vec!["QuadraticEaseIn", "Accelerating", "Building"],
                        vec!["BackEaseOut", "Pop effects", "Playful"],
                        vec!["ElasticEaseOut", "Bounce effects", "Energetic"],
                    ]
                );
        }

        builder
            .heading(2, "Animation Duration Guidelines")
            .table(
                vec!["Animation Type", "Duration Range", "Purpose"],
                vec![
                    vec!["Micro-interactions", "100-200ms", "Feedback"],
                    vec!["State changes", "200-300ms", "Transitions"],
                    vec!["Page transitions", "300-500ms", "Navigation"],
                    vec!["Loading spinners", "1000-2000ms", "Continuous"],
                ]
            )
            .build()
    }

    #[tool(description = "Creates page transitions for navigation between views in AvaloniaUI applications")]
    pub async fn generate_page_transition(
        &self,
        params: PageTransitionParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let transition_type = params.transition_type.as_deref().unwrap_or("slide").to_lowercase();
        let direction = params.direction.as_deref().unwrap_or("left").to_lowercase();
        let duration = params.duration.unwrap_or(350);

        tracing::info!(%transition_type, %direction, duration, "Generating page transition");

        let orientation = match direction.as_str() {
            "left" | "right" => "Horizontal",
            "up" | "down" => "Vertical",
            _ => "Horizontal",
        };

        let transition_xaml = match transition_type.as_str() {
            "slide" => format!(
                r#"<UserControl.PageTransition>
    <PageSlide Duration="0:0:{:.3}" Orientation="{}" />
</UserControl.PageTransition>"#,
                duration as f64 / 1000.0,
                orientation
            ),
            "fade" => format!(
                r#"<UserControl.PageTransition>
    <CrossFade Duration="0:0:{:.3}" />
</UserControl.PageTransition>"#,
                duration as f64 / 1000.0
            ),
            _ => format!(
                r#"<UserControl.PageTransition>
    <PageSlide Duration="0:0:{:.3}" Orientation="Horizontal" />
</UserControl.PageTransition>"#,
                duration as f64 / 1000.0
            ),
        };

        let transition_class = match transition_type.as_str() {
            "slide" => "PageSlide",
            "fade" => "CrossFade",
            _ => "PageSlide",
        };

        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Page Transition: {} ({})", transition_type, direction))
            .heading(2, "Configuration")
            .task_list(vec![
                (true, format!("Type: {}", transition_type)),
                (true, format!("Direction: {}", direction)),
                (true, format!("Duration: {}ms", duration)),
            ])
            .heading(2, "XAML Implementation")
            .code_block("xml", &transition_xaml)
            .heading(2, "Navigation Integration")
            .code_block("csharp", &format!(
                r#"// Apply transition in ViewLocator
public class ViewLocator : IDataTemplate
{{
    public Control Build(object data)
    {{
        var name = data.GetType().FullName!.Replace("ViewModel", "View");
        var type = Type.GetType(name);

        if (type != null)
        {{
            var control = (Control)Activator.CreateInstance(type)!;
            if (control is UserControl userControl)
            {{
                userControl.PageTransition = new {transition_class}
                {{
                    Duration = TimeSpan.FromMilliseconds({duration})
                }};
            }}
            return control;
        }}

        return new TextBlock {{ Text = "Not Found: " + name }};
    }}

    public bool Match(object data) => data is ViewModelBase;
}}"#
            ))
            .heading(2, "Available Transition Types")
            .list(&[
                "slide - Horizontal or vertical slide transition",
                "fade - Crossfade transition between pages",
                "scale - Scale and fade combination",
            ])
            .heading(2, "Integration Tips")
            .list(&[
                "Set PageTransition on UserControl for navigation",
                "Use ViewModels that implement ViewModelBase",
                "Consider using Router for navigation patterns",
                "Test transitions on target devices for performance",
            ]);

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Creates sophisticated storyboard animations with multiple properties and timing for AvaloniaUI")]
    pub async fn generate_storyboard(
        &self,
        params: StoryboardParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.sequence.is_empty() {
            return Err(AvaloniaMcpError::validation(
                "Animation sequence description cannot be empty",
            ));
        }

        let total_duration = params.total_duration.unwrap_or(1000);
        let storyboard_name = params.storyboard_name.as_deref().unwrap_or("MainStoryboard");

        tracing::info!(%storyboard_name, total_duration, "Generating storyboard animation");

        let storyboard_xaml = format!(
            "<Window.Resources>\n    <Storyboard x:Key=\"{name}\">\n        <!-- Animation sequence: {seq} -->\n        <DoubleAnimation Storyboard.TargetProperty=\"Opacity\"\n                         From=\"0\" To=\"1\"\n                         Duration=\"0:0:{:.3}\"\n                         BeginTime=\"0:0:0\" />\n        <DoubleAnimation Storyboard.TargetProperty=\"(TranslateTransform.Y)\"\n                         From=\"20\" To=\"0\"\n                         Duration=\"0:0:{:.3}\"\n                         BeginTime=\"0:0:0.2\" />\n    </Storyboard>\n</Window.Resources>",
            total_duration as f64 / 1000.0 * 0.3,
            total_duration as f64 / 1000.0 * 0.5,
            name = storyboard_name,
            seq = params.sequence,
        );

        let triggers_xaml = format!(
            "<Control.Triggers>\n    <EventTrigger Routes=\"Control.LoadedEvent\">\n        <BeginStoryboard>\n            <Storyboard BeginStoryboardName=\"{name}\" />\n        </BeginStoryboard>\n    </EventTrigger>\n</Control.Triggers>",
            name = storyboard_name
        );

        let code_control = format!(
            "// Programmatic control of storyboard\nvar storyboard = this.FindResource<Storyboard>(\"{name}\");\nif (storyboard != null)\n{{\n    // Start animation\n    await storyboard.RunAsync();\n    \n    // Or pause/resume\n    // storyboard.Pause();\n    // storyboard.Resume();\n    \n    // Or stop\n    // storyboard.Stop();\n}}",
            name = storyboard_name
        );

        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Complex Storyboard Animation: {}", storyboard_name))
            .heading(2, "Animation Sequence")
            .paragraph(&params.sequence)
            .heading(2, "Storyboard Definition")
            .code_block("xml", &storyboard_xaml)
            .heading(2, "Event Triggers")
            .code_block("xml", &triggers_xaml)
            .heading(2, "Programmatic Control")
            .code_block("csharp", &code_control)
            .heading(2, "Advanced Storyboard Techniques")
            .list(&[
                "Sequential Animations - Use BeginTime to stagger animations",
                "Parallel Animations - Animate multiple properties simultaneously",
                "Keyframe Animations - Use DoubleAnimationUsingKeyFrames for complex paths",
                "Easing Functions - Apply CubicBezierEasing for natural motion",
            ])
            .heading(2, "Performance Tips")
            .list(&[
                "Use Transform properties for GPU acceleration",
                "Batch animations in single storyboard when possible",
                "Test on target devices for performance validation",
                "Avoid animating layout properties (Width, Height, Margin)",
            ]);

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Generates custom easing functions and advanced animation effects for AvaloniaUI")]
    pub async fn generate_custom_animation(
        &self,
        params: CustomAnimationParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.effect_name.is_empty() {
            return Err(AvaloniaMcpError::validation(
                "Effect name cannot be empty",
            ));
        }
        if params.properties.is_empty() {
            return Err(AvaloniaMcpError::validation(
                "Properties to animate cannot be empty",
            ));
        }

        let pattern = params.pattern.as_deref().unwrap_or("wave").to_lowercase();
        let complexity = params.complexity.as_deref().unwrap_or("moderate").to_lowercase();
        let property_list: Vec<&str> = params.properties.split(',').map(|p| p.trim()).collect();

        tracing::info!(effect_name = %params.effect_name, ?property_list, %pattern, "Generating custom animation");

        let properties_xaml = property_list.iter().map(|p| {
            format!(r#"                <Setter Property="{}" Value="0" />"#, p)
        }).collect::<Vec<_>>().join("\n");

        let custom_animation_xaml = format!(
            "<UserControl.Styles>\n    <Style Selector=\"#{ename}\">\n        <Style.Animations>\n            <Animation Duration=\"0:0:0.5\" IterationCount=\"Infinite\">\n                <!-- Pattern: {pat} | Complexity: {comp} -->\n                <KeyFrame Cue=\"0%\">\n{properties_xaml}\n                </KeyFrame>\n                <KeyFrame Cue=\"50%\">\n                    <!-- Mid-point values -->\n                </KeyFrame>\n                <KeyFrame Cue=\"100%\">\n                    <!-- End-point values -->\n                </KeyFrame>\n            </Animation>\n        </Style.Animations>\n    </Style>\n</UserControl.Styles>",
            ename = params.effect_name,
            pat = pattern,
            comp = complexity,
        );

        let easing_functions = format!(
            "// Custom easing functions for {ename}\npublic static class CustomEasings\n{{\n    // Spring easing with configurable tension/friction\n    public static IEasing Spring(double tension = 300, double friction = 10)\n    {{\n        return new CubicBezierEasing(0.68, -0.55, 0.265, 1.55);\n    }}\n\n    // Bounce easing with configurable bounces\n    public static IEasing Bounce(int bounces = 3)\n    {{\n        return new CubicBezierEasing(0.68, -0.55, 0.265, 1.55);\n    }}\n\n    // Elastic easing with configurable elasticity\n    public static IEasing Elastic(double amplitude = 1.0, double period = 0.3)\n    {{\n        return new CubicBezierEasing(0.68, -0.55, 0.265, 1.55);\n    }}\n}}",
            ename = params.effect_name
        );

        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Custom Animation Effect: {}", params.effect_name))
            .heading(2, "Configuration")
            .task_list(vec![
                (true, format!("Properties: {}", params.properties)),
                (true, format!("Pattern: {}", pattern)),
                (true, format!("Complexity: {}", complexity)),
            ])
            .heading(2, "Custom Animation XAML")
            .code_block("xml", &custom_animation_xaml)
            .heading(2, "Custom Easing Functions")
            .code_block("csharp", &easing_functions)
            .heading(2, "Advanced Animation Techniques")
            .heading(3, "Physics-Based Animations")
            .code_block("csharp", r#"public class SpringAnimation
{
    public static IAnimation CreateSpring(double tension = 300, double friction = 10)
    {
        return new Animation {
            Duration = TimeSpan.FromMilliseconds(500),
            Easing = new CubicBezierEasing(0.68, -0.55, 0.265, 1.55),
            FillMode = FillMode.Forward
        };
    }
}"#)
            .heading(2, "Performance Considerations")
            .list(&[
                "GPU Acceleration: Use Transform properties when possible",
                "Memory Management: Dispose of long-running animations",
                "Frame Rate: Target 60fps for smooth animations",
                "Battery Impact: Consider device battery on mobile platforms",
            ]);

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_animation_fade() {
        let tool = AnimationTool::new();
        let params = AnimationParams {
            animation_type: Some("fade".to_string()),
            include_examples: Some(true),
            platform: None,
        };

        let result = tool.generate_animation(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_animation_all() {
        let tool = AnimationTool::new();
        let params = AnimationParams {
            animation_type: None,
            include_examples: Some(false),
            platform: None,
        };

        let result = tool.generate_animation(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_page_transition_slide() {
        let tool = AnimationTool::new();
        let params = PageTransitionParams {
            transition_type: Some("slide".to_string()),
            direction: Some("left".to_string()),
            duration: Some(400),
        };

        let result = tool.generate_page_transition(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_page_transition_fade() {
        let tool = AnimationTool::new();
        let params = PageTransitionParams {
            transition_type: Some("fade".to_string()),
            direction: Some("up".to_string()),
            duration: Some(300),
        };

        let result = tool.generate_page_transition(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_page_transition_defaults() {
        let tool = AnimationTool::new();
        let params = PageTransitionParams {
            transition_type: None,
            direction: None,
            duration: None,
        };

        let result = tool.generate_page_transition(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_storyboard_success() {
        let tool = AnimationTool::new();
        let params = StoryboardParams {
            sequence: "Fade in, then slide up".to_string(),
            total_duration: Some(1500),
            storyboard_name: Some("IntroAnimation".to_string()),
        };

        let result = tool.generate_storyboard(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_storyboard_defaults() {
        let tool = AnimationTool::new();
        let params = StoryboardParams {
            sequence: "Simple entrance animation".to_string(),
            total_duration: None,
            storyboard_name: None,
        };

        let result = tool.generate_storyboard(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_storyboard_empty_sequence() {
        let tool = AnimationTool::new();
        let params = StoryboardParams {
            sequence: "".to_string(),
            total_duration: None,
            storyboard_name: None,
        };

        let result = tool.generate_storyboard(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_custom_animation_success() {
        let tool = AnimationTool::new();
        let params = CustomAnimationParams {
            effect_name: "WaveEffect".to_string(),
            properties: "Opacity, TranslateTransform.Y".to_string(),
            pattern: Some("wave".to_string()),
            complexity: Some("moderate".to_string()),
        };

        let result = tool.generate_custom_animation(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_custom_animation_defaults() {
        let tool = AnimationTool::new();
        let params = CustomAnimationParams {
            effect_name: "BounceEffect".to_string(),
            properties: "ScaleTransform.ScaleX, ScaleTransform.ScaleY".to_string(),
            pattern: None,
            complexity: None,
        };

        let result = tool.generate_custom_animation(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_custom_animation_empty_name() {
        let tool = AnimationTool::new();
        let params = CustomAnimationParams {
            effect_name: "".to_string(),
            properties: "Opacity".to_string(),
            pattern: None,
            complexity: None,
        };

        let result = tool.generate_custom_animation(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_custom_animation_empty_properties() {
        let tool = AnimationTool::new();
        let params = CustomAnimationParams {
            effect_name: "TestEffect".to_string(),
            properties: "".to_string(),
            pattern: None,
            complexity: None,
        };

        let result = tool.generate_custom_animation(params).await;
        assert!(result.is_err());
    }
}
