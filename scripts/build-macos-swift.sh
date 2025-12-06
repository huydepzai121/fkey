#!/bin/bash
set -e

# Source rustup environment
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

echo "🍎 Building macOS app with swiftc..."

# Build core first
./scripts/build-core.sh

cd "$(dirname "$0")/../platforms/macos"

# Create build directory
mkdir -p build/Release

# Compile Swift files
echo "Compiling Swift sources..."
swiftc \
    -o build/Release/GoNhanh \
    -sdk $(xcrun --show-sdk-path) \
    -target arm64-apple-macos13.0 \
    -F /System/Library/Frameworks \
    -framework Foundation \
    -framework AppKit \
    -framework SwiftUI \
    -L . \
    -lgonhanh_core \
    -Xlinker -rpath -Xlinker @executable_path \
    App.swift MenuBar.swift SettingsView.swift RustBridge.swift

echo "✅ macOS app built successfully!"
echo "📦 Binary: platforms/macos/build/Release/GoNhanh"
echo ""
echo "To create .app bundle, run: make bundle"
