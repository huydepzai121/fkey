.PHONY: help build clean test core macos setup install lint release

.DEFAULT_GOAL := help

# Version from git tag or default
VERSION ?= $(shell git describe --tags 2>/dev/null || echo "dev")

help:
	@echo "GoNhanh $(VERSION) - Makefile commands:"
	@echo ""
	@echo "Development:"
	@echo "  make setup       - Setup dev environment"
	@echo "  make test        - Run tests"
	@echo "  make lint        - Run fmt + clippy"
	@echo "  make build       - Build everything (test → core → macos)"
	@echo "  make clean       - Clean build artifacts"
	@echo ""
	@echo "Build:"
	@echo "  make core        - Build Rust core only"
	@echo "  make macos       - Build macOS app"
	@echo "  make install     - Install to /Applications"
	@echo ""
	@echo "Release:"
	@echo "  make release v=1.0.0  - Tag and push release"

build: test core macos

core:
	@./scripts/build-core.sh

macos:
	@./scripts/build-macos.sh

test:
	@echo "🧪 Running tests..."
	@cd core && cargo test

lint:
	@echo "🔍 Linting..."
	@cd core && cargo fmt -- --check && cargo clippy -- -D warnings
	@echo "✅ Lint passed!"

clean:
	@echo "🧹 Cleaning..."
	@cd core && cargo clean
	@rm -rf platforms/macos/build
	@echo "✅ Clean complete!"

setup:
	@echo "🔧 Setting up..."
	@./scripts/setup.sh

install: build
	@echo "📦 Installing GoNhanh..."
	@cp -r platforms/macos/build/Release/GoNhanh.app /Applications/
	@echo "✅ Installed to /Applications/GoNhanh.app"

# Release: make release v=1.0.0
release:
ifndef v
	@echo "❌ Usage: make release v=1.0.0"
	@exit 1
endif
	@echo "🚀 Releasing v$(v)..."
	@git add -A
	@git commit -m "chore: release v$(v)" --allow-empty
	@git tag -a v$(v) -m "Release v$(v)"
	@git push origin main v$(v)
	@echo "✅ Released v$(v)!"
	@echo "👉 https://github.com/khaphanspace/gonhanh.org/releases/tag/v$(v)"
