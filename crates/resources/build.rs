//! Build script for embedding resources
//!
//! This script generates Rust code that embeds the knowledge base JSON files
//! at compile time for fast access.

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded_resources.rs");

    // Use the crate-local data directory so the package can be published
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_dir = Path::new(&manifest_dir).join("data");

    // Convert to string and normalize path separators for Rust strings
    let data_dir_str = data_dir.to_string_lossy().replace('\\', "/");

    // For now, generate a placeholder
    // In production, this would read actual JSON files and embed them
    let contents = format!(
        r#"
// Auto-generated embedded resources
// DO NOT EDIT MANUALLY

/// Get the embedded controls JSON
pub fn get_controls_json() -> &'static str {{
    include_str!(r"{}/controls.json")
}}

/// Get the embedded XAML patterns JSON
pub fn get_xaml_patterns_json() -> &'static str {{
    include_str!(r"{}/xaml-patterns.json")
}}

/// Get the embedded migration guide JSON
pub fn get_migration_guide_json() -> &'static str {{
    include_str!(r"{}/migration-guide.json")
}}
"#,
        data_dir_str,
        data_dir_str,
        data_dir_str
    );

    fs::write(&dest_path, contents).unwrap();

    // Tell Cargo to rerun if data files change
    println!("cargo:rerun-if-changed={}", data_dir.join("controls.json").display());
    println!("cargo:rerun-if-changed={}", data_dir.join("xaml-patterns.json").display());
    println!("cargo:rerun-if-changed={}", data_dir.join("migration-guide.json").display());
}
