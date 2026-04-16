//! Async file operations service
//!
//! This module provides async file operations using tokio's fs module
//! for non-blocking I/O operations.

use std::path::Path;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

use avalonia_mcp_core::error::AvaloniaMcpError;

/// Async file operations service
pub struct AsyncFileService;

impl AsyncFileService {
    /// Read file content asynchronously as string
    pub async fn read_to_string(path: impl AsRef<Path>) -> Result<String, AvaloniaMcpError> {
        let path = path.as_ref();
        let mut file = File::open(path).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        Ok(content)
    }

    /// Read file content asynchronously as bytes
    pub async fn read_to_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>, AvaloniaMcpError> {
        let path = path.as_ref();
        let mut file = File::open(path).await?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;
        Ok(content)
    }

    /// Write string content to file asynchronously
    pub async fn write_string(
        path: impl AsRef<Path>,
        content: &str,
    ) -> Result<(), AvaloniaMcpError> {
        let path = path.as_ref();

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = File::create(path).await?;
        file.write_all(content.as_bytes()).await?;
        file.sync_all().await?;
        Ok(())
    }

    /// Write bytes content to file asynchronously
    pub async fn write_bytes(
        path: impl AsRef<Path>,
        content: &[u8],
    ) -> Result<(), AvaloniaMcpError> {
        let path = path.as_ref();

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = File::create(path).await?;
        file.write_all(content).await?;
        file.sync_all().await?;
        Ok(())
    }

    /// Append string content to file asynchronously
    pub async fn append_string(
        path: impl AsRef<Path>,
        content: &str,
    ) -> Result<(), AvaloniaMcpError> {
        let path = path.as_ref();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        file.write_all(content.as_bytes()).await?;
        file.sync_all().await?;
        Ok(())
    }

    /// Check if file exists
    pub async fn exists(path: impl AsRef<Path>) -> bool {
        fs::metadata(path).await.is_ok()
    }

    /// Check if path is a file
    pub async fn is_file(path: impl AsRef<Path>) -> bool {
        fs::metadata(path)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    }

    /// Check if path is a directory
    pub async fn is_dir(path: impl AsRef<Path>) -> bool {
        fs::metadata(path)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    }

    /// Get file metadata
    pub async fn metadata(path: impl AsRef<Path>) -> Result<std::fs::Metadata, AvaloniaMcpError> {
        Ok(fs::metadata(path).await?)
    }

    /// Get file size in bytes
    pub async fn file_size(path: impl AsRef<Path>) -> Result<u64, AvaloniaMcpError> {
        let metadata = fs::metadata(path).await?;
        Ok(metadata.len())
    }

    /// Delete a file
    pub async fn remove_file(path: impl AsRef<Path>) -> Result<(), AvaloniaMcpError> {
        fs::remove_file(path).await?;
        Ok(())
    }

    /// Delete a directory and all its contents
    pub async fn remove_dir_all(path: impl AsRef<Path>) -> Result<(), AvaloniaMcpError> {
        fs::remove_dir_all(path).await?;
        Ok(())
    }

    /// Create a directory and all parent directories
    pub async fn create_dir_all(path: impl AsRef<Path>) -> Result<(), AvaloniaMcpError> {
        fs::create_dir_all(path).await?;
        Ok(())
    }

    /// List directory contents
    pub async fn read_dir(
        path: impl AsRef<Path>,
    ) -> Result<Vec<std::path::PathBuf>, AvaloniaMcpError> {
        let mut entries = Vec::new();
        let mut dir = fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            entries.push(entry.path());
        }

