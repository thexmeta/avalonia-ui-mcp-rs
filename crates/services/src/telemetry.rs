//! Telemetry service with async-safe metrics collection
//!
//! This module provides telemetry and metrics collection capabilities
//! with tracing integration for observability.

use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{info, info_span, instrument, warn, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use avalonia_mcp_core::error::AvaloniaMcpError;

#[cfg(test)]
use std::time::Duration;

/// Telemetry event type
#[derive(Debug, Clone)]
pub struct TelemetryEvent {
    pub name: String,
    pub metadata: serde_json::Value,
    pub timestamp: SystemTime,
    pub duration_ms: Option<u64>,
}

/// Telemetry service with async-safe metrics collection
pub struct TelemetryService {
    events: Arc<RwLock<Vec<TelemetryEvent>>>,
    metrics: Arc<RwLock<Vec<(String, serde_json::Value, SystemTime)>>>,
    max_events: usize,
}

impl TelemetryService {
    /// Create a new telemetry service
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(Vec::new())),
            max_events: 10000,
        }
    }

    /// Create telemetry service with custom max events limit
    pub fn with_max_events(mut self, max: usize) -> Self {
        self.max_events = max;
        self
    }

    /// Record a server event with optional metadata
    #[instrument(skip(self, metadata), fields(event = %event_name))]
    pub async fn record_server_event(
        &self,
        event_name: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<(), AvaloniaMcpError> {
        let mut events = self.events.write().await;

        // Trim old events if needed
        if events.len() >= self.max_events {
            let drain_count = events.len() / 10; // Remove oldest 10%
            events.drain(0..drain_count);
            warn!(drain_count, "Trimmed old telemetry events");
        }

        events.push(TelemetryEvent {
            name: event_name.to_string(),
            metadata: metadata.unwrap_or(serde_json::Value::Null),
            timestamp: SystemTime::now(),
            duration_ms: None,
        });

        Ok(())
    }

    /// Record a server event with duration
    #[instrument(skip(self, metadata), fields(event = %event_name, duration_ms))]
    pub async fn record_timed_event(
        &self,
        event_name: &str,
        metadata: Option<serde_json::Value>,
        duration_ms: u64,
    ) -> Result<(), AvaloniaMcpError> {
        let mut events = self.events.write().await;

        events.push(TelemetryEvent {
            name: event_name.to_string(),
            metadata: metadata.unwrap_or(serde_json::Value::Null),
            timestamp: SystemTime::now(),
            duration_ms: Some(duration_ms),
        });

        Ok(())
    }

    /// Start a tracing span for async operation
    #[instrument(skip(self))]
    pub fn start_activity(&self, name: &str) -> Span {
        let span = info_span!("activity", name);
        span.set_attribute("start_time", format!("{:?}", SystemTime::now()));
        span
    }

    /// Record a metric value
    #[instrument(skip(self, value), fields(metric = %name))]
    pub async fn record_metric(
        &self,
        name: &str,
        value: serde_json::Value,
    ) -> Result<(), AvaloniaMcpError> {
        let mut metrics = self.metrics.write().await;
        metrics.push((name.to_string(), value, SystemTime::now()));
        Ok(())
    }

    /// Get events snapshot
    pub async fn get_events_snapshot(&self) -> Vec<TelemetryEvent> {
        self.events.read().await.clone()
    }

    /// Get metrics snapshot
    pub async fn get_metrics_snapshot(&self) -> Vec<(String, serde_json::Value, SystemTime)> {
        self.metrics.read().await.clone()
    }

    /// Get event count
    pub async fn event_count(&self) -> usize {
        self.events.read().await.len()
    }

    /// Get metrics count
    pub async fn metrics_count(&self) -> usize {
        self.metrics.read().await.len()
    }

    /// Clear all events
    pub async fn clear_events(&self) -> usize {
        let mut events = self.events.write().await;
        let count = events.len();
        events.clear();
        info!(count, "Cleared telemetry events");
        count
    }

    /// Clear all metrics
    pub async fn clear_metrics(&self) -> usize {
        let mut metrics = self.metrics.write().await;
        let count = metrics.len();
        metrics.clear();
        info!(count, "Cleared metrics");
        count
    }

    /// Get events by name
    pub async fn get_events_by_name(&self, name: &str) -> Vec<TelemetryEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.name == name)
            .cloned()
            .collect()
    }

    /// Calculate average duration for an event type
    pub async fn average_duration(&self, event_name: &str) -> Option<f64> {
        let events = self.events.read().await;
        let durations: Vec<u64> = events
            .iter()
            .filter(|e| e.name == event_name)
            .filter_map(|e| e.duration_ms)
            .collect();

        if durations.is_empty() {
            return None;
        }

        let sum: u64 = durations.iter().sum();
        Some(sum as f64 / durations.len() as f64)
    }

    /// Get startup info
    pub async fn get_startup_info(&self) -> serde_json::Value {
        let events = self.events.read().await;
        let startup_event = events.iter().find(|e| e.name == "startup");

        if let Some(event) = startup_event {
            event.metadata.clone()
        } else {
            serde_json::Value::Null
        }
    }
}

