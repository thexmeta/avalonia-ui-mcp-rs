//! MCP Server implementation with full tool support
//!
//! This module defines the main MCP server with all 18 tools registered.

use rmcp::{
    model::{
        CallToolRequestParam, CallToolResult, Implementation, InitializeResult, ListToolsResult,
        ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
    },
    service::RequestContext,
    RoleServer, ServerHandler,
};
use std::sync::Arc;
use serde_json::{json, Map, Value};

use avalonia_mcp_tools::*;

type JsonObject = Map<String, Value>;

/// AvaloniaUI MCP Server with all tools
#[derive(Clone)]
pub struct AvaloniaMcpServer;

impl AvaloniaMcpServer {
    /// Create a new AvaloniaMcpServer instance
    pub fn new() -> Self {
        Self
    }

    /// List all available tools
    fn list_all_tools() -> Vec<Tool> {
        vec![
            Tool::new(
                "echo",
                "Echoes the message back to the client.",
                Arc::new(json!({"type": "object", "properties": {"message": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "server_info",
                "Gets information about the AvaloniaUI MCP server.",
                Arc::new(json!({"type": "object", "properties": {}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_project",
                "Generate a new AvaloniaUI project with MVVM architecture.",
                Arc::new(json!({"type": "object", "properties": {"name": {"type": "string"}, "output_dir": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "validate_xaml",
                "Validate XAML syntax and patterns for AvaloniaUI.",
                Arc::new(json!({"type": "object", "properties": {"xaml_content": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_security_pattern",
                "Generate security patterns and best practices.",
                Arc::new(json!({"type": "object", "properties": {"area": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "check_accessibility",
                "Check accessibility compliance for AvaloniaUI applications.",
                Arc::new(json!({"type": "object", "properties": {"xaml_content": {"type": "string"}, "wcag_level": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "analyze_performance",
                "Analyze performance and provide optimization guidance.",
                Arc::new(json!({"type": "object", "properties": {"area": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_animation",
                "Generate animation patterns and XAML templates.",
                Arc::new(json!({"type": "object", "properties": {"animation_type": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_theme",
                "Generate theme patterns and XAML styles.",
                Arc::new(json!({"type": "object", "properties": {"theme_type": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_localization",
                "Generate localization patterns and guidance.",
                Arc::new(json!({"type": "object", "properties": {"locales": {"type": "array", "items": {"type": "string"}}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "run_diagnostics",
                "Run system diagnostics and provide troubleshooting guidance.",
                Arc::new(json!({"type": "object", "properties": {"area": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_api_integration",
                "Generate REST API client integration patterns.",
                Arc::new(json!({"type": "object", "properties": {"api_type": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_testing_integration",
                "Generate test setup patterns.",
                Arc::new(json!({"type": "object", "properties": {"test_type": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_architecture_template",
                "Generate Clean Architecture templates.",
                Arc::new(json!({"type": "object", "properties": {"pattern": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_service_layer",
                "Generate service layer patterns.",
                Arc::new(json!({"type": "object", "properties": {"service_type": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_data_access_pattern",
                "Generate data access patterns.",
                Arc::new(json!({"type": "object", "properties": {"pattern": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_custom_control",
                "Generate custom control templates.",
                Arc::new(json!({"type": "object", "properties": {"control_type": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "provide_debugging_assistance",
                "Provide debugging assistance for AvaloniaUI applications.",
                Arc::new(json!({"type": "object", "properties": {"issue_type": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "provide_uiux_design",
                "Provide UI/UX design guidance.",
                Arc::new(json!({"type": "object", "properties": {"design_aspect": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_domain_service",
                "Creates domain service patterns for complex business logic in AvaloniaUI applications.",
                Arc::new(json!({"type": "object", "properties": {"domain_name": {"type": "string"}, "include_validation": {"type": "boolean"}, "include_events": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_ui_automation_tests",
                "Creates UI automation tests for AvaloniaUI applications using Avalonia.Headless.",
                Arc::new(json!({"type": "object", "properties": {"test_type": {"type": "string"}, "include_page_objects": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_mocks_and_builders",
                "Generates mock objects and test data builders for AvaloniaUI testing.",
                Arc::new(json!({"type": "object", "properties": {"entity_name": {"type": "string"}, "include_fluent_builder": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_performance_tests",
                "Creates performance and load tests for AvaloniaUI applications.",
                Arc::new(json!({"type": "object", "properties": {"test_area": {"type": "string"}, "include_profiling": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "convert_wpf_xaml_to_avalonia",
                "Converts WPF XAML to AvaloniaUI XAML by updating namespaces and incompatible elements.",
                Arc::new(json!({"type": "object", "properties": {"wpf_xaml": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_page_transition",
                "Creates page transitions for navigation between views.",
                Arc::new(json!({"type": "object", "properties": {"transition_type": {"type": "string"}, "direction": {"type": "string"}, "duration": {"type": "integer"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_storyboard",
                "Creates sophisticated storyboard animations with multiple properties and timing.",
                Arc::new(json!({"type": "object", "properties": {"sequence": {"type": "string"}, "total_duration": {"type": "integer"}, "storyboard_name": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_custom_animation",
                "Generates custom easing functions and advanced animation effects.",
                Arc::new(json!({"type": "object", "properties": {"effect_name": {"type": "string"}, "properties": {"type": "string"}, "pattern": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_api_models",
                "Creates data transfer objects (DTOs) and model classes for API integration.",
                Arc::new(json!({"type": "object", "properties": {"entity_name": {"type": "string"}, "model_type": {"type": "string"}, "include_validation": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "get_server_metrics",
                "Gets current server metrics and performance statistics.",
                Arc::new(json!({"type": "object", "properties": {}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "perform_health_check",
                "Performs a comprehensive health check of the MCP server.",
                Arc::new(json!({"type": "object", "properties": {}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "test_logging",
                "Tests logging functionality and telemetry recording.",
                Arc::new(json!({"type": "object", "properties": {"log_level": {"type": "string"}, "message": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_culture_formatting",
                "Creates culture-specific formatting and validation utilities.",
                Arc::new(json!({"type": "object", "properties": {"culture_code": {"type": "string"}, "format_type": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_async_data_access",
                "Creates async data access patterns with caching and error handling.",
                Arc::new(json!({"type": "object", "properties": {"service_name": {"type": "string"}, "include_caching": {"type": "boolean"}, "include_retry": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_debug_utilities",
                "Generates debug utilities and logging helpers.",
                Arc::new(json!({"type": "object", "properties": {"utility_type": {"type": "string"}, "include_devtools": {"type": "boolean"}, "include_telemetry": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_data_security_pattern",
                "Creates defensive data security patterns with encryption and audit logging.",
                Arc::new(json!({"type": "object", "properties": {"security_area": {"type": "string"}, "include_encryption": {"type": "boolean"}, "include_audit_logging": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_control_template",
                "Creates complex control templates with visual states and triggers.",
                Arc::new(json!({"type": "object", "properties": {"target_control": {"type": "string"}, "template_name": {"type": "string"}, "visual_states": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_attached_property_tool",
                "Generates attached properties for extending existing controls.",
                Arc::new(json!({"type": "object", "properties": {"property_name": {"type": "string"}, "property_type": {"type": "string"}, "target_controls": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_layout_panel",
                "Creates custom layout panels with arrangement logic.",
                Arc::new(json!({"type": "object", "properties": {"panel_name": {"type": "string"}, "orientation": {"type": "string"}, "include_spacing": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_selectors",
                "Creates CSS-like selectors for AvaloniaUI styling.",
                Arc::new(json!({"type": "object", "properties": {"selector_type": {"type": "string"}, "target_control": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_color_scheme",
                "Converts colors between formats and generates color schemes.",
                Arc::new(json!({"type": "object", "properties": {"base_color": {"type": "string"}, "format": {"type": "string"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_ux_patterns",
                "Generates UX patterns for improved user experience.",
                Arc::new(json!({"type": "object", "properties": {"pattern_type": {"type": "string"}, "include_examples": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_design_system",
                "Creates design systems with consistent visual components.",
                Arc::new(json!({"type": "object", "properties": {"project_name": {"type": "string"}, "include_tokens": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_microservices_architecture",
                "Creates microservices architecture templates for distributed applications.",
                Arc::new(json!({"type": "object", "properties": {"app_name": {"type": "string"}, "services": {"type": "string"}, "include_gateway": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_ddd_architecture",
                "Generates Domain-Driven Design templates with bounded contexts.",
                Arc::new(json!({"type": "object", "properties": {"domain_name": {"type": "string"}, "bounded_contexts": {"type": "string"}, "include_cqrs": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_plugin_architecture",
                "Creates plugin architecture templates with extensible module system.",
                Arc::new(json!({"type": "object", "properties": {"app_name": {"type": "string"}, "plugin_types": {"type": "string"}, "include_hot_reload": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "get_performance_recommendations",
                "Provides performance optimization recommendations.",
                Arc::new(json!({"type": "object", "properties": {"area": {"type": "string"}, "include_code": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_accessible_component",
                "Generates WCAG compliant accessible UI components.",
                Arc::new(json!({"type": "object", "properties": {"component_type": {"type": "string"}, "include_keyboard_nav": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "force_gc",
                "Reports memory statistics and explains Rust memory management.",
                Arc::new(json!({"type": "object", "properties": {}}).as_object().unwrap().clone())
            ),
            Tool::new(
                "generate_responsive_design",
                "Generates responsive design patterns with adaptive layouts and breakpoints.",
                Arc::new(json!({"type": "object", "properties": {"layout_type": {"type": "string"}, "target_devices": {"type": "string"}, "include_touch": {"type": "boolean"}}}).as_object().unwrap().clone())
            ),
        ]
    }

    /// Call a tool by name
    async fn call_tool_by_name(&self, name: &str, args: &JsonObject) -> Result<CallToolResult, rmcp::ErrorData> {
        match name {
            "echo" => {
                let tool = EchoTool::new();
                let message = args.get("message").and_then(|v| v.as_str()).unwrap_or("Hello").to_string();
                tool.echo(echo_tool::EchoParams { message }).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "server_info" => {
                EchoTool::new().server_info().await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_project" => {
                let tool = ProjectGeneratorTool::new();
                let p = project_generator_tool::ProjectGeneratorParams {
                    name: args.get("name").and_then(|v| v.as_str()).unwrap_or("MyProject").to_string(),
                    output_dir: args.get("output_dir").and_then(|v| v.as_str()).unwrap_or(".").to_string(),
                    enable_http: args.get("enable_http").and_then(|v| v.as_bool()),
                    include_tests: args.get("include_tests").and_then(|v| v.as_bool()),
                    use_reactiveui: args.get("use_reactiveui").and_then(|v| v.as_bool()),
                };
                tool.generate_project(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "validate_xaml" => {
                let tool = XamlValidationTool::new();
                let p = xaml_validation_tool::XamlValidationParams {
                    xaml_content: args.get("xaml_content").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    strict_mode: args.get("strict_mode").and_then(|v| v.as_bool()),
                    check_accessibility: args.get("check_accessibility").and_then(|v| v.as_bool()),
                };
                tool.validate_xaml(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_security_pattern" => {
                let tool = SecurityPatternTool::new();
                let p = security_pattern_tool::SecurityPatternParams {
                    area: args.get("area").and_then(|v| v.as_str()).map(String::from),
                    app_type: args.get("app_type").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.generate_security_pattern(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "check_accessibility" => {
                let tool = AccessibilityTool::new();
                let p = accessibility_tool::AccessibilityParams {
                    xaml_content: args.get("xaml_content").and_then(|v| v.as_str()).map(String::from),
                    wcag_level: args.get("wcag_level").and_then(|v| v.as_str()).map(String::from),
                    include_guidance: args.get("include_guidance").and_then(|v| v.as_bool()),
                };
                tool.check_accessibility(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "analyze_performance" => {
                let tool = PerformanceAnalysisTool::new();
                let p = performance_analysis_tool::PerformanceAnalysisParams {
                    area: args.get("area").and_then(|v| v.as_str()).map(String::from),
                    include_profiling_code: args.get("include_profiling_code").and_then(|v| v.as_bool()),
                    app_type: args.get("app_type").and_then(|v| v.as_str()).map(String::from),
                };
                tool.analyze_performance(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_animation" => {
                let tool = AnimationTool::new();
                let p = animation_tool::AnimationParams {
                    animation_type: args.get("animation_type").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                    platform: args.get("platform").and_then(|v| v.as_str()).map(String::from),
                };
                tool.generate_animation(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_theme" => {
                let tool = ThemingTool::new();
                let p = theming_tool::ThemingParams {
                    theme_type: args.get("theme_type").and_then(|v| v.as_str()).map(String::from),
                    include_palette: args.get("include_palette").and_then(|v| v.as_bool()),
                    app_type: args.get("app_type").and_then(|v| v.as_str()).map(String::from),
                };
                tool.generate_theme(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_localization" => {
                let tool = LocalizationTool::new();
                let locales = args.get("locales")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect());
                let p = localization_tool::LocalizationParams {
                    locales,
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.generate_localization(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "run_diagnostics" => {
                let tool = DiagnosticTool::new();
                let p = diagnostic_tool::DiagnosticParams {
                    area: args.get("area").and_then(|v| v.as_str()).map(String::from),
                    include_troubleshooting: args.get("include_troubleshooting").and_then(|v| v.as_bool()),
                };
                tool.run_diagnostics(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_api_integration" => {
                let tool = APIIntegrationTool::new();
                let p = api_integration_tool::APIIntegrationParams {
                    api_type: args.get("api_type").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.generate_api_integration(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_testing_integration" => {
                let tool = TestingIntegrationTool::new();
                let p = testing_integration_tool::TestingIntegrationParams {
                    test_type: args.get("test_type").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.generate_testing_integration(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_architecture_template" => {
                let tool = ArchitectureTemplateTool::new();
                let p = architecture_template_tool::ArchitectureTemplateParams {
                    pattern: args.get("pattern").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.generate_architecture_template(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_service_layer" => {
                let tool = ServiceLayerTool::new();
                let p = service_layer_tool::ServiceLayerParams {
                    service_type: args.get("service_type").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.generate_service_layer(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_data_access_pattern" => {
                let tool = DataAccessPatternTool::new();
                let p = data_access_pattern_tool::DataAccessPatternParams {
                    pattern: args.get("pattern").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.generate_data_access_pattern(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_custom_control" => {
                let tool = CustomControlGenerator::new();
                let p = custom_control_generator::CustomControlGeneratorParams {
                    control_type: args.get("control_type").and_then(|v| v.as_str()).map(String::from),
                    control_name: args.get("control_name").and_then(|v| v.as_str()).map(String::from),
                    include_styles: args.get("include_styles").and_then(|v| v.as_bool()),
                };
                tool.generate_custom_control(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "provide_debugging_assistance" => {
                let tool = DebuggingAssistantTool::new();
                let p = debugging_assistant_tool::DebuggingAssistantParams {
                    issue_type: args.get("issue_type").and_then(|v| v.as_str()).map(String::from),
                    include_solutions: args.get("include_solutions").and_then(|v| v.as_bool()),
                };
                tool.provide_debugging_assistance(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "provide_uiux_design" => {
                let tool = UIUXDesignTool::new();
                let p = uiux_design_tool::UIUXDesignParams {
                    design_aspect: args.get("design_aspect").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.provide_uiux_design(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_domain_service" => {
                let tool = ServiceLayerTool::new();
                let p = service_layer_tool::DomainServiceParams {
                    domain_name: args.get("domain_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    include_validation: args.get("include_validation").and_then(|v| v.as_bool()),
                    include_events: args.get("include_events").and_then(|v| v.as_bool()),
                };
                tool.generate_domain_service(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_ui_automation_tests" => {
                let tool = TestingIntegrationTool::new();
                let p = testing_integration_tool::UITestParams {
                    test_type: args.get("test_type").and_then(|v| v.as_str()).map(String::from),
                    include_page_objects: args.get("include_page_objects").and_then(|v| v.as_bool()),
                };
                tool.generate_ui_automation_tests(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_mocks_and_builders" => {
                let tool = TestingIntegrationTool::new();
                let p = testing_integration_tool::MocksAndBuildersParams {
                    entity_name: args.get("entity_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    include_fluent_builder: args.get("include_fluent_builder").and_then(|v| v.as_bool()),
                };
                tool.generate_mocks_and_builders(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_performance_tests" => {
                let tool = TestingIntegrationTool::new();
                let p = testing_integration_tool::PerformanceTestParams {
                    test_area: args.get("test_area").and_then(|v| v.as_str()).map(String::from),
                    include_profiling: args.get("include_profiling").and_then(|v| v.as_bool()),
                };
                tool.generate_performance_tests(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "convert_wpf_xaml_to_avalonia" => {
                let tool = XamlValidationTool::new();
                let p = xaml_validation_tool::WpfConversionParams {
                    wpf_xaml: args.get("wpf_xaml").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                };
                tool.convert_wpf_xaml_to_avalonia(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_page_transition" => {
                let tool = AnimationTool::new();
                let p = animation_tool::PageTransitionParams {
                    transition_type: args.get("transition_type").and_then(|v| v.as_str()).map(String::from),
                    direction: args.get("direction").and_then(|v| v.as_str()).map(String::from),
                    duration: args.get("duration").and_then(|v| v.as_i64()).map(|v| v as i32),
                };
                tool.generate_page_transition(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_storyboard" => {
                let tool = AnimationTool::new();
                let p = animation_tool::StoryboardParams {
                    sequence: args.get("sequence").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    total_duration: args.get("total_duration").and_then(|v| v.as_i64()).map(|v| v as i32),
                    storyboard_name: args.get("storyboard_name").and_then(|v| v.as_str()).map(String::from),
                };
                tool.generate_storyboard(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_custom_animation" => {
                let tool = AnimationTool::new();
                let p = animation_tool::CustomAnimationParams {
                    effect_name: args.get("effect_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    properties: args.get("properties").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    pattern: args.get("pattern").and_then(|v| v.as_str()).map(String::from),
                    complexity: args.get("complexity").and_then(|v| v.as_str()).map(String::from),
                };
                tool.generate_custom_animation(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_api_models" => {
                let tool = APIIntegrationTool::new();
                let p = api_integration_tool::ApiModelsParams {
                    entity_name: args.get("entity_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    model_type: args.get("model_type").and_then(|v| v.as_str()).map(String::from),
                    include_validation: args.get("include_validation").and_then(|v| v.as_bool()),
                };
                tool.generate_api_models(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "get_server_metrics" => {
                DiagnosticTool::new().get_server_metrics().await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "perform_health_check" => {
                DiagnosticTool::new().perform_health_check().await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "test_logging" => {
                let tool = DiagnosticTool::new();
                let p = diagnostic_tool::TestLoggingParams {
                    log_level: args.get("log_level").and_then(|v| v.as_str()).map(String::from),
                    message: args.get("message").and_then(|v| v.as_str()).map(String::from),
                };
                tool.test_logging(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_culture_formatting" => {
                let tool = LocalizationTool::new();
                let p = localization_tool::CultureFormattingParams {
                    culture_code: args.get("culture_code").and_then(|v| v.as_str()).map(String::from),
                    format_type: args.get("format_type").and_then(|v| v.as_str()).map(String::from),
                };
                tool.generate_culture_formatting(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_async_data_access" => {
                let tool = DataAccessPatternTool::new();
                let p = data_access_pattern_tool::AsyncDataAccessParams {
                    service_name: args.get("service_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    include_caching: args.get("include_caching").and_then(|v| v.as_bool()),
                    include_retry: args.get("include_retry").and_then(|v| v.as_bool()),
                    caching_provider: args.get("caching_provider").and_then(|v| v.as_str()).map(String::from),
                };
                tool.generate_async_data_access(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_debug_utilities" => {
                let tool = DebuggingAssistantTool::new();
                let p = debugging_assistant_tool::DebugUtilitiesParams {
                    utility_type: args.get("utility_type").and_then(|v| v.as_str()).map(String::from),
                    include_devtools: args.get("include_devtools").and_then(|v| v.as_bool()),
                    include_telemetry: args.get("include_telemetry").and_then(|v| v.as_bool()),
                };
                tool.generate_debug_utilities(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_data_security_pattern" => {
                let tool = SecurityPatternTool::new();
                let p = security_pattern_tool::DataSecurityParams {
                    security_area: args.get("security_area").and_then(|v| v.as_str()).map(String::from),
                    include_encryption: args.get("include_encryption").and_then(|v| v.as_bool()),
                    include_audit_logging: args.get("include_audit_logging").and_then(|v| v.as_bool()),
                };
                tool.generate_data_security_pattern(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_control_template" => {
                let tool = CustomControlGenerator::new();
                let p = custom_control_generator::ControlTemplateParams {
                    target_control: args.get("target_control").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    template_name: args.get("template_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    visual_states: args.get("visual_states").and_then(|v| v.as_str()).map(String::from),
                    include_animations: args.get("include_animations").and_then(|v| v.as_bool()),
                };
                tool.generate_control_template(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_attached_property_tool" => {
                let tool = CustomControlGenerator::new();
                let p = custom_control_generator::AttachedPropertyParams {
                    property_name: args.get("property_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    property_type: args.get("property_type").and_then(|v| v.as_str()).map(String::from),
                    target_controls: args.get("target_controls").and_then(|v| v.as_str()).map(String::from),
                    include_handler: args.get("include_handler").and_then(|v| v.as_bool()),
                };
                tool.generate_attached_property(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_layout_panel" => {
                let tool = CustomControlGenerator::new();
                let p = custom_control_generator::LayoutPanelParams {
                    panel_name: args.get("panel_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    orientation: args.get("orientation").and_then(|v| v.as_str()).map(String::from),
                    include_spacing: args.get("include_spacing").and_then(|v| v.as_bool()),
                };
                tool.generate_layout_panel(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_selectors" => {
                let tool = ThemingTool::new();
                let p = theming_tool::SelectorsParams {
                    selector_type: args.get("selector_type").and_then(|v| v.as_str()).map(String::from),
                    target_control: args.get("target_control").and_then(|v| v.as_str()).map(String::from),
                };
                tool.generate_selectors(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_color_scheme" => {
                let tool = ThemingTool::new();
                let p = theming_tool::ColorSchemeParams {
                    base_color: args.get("base_color").and_then(|v| v.as_str()).map(String::from),
                    format: args.get("format").and_then(|v| v.as_str()).map(String::from),
                };
                tool.generate_color_scheme(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_ux_patterns" => {
                let tool = UIUXDesignTool::new();
                let p = uiux_design_tool::UXPatternsParams {
                    pattern_type: args.get("pattern_type").and_then(|v| v.as_str()).map(String::from),
                    include_examples: args.get("include_examples").and_then(|v| v.as_bool()),
                };
                tool.generate_ux_patterns(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_design_system" => {
                let tool = UIUXDesignTool::new();
                let p = uiux_design_tool::DesignSystemParams {
                    project_name: args.get("project_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    include_tokens: args.get("include_tokens").and_then(|v| v.as_bool()),
                };
                tool.generate_design_system(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_microservices_architecture" => {
                let tool = ArchitectureTemplateTool::new();
                let p = architecture_template_tool::MicroservicesParams {
                    app_name: args.get("app_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    services: args.get("services").and_then(|v| v.as_str()).map(String::from),
                    include_gateway: args.get("include_gateway").and_then(|v| v.as_bool()),
                };
                tool.generate_microservices_architecture(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_ddd_architecture" => {
                let tool = ArchitectureTemplateTool::new();
                let p = architecture_template_tool::DDDParams {
                    domain_name: args.get("domain_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    bounded_contexts: args.get("bounded_contexts").and_then(|v| v.as_str()).map(String::from),
                    include_cqrs: args.get("include_cqrs").and_then(|v| v.as_bool()),
                };
                tool.generate_ddd_architecture(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_plugin_architecture" => {
                let tool = ArchitectureTemplateTool::new();
                let p = architecture_template_tool::PluginParams {
                    app_name: args.get("app_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    plugin_types: args.get("plugin_types").and_then(|v| v.as_str()).map(String::from),
                    include_hot_reload: args.get("include_hot_reload").and_then(|v| v.as_bool()),
                };
                tool.generate_plugin_architecture(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "get_performance_recommendations" => {
                let tool = PerformanceAnalysisTool::new();
                let p = performance_analysis_tool::PerformanceRecommendationsParams {
                    area: args.get("area").and_then(|v| v.as_str()).map(String::from),
                    include_code: args.get("include_code").and_then(|v| v.as_bool()),
                };
                tool.get_performance_recommendations(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_accessible_component" => {
                let tool = AccessibilityTool::new();
                let p = accessibility_tool::AccessibleComponentParams {
                    component_type: args.get("component_type").and_then(|v| v.as_str()).map(String::from),
                    include_keyboard_nav: args.get("include_keyboard_nav").and_then(|v| v.as_bool()),
                };
                tool.generate_accessible_component(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "force_gc" => {
                DiagnosticTool::new().force_gc().await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            "generate_responsive_design" => {
                let tool = UIUXDesignTool::new();
                let p = uiux_design_tool::ResponsiveDesignParams {
                    layout_type: args.get("layout_type").and_then(|v| v.as_str()).map(String::from),
                    target_devices: args.get("target_devices").and_then(|v| v.as_str()).map(String::from),
                    include_touch: args.get("include_touch").and_then(|v| v.as_bool()),
                };
                tool.generate_responsive_design(p).await
                    .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
            }
            _ => Err(rmcp::ErrorData::invalid_params(format!("Unknown tool: {}", name), None))
        }
    }

    /// List tools for HTTP API (returns serializable format)
    #[allow(dead_code)]
    pub async fn list_tools_for_http(&self) -> Result<Vec<serde_json::Value>, String> {
        let tools = Self::list_all_tools();
        let tool_jsons: Vec<serde_json::Value> = tools.iter().map(|tool| {
            json!({
                "name": tool.name,
                "description": tool.description,
                "inputSchema": tool.input_schema
            })
        }).collect();
        Ok(tool_jsons)
    }

    /// Call tool for HTTP API
    #[allow(dead_code)]
    pub async fn call_tool_for_http(&self, name: &str, args: &JsonObject) -> Result<CallToolResult, String> {
        self.call_tool_by_name(name, args)
            .await
            .map_err(|e| e.message.to_string())
    }
}

impl Default for AvaloniaMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerHandler for AvaloniaMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_06_18,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "I provide comprehensive AvaloniaUI development assistance with 49 tools."
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: rmcp::model::InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, rmcp::ErrorData> {
        Ok(self.get_info())
    }

    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        Ok(ListToolsResult {
            tools: Self::list_all_tools(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let args = request.arguments.unwrap_or_default();
        self.call_tool_by_name(&request.name, &args).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let _server = AvaloniaMcpServer::new();
        assert!(true);
    }

    #[test]
    fn test_tools_list() {
        let tools = AvaloniaMcpServer::list_all_tools();
        assert_eq!(tools.len(), 49);
    }
}
