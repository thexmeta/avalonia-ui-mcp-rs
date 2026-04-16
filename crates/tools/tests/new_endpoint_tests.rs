// ============================================================================
// NEW: Integration tests for 29 newly added tool endpoints
// ============================================================================

// ============================================================================
// XamlValidationTool - convert_wpf_xaml_to_avalonia
// ============================================================================
#[cfg(test)]
mod convert_wpf_xaml_tests {
    use avalonia_mcp_tools::xaml_validation_tool::{WpfConversionParams, XamlValidationTool};

    #[tokio::test]
    async fn test_convert_wpf_xaml_endpoint_with_valid_xaml() {
        let tool = XamlValidationTool::new();
        let params = WpfConversionParams {
            wpf_xaml: "<Window xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\n    <Button Content='Click'/>\n</Window>".to_string(),
        };
        let result = tool.convert_wpf_xaml_to_avalonia(params).await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(!resp.content.is_empty());
    }

    #[tokio::test]
    async fn test_convert_wpf_xaml_endpoint_empty() {
        let tool = XamlValidationTool::new();
        let params = WpfConversionParams { wpf_xaml: "".to_string() };
        let result = tool.convert_wpf_xaml_to_avalonia(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// AnimationTool - generate_page_transition
// ============================================================================
#[cfg(test)]
mod page_transition_tests {
    use avalonia_mcp_tools::animation_tool::{AnimationTool, PageTransitionParams};

    #[tokio::test]
    async fn test_generate_page_transition_endpoint_slide() {
        let tool = AnimationTool::new();
        let params = PageTransitionParams {
            transition_type: Some("slide".to_string()), direction: Some("left".to_string()), duration: Some(350),
        };
        let result = tool.generate_page_transition(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_page_transition_endpoint_fade() {
        let tool = AnimationTool::new();
        let params = PageTransitionParams {
            transition_type: Some("fade".to_string()), direction: None, duration: None,
        };
        let result = tool.generate_page_transition(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// AnimationTool - generate_storyboard
// ============================================================================
#[cfg(test)]
mod storyboard_endpoint_tests {
    use avalonia_mcp_tools::animation_tool::{AnimationTool, StoryboardParams};

    #[tokio::test]
    async fn test_generate_storyboard_endpoint() {
        let tool = AnimationTool::new();
        let params = StoryboardParams {
            sequence: "fade in button then slide panel".to_string(),
            total_duration: Some(1000), storyboard_name: Some("TestStoryboard".to_string()),
        };
        let result = tool.generate_storyboard(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_storyboard_endpoint_empty_sequence() {
        let tool = AnimationTool::new();
        let params = StoryboardParams { sequence: "".to_string(), total_duration: None, storyboard_name: None };
        let result = tool.generate_storyboard(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// AnimationTool - generate_custom_animation
// ============================================================================
#[cfg(test)]
mod custom_animation_endpoint_tests {
    use avalonia_mcp_tools::animation_tool::{AnimationTool, CustomAnimationParams};

    #[tokio::test]
    async fn test_generate_custom_animation_endpoint() {
        let tool = AnimationTool::new();
        let params = CustomAnimationParams {
            effect_name: "WaveEffect".to_string(), properties: "Opacity,RenderTransform".to_string(),
            pattern: Some("wave".to_string()), complexity: Some("moderate".to_string()),
        };
        let result = tool.generate_custom_animation(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_custom_animation_endpoint_empty_name() {
        let tool = AnimationTool::new();
        let params = CustomAnimationParams {
            effect_name: "".to_string(), properties: "Opacity".to_string(), pattern: None, complexity: None,
        };
        let result = tool.generate_custom_animation(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// APIIntegrationTool - generate_api_models
// ============================================================================
#[cfg(test)]
mod api_models_endpoint_tests {
    use avalonia_mcp_tools::api_integration_tool::{APIIntegrationTool, ApiModelsParams};

    #[tokio::test]
    async fn test_generate_api_models_endpoint() {
        let tool = APIIntegrationTool::new();
        let params = ApiModelsParams {
            entity_name: "User".to_string(), model_type: Some("dto".to_string()), include_validation: Some(true),
        };
        let result = tool.generate_api_models(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_api_models_endpoint_empty() {
        let tool = APIIntegrationTool::new();
        let params = ApiModelsParams { entity_name: "".to_string(), model_type: None, include_validation: None };
        let result = tool.generate_api_models(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// DiagnosticTool - get_server_metrics
// ============================================================================
#[cfg(test)]
mod server_metrics_endpoint_tests {
    use avalonia_mcp_tools::diagnostic_tool::DiagnosticTool;

    #[tokio::test]
    async fn test_get_server_metrics_endpoint() {
        let tool = DiagnosticTool::new();
        let result = tool.get_server_metrics().await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(!resp.content.is_empty());
    }
}

// ============================================================================
// DiagnosticTool - perform_health_check
// ============================================================================
#[cfg(test)]
mod health_check_endpoint_tests {
    use avalonia_mcp_tools::diagnostic_tool::DiagnosticTool;

    #[tokio::test]
    async fn test_perform_health_check_endpoint() {
        let tool = DiagnosticTool::new();
        let result = tool.perform_health_check().await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(!resp.content.is_empty());
    }
}

// ============================================================================
// DiagnosticTool - test_logging
// ============================================================================
#[cfg(test)]
mod test_logging_endpoint_tests {
    use avalonia_mcp_tools::diagnostic_tool::{DiagnosticTool, TestLoggingParams};

    #[tokio::test]
    async fn test_test_logging_endpoint_defaults() {
        let tool = DiagnosticTool::new();
        let params = TestLoggingParams { log_level: None, message: None };
        let result = tool.test_logging(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_test_logging_endpoint_custom() {
        let tool = DiagnosticTool::new();
        let params = TestLoggingParams {
            log_level: Some("debug".to_string()), message: Some("Custom test message".to_string()),
        };
        let result = tool.test_logging(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// LocalizationTool - generate_culture_formatting
// ============================================================================
#[cfg(test)]
mod culture_formatting_endpoint_tests {
    use avalonia_mcp_tools::localization_tool::{CultureFormattingParams, LocalizationTool};

    #[tokio::test]
    async fn test_generate_culture_formatting_endpoint_date() {
        let tool = LocalizationTool::new();
        let params = CultureFormattingParams {
            culture_code: Some("en-US".to_string()), format_type: Some("date".to_string()),
        };
        let result = tool.generate_culture_formatting(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_culture_formatting_endpoint_currency() {
        let tool = LocalizationTool::new();
        let params = CultureFormattingParams {
            culture_code: Some("de-DE".to_string()), format_type: Some("currency".to_string()),
        };
        let result = tool.generate_culture_formatting(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// DataAccessPatternTool - generate_async_data_access
// ============================================================================
#[cfg(test)]
mod async_data_access_endpoint_tests {
    use avalonia_mcp_tools::data_access_pattern_tool::{AsyncDataAccessParams, DataAccessPatternTool};

    #[tokio::test]
    async fn test_generate_async_data_access_endpoint() {
        let tool = DataAccessPatternTool::new();
        let params = AsyncDataAccessParams {
            service_name: "UserDataService".to_string(), include_caching: Some(true),
            include_retry: Some(true), caching_provider: Some("memory".to_string()),
        };
        let result = tool.generate_async_data_access(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_async_data_access_endpoint_empty() {
        let tool = DataAccessPatternTool::new();
        let params = AsyncDataAccessParams {
            service_name: "".to_string(), include_caching: None, include_retry: None, caching_provider: None,
        };
        let result = tool.generate_async_data_access(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// DebuggingAssistantTool - generate_debug_utilities
// ============================================================================
#[cfg(test)]
mod debug_utilities_endpoint_tests {
    use avalonia_mcp_tools::debugging_assistant_tool::{DebugUtilitiesParams, DebuggingAssistantTool};

    #[tokio::test]
    async fn test_generate_debug_utilities_endpoint_logger() {
        let tool = DebuggingAssistantTool::new();
        let params = DebugUtilitiesParams {
            utility_type: Some("logger".to_string()), include_devtools: Some(true), include_telemetry: Some(false),
        };
        let result = tool.generate_debug_utilities(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_debug_utilities_endpoint_visualtree() {
        let tool = DebuggingAssistantTool::new();
        let params = DebugUtilitiesParams {
            utility_type: Some("visualtree".to_string()), include_devtools: Some(false), include_telemetry: Some(false),
        };
        let result = tool.generate_debug_utilities(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// SecurityPatternTool - generate_data_security_pattern
// ============================================================================
#[cfg(test)]
mod data_security_endpoint_tests {
    use avalonia_mcp_tools::security_pattern_tool::{DataSecurityParams, SecurityPatternTool};

    #[tokio::test]
    async fn test_generate_data_security_pattern_endpoint() {
        let tool = SecurityPatternTool::new();
        let params = DataSecurityParams {
            security_area: Some("encryption".to_string()), include_encryption: Some(true), include_audit_logging: Some(true),
        };
        let result = tool.generate_data_security_pattern(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_data_security_pattern_endpoint_minimal() {
        let tool = SecurityPatternTool::new();
        let params = DataSecurityParams {
            security_area: None, include_encryption: Some(false), include_audit_logging: Some(false),
        };
        let result = tool.generate_data_security_pattern(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// CustomControlGenerator - generate_control_template
// ============================================================================
#[cfg(test)]
mod control_template_endpoint_tests {
    use avalonia_mcp_tools::custom_control_generator::{ControlTemplateParams, CustomControlGenerator};

    #[tokio::test]
    async fn test_generate_control_template_endpoint() {
        let tool = CustomControlGenerator::new();
        let params = ControlTemplateParams {
            target_control: "Button".to_string(), template_name: "CustomButtonTemplate".to_string(),
            visual_states: Some("Normal,PointerOver,Pressed".to_string()), include_animations: Some(true),
        };
        let result = tool.generate_control_template(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_control_template_endpoint_empty() {
        let tool = CustomControlGenerator::new();
        let params = ControlTemplateParams {
            target_control: "".to_string(), template_name: "Test".to_string(), visual_states: None, include_animations: None,
        };
        let result = tool.generate_control_template(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// CustomControlGenerator - generate_attached_property
// ============================================================================
#[cfg(test)]
mod attached_property_endpoint_tests {
    use avalonia_mcp_tools::custom_control_generator::{AttachedPropertyParams, CustomControlGenerator};

    #[tokio::test]
    async fn test_generate_attached_property_endpoint() {
        let tool = CustomControlGenerator::new();
        let params = AttachedPropertyParams {
            property_name: "IsLoading".to_string(), property_type: Some("bool".to_string()),
            target_controls: Some("Control".to_string()), include_handler: Some(true),
        };
        let result = tool.generate_attached_property(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_attached_property_endpoint_empty() {
        let tool = CustomControlGenerator::new();
        let params = AttachedPropertyParams {
            property_name: "".to_string(), property_type: None, target_controls: None, include_handler: None,
        };
        let result = tool.generate_attached_property(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// CustomControlGenerator - generate_layout_panel
// ============================================================================
#[cfg(test)]
mod layout_panel_endpoint_tests {
    use avalonia_mcp_tools::custom_control_generator::{CustomControlGenerator, LayoutPanelParams};

    #[tokio::test]
    async fn test_generate_layout_panel_endpoint() {
        let tool = CustomControlGenerator::new();
        let params = LayoutPanelParams {
            panel_name: "CustomWrapPanel".to_string(), orientation: Some("wrap".to_string()), include_spacing: Some(true),
        };
        let result = tool.generate_layout_panel(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_layout_panel_endpoint_empty() {
        let tool = CustomControlGenerator::new();
        let params = LayoutPanelParams {
            panel_name: "".to_string(), orientation: None, include_spacing: None,
        };
        let result = tool.generate_layout_panel(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// ThemingTool - generate_selectors
// ============================================================================
#[cfg(test)]
mod selectors_endpoint_tests {
    use avalonia_mcp_tools::theming_tool::{SelectorsParams, ThemingTool};

    #[tokio::test]
    async fn test_generate_selectors_endpoint_class() {
        let tool = ThemingTool::new();
        let params = SelectorsParams { selector_type: Some("class".to_string()), target_control: Some("Button".to_string()) };
        let result = tool.generate_selectors(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_selectors_endpoint_type() {
        let tool = ThemingTool::new();
        let params = SelectorsParams { selector_type: Some("type".to_string()), target_control: Some("TextBox".to_string()) };
        let result = tool.generate_selectors(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// ThemingTool - generate_color_scheme
// ============================================================================
#[cfg(test)]
mod color_scheme_endpoint_tests {
    use avalonia_mcp_tools::theming_tool::{ColorSchemeParams, ThemingTool};

    #[tokio::test]
    async fn test_generate_color_scheme_endpoint() {
        let tool = ThemingTool::new();
        let params = ColorSchemeParams { base_color: Some("#FF5722".to_string()), format: Some("hex".to_string()) };
        let result = tool.generate_color_scheme(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_color_scheme_endpoint_defaults() {
        let tool = ThemingTool::new();
        let params = ColorSchemeParams { base_color: None, format: None };
        let result = tool.generate_color_scheme(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// UIUXDesignTool - generate_ux_patterns
// ============================================================================
#[cfg(test)]
mod ux_patterns_endpoint_tests {
    use avalonia_mcp_tools::uiux_design_tool::{UIUXDesignTool, UXPatternsParams};

    #[tokio::test]
    async fn test_generate_ux_patterns_endpoint_loading() {
        let tool = UIUXDesignTool::new();
        let params = UXPatternsParams { pattern_type: Some("loading".to_string()), include_examples: Some(true) };
        let result = tool.generate_ux_patterns(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_ux_patterns_endpoint_empty_state() {
        let tool = UIUXDesignTool::new();
        let params = UXPatternsParams { pattern_type: Some("empty_state".to_string()), include_examples: Some(true) };
        let result = tool.generate_ux_patterns(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// UIUXDesignTool - generate_design_system
// ============================================================================
#[cfg(test)]
mod design_system_endpoint_tests {
    use avalonia_mcp_tools::uiux_design_tool::{DesignSystemParams, UIUXDesignTool};

    #[tokio::test]
    async fn test_generate_design_system_endpoint() {
        let tool = UIUXDesignTool::new();
        let params = DesignSystemParams { project_name: "MyDesignSystem".to_string(), include_tokens: Some(true) };
        let result = tool.generate_design_system(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_design_system_endpoint_empty() {
        let tool = UIUXDesignTool::new();
        let params = DesignSystemParams { project_name: "".to_string(), include_tokens: None };
        let result = tool.generate_design_system(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// ArchitectureTemplateTool - generate_microservices_architecture
// ============================================================================
#[cfg(test)]
mod microservices_endpoint_tests {
    use avalonia_mcp_tools::architecture_template_tool::{ArchitectureTemplateTool, MicroservicesParams};

    #[tokio::test]
    async fn test_generate_microservices_architecture_endpoint() {
        let tool = ArchitectureTemplateTool::new();
        let params = MicroservicesParams {
            app_name: "MyApp".to_string(), services: Some("user,order".to_string()), include_gateway: Some(true),
        };
        let result = tool.generate_microservices_architecture(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_microservices_architecture_endpoint_empty() {
        let tool = ArchitectureTemplateTool::new();
        let params = MicroservicesParams { app_name: "".to_string(), services: None, include_gateway: None };
        let result = tool.generate_microservices_architecture(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// ArchitectureTemplateTool - generate_ddd_architecture
// ============================================================================
#[cfg(test)]
mod ddd_architecture_endpoint_tests {
    use avalonia_mcp_tools::architecture_template_tool::{ArchitectureTemplateTool, DDDParams};

    #[tokio::test]
    async fn test_generate_ddd_architecture_endpoint() {
        let tool = ArchitectureTemplateTool::new();
        let params = DDDParams {
            domain_name: "ECommerce".to_string(), bounded_contexts: Some("sales,inventory".to_string()), include_cqrs: Some(true),
        };
        let result = tool.generate_ddd_architecture(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_ddd_architecture_endpoint_empty() {
        let tool = ArchitectureTemplateTool::new();
        let params = DDDParams { domain_name: "".to_string(), bounded_contexts: None, include_cqrs: None };
        let result = tool.generate_ddd_architecture(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// ArchitectureTemplateTool - generate_plugin_architecture
// ============================================================================
#[cfg(test)]
mod plugin_architecture_endpoint_tests {
    use avalonia_mcp_tools::architecture_template_tool::{ArchitectureTemplateTool, PluginParams};

    #[tokio::test]
    async fn test_generate_plugin_architecture_endpoint() {
        let tool = ArchitectureTemplateTool::new();
        let params = PluginParams {
            app_name: "PluginApp".to_string(), plugin_types: Some("editor,exporter".to_string()), include_hot_reload: Some(false),
        };
        let result = tool.generate_plugin_architecture(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_plugin_architecture_endpoint_empty() {
        let tool = ArchitectureTemplateTool::new();
        let params = PluginParams { app_name: "".to_string(), plugin_types: None, include_hot_reload: None };
        let result = tool.generate_plugin_architecture(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// PerformanceAnalysisTool - get_performance_recommendations
// ============================================================================
#[cfg(test)]
mod performance_recommendations_endpoint_tests {
    use avalonia_mcp_tools::performance_analysis_tool::{PerformanceAnalysisTool, PerformanceRecommendationsParams};

    #[tokio::test]
    async fn test_get_performance_recommendations_endpoint_all() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceRecommendationsParams { area: Some("all".to_string()), include_code: Some(true) };
        let result = tool.get_performance_recommendations(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_performance_recommendations_endpoint_bindings() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceRecommendationsParams { area: Some("bindings".to_string()), include_code: Some(true) };
        let result = tool.get_performance_recommendations(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_performance_recommendations_endpoint_rendering() {
        let tool = PerformanceAnalysisTool::new();
        let params = PerformanceRecommendationsParams { area: Some("rendering".to_string()), include_code: Some(false) };
        let result = tool.get_performance_recommendations(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// AccessibilityTool - generate_accessible_component
// ============================================================================
#[cfg(test)]
mod accessible_component_endpoint_tests {
    use avalonia_mcp_tools::accessibility_tool::{AccessibilityTool, AccessibleComponentParams};

    #[tokio::test]
    async fn test_generate_accessible_component_endpoint_button() {
        let tool = AccessibilityTool::new();
        let params = AccessibleComponentParams { component_type: Some("button".to_string()), include_keyboard_nav: Some(true) };
        let result = tool.generate_accessible_component(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_accessible_component_endpoint_input() {
        let tool = AccessibilityTool::new();
        let params = AccessibleComponentParams { component_type: Some("input".to_string()), include_keyboard_nav: Some(false) };
        let result = tool.generate_accessible_component(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_accessible_component_endpoint_dialog() {
        let tool = AccessibilityTool::new();
        let params = AccessibleComponentParams { component_type: Some("dialog".to_string()), include_keyboard_nav: Some(true) };
        let result = tool.generate_accessible_component(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// ServiceLayerTool - generate_domain_service
// ============================================================================
#[cfg(test)]
mod domain_service_endpoint_tests {
    use avalonia_mcp_tools::service_layer_tool::{DomainServiceParams, ServiceLayerTool};

    #[tokio::test]
    async fn test_generate_domain_service_endpoint() {
        let tool = ServiceLayerTool::new();
        let params = DomainServiceParams {
            domain_name: "Order".to_string(), include_validation: Some(true), include_events: Some(true),
        };
        let result = tool.generate_domain_service(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_domain_service_endpoint_empty() {
        let tool = ServiceLayerTool::new();
        let params = DomainServiceParams { domain_name: "".to_string(), include_validation: None, include_events: None };
        let result = tool.generate_domain_service(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// TestingIntegrationTool - generate_ui_automation_tests
// ============================================================================
#[cfg(test)]
mod ui_automation_tests_endpoint_tests {
    use avalonia_mcp_tools::testing_integration_tool::{TestingIntegrationTool, UITestParams};

    #[tokio::test]
    async fn test_generate_ui_automation_tests_endpoint() {
        let tool = TestingIntegrationTool::new();
        let params = UITestParams { test_type: Some("button_click".to_string()), include_page_objects: Some(true) };
        let result = tool.generate_ui_automation_tests(params).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// TestingIntegrationTool - generate_mocks_and_builders
// ============================================================================
#[cfg(test)]
mod mocks_and_builders_endpoint_tests {
    use avalonia_mcp_tools::testing_integration_tool::{MocksAndBuildersParams, TestingIntegrationTool};

    #[tokio::test]
    async fn test_generate_mocks_and_builders_endpoint() {
        let tool = TestingIntegrationTool::new();
        let params = MocksAndBuildersParams { entity_name: "User".to_string(), include_fluent_builder: Some(true) };
        let result = tool.generate_mocks_and_builders(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_mocks_and_builders_endpoint_empty() {
        let tool = TestingIntegrationTool::new();
        let params = MocksAndBuildersParams { entity_name: "".to_string(), include_fluent_builder: None };
        let result = tool.generate_mocks_and_builders(params).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// TestingIntegrationTool - generate_performance_tests
// ============================================================================
#[cfg(test)]
mod performance_tests_endpoint_tests {
    use avalonia_mcp_tools::testing_integration_tool::{PerformanceTestParams, TestingIntegrationTool};

    #[tokio::test]
    async fn test_generate_performance_tests_endpoint() {
        let tool = TestingIntegrationTool::new();
        let params = PerformanceTestParams { test_area: Some("rendering".to_string()), include_profiling: Some(true) };
        let result = tool.generate_performance_tests(params).await;
        assert!(result.is_ok());
    }
}
