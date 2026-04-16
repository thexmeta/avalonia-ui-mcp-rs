//! Debugging Assistant tool - Debugging guidance and patterns
use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DebuggingAssistantParams {
    pub issue_type: Option<String>,
    pub include_solutions: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DebugUtilitiesParams {
    pub utility_type: Option<String>,
    pub include_devtools: Option<bool>,
    pub include_telemetry: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct DebuggingAssistantTool;

impl DebuggingAssistantTool {
    pub fn new() -> Self { Self }

    #[tool(description = "Provide debugging assistance for AvaloniaUI applications. Covers common issues, debugging tools, breakpoints, and troubleshooting strategies.")]
    pub async fn provide_debugging_assistance(
        &self,
        params: DebuggingAssistantParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_solutions = params.include_solutions.unwrap_or(true);
        let issue_type = params.issue_type.as_deref().unwrap_or("common");

        let output = match issue_type {
            "binding" => self.debug_binding_issues(include_solutions),
            "layout" => self.debug_layout_issues(include_solutions),
            "performance" => self.debug_performance_issues(include_solutions),
            "memory" => self.debug_memory_issues(include_solutions),
            _ => self.debug_common_issues(include_solutions),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(description = "Generates debug utilities and logging helpers for AvaloniaUI applications")]
    pub async fn generate_debug_utilities(
        &self,
        params: DebugUtilitiesParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let utility_type = params.utility_type.as_deref().unwrap_or("logger").to_lowercase();
        let include_devtools = params.include_devtools.unwrap_or(true);
        let include_telemetry = params.include_telemetry.unwrap_or(false);

        let utility_code = match utility_type.as_str() {
            "logger" => r#"public static class DebugLogger
{
    [Conditional("DEBUG")]
    public static void Log(string message, [CallerMemberName] string member = "")
    {
        Console.WriteLine($"[{member}] {message}");
    }

    [Conditional("DEBUG")]
    public static void LogBinding(object source, string property, object value)
    {
        Log($"Binding: {source.GetType().Name}.{property} = {value}");
    }
}"#,
            "visualtree" => r#"public static class VisualTreeHelper
{
    public static void PrintVisualTree(Visual visual, int indent = 0)
    {
        var padding = new string(' ', indent * 2);
        Console.WriteLine($"{padding}{visual.GetType().Name}");
        foreach (var child in visual.GetVisualChildren())
        {
            PrintVisualTree(child, indent + 1);
        }
    }
}"#,
            "binding" => r#"public static class BindingDiagnostics
{
    public static void EnableBindingDebugging()
    {
        BindingLog.Enabled = true;
        BindingLog.Subscribe(log =>
        {
            Console.WriteLine($"Binding: {log.Message}");
        });
    }
}"#,
            _ => "// Unknown utility type",
        };

        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Debug Utilities: {}", utility_type))
            .heading(2, "Utility Implementation")
            .code_block("csharp", utility_code);

        if include_devtools {
            builder = builder
                .heading(2, "DevTools Integration")
                .code_block(
                    "csharp",
                    r#"// Enable DevTools in debug builds
#if DEBUG
app.SetupWithDevTools(options);
#endif

// Programmatic DevTools access
public static void ShowDevTools()
{
    if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
    {
        var window = desktop.Windows.FirstOrDefault(w => w is DevToolsWindow);
        window?.Show();
    }
}"#,
                );
        }

        if include_telemetry {
            builder = builder
                .heading(2, "Telemetry Integration")
                .code_block(
                    "csharp",
                    r#"public class AppTelemetry
{
    private readonly ILogger _logger;
    public AppTelemetry(ILogger logger) => _logger = logger;

    public void TrackEvent(string name, Dictionary<string, object>? properties = null)
    {
        _logger.LogInformation("Event: {Name} {@Properties}", name, properties);
    }
}"#,
                );
        }

        builder = builder
            .heading(2, "Setup Instructions")
            .list(vec![
                "Add utility classes to your project",
                "Register services in Program.cs",
                "Enable DevTools in debug builds only",
                "Use structured logging",
            ])
            .heading(2, "Usage Tips")
            .list(vec![
                "Enable DevTools in debug builds only",
                "Use structured logging for better analysis",
                "Monitor performance metrics in production",
            ]);

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    fn debug_common_issues(&self, include_solutions: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Debugging Common Issues")
            .paragraph("Troubleshooting guide for common AvaloniaUI issues.")
            .heading(2, "Debugging Tools")
            .list(vec![
                "Visual Studio / Rider debugger",
                "Avalonia DevTools (F12)",
                "Serilog with file output",
                "dotnet-trace for profiling",
            ]);

        if include_solutions {
            builder = builder
                .heading(2, "Binding Issues")
                .code_block("csharp", r#"// Enable binding debug output
AvaloniaLogging.LogLevel = LogLevel.Debug;
AvaloniaLogging.Sink = new ConsoleSink
{
    Area = LogArea.Binding
};

// Common binding errors:
// 1. Property not found: Check property name and INotifyPropertyChanged
// 2. Type mismatch: Ensure source and target types match
// 3. DataContext null: Set DataContext before bindings evaluate
// 4. Thread affinity: Update UI on UI thread

// Fix: Implement INotifyPropertyChanged properly
public class ViewModel : INotifyPropertyChanged
{
    private string _name;
    public string Name
    {
        get => _name;
        set
        {
            _name = value;
            OnPropertyChanged();
        }
    }
    
    public event PropertyChangedEventHandler PropertyChanged;
    protected virtual void OnPropertyChanged([CallerMemberName] string propertyName = null) =>
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
}"#)
                .heading(2, "Layout Issues")
                .code_block("csharp", r#"// Debug layout with DevTools
// Press F12 in running app

// Common layout issues:
// 1. Control not visible: Check Width/Height, Visibility, Parent container
// 2. Overlapping controls: Check Grid.Row/Column, ZIndex
// 3. Not filling space: Check Horizontal/VerticalAlignment
// 4. Infinite measure: Avoid SizeToContent in ScrollViewer

// Fix: Use layout debugging
AvaloniaLogging.LogLevel = LogLevel.Debug;
AvaloniaLogging.Sink = new ConsoleSink
{
    Area = LogArea.Layout
};"#)
                .heading(2, "Memory Leaks")
                .code_block("csharp", r#"// Common memory leak sources:
// 1. Event handlers not unsubscribed
// 2. Static collections holding references
// 3. Timers not stopped
// 4. Observables not disposed

// Fix: Use WeakEventManager for events
WeakEventManager<EventHandler>.AddHandler(
    button, 
    nameof(button.Click), 
    OnClick);

// Fix: Dispose subscriptions
private CompositeDisposable _disposables = new();

observable.Subscribe(x => /* ... */)
    .DisposeWith(_disposables);

// Dispose in OnClosed
protected override void OnClosed(EventArgs e)
{
    _disposables.Dispose();
    base.OnClosed(e);
}"#)
                .heading(2, "Performance Issues")
                .code_block("csharp", r#"// Profile with dotnet-trace
// dotnet-trace collect --process-id <PID>

// Common performance issues:
// 1. Too many visual elements: Use virtualization
// 2. Expensive bindings: Use x:CompileBindings
// 3. Layout thrashing: Avoid frequent size changes
// 4. Blocking UI thread: Use async/await

// Fix: Enable UI virtualization
<ListBox VirtualizationMode=\"Simple\">
    <ListBox.ItemsPanel>
        <ItemsPanelTemplate>
            <VirtualizingStackPanel/>
        </ItemsPanelTemplate>
    </ListBox.ItemsPanel>
</ListBox>"#);
        }

        builder.heading(2, "Debugging Checklist")
            .task_list(vec![
                (true, "Enable Avalonia DevTools"),
                (true, "Check binding errors in output"),
                (true, "Profile memory with dotnet-gcdump"),
                (true, "Use breakpoints in ViewModels"),
                (false, "Add logging for async operations"),
            ])
            .build()
    }

    fn debug_binding_issues(&self, _include_solutions: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Debugging Binding Issues")
            .paragraph("Troubleshoot data binding problems.")
            .heading(2, "Common Binding Errors")
            .list(vec![
                "Property not found on DataContext",
                "Type conversion failed",
                "Null reference in binding path",
                "Collection not notifying changes",
            ])
            .heading(2, "Solutions")
            .code_block("csharp", r#"// 1. Enable binding debug
AvaloniaLogging.LogLevel = LogLevel.Debug;

// 2. Check DataContext
Debug.WriteLine($\"DataContext: {DataContext?.GetType().Name}\");

// 3. Use compiled bindings for performance
<x:CompileBindings>true</x:CompileBindings>

// 4. Implement ObservableCollection for collections
public ObservableCollection<Item> Items { get; } = new();"#)
            .build()
    }

    fn debug_layout_issues(&self, _include_solutions: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Debugging Layout Issues")
            .paragraph("Troubleshoot layout and rendering problems.")
            .heading(2, "Common Layout Errors")
            .list(vec![
                "Control not visible",
                "Incorrect sizing",
                "Overlapping elements",
                "Infinite measure loop",
            ])
            .heading(2, "Solutions")
            .code_block("csharp", r#"// 1. Use DevTools (F12) to inspect visual tree
// 2. Check HorizontalAlignment and VerticalAlignment
// 3. Verify Grid.Row and Grid.Column attached properties
// 4. Avoid SizeToContent=\"WidthAndHeight\" with ScrollViewer

// Debug layout passes
AvaloniaLogging.Sink = new ConsoleSink { Area = LogArea.Layout };"#)
            .build()
    }

    fn debug_performance_issues(&self, _include_solutions: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Debugging Performance Issues")
            .paragraph("Identify and fix performance bottlenecks.")
            .heading(2, "Profiling Tools")
            .list(vec!["dotnet-trace", "dotnet-gcdump", "Avalonia DevTools", "Visual Studio Profiler"])
            .heading(2, "Common Issues")
            .list(vec!["UI thread blocking", "Excessive GC", "Layout thrashing", "No virtualization"])
            .build()
    }

    fn debug_memory_issues(&self, _include_solutions: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Debugging Memory Issues")
            .paragraph("Find and fix memory leaks.")
            .heading(2, "Memory Profiling")
            .code_block("bash", r#"# Collect memory dump
dotnet-gcdump collect --process-id <PID>

# Analyze
dotnet-dump analyze <dump>
> gcroot
> gcstat"#)
            .heading(2, "Common Leaks")
            .list(vec!["Event handlers", "Static collections", "Timers", "Observables"])
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_provide_debugging_assistance() {
        let tool = DebuggingAssistantTool::new();
        let result = tool.provide_debugging_assistance(DebuggingAssistantParams { issue_type: None, include_solutions: Some(true) }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_debug_utilities_logger() {
        let tool = DebuggingAssistantTool::new();
        let params = DebugUtilitiesParams {
            utility_type: Some("logger".to_string()),
            include_devtools: Some(true),
            include_telemetry: Some(false),
        };
        let result = tool.generate_debug_utilities(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
        assert!(result.content.len() > 0);
    }

    #[tokio::test]
    async fn test_generate_debug_utilities_visualtree() {
        let tool = DebuggingAssistantTool::new();
        let params = DebugUtilitiesParams {
            utility_type: Some("visualtree".to_string()),
            include_devtools: Some(false),
            include_telemetry: Some(false),
        };
        let result = tool.generate_debug_utilities(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_debug_utilities_with_telemetry() {
        let tool = DebuggingAssistantTool::new();
        let params = DebugUtilitiesParams {
            utility_type: Some("binding".to_string()),
            include_devtools: Some(true),
            include_telemetry: Some(true),
        };
        let result = tool.generate_debug_utilities(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_debug_utilities_unknown_type() {
        let tool = DebuggingAssistantTool::new();
        let params = DebugUtilitiesParams {
            utility_type: Some("unknown".to_string()),
            include_devtools: Some(false),
            include_telemetry: Some(false),
        };
        let result = tool.generate_debug_utilities(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_debug_utilities_defaults() {
        let tool = DebuggingAssistantTool::new();
        let params = DebugUtilitiesParams {
            utility_type: None,
            include_devtools: None,
            include_telemetry: None,
        };
        let result = tool.generate_debug_utilities(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
