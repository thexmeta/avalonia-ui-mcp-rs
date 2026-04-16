//! Resource loader for knowledge base
//!
//! This module provides loading and caching of embedded knowledge base resources.

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use avalonia_mcp_core::error::AvaloniaMcpError;
use crate::types::KnowledgeBase;

/// Resource loader service
pub struct ResourceLoader {
    cache: Arc<RwLock<Option<KnowledgeBase>>>,
}

impl ResourceLoader {
    /// Create a new resource loader
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Load knowledge base from embedded resources
    pub async fn load_knowledge_base(&self) -> Result<KnowledgeBase, AvaloniaMcpError> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(kb) = cache.as_ref() {
                info!("Loading knowledge base from cache");
                return Ok(kb.clone());
            }
        }

        // Load from embedded resources
        info!("Loading knowledge base from embedded resources");
        let kb = self.load_from_embedded().await?;

        // Cache the result
        {
            let mut cache = self.cache.write().await;
            *cache = Some(kb.clone());
        }

        Ok(kb)
    }

    /// Load from embedded resources (compile-time included)
    async fn load_from_embedded(&self) -> Result<KnowledgeBase, AvaloniaMcpError> {
        // For now, return a minimal knowledge base
        // In production, this would load from embedded JSON files
        Ok(KnowledgeBase {
            controls: Self::load_default_controls(),
            xaml_patterns: Self::load_default_patterns(),
            wpf_migration: Self::load_default_wpf_migration(),
        })
    }

    /// Load default control references
    fn load_default_controls() -> Vec<crate::types::ControlReference> {
        vec![
            crate::types::ControlReference {
                name: "Button".to_string(),
                category: "Input".to_string(),
                description: "A standard button control that responds to user clicks".to_string(),
                documentation: "The Button control provides a Click event that is raised when the user clicks the button. It can contain content such as text or images.".to_string(),
                namespace: "Avalonia.Controls".to_string(),
                common_properties: vec![
                    crate::types::PropertyReference {
                        name: "Content".to_string(),
                        property_type: "object".to_string(),
                        description: "The content to display inside the button".to_string(),
                        default_value: Some("null".to_string()),
                        is_required: false,
                    },
                    crate::types::PropertyReference {
                        name: "Command".to_string(),
                        property_type: "ICommand".to_string(),
                        description: "The command to execute when the button is clicked".to_string(),
                        default_value: Some("null".to_string()),
                        is_required: false,
                    },
                    crate::types::PropertyReference {
                        name: "IsEnabled".to_string(),
                        property_type: "bool".to_string(),
                        description: "Gets or sets whether the button is enabled".to_string(),
                        default_value: Some("true".to_string()),
                        is_required: false,
                    },
                ],
                xaml_example: r#"<Button Content="Click Me" Click="Button_OnClick"/>
<Button Command="{Binding MyCommand}" CommandParameter="parameter"/>"#.to_string(),
                related_controls: vec!["ToggleButton".to_string(), "RepeatButton".to_string()],
                wpf_migration_notes: Some("Button in Avalonia works similarly to WPF. The main difference is that Avalonia uses standard .NET events instead of routed events.".to_string()),
            },
            crate::types::ControlReference {
                name: "TextBox".to_string(),
                category: "Input".to_string(),
                description: "A control for displaying and editing single-line text".to_string(),
                documentation: "The TextBox control is used to display and edit text. It supports watermarks, validation, and various text input modes.".to_string(),
                namespace: "Avalonia.Controls".to_string(),
                common_properties: vec![
                    crate::types::PropertyReference {
                        name: "Text".to_string(),
                        property_type: "string".to_string(),
                        description: "The text content of the TextBox".to_string(),
                        default_value: Some("string.Empty".to_string()),
                        is_required: false,
                    },
                    crate::types::PropertyReference {
                        name: "Watermark".to_string(),
                        property_type: "string".to_string(),
                        description: "Placeholder text displayed when the TextBox is empty".to_string(),
                        default_value: Some("null".to_string()),
                        is_required: false,
                    },
                    crate::types::PropertyReference {
                        name: "IsReadOnly".to_string(),
                        property_type: "bool".to_string(),
                        description: "Gets or sets whether the text is read-only".to_string(),
                        default_value: Some("false".to_string()),
                        is_required: false,
                    },
                    crate::types::PropertyReference {
                        name: "MaxLength".to_string(),
                        property_type: "int".to_string(),
                        description: "Maximum number of characters allowed".to_string(),
                        default_value: Some("int.MaxValue".to_string()),
                        is_required: false,
                    },
                ],
                xaml_example: r#"<TextBox Text="{Binding Name}" Watermark="Enter name"/>
<TextBox Text="{Binding Email}" Classes="floatingWatermark"/>"#.to_string(),
                related_controls: vec!["TextArea".to_string(), "AutoCompleteBox".to_string()],
                wpf_migration_notes: Some("TextBox in Avalonia is similar to WPF. Key differences: 1) No TextWrapping property (use AcceptsReturn instead), 2) Watermark is a native property in Avalonia.".to_string()),
            },
            crate::types::ControlReference {
                name: "Grid".to_string(),
                category: "Layout".to_string(),
                description: "A layout control that arranges child elements in rows and columns".to_string(),
                documentation: "The Grid control provides a flexible way to arrange child elements in a tabular layout with rows and columns.".to_string(),
                namespace: "Avalonia.Controls".to_string(),
                common_properties: vec![
                    crate::types::PropertyReference {
                        name: "RowDefinitions".to_string(),
                        property_type: "RowDefinitions".to_string(),
                        description: "Defines the rows in the Grid".to_string(),
                        default_value: None,
                        is_required: false,
                    },
                    crate::types::PropertyReference {
                        name: "ColumnDefinitions".to_string(),
                        property_type: "ColumnDefinitions".to_string(),
                        description: "Defines the columns in the Grid".to_string(),
                        default_value: None,
                        is_required: false,
                    },
                ],
                xaml_example: r#"<Grid>
    <Grid.RowDefinitions>
        <RowDefinition Height="Auto"/>
        <RowDefinition Height="*"/>
    </Grid.RowDefinitions>
    <Grid.ColumnDefinitions>
        <ColumnDefinition Width="100"/>
        <ColumnDefinition Width="*"/>
    </Grid.ColumnDefinitions>
    <TextBlock Grid.Row="0" Grid.Column="0" Text="Header"/>
    <ContentControl Grid.Row="1" Grid.Column="1"/>
</Grid>"#.to_string(),
                related_controls: vec!["StackPanel".to_string(), "DockPanel".to_string(), "WrapPanel".to_string()],
                wpf_migration_notes: Some("Grid in Avalonia works very similarly to WPF. The main difference is in the syntax for defining rows and columns.".to_string()),
            },
            crate::types::ControlReference {
                name: "ListBox".to_string(),
                category: "Selection".to_string(),
                description: "A control that displays a list of items for selection".to_string(),
                documentation: "The ListBox control displays a collection of items from which the user can select one or more items.".to_string(),
                namespace: "Avalonia.Controls".to_string(),
                common_properties: vec![
                    crate::types::PropertyReference {
                        name: "Items".to_string(),
                        property_type: "IList".to_string(),
                        description: "The collection of items to display".to_string(),
                        default_value: None,
                        is_required: false,
                    },
                    crate::types::PropertyReference {
                        name: "SelectedItem".to_string(),
                        property_type: "object".to_string(),
                        description: "The currently selected item".to_string(),
                        default_value: Some("null".to_string()),
                        is_required: false,
                    },
                    crate::types::PropertyReference {
                        name: "SelectionMode".to_string(),
                        property_type: "SelectionMode".to_string(),
                        description: "Defines the selection behavior (Single, Multiple, Toggle, AlwaysSelected)".to_string(),
                        default_value: Some("Single".to_string()),
                        is_required: false,
                    },
                ],
                xaml_example: r#"<ListBox Items="{Binding Items}" SelectedItem="{Binding SelectedItem}"/>
<ListBox SelectionMode="Multiple">
    <ListBoxItem Content="Item 1"/>
    <ListBoxItem Content="Item 2"/>
</ListBox>"#.to_string(),
                related_controls: vec!["ComboBox".to_string(), "DataGrid".to_string()],
                wpf_migration_notes: Some("ListBox in Avalonia is similar to WPF but uses SelectionMode enum instead of SelectionMode property.".to_string()),
            },
        ]
    }

    /// Load default XAML patterns
    fn load_default_patterns() -> Vec<crate::types::XamlPattern> {
        vec![
            crate::types::XamlPattern {
                name: "MVVM Binding".to_string(),
                category: "Data Binding".to_string(),
                description: "Standard MVVM binding pattern with INotifyPropertyChanged".to_string(),
                xaml_template: r#"<TextBox Text="{Binding Name, Mode=TwoWay, UpdateSourceTrigger=PropertyChanged}"/>"#.to_string(),
                usage_guidance: "Use for two-way data binding in MVVM scenarios. UpdateSourceTrigger=PropertyChanged ensures immediate updates.".to_string(),
                related_patterns: vec!["Command Binding".to_string(), "Data Validation".to_string()],
            },
            crate::types::XamlPattern {
                name: "Styles and Resources".to_string(),
                category: "Styling".to_string(),
                description: "Define reusable styles in application resources".to_string(),
                xaml_template: r#"<Application.Styles>
    <Style Selector="Button.primary">
        <Setter Property="Background" Value="Blue"/>
        <Setter Property="Foreground" Value="White"/>
    </Style>
</Application.Styles>

<Button Classes="primary" Content="Primary Action"/>"#.to_string(),
                usage_guidance: "Use classes for reusable style variants. Define styles at application level for consistency.".to_string(),
                related_patterns: vec!["Theme Dictionaries".to_string(), "Control Templates".to_string()],
            },
        ]
    }

    /// Load default WPF migration mappings
    fn load_default_wpf_migration() -> Vec<crate::types::WpfMigrationMapping> {
        vec![
            crate::types::WpfMigrationMapping {
                wpf_name: "TextBlock".to_string(),
                avalonia_name: "TextBlock".to_string(),
                difficulty: "Easy".to_string(),
                notes: "Nearly identical API. Main difference is no Inlines support in basic scenarios.".to_string(),
                code_changes: vec!["Update namespace from System.Windows.Controls to Avalonia.Controls".to_string()],
                xaml_changes: vec!["No changes typically needed".to_string()],
            },
            crate::types::WpfMigrationMapping {
                wpf_name: "DataGrid".to_string(),
                avalonia_name: "DataGrid".to_string(),
                difficulty: "Medium".to_string(),
                notes: "Avalonia DataGrid requires the Avalonia.Controls.DataGrid package. API is similar but not identical.".to_string(),
                code_changes: vec![
                    "Add NuGet package: Avalonia.Controls.DataGrid".to_string(),
                    "Update namespace references".to_string(),
                ],
                xaml_changes: vec![
                    "Add xmlns:datagrid='using:Avalonia.Controls.DataGrid'".to_string(),
                    "Use datagrid:DataGrid instead of DataGrid".to_string(),
                ],
            },
        ]
    }

    /// Clear the cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        *cache = None;
        info!("Resource loader cache cleared");
    }

    /// Check if cache is populated
    pub async fn is_cached(&self) -> bool {
        let cache = self.cache.read().await;
        cache.is_some()
    }
}

impl Default for ResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_knowledge_base() {
        let loader = ResourceLoader::new();

        // First load (should populate cache)
        let kb1 = loader.load_knowledge_base().await.unwrap();
        assert!(!kb1.controls.is_empty());

        // Second load (should use cache)
        let kb2 = loader.load_knowledge_base().await.unwrap();
        assert_eq!(kb1.controls.len(), kb2.controls.len());
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let loader = ResourceLoader::new();

        // Load to populate cache
        loader.load_knowledge_base().await.unwrap();
        assert!(loader.is_cached().await);

        // Clear cache
        loader.clear_cache().await;
        assert!(!loader.is_cached().await);
    }
}
