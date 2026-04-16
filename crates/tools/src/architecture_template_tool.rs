//! Architecture Template tool - Clean Architecture patterns
use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArchitectureTemplateParams {
    pub pattern: Option<String>,
    pub include_examples: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MicroservicesParams { pub app_name: String, pub services: Option<String>, pub include_gateway: Option<bool> }
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DDDParams { pub domain_name: String, pub bounded_contexts: Option<String>, pub include_cqrs: Option<bool> }
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PluginParams { pub app_name: String, pub plugin_types: Option<String>, pub include_hot_reload: Option<bool> }

#[derive(Debug, Clone, Default)]
pub struct ArchitectureTemplateTool;

impl ArchitectureTemplateTool {
    pub fn new() -> Self { Self }

    #[tool(description = "Generate Clean Architecture templates for AvaloniaUI applications. Covers MVVM, layered architecture, dependency injection, and project structure.")]
    pub async fn generate_architecture_template(
        &self,
        params: ArchitectureTemplateParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_examples = params.include_examples.unwrap_or(true);
        let pattern = params.pattern.as_deref().unwrap_or("mvvm");

        let output = match pattern {
            "mvvm" => self.generate_mvvm(include_examples),
            "clean" => self.generate_clean_architecture(include_examples),
            "layered" => self.generate_layered(include_examples),
            _ => self.generate_mvvm(include_examples),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    fn generate_mvvm(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "MVVM Architecture Template")
            .paragraph("Model-View-ViewModel pattern for AvaloniaUI applications.")
            .heading(2, "Project Structure")
            .code_block("text", r#"/src
    /YourApp
        /Models
        /ViewModels
        /Views
        /Services
        /Converters
        App.axaml
        Program.cs"#);

        if include_examples {
            builder = builder
                .heading(2, "ViewModel Base")
                .code_block("csharp", r#"public class ViewModelBase : ReactiveObject
{
    private bool _isLoading;
    public bool IsLoading
    {
        get => _isLoading;
        set => this.RaiseAndSetIfChanged(ref _isLoading, value);
    }
}

public class MainViewModel : ViewModelBase
{
    private readonly IDataService _dataService;
    private string _title;
    
    public MainViewModel(IDataService dataService)
    {
        _dataService = dataService;
        LoadDataCommand = ReactiveCommand.CreateFromTask(LoadDataAsync);
    }
    
    public string Title
    {
        get => _title;
        set => this.RaiseAndSetIfChanged(ref _title, value);
    }
    
    public ReactiveCommand<Unit, Unit> LoadDataCommand { get; }
    
    private async Task LoadDataAsync()
    {
        IsLoading = true;
        try
        {
            var data = await _dataService.GetDataAsync();
            Title = data.Title;
        }
        finally
        {
            IsLoading = false;
        }
    }
}"#)
                .heading(2, "View Code-Behind")
                .code_block("csharp", r#"public partial class MainWindow : Window
{
    public MainWindow(MainViewModel viewModel)
    {
        InitializeComponent();
        DataContext = viewModel;
    }
}"#)
                .heading(2, "Dependency Injection")
                .code_block("csharp", r#"// Program.cs
var builder = AppBuilder.Configure<App>()
    .UsePlatformDetect()
    .LogToTrace();

// Setup DI
var services = new ServiceCollection();
services.AddSingleton<MainWindow>();
services.AddSingleton<MainViewModel>();
services.AddSingleton<IDataService, DataService>();
var serviceProvider = services.BuildServiceProvider();

var mainWindow = serviceProvider.GetRequiredService<MainWindow>();
builder.StartWithClassicDesktopLifetime(mainWindow);"#);
        }

        builder.heading(2, "MVVM Best Practices")
            .task_list(vec![(true, "ViewModels never reference Views"), (true, "Use commands for actions"), (true, "Implement INotifyPropertyChanged"), (true, "Inject services via constructor"), (false, "Use weak events for subscriptions")])
            .build()
    }

    fn generate_clean_architecture(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Clean Architecture")
            .paragraph("Clean Architecture with domain-centric design.")
            .heading(2, "Layers")
            .code_block("text", r#"/src
    /Domain          (Entities, Value Objects, Interfaces)
    /Application     (Use Cases, DTOs, Interfaces)
    /Infrastructure  (Implementations, EF Core, External APIs)
    /Presentation    (AvaloniaUI, ViewModels, Views)"#)
            .heading(2, "Dependencies")
            .code_block("text", "Presentation → Application → Domain\nInfrastructure → Application\n(Dependencies point inward)")
            .build()
    }

    fn generate_layered(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Layered Architecture")
            .paragraph("Traditional N-tier layered architecture.")
            .heading(2, "Layers")
            .list(vec!["Presentation Layer (UI)", "Business Logic Layer", "Data Access Layer", "Database"])
            .heading(2, "Project Structure")
            .code_block("text", r#"/YourApp.sln
    /YourApp.UI         (AvaloniaUI)
    /YourApp.Business   (Business logic)
    /YourApp.Data       (Data access)
    /YourApp.Entities   (Domain models)"#)
            .build()
    }

    #[tool(description = "Creates microservices architecture templates for distributed AvaloniaUI applications")]
    pub async fn generate_microservices_architecture(&self, params: MicroservicesParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.app_name.is_empty() { return Err(AvaloniaMcpError::validation("App name cannot be empty")); }
        let services = params.services.as_deref().unwrap_or("user,order,inventory");
        let include_gateway = params.include_gateway.unwrap_or(true);
        let svc_list: Vec<&str> = services.split(',').map(|s| s.trim()).collect();
        let a = &params.app_name;
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Microservices: {}", a))
            .heading(2, "Services").list(&svc_list.iter().map(|s| format!("{} Service", s)).collect::<Vec<_>>())
            .heading(2, "Structure").code_block("text", &format!("{}/\n├── src/\n{}", a, svc_list.iter().map(|s| format!("├── {}/", s)).collect::<Vec<_>>().join("\n")))
            .heading(2, "Communication").list(&["REST API between services", "Message bus for events", "gRPC for performance"]);
        let builder = if include_gateway { builder.heading(2, "API Gateway").code_block("csharp", "// Ocelot configuration\n{ \"Routes\": [{ \"DownstreamPathTemplate\": \"/api/users\" }] }") } else { builder };
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Generates Domain-Driven Design (DDD) templates with bounded contexts and aggregates")]
    pub async fn generate_ddd_architecture(&self, params: DDDParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.domain_name.is_empty() { return Err(AvaloniaMcpError::validation("Domain name cannot be empty")); }
        let contexts_str = params.bounded_contexts.as_deref().unwrap_or("sales,inventory,shipping");
        let include_cqrs = params.include_cqrs.unwrap_or(true);
        let contexts: Vec<&str> = contexts_str.split(',').map(|s| s.trim()).collect();
        let d = &params.domain_name;
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("DDD: {}", d))
            .heading(2, "Bounded Contexts").list(&contexts.iter().map(|c| format!("{} Context", c)).collect::<Vec<_>>())
            .heading(2, "Aggregate").code_block("csharp", &format!("public class Order : Entity\n{{\n    public OrderId Id {{ get; private set; }}\n    public List<OrderItem> Items {{ get; private set; }}\n    public void AddItem(Product p, int qty) => Items.Add(new OrderItem(p, qty));\n}}"))
            .heading(2, "Strategic Design").list(&["Ubiquitous language per context", "Anti-corruption layers", "Context mapping", "Event-driven integration"]);
        let builder = if include_cqrs { builder.heading(2, "CQRS").code_block("csharp", "public class CreateOrderCommand : IRequest<int> { public string CustomerId { get; set; } }\npublic class GetOrderQuery : IRequest<OrderDto> { public int OrderId { get; set; } }") } else { builder };
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Creates plugin architecture templates with extensible module system")]
    pub async fn generate_plugin_architecture(&self, params: PluginParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.app_name.is_empty() { return Err(AvaloniaMcpError::validation("App name cannot be empty")); }
        let plugin_types = params.plugin_types.as_deref().unwrap_or("editor,filter,exporter");
        let include_hot_reload = params.include_hot_reload.unwrap_or(false);
        let types: Vec<&str> = plugin_types.split(',').map(|s| s.trim()).collect();
        let a = &params.app_name;
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Plugin Architecture: {}", a))
            .heading(2, "Plugin Types").list(&types.iter().map(|t| format!("{} Plugin", t)).collect::<Vec<_>>())
            .heading(2, "Interface").code_block("csharp", "public interface IPlugin\n{\n    string Name { get; }\n    string Version { get; }\n    Task InitializeAsync(IPluginContext ctx);\n    Task ShutdownAsync();\n}")
            .heading(2, "Host").code_block("csharp", "public class PluginHost\n{\n    private readonly List<IPlugin> _plugins = new();\n    public async Task LoadPluginsAsync(string dir)\n    {\n        foreach (var dll in Directory.GetFiles(dir, \"*.dll\"))\n        {\n            var asm = AssemblyLoadContext.Default.LoadFromAssemblyPath(dll);\n            var type = asm.GetTypes().FirstOrDefault(t => typeof(IPlugin).IsAssignableFrom(t));\n            if (type != null) { var p = (IPlugin)Activator.CreateInstance(type)!; await p.InitializeAsync(new PluginContext()); _plugins.Add(p); }\n        }\n    }\n}");
        let builder = if include_hot_reload { builder.heading(2, "Hot Reload").code_block("csharp", "var watcher = new FileSystemWatcher(pluginsPath, \"*.dll\");\nwatcher.Changed += async (s, e) => await ReloadPlugin(e.FullPath);") } else { builder };
        let builder = builder.heading(2, "Security").list(&["Verify signatures", "Sandbox untrusted plugins", "Limit resource access"]);
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_generate_architecture() {
        let tool = ArchitectureTemplateTool::new();
        let result = tool.generate_architecture_template(ArchitectureTemplateParams { pattern: None, include_examples: Some(true) }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_microservices_architecture() {
        let tool = ArchitectureTemplateTool::new();
        let params = MicroservicesParams {
            app_name: "MyApp".to_string(),
            services: Some("user,order".to_string()),
            include_gateway: Some(true),
        };
        let result = tool.generate_microservices_architecture(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_ddd_architecture() {
        let tool = ArchitectureTemplateTool::new();
        let params = DDDParams {
            domain_name: "ECommerce".to_string(),
            bounded_contexts: Some("sales,inventory".to_string()),
            include_cqrs: Some(true),
        };
        let result = tool.generate_ddd_architecture(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_plugin_architecture() {
        let tool = ArchitectureTemplateTool::new();
        let params = PluginParams {
            app_name: "PluginApp".to_string(),
            plugin_types: Some("editor,filter".to_string()),
            include_hot_reload: Some(false),
        };
        let result = tool.generate_plugin_architecture(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