impl Default for TelemetryService {
    fn default() -> Self {
        Self::new()
    }
}

/// Guard for timing a block of code
pub struct TimingGuard {
    event_name: String,
    start: std::time::Instant,
    metadata: Option<serde_json::Value>,
    telemetry: Arc<RwLock<Vec<TelemetryEvent>>>,
}

impl TimingGuard {
    pub fn new(telemetry: &TelemetryService, event_name: &str) -> Self {
        Self {
            event_name: event_name.to_string(),
            start: std::time::Instant::now(),
            metadata: None,
            telemetry: Arc::clone(&telemetry.events),
        }
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Drop for TimingGuard {
    fn drop(&mut self) {
        let duration_ms = self.start.elapsed().as_millis() as u64;
        let event_name = self.event_name.clone();
        let metadata = self.metadata.take();
        let telemetry = Arc::clone(&self.telemetry);

        // Spawn a task to record the event (non-blocking)
        tokio::spawn(async move {
            let mut events = telemetry.write().await;
            events.push(TelemetryEvent {
                name: event_name,
                metadata: metadata.unwrap_or(serde_json::Value::Null),
                timestamp: SystemTime::now(),
                duration_ms: Some(duration_ms),
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_server_event() {
        let telemetry = TelemetryService::new();

        telemetry
            .record_server_event("test_event", Some(serde_json::json!({"key": "value"})))
            .await
            .unwrap();

        let events = telemetry.get_events_snapshot().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "test_event");
    }

    #[tokio::test]
    async fn test_record_timed_event() {
        let telemetry = TelemetryService::new();

        telemetry
            .record_timed_event("timed_event", None, 100)
            .await
            .unwrap();

        let events = telemetry.get_events_snapshot().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].duration_ms, Some(100));
    }

    #[tokio::test]
    async fn test_record_metric() {
        let telemetry = TelemetryService::new();

        telemetry
            .record_metric("cpu_usage", serde_json::json!(45.5))
            .await
            .unwrap();

        let metrics = telemetry.get_metrics_snapshot().await;
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].0, "cpu_usage");
    }

    #[tokio::test]
    async fn test_clear_events() {
        let telemetry = TelemetryService::new();

        telemetry
            .record_server_event("event1", None)
            .await
            .unwrap();
        telemetry
            .record_server_event("event2", None)
            .await
            .unwrap();

        let count = telemetry.clear_events().await;
        assert_eq!(count, 2);
        assert!(telemetry.get_events_snapshot().await.is_empty());
    }

    #[tokio::test]
    async fn test_average_duration() {
        let telemetry = TelemetryService::new();

        telemetry
            .record_timed_event("operation", None, 100)
            .await
            .unwrap();
        telemetry
            .record_timed_event("operation", None, 200)
            .await
            .unwrap();
        telemetry
            .record_timed_event("operation", None, 300)
            .await
            .unwrap();

        let avg = telemetry.average_duration("operation").await;
        assert_eq!(avg, Some(200.0));
    }

    #[tokio::test]
    async fn test_timing_guard() {
        let telemetry = TelemetryService::new();

        {
            let _guard = TimingGuard::new(&telemetry, "timed_block");
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        // Give async task time to complete
        tokio::time::sleep(Duration::from_millis(10)).await;

        let events = telemetry.get_events_snapshot().await;
        assert!(!events.is_empty());
        let timed_event = events.iter().find(|e| e.name == "timed_block");
        assert!(timed_event.is_some());
        assert!(timed_event.unwrap().duration_ms.unwrap() >= 50);
    }

    #[tokio::test]
    async fn test_event_count_limit() {
        let telemetry = TelemetryService::new().with_max_events(100);

        // Record more than max events
        for i in 0..150 {
            telemetry
                .record_server_event(&format!("event_{}", i), None)
                .await
                .unwrap();
        }

        let events = telemetry.get_events_snapshot().await;
        assert!(events.len() <= 100);
    }
}
