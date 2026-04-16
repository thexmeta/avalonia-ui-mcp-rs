//! Testing Integration tool - Test setup patterns
use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TestingIntegrationParams {
    pub test_type: Option<String>,
    pub include_examples: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UITestParams { pub test_type: Option<String>, pub include_page_objects: Option<bool> }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MocksAndBuildersParams { pub entity_name: String, pub include_fluent_builder: Option<bool> }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PerformanceTestParams { pub test_area: Option<String>, pub include_profiling: Option<bool> }

#[derive(Debug, Clone, Default)]
pub struct TestingIntegrationTool;

impl TestingIntegrationTool {
    pub fn new() -> Self { Self }

    #[tool(description = "Generate test setup patterns for AvaloniaUI applications. Covers unit tests, UI tests with Avalonia.Headless, and mocking strategies.")]
    pub async fn generate_testing_integration(
        &self,
        params: TestingIntegrationParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_examples = params.include_examples.unwrap_or(true);
        let test_type = params.test_type.as_deref().unwrap_or("all");

        let output = match test_type {
            "unit" => self.generate_unit_tests(include_examples),
            "ui" => self.generate_ui_tests(include_examples),
            "integration" => self.generate_integration_tests(include_examples),
            _ => self.generate_all_tests(include_examples),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    fn generate_unit_tests(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Unit Testing Patterns")
            .paragraph("Unit testing best practices for AvaloniaUI applications.")
            .heading(2, "Test Framework");

        if include_examples {
            builder = builder
                .heading(3, "xUnit Test Setup")
                .code_block("csharp", r#"// Test project setup
// dotnet add package xunit
// dotnet add package xunit.runner.visualstudio
// dotnet add package Moq

public class ViewModelTests
{
    [Fact]
    public void ViewModel_PropertyChange_UpdatesView()
    {
        // Arrange
        var mockService = new Mock<IDataService>();
        mockService.Setup(s => s.GetData()).ReturnsAsync(new Data());
        var vm = new MainViewModel(mockService.Object);
        
        // Act
        vm.LoadCommand.Execute(null);
        
        // Assert
        Assert.NotNull(vm.Data);
        mockService.Verify(s => s.GetData(), Times.Once);
    }
    
    [Fact]
    public async Task ViewModel_ErrorHandling_ShowsMessage()
    {
        // Arrange
        var mockService = new Mock<IDataService>();
        mockService.Setup(s => s.GetData()).ThrowsAsync(new Exception("Test"));
        var vm = new MainViewModel(mockService.Object);
        
        // Act
        await vm.LoadCommand.ExecuteAsync(null);
        
        // Assert
        Assert.NotNull(vm.ErrorMessage);
    }
}"#)
                .heading(3, "Mocking Services")
                .code_block("csharp", r#"// Using Moq for mocking
var mockRepo = new Mock<IRepository>();
mockRepo.Setup(r => r.GetById(It.IsAny<int>())).ReturnsAsync(new Entity());
mockRepo.Setup(r => r.Save(It.IsAny<Entity>())).Returns(Task.CompletedTask);

// Using NSubstitute (alternative)
var mockRepo = Substitute.For<IRepository>();
mockRepo.GetById(1).Returns(new Entity());
await mockRepo.Save(Arg.Any<Entity>()).Returns(Task.CompletedTask);"#);
        }

        builder.heading(2, "Best Practices")
            .task_list(vec![(true, "Test one thing per test"), (true, "Use descriptive names"), (true, "Arrange-Act-Assert pattern"), (true, "Mock external dependencies"), (false, "Aim for 80%+ coverage")])
            .build()
    }

    fn generate_ui_tests(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "UI Testing Patterns")
            .paragraph("UI testing with Avalonia.Headless for automated UI tests.")
            .heading(2, "Headless Testing");

        if include_examples {
            builder = builder
                .heading(3, "Avalonia.Headless Setup")
                .code_block("csharp", r#"// Test project: dotnet add package Avalonia.Headless.XUnit

public class MainWindowTests : IClassFixture<AppFixture>
{
    private readonly AppFixture _fixture;
    
    public MainWindowTests(AppFixture fixture) => _fixture = fixture;
    
    [Fact]
    public async Task Button_Click_UpdatesCounter()
    {
        // Arrange
        var window = new MainWindow();
        window.Show(_fixture.App);
        
        // Act
        var button = window.FindControl<Button>("IncrementButton");
        button.RaiseEvent(new RoutedEventArgs(Button.ClickEvent));
        await _fixture.App.Dispatcher.UIThread.RunJobs();
        
        // Assert
        var textBlock = window.FindControl<TextBlock>("CounterText");
        Assert.Equal("1", textBlock.Text);
    }
    
    [Fact]
    public async Task TextBox_Input_Validates()
    {
        // Arrange
        var window = new MainWindow();
        window.Show(_fixture.App);
        
        // Act
        var textBox = window.FindControl<TextBox>("NameInput");
        textBox.Text = "Test";
        textBox.RaiseEvent(new RoutedEventArgs(TextInputEvent));
        
        // Assert
        Assert.False(textBox.Classes.Contains("error"));
    }
}"#);
        }

        builder.heading(2, "UI Test Best Practices")
            .task_list(vec![(true, "Use Page Object pattern"), (true, "Wait for async operations"), (true, "Test user workflows"), (true, "Verify visual states"), (false, "Run in CI pipeline")])
            .build()
    }

    fn generate_integration_tests(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Integration Testing Patterns")
            .paragraph("Integration testing for full application workflows.")
            .heading(2, "Test Setup");

        if include_examples {
            builder = builder
                .heading(3, "Integration Test Base")
                .code_block("csharp", r#"public class IntegrationTestBase : IDisposable
{
    protected readonly IHost _host;
    protected readonly IServiceProvider _services;
    
    public IntegrationTestBase()
    {
        _host = Host.CreateDefaultBuilder()
            .ConfigureServices(ConfigureServices)
            .Build();
        _host.Start();
        _services = _host.Services;
    }
    
    protected virtual void ConfigureServices(IServiceCollection services)
    {
        // Register test services
        services.AddSingleton<ITestDatabase, TestDatabase>();
    }
    
    public void Dispose() => _host.Dispose();
}"#);
        }

        builder.heading(2, "Integration Test Checklist")
            .task_list(vec![(true, "Use test database"), (true, "Mock external services"), (true, "Test full workflows"), (true, "Clean up after tests"), (false, "Run in isolated environment")])
            .build()
    }

    fn generate_all_tests(&self, include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Complete Testing Guide")
            .paragraph("Comprehensive testing strategy for AvaloniaUI applications.")
            .heading(2, "Testing Pyramid")
            .list(vec!["Unit Tests (70%)", "Integration Tests (20%)", "UI Tests (10%)"])
            .heading(2, "Unit Tests")
            .paragraph(&self.generate_unit_tests(include_examples))
            .heading(2, "UI Tests")
            .paragraph(&self.generate_ui_tests(include_examples))
            .heading(2, "Integration Tests")
            .paragraph(&self.generate_integration_tests(include_examples))
            .heading(2, "CI/CD Integration")
            .code_block("yaml", r#"# GitHub Actions
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Run tests
      run: dotnet test --collect:"XPlat Code Coverage"
    - name: Upload coverage
      uses: codecov/codecov-action@v4"#)
            .build()
    }

    #[tool(description = "Creates UI automation tests for AvaloniaUI applications using Avalonia.Headless")]
    pub async fn generate_ui_automation_tests(&self, params: UITestParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let test_type = params.test_type.as_deref().unwrap_or("button_click").to_lowercase();
        let include_po = params.include_page_objects.unwrap_or(true);
        let po = if include_po { "\n## Page Objects\n```csharp\npublic class LoginPageObject\n{\n    private readonly IControl _root;\n    public LoginPageObject(IControl root) => _root = root;\n    public IControl UsernameInput => _root.FindControl<TextBox>(\"UsernameBox\");\n    public IControl LoginButton => _root.FindControl<Button>(\"LoginBtn\");\n    public void Login(string user, string pass)\n    {\n        UsernameInput.Text = user;\n        LoginButton.Click();\n    }\n}\n```" } else { "" };
        let builder = MarkdownOutputBuilder::new()
            .heading(1, "UI Automation Tests")
            .heading(2, "Configuration").task_list(vec![(true, format!("Type: {}", test_type)), (true, format!("Page Objects: {}", include_po))])
            .heading(2, "Headless Test").code_block("csharp", "[TestClass]\npublic class UITests\n{\n    [TestMethod]\n    public async Task Button_Click_ShouldUpdateText()\n    {\n        var window = new MainWindow();\n        var button = window.Find<Button>(\"MyButton\");\n        button.Click();\n        Assert.AreEqual(\"Clicked\", window.Find<TextBlock>(\"Status\").Text);\n    }\n}");
        let builder = if include_po { builder.heading(2, "Page Objects").code_block("csharp", &po.replace("\n## Page Objects\n```csharp\n", "").replace("\n```", "")) } else { builder };
        let builder = builder.heading(2, "Setup").list(&["Install Avalonia.Headless.NUnit", "Use BuildMainWindow() for test windows", "Simulate input with Click(), Focus(), Text"]);
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Generates mock objects and test data builders for AvaloniaUI testing")]
    pub async fn generate_mocks_and_builders(&self, params: MocksAndBuildersParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.entity_name.is_empty() { return Err(AvaloniaMcpError::validation("Entity name cannot be empty")); }
        let include_fluent = params.include_fluent_builder.unwrap_or(true);
        let e = &params.entity_name;
        let builder_code = if include_fluent {
            format!("public class {e}Builder\n{{\n    private int _id = 1;\n    private string _name = \"Test {e}\";\n    public {e}Builder WithId(int id) {{ _id = id; return this; }}\n    public {e}Builder WithName(string name) {{ _name = name; return this; }}\n    public {e} Build() => new() {{ Id = _id, Name = _name }};\n    public static implicit operator {e}({e}Builder b) => b.Build();\n}}")
        } else {
            format!("public static class {e}Factory\n{{\n    public static {e} Create(int id = 1, string name = \"Test\") =>\n        new() {{ Id = id, Name = name }};\n}}")
        };
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Test Builders: {}", e))
            .heading(2, "Configuration").task_list(vec![(true, format!("Entity: {}", e)), (true, format!("Fluent: {}", include_fluent))])
            .heading(2, "Implementation").code_block("csharp", &builder_code)
            .heading(2, "Usage").code_block("csharp", &format!("var entity = new {e}Builder().WithId(42).WithName(\"Custom\").Build();"))
            .heading(2, "Mock Setup").code_block("csharp", &format!("var mock = new Mock<I{e}Service>();\nmock.Setup(s => s.GetAsync(It.IsAny<int>())).ReturnsAsync(new {e}Builder().Build());"));
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    #[tool(description = "Creates performance and load tests for AvaloniaUI applications")]
    pub async fn generate_performance_tests(&self, params: PerformanceTestParams) -> Result<CallToolResult, AvaloniaMcpError> {
        let test_area = params.test_area.as_deref().unwrap_or("rendering").to_lowercase();
        let include_profiling = params.include_profiling.unwrap_or(true);
        let profiling = if include_profiling { "\n## Profiling\n```csharp\npublic static class PerformanceTester\n{\n    public static async Task<long> MeasureAsync(Func<Task> op, int iterations = 100)\n    {\n        var sw = Stopwatch.StartNew();\n        for (int i = 0; i < iterations; i++) await op();\n        sw.Stop();\n        return sw.ElapsedMilliseconds;\n    }\n}\n```" } else { "" };
        let builder = MarkdownOutputBuilder::new()
            .heading(1, "Performance Tests")
            .heading(2, "Configuration").task_list(vec![(true, format!("Area: {}", test_area)), (true, format!("Profiling: {}", include_profiling))])
            .heading(2, "Performance Test").code_block("csharp", "[TestClass]\npublic class PerformanceTests\n{\n    [TestMethod]\n    public async Task Render_With1000Items_ShouldCompleteInTime()\n    {\n        var sw = Stopwatch.StartNew();\n        var listBox = new ListBox { Items = Enumerable.Range(0, 1000) };\n        listBox.Measure(new Size(800, 600));\n        listBox.Arrange(new Rect(0, 0, 800, 600));\n        sw.Stop();\n        Assert.IsTrue(sw.ElapsedMilliseconds < 500);\n    }\n}");
        let builder = if include_profiling { builder.heading(2, "Profiling").code_block("csharp", &profiling.replace("\n## Profiling\n```csharp\n", "").replace("\n```", "")) } else { builder };
        let builder = builder.heading(2, "Metrics").list(&["<500ms for 1000 item rendering", "Memory stable over time", "CPU <10% idle", "60fps animations"]);
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_generate_testing() {
        let tool = TestingIntegrationTool::new();
        let result = tool.generate_testing_integration(TestingIntegrationParams { test_type: None, include_examples: Some(true) }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_ui_automation_tests_success() {
        let tool = TestingIntegrationTool::new();
        let params = UITestParams {
            test_type: Some("button_click".to_string()),
            include_page_objects: Some(true),
        };
        let result = tool.generate_ui_automation_tests(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_mocks_and_builders_success() {
        let tool = TestingIntegrationTool::new();
        let params = MocksAndBuildersParams {
            entity_name: "Order".to_string(),
            include_fluent_builder: Some(true),
        };
        let result = tool.generate_mocks_and_builders(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_mocks_and_builders_empty_name() {
        let tool = TestingIntegrationTool::new();
        let params = MocksAndBuildersParams {
            entity_name: "".to_string(),
            include_fluent_builder: None,
        };
        let result = tool.generate_mocks_and_builders(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_performance_tests_success() {
        let tool = TestingIntegrationTool::new();
        let params = PerformanceTestParams {
            test_area: Some("rendering".to_string()),
            include_profiling: Some(true),
        };
        let result = tool.generate_performance_tests(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