        Ok(entries)
    }

    /// Copy a file
    pub async fn copy(
        from: impl AsRef<Path>,
        to: impl AsRef<Path>,
    ) -> Result<u64, AvaloniaMcpError> {
        let from = from.as_ref().to_path_buf();
        let to = to.as_ref().to_path_buf();

        // Create parent directories for destination
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).await?;
        }

        let bytes_copied = fs::copy(&from, &to).await?;
        Ok(bytes_copied)
    }

    /// Move/rename a file
    pub async fn rename(
        from: impl AsRef<Path>,
        to: impl AsRef<Path>,
    ) -> Result<(), AvaloniaMcpError> {
        fs::rename(from, to).await?;
        Ok(())
    }

    /// Read file with buffering for large files
    pub async fn read_buffered(
        path: impl AsRef<Path>,
        buffer_size: usize,
    ) -> Result<String, AvaloniaMcpError> {
        let file = File::open(path).await?;
        let mut reader = BufReader::with_capacity(buffer_size, file);
        let mut content = String::new();
        reader.read_to_string(&mut content).await?;
        Ok(content)
    }

    /// Write file with buffering for large content
    pub async fn write_buffered(
        path: impl AsRef<Path>,
        content: &str,
        buffer_size: usize,
    ) -> Result<(), AvaloniaMcpError> {
        let path = path.as_ref();

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let file = File::create(path).await?;
        let mut writer = BufWriter::with_capacity(buffer_size, file);
        writer.write_all(content.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }

    /// Check if file is readable
    pub async fn is_readable(path: impl AsRef<Path>) -> bool {
        File::open(path).await.is_ok()
    }

    /// Check if file is writable (create mode)
    pub async fn is_writable(path: impl AsRef<Path>) -> bool {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .await
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    async fn get_temp_path(name: &str) -> std::path::PathBuf {
        env::temp_dir().join(format!("async_file_test_{}", name))
    }

    #[tokio::test]
    async fn test_read_write_string() {
        let path = get_temp_path("test1.txt").await;
        let content = "Hello, async world!";

        // Write
        AsyncFileService::write_string(&path, content).await.unwrap();

        // Read
        let read_content = AsyncFileService::read_to_string(&path).await.unwrap();
        assert_eq!(read_content, content);

        // Cleanup
        AsyncFileService::remove_file(&path).await.unwrap();
    }

    #[tokio::test]
    async fn test_exists() {
        let path = get_temp_path("test2.txt").await;

        // File doesn't exist yet
        assert!(!AsyncFileService::exists(&path).await);

        // Create file
        AsyncFileService::write_string(&path, "test").await.unwrap();

        // File exists now
        assert!(AsyncFileService::exists(&path).await);

        // Cleanup
        AsyncFileService::remove_file(&path).await.unwrap();
    }

    #[tokio::test]
    async fn test_append() {
        let path = get_temp_path("test3.txt").await;

        AsyncFileService::write_string(&path, "Line 1\n").await.unwrap();
        AsyncFileService::append_string(&path, "Line 2\n").await.unwrap();

        let content = AsyncFileService::read_to_string(&path).await.unwrap();
        assert_eq!(content, "Line 1\nLine 2\n");

        // Cleanup
        AsyncFileService::remove_file(&path).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_size() {
        let path = get_temp_path("test4.txt").await;
        let content = "Hello, World!";

        AsyncFileService::write_string(&path, content).await.unwrap();

        let size = AsyncFileService::file_size(&path).await.unwrap();
        assert_eq!(size, content.len() as u64);

        // Cleanup
        AsyncFileService::remove_file(&path).await.unwrap();
    }

    #[tokio::test]
    async fn test_copy() {
        let from_path = get_temp_path("test5_from.txt").await;
        let to_path = get_temp_path("test5_to.txt").await;
        let content = "Copy me!";

        AsyncFileService::write_string(&from_path, content).await.unwrap();

        let bytes = AsyncFileService::copy(&from_path, &to_path).await.unwrap();
        assert_eq!(bytes, content.len() as u64);

        let copied = AsyncFileService::read_to_string(&to_path).await.unwrap();
        assert_eq!(copied, content);

        // Cleanup
        AsyncFileService::remove_file(&from_path).await.unwrap();
        AsyncFileService::remove_file(&to_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_dir_all() {
        let dir_path = get_temp_path("nested/deep/directory").await;

        AsyncFileService::create_dir_all(&dir_path).await.unwrap();
        assert!(AsyncFileService::is_dir(&dir_path).await);

        // Cleanup
        AsyncFileService::remove_dir_all(get_temp_path("nested").await)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_read_dir() {
        let dir_path = get_temp_path("test_dir").await;

        AsyncFileService::create_dir_all(&dir_path).await.unwrap();

        // Create some files
        AsyncFileService::write_string(dir_path.join("file1.txt"), "content1")
            .await
            .unwrap();
        AsyncFileService::write_string(dir_path.join("file2.txt"), "content2")
            .await
            .unwrap();

        let entries = AsyncFileService::read_dir(&dir_path).await.unwrap();
        assert_eq!(entries.len(), 2);

        // Cleanup
        AsyncFileService::remove_dir_all(&dir_path).await.unwrap();
    }
}
