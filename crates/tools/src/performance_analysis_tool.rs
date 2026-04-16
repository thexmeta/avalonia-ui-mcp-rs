//! Performance analysis tool - Performance profiling guidance
//!
//! This tool provides performance analysis and optimization guidance for AvaloniaUI applications.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Performance analysis tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PerformanceAnalysisParams {
    /// Performance concern area (e.g., "startup", "rendering", "memory")
    pub area: Option<String>,
    /// Include profiling code examples
    pub include_profiling_code: Option<bool>,
    /// Application type (e.g., "desktop", "mobile")
    pub app_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PerformanceRecommendationsParams { pub area: Option<String>, pub include_code: Option<bool> }

/// Performance analysis tool for profiling and optimization
#[derive(Debug, Clone, Default)]
pub struct PerformanceAnalysisTool;

impl PerformanceAnalysisTool {
    /// Create a new PerformanceAnalysisTool instance
    pub fn new() -> Self {
        Self
    }

    /// Analyze performance and provide optimization guidance
    #[tool(description = "Analyze performance and provide optimization guidance for AvaloniaUI applications. Covers startup time, rendering performance, memory management, and profiling techniques.")]
    pub async fn analyze_performance(
        &self,
        params: PerformanceAnalysisParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let area = params.area.as_deref().unwrap_or("general");
        let include_profiling = params.include_profiling_code.unwrap_or(true);

        tracing::info!(area, "Analyzing performance");

        let output = match area {
            "startup" => self.analyze_startup_performance(include_profiling),
            "rendering" => self.analyze_rendering_performance(include_profiling),
            "memory" => self.analyze_memory_usage(include_profiling),
            "binding" => self.analyze_binding_performance(include_profiling),
            _ => self.analyze_general_performance(include_profiling),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Analyze startup performance
    fn analyze_startup_performance(&self, include_profiling: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Startup Performance Optimization")
            .paragraph("Optimize your AvaloniaUI application's startup time for better user experience.")
            .heading(2, "Common Bottlenecks")
            .list(vec![
                "Loading large resource dictionaries",
                "Initializing view models in constructors",
                "Synchronous I/O operations",
                "Loading images/assets synchronously",
                "Complex visual trees at startup",
            ])
            .heading(2, "Optimization Strategies");

        if include_profiling {
            builder = builder
                .heading(3, "Measure Startup Time")
                .code_block("csharp", r#"// Measure startup time with Stopwatch
public class App : Application
{
    private Stopwatch _startupStopwatch;
    
    public override void Initialize()
    {
        _startupStopwatch = Stopwatch.StartNew();
        
        // Your initialization code
        base.Initialize();
        
        _startupStopwatch.Stop();
        Logger.Information($"Startup time: {_startupStopwatch.ElapsedMilliseconds}ms");
    }
}"#)
                .heading(3, "Lazy Loading Resources")
                .code_block("csharp", r#"// Load resources lazily
public class MainWindow : Window
{
    private Lazy<ResourceDictionary> _themeResources;
    
    public MainWindow()
    {
        _themeResources = new Lazy<ResourceDictionary>(() => 
            new ResourceDictionary { Source = new Uri("avares://Themes/Default") });
        
        InitializeComponent();
        
        // Load heavy resources after window is shown
        this.Opened += async (s, e) => await LoadHeavyResourcesAsync();
    }
}"#)
                .heading(3, "Async Initialization")
                .code_block("csharp", r#"// Initialize data asynchronously
public class MainViewModel : ViewModelBase
{
    public MainViewModel()
    {
        // Don't block UI thread
        _ = InitializeDataAsync();
    }
    
    private async Task InitializeDataAsync()
    {
        Data = await _dataService.LoadDataAsync();
    }
}"#);
        }

        builder
            .heading(2, "Best Practices")
            .task_list(vec![
                (true, "Use splash screen for long startups"),
                (true, "Defer non-critical initialization"),
                (true, "Load assets asynchronously"),
                (false, "Enable AOT compilation"),
                (false, "Use source generators"),
            ])
            .heading(2, "Target Metrics")
            .table(
                vec!["Application Type", "Target Startup Time"],
                vec![
                    vec!["Simple desktop app", "< 500ms"],
                    vec!["Enterprise app", "< 2000ms"],
                    vec!["Data-heavy app", "< 5000ms"],
                ]
            )
            .build()
    }

    /// Analyze rendering performance
    fn analyze_rendering_performance(&self, include_profiling: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Rendering Performance Optimization")
            .paragraph("Optimize rendering for smooth 60 FPS user experience.")
            .heading(2, "Common Rendering Issues")
            .list(vec![
                "Complex visual trees with many nested controls",
                "Expensive effects and transparency",
                "Frequent layout invalidations",
                "Large images not properly scaled",
                "Animation without hardware acceleration",
            ])
            .heading(2, "Optimization Strategies");

        if include_profiling {
            builder = builder
                .heading(3, "Enable Rendering Debugging")
                .code_block("csharp", r#"// Enable rendering debugging
AppBuilder.Configure<App>()
    .UsePlatformDetect()
    .With(new Win32PlatformOptions { 
        AllowEglInitialization = true,
        UseWgl = true 
    })
    .LogToTrace(LogEventLevel.Verbose, "Avalonia.Rendering")
    .StartWithClassicDesktopLifetime(args);"#)
                .heading(3, "Optimize Visual Trees")
                .code_block("xml", r#"<!-- Avoid deeply nested layouts -->
<!-- Bad: Multiple nested panels -->
<Grid>
    <StackPanel>
        <DockPanel>
            <Grid>
                <!-- Content -->
            </Grid>
        </DockPanel>
    </StackPanel>
</Grid>

<!-- Good: Flatter structure -->
<Grid RowDefinitions="Auto,*">
    <StackPanel Grid.Row="0">
        <!-- Content -->
    </StackPanel>
    <ContentControl Grid.Row="1"/>
</Grid>"#)
                .heading(3, "Use Virtualization")
                .code_block("xml", r#"<!-- Enable virtualization for lists -->
<ListBox ItemsSource="{Binding Items}"
         VirtualizationMode="Simple">
    <ListBox.ItemsPanel>
        <ItemsPanelTemplate>
            <VirtualizingStackPanel/>
        </ItemsPanelTemplate>
    </ListBox.ItemsPanel>
</ListBox>"#);
        }

        builder
            .heading(2, "Performance Checklist")
            .task_list(vec![
                (true, "Enable UI virtualization"),
                (true, "Use appropriate image formats"),
                (true, "Minimize transparency usage"),
                (false, "Enable GPU acceleration"),
                (false, "Profile with Avalonia DevTools"),
            ])
            .heading(2, "Target FPS")
            .table(
                vec!["Scenario", "Target FPS"],
                vec![
                    vec!["Static UI", "60 FPS"],
                    vec!["Animations", "60 FPS"],
                    vec!["Scrolling", "60 FPS"],
                    vec!["Complex data viz", "30+ FPS"],
                ]
            )
            .build()
    }

    /// Analyze memory usage
    fn analyze_memory_usage(&self, include_profiling: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Memory Usage Optimization")
            .paragraph("Optimize memory usage to prevent leaks and reduce footprint.")
            .heading(2, "Common Memory Issues")
            .list(vec![
                "Event handler leaks (not unsubscribing)",
                "Static collections holding references",
                "Large bitmaps not disposed",
                "Memory leaks in bindings",
                "Unnecessary object allocations",
            ])
            .heading(2, "Optimization Strategies");

        if include_profiling {
            builder = builder
                .heading(3, "Profile Memory Usage")
                .code_block("csharp", r#"// Use dotnet-gcdump for memory profiling
// Install: dotnet tool install -g dotnet-gcdump
// Collect: dotnet-gcdump collect --process-id <pid>
// Analyze: Open .gcdump file in VS or PerfView

// Or use BenchmarkDotNet for allocations
[Benchmark]
public void TestMethod()
{
    // Your code here
}"#)
                .heading(3, "Prevent Event Leaks")
                .code_block("csharp", r#"// Use WeakEventManager to prevent leaks
// Bad: Strong reference
button.Click += OnButtonClick;

// Good: Weak reference
WeakEventManager<EventHandler>.AddHandler(
    button, 
    nameof(button.Click), 
    OnButtonClick);

// Or unsubscribe in Dispose
public void Dispose()
{
    button.Click -= OnButtonClick;
}"#)
                .heading(3, "Dispose Bitmaps")
                .code_block("csharp", r#"// Properly dispose bitmaps
public class ImageView : IDisposable
{
    private Bitmap? _bitmap;
    
    public void LoadImage(string path)
    {
        _bitmap?.Dispose();
        _bitmap = new Bitmap(path);
    }
    
    public void Dispose()
    {
        _bitmap?.Dispose();
        _bitmap = null;
    }
}"#);
        }

        builder
            .heading(2, "Memory Checklist")
            .task_list(vec![
                (true, "Unsubscribe from events"),
                (true, "Dispose bitmaps and streams"),
                (true, "Avoid static collections"),
                (false, "Use object pooling"),
                (false, "Profile regularly"),
            ])
            .heading(2, "Target Memory Usage")
            .table(
                vec!["Application Type", "Target RAM"],
                vec![
                    vec!["Simple app", "< 100 MB"],
                    vec!["Standard app", "< 200 MB"],
                    vec!["Complex app", "< 500 MB"],
                ]
            )
            .build()
    }

    /// Analyze binding performance
    fn analyze_binding_performance(&self, include_profiling: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Data Binding Performance")
            .paragraph("Optimize data binding for responsive UI.")
            .heading(2, "Common Binding Issues")
            .list(vec![
                "Too frequent INotifyPropertyChanged notifications",
                "Binding to complex objects",
                "Value converters with heavy logic",
                "Bindings in loops without virtualization",
                "Two-way bindings when one-way suffices",
            ])
            .heading(2, "Optimization Strategies");

        if include_profiling {
            builder = builder
                .heading(3, "Optimize INotifyPropertyChanged")
                .code_block("csharp", r#"// Use ReactiveUI or CommunityToolkit.Mvvm
// Batch property changes
public class ViewModel : ObservableObject
{
    [ObservableProperty]
    private string _name;
    
    // Or manual with batching
    public void UpdateMultipleProperties()
    {
        OnPropertyChanged(nameof(Property1));
        OnPropertyChanged(nameof(Property2));
        // Single notification for related changes
    }
}"#)
                .heading(3, "Efficient Value Converters")
                .code_block("csharp", r#"// Cache converter results when possible
public class CachedValueConverter : IValueConverter
{
    private readonly ConcurrentDictionary<object, object> _cache = new();
    
    public object Convert(object value, Type targetType, object parameter, CultureInfo culture)
    {
        return _cache.GetOrAdd(value, v => 
            // Expensive conversion logic
            ConvertValue(v));
    }
}"#);
        }

        builder
            .heading(2, "Binding Checklist")
            .task_list(vec![
                (true, "Use compiled bindings (x:CompileBindings)"),
                (true, "Prefer one-way bindings"),
                (true, "Cache converter results"),
                (false, "Use source generators"),
                (false, "Profile binding paths"),
            ])
            .build()
    }

    /// Analyze general performance
    fn analyze_general_performance(&self, include_profiling: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "General Performance Optimization")
            .paragraph("Comprehensive performance optimization guide for AvaloniaUI applications.")
            .heading(2, "Performance Categories")
            .list(vec![
                "Startup Time",
                "Rendering Performance",
                "Memory Management",
                "Data Binding",
                "I/O Operations",
                "Threading",
            ])
            .heading(2, "Quick Wins")
            .task_list(vec![
                (false, "Enable compiled bindings"),
                (false, "Add virtualization to lists"),
                (false, "Load data asynchronously"),
                (false, "Dispose resources properly"),
                (false, "Profile with Avalonia DevTools"),
            ]);

        if include_profiling {
            builder = builder
                .heading(2, "Profiling Tools")
                .list(vec![
                    "Avalonia DevTools (built-in)",
                    "dotnet-gcdump (memory)",
                    "dotnet-trace (CPU)",
                    "BenchmarkDotNet (micro-benchmarks)",
                    "Visual Studio Profiler",
                ])
                .heading(2, "Enable DevTools")
                .code_block("csharp", r#"// Enable Avalonia DevTools
#if DEBUG
AppBuilder.Configure<App>()
    .UsePlatformDetect()
    .LogToTrace()
    .StartWithClassicDesktopLifetime(args);

// Press F12 in running app to open DevTools
#endif"#);
        }

        builder
            .heading(2, "Performance Budget")
            .table(
                vec!["Metric", "Target", "Acceptable"],
                vec![
                    vec!["Startup Time", "500ms", "2000ms"],
                    vec!["Frame Time", "16ms (60 FPS)", "33ms (30 FPS)"],
                    vec!["Memory Usage", "150MB", "500MB"],
                    vec!["Input Latency", "50ms", "100ms"],
                ]
            )
            .build()
    }

    #[tool(description = "Provides performance optimization recommendations for AvaloniaUI applications across all categories")]
    pub async fn get_performance_recommendations(&self, params: PerformanceRecommendationsParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let area = params.area.as_deref().unwrap_or("all").to_lowercase();
        let include_code = params.include_code.unwrap_or(true);
        let recs = match area.as_str() {
            "bindings" => vec!["Use x:CompileBindings", "Prefer OneWay over TwoWay", "Cache converters", "Batch notifications", "Use source generators"],
            "rendering" => vec!["Enable UI virtualization", "Flatten visual trees", "GPU-accelerated animations", "Minimize transparency", "Pre-render complex visuals"],
            "memory" => vec!["Unsubscribe from events", "Dispose bitmaps", "Avoid static collections", "Use WeakEventManager", "Profile with dotnet-gcdump"],
            "startup" => vec!["Lazy load resources", "Async initialization", "Defer non-critical work", "Use splash screen", "Enable ReadyToRun"],
            _ => vec!["Enable compiled bindings", "Add virtualization", "Load data async", "Dispose resources", "Use DevTools", "Use source generators"],
        };
        let code = if include_code { "\n## Quick Wins\n```csharp\n// 1. Compiled bindings\n{x:CompileBindings True}\n\n// 2. Source generator\n[ObservableProperty] private string _name;\n\n// 3. Virtualization\n<ListBox VirtualizationMode=\"Simple\">\n    <ListBox.ItemsPanel>\n        <ItemsPanelTemplate><VirtualizingStackPanel/></ItemsPanelTemplate>\n    </ListBox.ItemsPanel>\n</ListBox>\n```" } else { "" };
        let builder = MarkdownOutputBuilder::new()
            .heading(1, "Performance Recommendations")
            .heading(2, &format!("Area: {}", area))
            .task_list(recs.iter().map(|r| (false, r.to_string())).collect::<Vec<_>>())
            .heading(2, "Budget").list(&["Startup: <500ms", "Frame: 16ms (60fps)", "Memory: <200MB", "Input: <50ms"]);
        let builder = if include_code { builder.heading(2, "Quick Wins").code_block("csharp", &code.replace("\n## Quick Wins\n```csharp\n", "").replace("\n```", "")) } else { builder };
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analyze_performance_startup() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceAnalysisParams {
            area: Some("startup".to_string()),
            include_profiling_code: Some(true),
            app_type: None,
        };

        let result = tool.analyze_performance(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_analyze_performance_general() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceAnalysisParams {
            area: None,
            include_profiling_code: Some(false),
            app_type: None,
        };

        let result = tool.analyze_performance(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_get_performance_recommendations() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceRecommendationsParams {
            area: Some("bindings".to_string()),
            include_code: Some(true),
        };
        let result = tool.get_performance_recommendations(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
