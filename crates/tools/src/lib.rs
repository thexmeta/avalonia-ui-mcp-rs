//! AvaloniaUI MCP Tools Library
//!
//! This crate provides all 18 MCP tools for AvaloniaUI development assistance.

pub mod accessibility_tool;
pub mod animation_tool;
pub mod api_integration_tool;
pub mod architecture_template_tool;
pub mod custom_control_generator;
pub mod data_access_pattern_tool;
pub mod debugging_assistant_tool;
pub mod diagnostic_tool;
pub mod echo_tool;
pub mod localization_tool;
pub mod performance_analysis_tool;
pub mod project_generator_tool;
pub mod security_pattern_tool;
pub mod service_layer_tool;
pub mod testing_integration_tool;
pub mod theming_tool;
pub mod uiux_design_tool;
pub mod xaml_validation_tool;

// Re-export all tools for convenient access
pub use accessibility_tool::AccessibilityTool;
pub use animation_tool::AnimationTool;
pub use api_integration_tool::APIIntegrationTool;
pub use architecture_template_tool::ArchitectureTemplateTool;
pub use custom_control_generator::CustomControlGenerator;
pub use data_access_pattern_tool::DataAccessPatternTool;
pub use debugging_assistant_tool::DebuggingAssistantTool;
pub use diagnostic_tool::DiagnosticTool;
pub use echo_tool::EchoTool;
pub use localization_tool::LocalizationTool;
pub use performance_analysis_tool::PerformanceAnalysisTool;
pub use project_generator_tool::ProjectGeneratorTool;
pub use security_pattern_tool::SecurityPatternTool;
pub use service_layer_tool::ServiceLayerTool;
pub use testing_integration_tool::TestingIntegrationTool;
pub use theming_tool::ThemingTool;
pub use uiux_design_tool::UIUXDesignTool;
pub use xaml_validation_tool::XamlValidationTool;
