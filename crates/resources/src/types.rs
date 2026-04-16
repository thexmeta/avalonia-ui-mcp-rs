//! Resource types for knowledge base
//!
//! This module defines the data structures for the embedded knowledge base.

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// AvaloniaUI control reference
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ControlReference {
    /// Control name (e.g., "Button", "TextBox")
    pub name: String,
    /// Control category (e.g., "Input", "Layout", "Display")
    pub category: String,
    /// Brief description
    pub description: String,
    /// Full documentation
    pub documentation: String,
    /// XAML namespace
    pub namespace: String,
    /// Common properties
    pub common_properties: Vec<PropertyReference>,
    /// XAML usage example
    pub xaml_example: String,
    /// Related controls
    pub related_controls: Vec<String>,
    /// Migration notes from WPF (if applicable)
    pub wpf_migration_notes: Option<String>,
}

/// Property reference
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PropertyReference {
    /// Property name
    pub name: String,
    /// Property type
    pub property_type: String,
    /// Description
    pub description: String,
    /// Default value
    pub default_value: Option<String>,
    /// Is required
    pub is_required: bool,
}

/// XAML pattern reference
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct XamlPattern {
    /// Pattern name
    pub name: String,
    /// Pattern category
    pub category: String,
    /// Description
    pub description: String,
    /// XAML template
    pub xaml_template: String,
    /// Usage guidance
    pub usage_guidance: String,
    /// Related patterns
    pub related_patterns: Vec<String>,
}

/// WPF to Avalonia migration mapping
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WpfMigrationMapping {
    /// WPF control/feature name
    pub wpf_name: String,
    /// Avalonia equivalent
    pub avalonia_name: String,
    /// Migration difficulty (Easy, Medium, Hard)
    pub difficulty: String,
    /// Migration notes
    pub notes: String,
    /// Code changes required
    pub code_changes: Vec<String>,
    /// XAML changes required
    pub xaml_changes: Vec<String>,
}

/// Knowledge base container
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KnowledgeBase {
    /// Control references
    pub controls: Vec<ControlReference>,
    /// XAML patterns
    pub xaml_patterns: Vec<XamlPattern>,
    /// WPF migration mappings
    pub wpf_migration: Vec<WpfMigrationMapping>,
}

impl KnowledgeBase {
    /// Find a control by name
    pub fn find_control(&self, name: &str) -> Option<&ControlReference> {
        self.controls.iter().find(|c| c.name.eq_ignore_ascii_case(name))
    }

    /// Find controls by category
    pub fn find_controls_by_category(&self, category: &str) -> Vec<&ControlReference> {
        self.controls
            .iter()
            .filter(|c| c.category.eq_ignore_ascii_case(category))
            .collect()
    }

    /// Find a XAML pattern by name
    pub fn find_pattern(&self, name: &str) -> Option<&XamlPattern> {
        self.xaml_patterns
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(name))
    }

    /// Find WPF migration mapping
    pub fn find_wpf_mapping(&self, wpf_name: &str) -> Option<&WpfMigrationMapping> {
        self.wpf_migration
            .iter()
            .find(|m| m.wpf_name.eq_ignore_ascii_case(wpf_name))
    }

    /// Get all control categories
    pub fn get_control_categories(&self) -> Vec<&str> {
        let mut categories: Vec<&str> = self.controls.iter().map(|c| c.category.as_str()).collect();
        categories.sort();
        categories.dedup();
        categories
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_kb() -> KnowledgeBase {
        KnowledgeBase {
            controls: vec![
                ControlReference {
                    name: "Button".to_string(),
                    category: "Input".to_string(),
                    description: "A button control".to_string(),
                    documentation: "Represents a button".to_string(),
                    namespace: "Avalonia.Controls".to_string(),
                    common_properties: vec![],
                    xaml_example: "<Button Content=\"Click Me\"/>".to_string(),
                    related_controls: vec!["ToggleButton".to_string()],
                    wpf_migration_notes: Some("Similar to WPF Button".to_string()),
                },
                ControlReference {
                    name: "TextBox".to_string(),
                    category: "Input".to_string(),
                    description: "A text input control".to_string(),
                    documentation: "Represents a text box".to_string(),
                    namespace: "Avalonia.Controls".to_string(),
                    common_properties: vec![],
                    xaml_example: "<TextBox Text=\"Hello\"/>".to_string(),
                    related_controls: vec!["TextArea".to_string()],
                    wpf_migration_notes: Some("Similar to WPF TextBox".to_string()),
                },
            ],
            xaml_patterns: vec![],
            wpf_migration: vec![],
        }
    }

    #[test]
    fn test_find_control() {
        let kb = create_test_kb();
        assert!(kb.find_control("Button").is_some());
        assert!(kb.find_control("button").is_some()); // Case insensitive
        assert!(kb.find_control("NonExistent").is_none());
    }

    #[test]
    fn test_find_controls_by_category() {
        let kb = create_test_kb();
        let input_controls = kb.find_controls_by_category("Input");
        assert_eq!(input_controls.len(), 2);
    }

    #[test]
    fn test_get_control_categories() {
        let kb = create_test_kb();
        let categories = kb.get_control_categories();
        assert_eq!(categories.len(), 1);
        assert!(categories.contains(&"Input"));
    }
}
