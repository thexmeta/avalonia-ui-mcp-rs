//! Markdown output builder for formatted responses
//!
//! This module provides a builder pattern for constructing well-formatted
//! markdown output from tool executions.

/// Markdown output builder for formatted responses
#[derive(Debug, Clone, Default)]
pub struct MarkdownOutputBuilder {
    content: String,
}

impl MarkdownOutputBuilder {
    /// Create a new markdown builder
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// Add a heading
    pub fn heading(mut self, level: u8, text: &str) -> Self {
        let hashes = "#".repeat(level as usize);
        self.content.push_str(&format!("{} {}\n\n", hashes, text));
        self
    }

    /// Add a paragraph of text
    pub fn paragraph(mut self, text: &str) -> Self {
        self.content.push_str(text);
        self.content.push_str("\n\n");
        self
    }

    /// Add a code block with language specification
    pub fn code_block(mut self, language: &str, code: &str) -> Self {
        self.content
            .push_str(&format!("```{}\n{}\n```\n\n", language, code));
        self
    }

    /// Add an inline code span
    pub fn inline_code(mut self, code: &str) -> Self {
        self.content.push_str(&format!("`{}`", code));
        self
    }

    /// Add a bulleted list
    pub fn list<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for item in items {
            self.content.push_str(&format!("- {}\n", item.as_ref()));
        }
        self.content.push('\n');
        self
    }

    /// Add a numbered list
    pub fn numbered_list<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for (i, item) in items.into_iter().enumerate() {
            self.content
                .push_str(&format!("{}. {}\n", i + 1, item.as_ref()));
        }
        self.content.push('\n');
        self
    }

    /// Add a blockquote
    pub fn blockquote(mut self, text: &str) -> Self {
        for line in text.lines() {
            self.content.push_str(&format!("> {}\n", line));
        }
        self.content.push('\n');
        self
    }

    /// Add a horizontal rule
    pub fn horizontal_rule(mut self) -> Self {
        self.content.push_str("---\n\n");
        self
    }

    /// Add bold text
    pub fn bold(mut self, text: &str) -> Self {
        self.content.push_str(&format!("**{}**", text));
        self
    }

    /// Add italic text
    pub fn italic(mut self, text: &str) -> Self {
        self.content.push_str(&format!("*{}*", text));
        self
    }

    /// Add a link
    pub fn link(mut self, text: &str, url: &str) -> Self {
        self.content.push_str(&format!("[{}]({})", text, url));
        self
    }

    /// Add a table
    pub fn table(mut self, headers: Vec<&str>, rows: Vec<Vec<&str>>) -> Self {
        // Header row
        self.content.push_str("| ");
        self.content.push_str(&headers.join(" | "));
        self.content.push_str(" |\n");

        // Separator
        self.content.push_str("| ");
        self.content.push_str(&vec!["---"; headers.len()].join(" | "));
        self.content.push_str(" |\n");

        // Data rows
        for row in rows {
            self.content.push_str("| ");
            self.content.push_str(&row.join(" | "));
            self.content.push_str(" |\n");
        }

        self.content.push('\n');
        self
    }

    /// Add a task list
    pub fn task_list<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = (bool, S)>,
        S: AsRef<str>,
    {
        for (checked, item) in items {
            let checkbox = if checked { "[x]" } else { "[ ]" };
            self.content
                .push_str(&format!("- {} {}\n", checkbox, item.as_ref()));
        }
        self.content.push('\n');
        self
    }

    /// Build the final markdown string
    pub fn build(self) -> String {
        self.content
    }

    /// Get the current content without consuming the builder
    pub fn as_str(&self) -> &str {
        &self.content
    }

    /// Clear the current content
    pub fn clear(&mut self) {
        self.content.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading() {
        let output = MarkdownOutputBuilder::new()
            .heading(1, "Title")
            .heading(2, "Subtitle")
            .build();
        assert!(output.contains("# Title"));
        assert!(output.contains("## Subtitle"));
    }

    #[test]
    fn test_code_block() {
        let output = MarkdownOutputBuilder::new()
            .code_block("rust", "fn main() {}")
            .build();
        assert!(output.contains("```rust"));
        assert!(output.contains("fn main() {}"));
    }

    #[test]
    fn test_list() {
        let output = MarkdownOutputBuilder::new()
            .list(vec!["item1", "item2", "item3"])
            .build();
        assert!(output.contains("- item1"));
        assert!(output.contains("- item2"));
        assert!(output.contains("- item3"));
    }

    #[test]
    fn test_numbered_list() {
        let output = MarkdownOutputBuilder::new()
            .numbered_list(vec!["first", "second", "third"])
            .build();
        assert!(output.contains("1. first"));
        assert!(output.contains("2. second"));
        assert!(output.contains("3. third"));
    }

    #[test]
    fn test_table() {
        let output = MarkdownOutputBuilder::new()
            .table(vec!["Name", "Value"], vec![vec!["A", "1"], vec!["B", "2"]])
            .build();
        assert!(output.contains("| Name | Value |"));
        assert!(output.contains("| A | 1 |"));
        assert!(output.contains("| B | 2 |"));
    }

    #[test]
    fn test_task_list() {
        let output = MarkdownOutputBuilder::new()
            .task_list(vec![(true, "completed"), (false, "pending")])
            .build();
        assert!(output.contains("- [x] completed"));
        assert!(output.contains("- [ ] pending"));
    }

    #[test]
    fn test_builder_chain() {
        let output = MarkdownOutputBuilder::new()
            .heading(1, "Title")
            .paragraph("This is a paragraph.")
            .code_block("rust", "fn main() {}")
            .list(vec!["item1", "item2"])
            .build();

        assert!(output.contains("# Title"));
        assert!(output.contains("This is a paragraph."));
        assert!(output.contains("```rust"));
        assert!(output.contains("- item1"));
    }

    #[test]
    fn test_clear() {
        let mut builder = MarkdownOutputBuilder::new().heading(1, "Title");
        assert!(!builder.as_str().is_empty());
        builder.clear();
        assert!(builder.as_str().is_empty());
    }
}
