#!/bin/bash

set -e  # Exit on error

echo "=========================================="
echo "  Multi-platform build with cargo-zigbuild"
echo "=========================================="
echo ""

# Check if cargo-zigbuild is installed
if ! command -v cargo-zigbuild &> /dev/null; then
    echo "❌ cargo-zigbuild not found!"
    echo "Install it with: cargo install cargo-zigbuild"
    exit 1
fi

# Check if zig is installed
if ! command -v zig &> /dev/null; then
    echo "❌ zig not found!"
    echo "Install it from: https://ziglang.org/download/"
    exit 1
fi

echo "✅ cargo-zigbuild found"
echo "✅ zig found"
echo ""

# Create output directory
mkdir -p dist

# Build function
build_target() {
    local target=$1
    local name=$2
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Building for: $name ($target)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    # Build without GUI for non-Windows/macOS targets (headless servers)
    if [[ "$target" == *"linux"* ]] && [[ "$target" != *"gnu"* ]]; then
        echo "Building headless version (no GUI) for $target"
        cargo zigbuild --release --target "$target" --no-default-features --features dc-updater
    else
        echo "Building full version (with GUI) for $target"
        cargo zigbuild --release --target "$target"
    fi
    
    if [ $? -eq 0 ]; then
        echo "✅ Build successful for $target"
        
        # Copy binary to dist folder
        if [[ "$target" == *"windows"* ]]; then
            cp "target/$target/release/tg-ws-proxy.exe" "dist/tg-ws-proxy-$name.exe" 2>/dev/null || true
        else
            cp "target/$target/release/tg-ws-proxy" "dist/tg-ws-proxy-$name" 2>/dev/null || true
        fi
    else
        echo "❌ Build failed for $target"
    fi
    echo ""
}

# Linux targets (headless - no GUI)
echo "🐧 Building Linux targets..."
build_target "x86_64-unknown-linux-musl" "linux-x86_64"
build_target "aarch64-unknown-linux-musl" "linux-aarch64"
build_target "armv7-unknown-linux-musleabihf" "linux-armv7"
build_target "mipsel-unknown-linux-musl" "linux-mipsel"

# macOS targets (with GUI)
echo "🍎 Building macOS targets..."
build_target "x86_64-apple-darwin" "macos-x86_64"
build_target "aarch64-apple-darwin" "macos-aarch64"

# Windows targets (with GUI)
echo "🪟 Building Windows targets..."
build_target "x86_64-pc-windows-gnu" "windows-x86_64"

echo "=========================================="
echo "  Build complete!"
echo "=========================================="
echo ""
echo "Binaries are in the 'dist' folder:"
ls -lh dist/ 2>/dev/null || dir dist\
echo ""
echo "Press enter to exit"
read -p "" </dev/tty
