.PHONY: help build clean test core macos setup install

help:
	@echo "GoNhanh - Makefile commands:"
	@echo "  make build       - Build everything (core + macOS app)"
	@echo "  make core        - Build Rust core only"
	@echo "  make macos       - Build macOS app"
	@echo "  make test        - Run tests"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make setup       - Setup development environment"
	@echo "  make install     - Install the app"

build: core macos

core:
	@echo "🦀 Building Rust core..."
	cd core && cargo build --release
	@echo "✅ Core built successfully!"

macos: core
	@echo "🍎 Building macOS app..."
	./scripts/build-macos.sh

test:
	@echo "🧪 Running tests..."
	cd core && cargo test

clean:
	@echo "🧹 Cleaning..."
	cd core && cargo clean
	rm -rf platforms/macos/build
	@echo "✅ Clean complete!"

setup:
	@echo "🔧 Setting up..."
	./scripts/setup.sh

install: build
	@echo "📦 Installing GoNhanh..."
	@echo "Please drag platforms/macos/build/Release/GoNhanh.app to /Applications"
