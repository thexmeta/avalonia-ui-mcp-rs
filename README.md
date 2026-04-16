# AvaloniaUI.MCP (Rust)

[](https://www.rust-lang.org/)
[](https://opensource.org/licenses/MIT)
[](https://modelcontextprotocol.io/)

**AvaloniaUI.MCP-rs** is a high-performance Model Context Protocol (MCP) server engineered in Rust and complete rewrite Rust port of https://github.com/decriptor/AvaloniaUI.MCP. It provides a comprehensive suite of AI-powered development tools for building, validating, and migrating AvaloniaUI cross-platform applications.

## 🚀 Core Capabilities

- **18+ Specialized Tools:** Spanning project scaffolding, XAML validation, and WPF-to-Avalonia migration.
- **High Performance:** Sub-100ms response times with a minimal memory footprint (\< 2MB).
- **Dual Transport:** Supports both **STDIO** (local CLI use) and **HTTP Streamable** (remote/distributed) modes.
- **Deep Knowledge Base:** Embedded reference for 500+ Avalonia controls and enterprise design patterns.

---

## 🛠 Technology Stack

| Layer             | Technology | Version     |
| :---------------- | :--------- | :---------- |
| **Framework**     | `rmcp`     | 0.11.0      |
| **Runtime**       | `tokio`    | 1.48 (Full) |
| **API/Web**       | `axum`     | 0.8         |
| **Serialization** | `serde`    | 1.0         |
| **Diagnostics**   | `tracing`  | 0.1         |

---

## ⚡ Quick Start

### Prerequisites

- Rust Toolchain (v1.86+)

### Installation & Build

```bash
# Clone and build for release
git clone https://github.com/thexmeta/avalonia-ui-mcp-rs
cd avalonia-rust-mcp
cargo build --release
```

### Execution Modes

**1. Local (STDIO):**
Default mode for Claude Desktop or VS Code.

```bash
cargo run --release -p avalonia-mcp-server
```

**2. Remote (HTTP):**
Enables remote connectivity via Axum.

```bash
cargo run --release -p avalonia-mcp-server -- --http --port 8080
```

---

## 🔌 MCP Client Configuration

### Claude Desktop

Add this to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "avalonia-rust": {
      "command": "/absolute/path/to/target/release/avalonia-mcp-server",
      "args": [],
      "cwd": "/absolute/path/to/working/dir"
    }
  }
}
```

---

## 🧰 Available Tools (Summary)

### Project & Architecture

- `generate_project`: Scaffold MVVM-based Avalonia applications.
- `generate_architecture_template`: Implement enterprise patterns (Clean Architecture/DDD).
- `migrate_from_wpf`: Automated guidance for legacy WPF migrations.

### Validation & UX

- `validate_xaml`: Real-time XAML syntax and pattern auditing.
- `check_accessibility`: WCAG compliance checks for UI definitions.
- `analyze_performance`: Profiling and optimization guidance for complex views.

### Backend & Integration

- `generate_api_integration`: Robust API client patterns.
- `generate_data_access`: Optimized data persistence logic.
- `generate_security_pattern`: Implementation of security best practices.

---

## 🏗 Project Structure

The project utilizes a multi-crate workspace for strict separation of concerns:

- **`crates/core`**: Common types and error handling (`thiserror`).
- **`crates/services`**: Stateless logic (Caching, Telemetry, Validation).
- **`crates/tools`**: Implementation of the 18+ MCP tools.
- **`crates/server`**: Binary entry point and transport orchestration.
- **`crates/resources/data`**: Embedded JSON knowledge base for offline intelligence.

---

## 📈 Performance & Troubleshooting

### Metrics

| Metric               | Target |
| :------------------- | :----- |
| **Memory Footprint** | \< 2MB |
| **Binary**           | ~ 5MB  |

### Troubleshooting

If the server fails to initialize:

1.  **Check Rust Version:** Ensure `rustc --version` is 1.85+.
2.  **Environment Logs:** Run with `RUST_LOG=debug` to view internal tracing.
3.  **Port Conflict:** For HTTP mode, ensure port `8080` (default) is available.

---

## 📄 License

Distributed under the **MIT License**. See `LICENSE` for more information.

---

## Stability Check [Epistemic Gaps]

- **Configuration Paths:** Ensure the `command` path in MCP client config is absolute to avoid environment resolution errors.
- **Transport Choice:** STDIO is recommended for single-user local development; HTTP should be restricted via firewall if used in production.
