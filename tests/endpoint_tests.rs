//! Integration tests for all MCP tool endpoints
//!
//! These tests verify that each tool endpoint works correctly
//! through the MCP protocol interface.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_tools::*;

// ============================================================================
// EchoTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod echo_tool_tests {
    use super::*;

    #[tokio::test]
    async fn test_echo_endpoint_with_valid_message() {
        let tool = EchoTool::new();
        let params = echo_tool::EchoParams {
            message: "Hello World".to_string(),
        };

        let result = tool.echo(params).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_error.unwrap_or(false));
        assert!(!response.content.is_empty());
    }

    #[tokio::test]
    async fn test_echo_endpoint_with_empty_message() {
        let tool = EchoTool::new();
        let params = echo_tool::EchoParams {
            message: "".to_string(),
        };

        let result = tool.echo(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_echo_endpoint_with_special_characters() {
        let tool = EchoTool::new();
        let params = echo_tool::EchoParams {
            message: "Test with special chars: @#$%^&*()".to_string(),
        };

        let result = tool.echo(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_server_info_endpoint() {
        let tool = EchoTool::new();
        let result = tool.server_info().await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_error.unwrap_or(false));
        assert!(!response.content.is_empty());
    }
}

// ============================================================================
// ProjectGeneratorTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod project_generator_tool_tests {
    use super::*;
    use project_generator_tool::{ProjectGeneratorParams, ProjectGeneratorTool};

    #[tokio::test]
    async fn test_generate_project_endpoint_with_valid_params() {
        let tool = ProjectGeneratorTool::new();
        let params = ProjectGeneratorParams {
            name: "TestProject".to_string(),
            output_dir: std::env::temp_dir().to_string_lossy().to_string(),
            enable_http: Some(false),
            include_tests: Some(true),
            use_reactiveui: Some(false),
        };

        let result = tool.generate_project(params).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_error.unwrap_or(false));
    }

    #[tokio::test]
    async fn test_generate_project_endpoint_with_empty_name() {
        let tool = ProjectGeneratorTool::new();
        let params = ProjectGeneratorParams {
            name: "".to_string(),
            output_dir: "/tmp".to_string(),
            enable_http: None,
            include_tests: None,
            use_reactiveui: None,
        };

        let result = tool.generate_project(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_project_endpoint_with_http_enabled() {
        let tool = ProjectGeneratorTool::new();
        let params = ProjectGeneratorParams {
            name: "HttpProject".to_string(),
            output_dir: std::env::temp_dir().to_string_lossy().to_string(),
            enable_http: Some(true),
            include_tests: Some(false),
            use_reactiveui: Some(false),
        };

        let result = tool.generate_project(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_project_endpoint_with_reactiveui() {
        let tool = ProjectGeneratorTool::new();
        let params = ProjectGeneratorParams {
            name: "ReactiveProject".to_string(),
            output_dir: std::env::temp_dir().to_string_lossy().to_string(),
            enable_http: Some(false),
            include_tests: Some(true),
            use_reactiveui: Some(true),
        };

        let result = tool.generate_project(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// XamlValidationTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod xaml_validation_tool_tests {
    use super::*;
    use xaml_validation_tool::{XamlValidationParams, XamlValidationTool};

    #[tokio::test]
    async fn test_validate_xaml_endpoint_with_valid_xaml() {
        let tool = XamlValidationTool::new();
        let params = XamlValidationParams {
            xaml_content: "<Grid><TextBlock Text=\"Hello\"/></Grid>".to_string(),
            strict_mode: Some(false),
            check_accessibility: Some(false),
        };

        let result = tool.validate_xaml(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_xaml_endpoint_with_mismatched_tags() {
        let tool = XamlValidationTool::new();
        let params = XamlValidationParams {
            xaml_content: "<Grid><TextBlock></Grid>".to_string(),
            strict_mode: Some(false),
            check_accessibility: Some(false),
        };

        let result = tool.validate_xaml(params).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        // Should contain error information in content
        assert!(!response.content.is_empty());
    }

    #[tokio::test]
    async fn test_validate_xaml_endpoint_with_strict_mode() {
        let tool = XamlValidationTool::new();
        let params = XamlValidationParams {
            xaml_content: "<Grid Background=\"#FF0000\"><Button/></Grid>".to_string(),
            strict_mode: Some(true),
            check_accessibility: Some(true),
        };

        let result = tool.validate_xaml(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_xaml_endpoint_with_empty_content() {
        let tool = XamlValidationTool::new();
        let params = XamlValidationParams {
            xaml_content: "".to_string(),
            strict_mode: None,
            check_accessibility: None,
        };

        let result = tool.validate_xaml(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_xaml_endpoint_with_self_closing_tags() {
        let tool = XamlValidationTool::new();
        let params = XamlValidationParams {
            xaml_content: "<Grid><TextBlock/><Button/></Grid>".to_string(),
            strict_mode: Some(false),
            check_accessibility: Some(false),
        };

        let result = tool.validate_xaml(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// SecurityPatternTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod security_pattern_tool_tests {
    use super::*;
    use security_pattern_tool::{SecurityPatternParams, SecurityPatternTool};

    #[tokio::test]
    async fn test_generate_security_pattern_endpoint_general() {
        let tool = SecurityPatternTool::new();
        let params = SecurityPatternParams {
            area: None,
            app_type: None,
            include_examples: Some(true),
        };

        let result = tool.generate_security_pattern(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_security_pattern_endpoint_authentication() {
        let tool = SecurityPatternTool::new();
        let params = SecurityPatternParams {
            area: Some("authentication".to_string()),
            app_type: Some("enterprise".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_security_pattern(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_security_pattern_endpoint_data_protection() {
        let tool = SecurityPatternTool::new();
        let params = SecurityPatternParams {
            area: Some("data-protection".to_string()),
            app_type: None,
            include_examples: Some(true),
        };

        let result = tool.generate_security_pattern(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_security_pattern_endpoint_without_examples() {
        let tool = SecurityPatternTool::new();
        let params = SecurityPatternParams {
            area: Some("general".to_string()),
            app_type: None,
            include_examples: Some(false),
        };

        let result = tool.generate_security_pattern(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// AccessibilityTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod accessibility_tool_tests {
    use super::*;
    use accessibility_tool::{AccessibilityParams, AccessibilityTool};

    #[tokio::test]
    async fn test_check_accessibility_endpoint_with_xaml() {
        let tool = AccessibilityTool::new();
        let params = AccessibilityParams {
            xaml_content: Some("<Image Source=\"test.png\"/>".to_string()),
            wcag_level: Some("AA".to_string()),
            include_guidance: Some(true),
        };

        let result = tool.check_accessibility(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_accessibility_endpoint_general_guidance() {
        let tool = AccessibilityTool::new();
        let params = AccessibilityParams {
            xaml_content: None,
            wcag_level: Some("AAA".to_string()),
            include_guidance: Some(true),
        };

        let result = tool.check_accessibility(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_accessibility_endpoint_without_guidance() {
        let tool = AccessibilityTool::new();
        let params = AccessibilityParams {
            xaml_content: None,
            wcag_level: Some("AA".to_string()),
            include_guidance: Some(false),
        };

        let result = tool.check_accessibility(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// PerformanceAnalysisTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod performance_analysis_tool_tests {
    use super::*;
    use performance_analysis_tool::{PerformanceAnalysisParams, PerformanceAnalysisTool};

    #[tokio::test]
    async fn test_analyze_performance_endpoint_startup() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceAnalysisParams {
            area: Some("startup".to_string()),
            include_profiling_code: Some(true),
            app_type: Some("desktop".to_string()),
        };

        let result = tool.analyze_performance(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_performance_endpoint_rendering() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceAnalysisParams {
            area: Some("rendering".to_string()),
            include_profiling_code: Some(true),
            app_type: None,
        };

        let result = tool.analyze_performance(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_performance_endpoint_memory() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceAnalysisParams {
            area: Some("memory".to_string()),
            include_profiling_code: Some(false),
            app_type: None,
        };

        let result = tool.analyze_performance(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_performance_endpoint_general() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceAnalysisParams {
            area: None,
            include_profiling_code: Some(true),
            app_type: None,
        };

        let result = tool.analyze_performance(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// AnimationTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod animation_tool_tests {
    use super::*;
    use animation_tool::{AnimationParams, AnimationTool};

    #[tokio::test]
    async fn test_generate_animation_endpoint_fade() {
        let tool = AnimationTool::new();
        let params = AnimationParams {
            animation_type: Some("fade".to_string()),
            include_examples: Some(true),
            platform: None,
        };

        let result = tool.generate_animation(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_animation_endpoint_slide() {
        let tool = AnimationTool::new();
        let params = AnimationParams {
            animation_type: Some("slide".to_string()),
            include_examples: Some(true),
            platform: None,
        };

        let result = tool.generate_animation(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_animation_endpoint_scale() {
        let tool = AnimationTool::new();
        let params = AnimationParams {
            animation_type: Some("scale".to_string()),
            include_examples: Some(true),
            platform: None,
        };

        let result = tool.generate_animation(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_animation_endpoint_rotate() {
        let tool = AnimationTool::new();
        let params = AnimationParams {
            animation_type: Some("rotate".to_string()),
            include_examples: Some(true),
            platform: None,
        };

        let result = tool.generate_animation(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_animation_endpoint_transitions() {
        let tool = AnimationTool::new();
        let params = AnimationParams {
            animation_type: Some("transition".to_string()),
            include_examples: Some(true),
            platform: None,
        };

        let result = tool.generate_animation(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// ThemingTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod theming_tool_tests {
    use super::*;
    use theming_tool::{ThemingParams, ThemingTool};

    #[tokio::test]
    async fn test_generate_theme_endpoint_light() {
        let tool = ThemingTool::new();
        let params = ThemingParams {
            theme_type: Some("light".to_string()),
            include_palette: Some(true),
            app_type: None,
        };

        let result = tool.generate_theme(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_theme_endpoint_dark() {
        let tool = ThemingTool::new();
        let params = ThemingParams {
            theme_type: Some("dark".to_string()),
            include_palette: Some(true),
            app_type: None,
        };

        let result = tool.generate_theme(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_theme_endpoint_fluent() {
        let tool = ThemingTool::new();
        let params = ThemingParams {
            theme_type: Some("fluent".to_string()),
            include_palette: Some(true),
            app_type: None,
        };

        let result = tool.generate_theme(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_theme_endpoint_material() {
        let tool = ThemingTool::new();
        let params = ThemingParams {
            theme_type: Some("material".to_string()),
            include_palette: Some(true),
            app_type: None,
        };

        let result = tool.generate_theme(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// LocalizationTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod localization_tool_tests {
    use super::*;
    use localization_tool::{LocalizationParams, LocalizationTool};

    #[tokio::test]
    async fn test_generate_localization_endpoint_with_locales() {
        let tool = LocalizationTool::new();
        let params = LocalizationParams {
            locales: Some(vec!["en-US".to_string(), "es-ES".to_string(), "fr-FR".to_string()]),
            include_examples: Some(true),
        };

        let result = tool.generate_localization(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_localization_endpoint_rtl() {
        let tool = LocalizationTool::new();
        let params = LocalizationParams {
            locales: Some(vec!["ar-SA".to_string(), "he-IL".to_string()]),
            include_examples: Some(true),
        };

        let result = tool.generate_localization(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_localization_endpoint_without_examples() {
        let tool = LocalizationTool::new();
        let params = LocalizationParams {
            locales: None,
            include_examples: Some(false),
        };

        let result = tool.generate_localization(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// DiagnosticTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod diagnostic_tool_tests {
    use super::*;
    use diagnostic_tool::{DiagnosticParams, DiagnosticTool};

    #[tokio::test]
    async fn test_run_diagnostics_endpoint_performance() {
        let tool = DiagnosticTool::new();
        let params = DiagnosticParams {
            area: Some("performance".to_string()),
            include_troubleshooting: Some(true),
        };

        let result = tool.run_diagnostics(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_diagnostics_endpoint_memory() {
        let tool = DiagnosticTool::new();
        let params = DiagnosticParams {
            area: Some("memory".to_string()),
            include_troubleshooting: Some(true),
        };

        let result = tool.run_diagnostics(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_diagnostics_endpoint_rendering() {
        let tool = DiagnosticTool::new();
        let params = DiagnosticParams {
            area: Some("rendering".to_string()),
            include_troubleshooting: Some(true),
        };

        let result = tool.run_diagnostics(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_diagnostics_endpoint_all() {
        let tool = DiagnosticTool::new();
        let params = DiagnosticParams {
            area: None,
            include_troubleshooting: Some(true),
        };

        let result = tool.run_diagnostics(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// APIIntegrationTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod api_integration_tool_tests {
    use super::*;
    use api_integration_tool::{APIIntegrationParams, APIIntegrationTool};

    #[tokio::test]
    async fn test_generate_api_integration_endpoint_rest() {
        let tool = APIIntegrationTool::new();
        let params = APIIntegrationParams {
            api_type: Some("rest".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_api_integration(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_api_integration_endpoint_graphql() {
        let tool = APIIntegrationTool::new();
        let params = APIIntegrationParams {
            api_type: Some("graphql".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_api_integration(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_api_integration_endpoint_grpc() {
        let tool = APIIntegrationTool::new();
        let params = APIIntegrationParams {
            api_type: Some("grpc".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_api_integration(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// TestingIntegrationTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod testing_integration_tool_tests {
    use super::*;
    use testing_integration_tool::{TestingIntegrationParams, TestingIntegrationTool};

    #[tokio::test]
    async fn test_generate_testing_integration_endpoint_unit() {
        let tool = TestingIntegrationTool::new();
        let params = TestingIntegrationParams {
            test_type: Some("unit".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_testing_integration(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_testing_integration_endpoint_ui() {
        let tool = TestingIntegrationTool::new();
        let params = TestingIntegrationParams {
            test_type: Some("ui".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_testing_integration(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_testing_integration_endpoint_integration() {
        let tool = TestingIntegrationTool::new();
        let params = TestingIntegrationParams {
            test_type: Some("integration".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_testing_integration(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// ArchitectureTemplateTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod architecture_template_tool_tests {
    use super::*;
    use architecture_template_tool::{ArchitectureTemplateParams, ArchitectureTemplateTool};

    #[tokio::test]
    async fn test_generate_architecture_template_endpoint_mvvm() {
        let tool = ArchitectureTemplateTool::new();
        let params = ArchitectureTemplateParams {
            pattern: Some("mvvm".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_architecture_template(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_architecture_template_endpoint_clean() {
        let tool = ArchitectureTemplateTool::new();
        let params = ArchitectureTemplateParams {
            pattern: Some("clean".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_architecture_template(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_architecture_template_endpoint_layered() {
        let tool = ArchitectureTemplateTool::new();
        let params = ArchitectureTemplateParams {
            pattern: Some("layered".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_architecture_template(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// ServiceLayerTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod service_layer_tool_tests {
    use super::*;
    use service_layer_tool::{ServiceLayerParams, ServiceLayerTool};

    #[tokio::test]
    async fn test_generate_service_layer_endpoint_repository() {
        let tool = ServiceLayerTool::new();
        let params = ServiceLayerParams {
            service_type: Some("repository".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_service_layer(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_service_layer_endpoint_unitofwork() {
        let tool = ServiceLayerTool::new();
        let params = ServiceLayerParams {
            service_type: Some("unitofwork".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_service_layer(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_service_layer_endpoint_mediator() {
        let tool = ServiceLayerTool::new();
        let params = ServiceLayerParams {
            service_type: Some("mediator".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_service_layer(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// DataAccessPatternTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod data_access_pattern_tool_tests {
    use super::*;
    use data_access_pattern_tool::{DataAccessPatternParams, DataAccessPatternTool};

    #[tokio::test]
    async fn test_generate_data_access_pattern_endpoint_efcore() {
        let tool = DataAccessPatternTool::new();
        let params = DataAccessPatternParams {
            pattern: Some("efcore".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_data_access_pattern(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_data_access_pattern_endpoint_dapper() {
        let tool = DataAccessPatternTool::new();
        let params = DataAccessPatternParams {
            pattern: Some("dapper".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_data_access_pattern(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_data_access_pattern_endpoint_repository() {
        let tool = DataAccessPatternTool::new();
        let params = DataAccessPatternParams {
            pattern: Some("repository".to_string()),
            include_examples: Some(true),
        };

        let result = tool.generate_data_access_pattern(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// CustomControlGenerator Endpoint Tests
// ============================================================================

#[cfg(test)]
mod custom_control_generator_tests {
    use super::*;
    use custom_control_generator::{CustomControlGenerator, CustomControlGeneratorParams};

    #[tokio::test]
    async fn test_generate_custom_control_endpoint_templated() {
        let tool = CustomControlGenerator::new();
        let params = CustomControlGeneratorParams {
            control_type: Some("templated".to_string()),
            control_name: Some("CustomButton".to_string()),
            include_styles: Some(true),
        };

        let result = tool.generate_custom_control(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_custom_control_endpoint_composite() {
        let tool = CustomControlGenerator::new();
        let params = CustomControlGeneratorParams {
            control_type: Some("composite".to_string()),
            control_name: Some("SearchBox".to_string()),
            include_styles: Some(true),
        };

        let result = tool.generate_custom_control(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_custom_control_endpoint_attached() {
        let tool = CustomControlGenerator::new();
        let params = CustomControlGeneratorParams {
            control_type: Some("attached".to_string()),
            control_name: Some("Behavior".to_string()),
            include_styles: Some(false),
        };

        let result = tool.generate_custom_control(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// DebuggingAssistantTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod debugging_assistant_tool_tests {
    use super::*;
    use debugging_assistant_tool::{DebuggingAssistantParams, DebuggingAssistantTool};

    #[tokio::test]
    async fn test_provide_debugging_assistance_endpoint_binding() {
        let tool = DebuggingAssistantTool::new();
        let params = DebuggingAssistantParams {
            issue_type: Some("binding".to_string()),
            include_solutions: Some(true),
        };

        let result = tool.provide_debugging_assistance(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_provide_debugging_assistance_endpoint_layout() {
        let tool = DebuggingAssistantTool::new();
        let params = DebuggingAssistantParams {
            issue_type: Some("layout".to_string()),
            include_solutions: Some(true),
        };

        let result = tool.provide_debugging_assistance(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_provide_debugging_assistance_endpoint_performance() {
        let tool = DebuggingAssistantTool::new();
        let params = DebuggingAssistantParams {
            issue_type: Some("performance".to_string()),
            include_solutions: Some(true),
        };

        let result = tool.provide_debugging_assistance(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_provide_debugging_assistance_endpoint_memory() {
        let tool = DebuggingAssistantTool::new();
        let params = DebuggingAssistantParams {
            issue_type: Some("memory".to_string()),
            include_solutions: Some(true),
        };

        let result = tool.provide_debugging_assistance(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// UIUXDesignTool Endpoint Tests
// ============================================================================

#[cfg(test)]
mod uiux_design_tool_tests {
    use super::*;
    use uiux_design_tool::{UIUXDesignParams, UIUXDesignTool};

    #[tokio::test]
    async fn test_provide_uiux_design_endpoint_layout() {
        let tool = UIUXDesignTool::new();
        let params = UIUXDesignParams {
            design_aspect: Some("layout".to_string()),
            include_examples: Some(true),
        };

        let result = tool.provide_uiux_design(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_provide_uiux_design_endpoint_color() {
        let tool = UIUXDesignTool::new();
        let params = UIUXDesignParams {
            design_aspect: Some("color".to_string()),
            include_examples: Some(true),
        };

        let result = tool.provide_uiux_design(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_provide_uiux_design_endpoint_typography() {
        let tool = UIUXDesignTool::new();
        let params = UIUXDesignParams {
            design_aspect: Some("typography".to_string()),
            include_examples: Some(true),
        };

        let result = tool.provide_uiux_design(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_provide_uiux_design_endpoint_accessibility() {
        let tool = UIUXDesignTool::new();
        let params = UIUXDesignParams {
            design_aspect: Some("accessibility".to_string()),
            include_examples: Some(true),
        };

        let result = tool.provide_uiux_design(params).await;
        assert!(result.is_ok());
    }
}
// New tests will be appended 
