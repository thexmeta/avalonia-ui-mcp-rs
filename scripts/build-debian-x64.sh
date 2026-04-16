#!/bin/bash
# ============================================================================
# AvaloniaUI MCP Server - Debian x64 Release Build Script
# ============================================================================
# This script builds a production-ready Debian x64 executable and installs it
# system-wide
# ============================================================================

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/target/release"
OUTPUT_DIR="$PROJECT_ROOT/build/debian-x64"
BINARY_NAME="avalonia-mcp-server"
INSTALL_DIR="/usr/local/bin"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Helper functions
echo_success() {
    echo -e "${GREEN}✓${NC} $1"
}

echo_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

echo_error() {
    echo -e "${RED}✗${NC} $1"
}

echo_info() {
    echo "$1"
}

# ============================================================================
# Main Build Process
# ============================================================================

echo ""
echo "============================================================================"
echo "  AvaloniaUI MCP Server - Debian x64 Release Build"
echo "============================================================================"
echo ""

# Step 1: Check Rust installation
echo_info "Step 1: Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo_error "ERROR: Rust is not installed or not in PATH"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi
rustc --version
cargo --version
echo ""

# Step 2: Verify project structure
echo_info "Step 2: Verifying project structure..."
if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
    echo_error "ERROR: Cargo.toml not found in project root"
    exit 1
fi
if [ ! -d "$PROJECT_ROOT/crates" ]; then
    echo_error "ERROR: crates folder not found"
    exit 1
fi
echo_success "Project structure verified."
echo ""

# Step 3: Clean previous builds
echo_info "Step 3: Cleaning previous builds..."
cargo clean
echo_success "Clean completed."
echo ""

# Step 4: Run tests (optional but recommended)
echo_info "Step 4: Running tests..."
if cargo test --quiet 2>&1; then
    echo_success "All tests passed!"
else
    echo_warning "WARNING: Some tests failed, continuing with build..."
fi
echo ""

# Step 5: Build release binary
echo_info "Step 5: Building Debian x64 release binary..."
echo "This may take several minutes for the first build..."
echo ""

# Set release optimizations
export RUSTFLAGS="-C target-cpu=x86-64"
export CARGO_INCREMENTAL=0

cargo build --release --workspace
if [ $? -ne 0 ]; then
    echo_error "ERROR: Build failed!"
    exit 1
fi
echo ""
echo_success "Build completed successfully!"
echo ""

# Step 6: Verify binary
echo_info "Step 6: Verifying binary..."
if [ ! -f "$BUILD_DIR/$BINARY_NAME" ]; then
    echo_error "ERROR: Binary not found at $BUILD_DIR/$BINARY_NAME"
    exit 1
fi

# Get file size
FILE_SIZE=$(stat -c%s "$BUILD_DIR/$BINARY_NAME")
FILE_SIZE_MB=$((FILE_SIZE / 1048576))
echo "Binary: $BUILD_DIR/$BINARY_NAME"
echo "File size: ${FILE_SIZE_MB} MB"

# Check if binary is executable
if file "$BUILD_DIR/$BINARY_NAME" | grep -q "ELF 64-bit"; then
    echo_success "Binary is a valid ELF 64-bit executable"
else
    echo_warning "Binary may not be a valid ELF executable"
fi
echo ""

# Step 7: Create distribution package
echo_info "Step 7: Creating distribution package..."
mkdir -p "$OUTPUT_DIR"

# Copy binary
cp "$BUILD_DIR/$BINARY_NAME" "$OUTPUT_DIR/"
chmod +x "$OUTPUT_DIR/$BINARY_NAME"

# Copy README
cp "$PROJECT_ROOT/README.md" "$OUTPUT_DIR/"

# Create version info
cat > "$OUTPUT_DIR/VERSION.txt" << EOF
AvaloniaUI MCP Server
Version: 0.1.0
Build Date: $(date)
Platform: Debian x64
EOF

# Create MCP config example
cat > "$OUTPUT_DIR/mcp-config.example.json" << EOF
{
  "mcpServers": {
    "avalonia": {
      "command": "$INSTALL_DIR/$BINARY_NAME",
      "args": [],
      "cwd": "/path/to/your/project"
    }
  }
}
EOF

echo_success "Distribution package created at: $OUTPUT_DIR/"
echo ""

# Step 8: Install system-wide
echo_info "Step 8: Installing system-wide..."
echo "This requires sudo privileges."
echo ""

if [ "$EUID" -eq 0 ]; then
    # Already root
    echo_info "Installing binary to $INSTALL_DIR..."
    cp "$BUILD_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    echo_success "Binary installed to $INSTALL_DIR/$BINARY_NAME"
else
    # Use sudo
    echo_info "Installing binary to $INSTALL_DIR (using sudo)..."
    sudo cp "$BUILD_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    echo_success "Binary installed to $INSTALL_DIR/$BINARY_NAME"
fi

# Verify installation
if command -v "$BINARY_NAME" &> /dev/null; then
    echo_success "Binary is now available system-wide"
    echo "Version info:"
    $BINARY_NAME --version 2>&1 || echo "(binary does not support --version flag)"
else
    echo_warning "Binary may not be in PATH. Ensure $INSTALL_DIR is in your PATH"
fi
echo ""

# Step 9: Show build summary
echo_info "Step 9: Build Summary"
echo "============================================================================"
echo "  BUILD COMPLETE"
echo "============================================================================"
echo ""
echo "  Binary:         $BINARY_NAME"
echo "  Location:       $BUILD_DIR/$BINARY_NAME"
echo "  Size:           ${FILE_SIZE_MB} MB"
echo "  Platform:       Debian x64"
echo "  Build Type:     Release (optimized)"
echo "  Installed:      $INSTALL_DIR/$BINARY_NAME"
echo ""
echo "  Distribution Package:"
echo "  Location:       $OUTPUT_DIR/"
echo "  Contents:"
echo "    - $BINARY_NAME"
echo "    - README.md"
echo "    - VERSION.txt"
echo "    - mcp-config.example.json"
echo ""
echo "  Usage:"
echo "    STDIO mode:   $BINARY_NAME"
echo "    HTTP mode:    $BINARY_NAME --http"
echo "    Custom port:  $BINARY_NAME --http --port=8080"
echo ""
echo "============================================================================"
echo ""

echo_success "Build and installation completed successfully!"
echo ""
