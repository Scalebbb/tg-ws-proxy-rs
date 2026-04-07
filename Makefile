.PHONY: all build build-all clean install help test check

# Default target
all: build

# Build for current platform
build:
	@echo "Building for current platform..."
	cargo build --release

# Build for all platforms
build-all:
	@echo "Building for all platforms..."
	@chmod +x platforms.sh
	@./platforms.sh

# Build specific targets
build-linux-x64:
	cargo zigbuild --release --target x86_64-unknown-linux-musl --no-default-features --features dc-updater

build-linux-arm64:
	cargo zigbuild --release --target aarch64-unknown-linux-musl --no-default-features --features dc-updater

build-linux-armv7:
	cargo zigbuild --release --target armv7-unknown-linux-musleabihf --no-default-features --features dc-updater

build-macos-x64:
	cargo zigbuild --release --target x86_64-apple-darwin

build-macos-arm64:
	cargo zigbuild --release --target aarch64-apple-darwin

build-windows-x64:
	cargo zigbuild --release --target x86_64-pc-windows-gnu

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf dist

# Install locally
install: build
	@echo "Installing to ~/.cargo/bin..."
	cargo install --path .

# Run tests
test:
	cargo test

# Check code
check:
	cargo check
	cargo clippy -- -D warnings
	cargo fmt -- --check

# Format code
fmt:
	cargo fmt

# Show help
help:
	@echo "Available targets:"
	@echo "  make build          - Build for current platform"
	@echo "  make build-all      - Build for all platforms"
	@echo "  make build-linux-x64   - Build for Linux x86_64"
	@echo "  make build-linux-arm64 - Build for Linux ARM64"
	@echo "  make build-linux-armv7 - Build for Linux ARMv7"
	@echo "  make build-macos-x64   - Build for macOS x86_64"
	@echo "  make build-macos-arm64 - Build for macOS ARM64"
	@echo "  make build-windows-x64 - Build for Windows x86_64"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make install        - Install locally"
	@echo "  make test           - Run tests"
	@echo "  make check          - Check code (clippy, fmt)"
	@echo "  make fmt            - Format code"
	@echo "  make help           - Show this help"
