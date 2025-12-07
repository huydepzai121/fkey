# Development Guide

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | 1.70+ | [rustup.rs](https://rustup.rs) |
| Xcode | 15+ | App Store |
| macOS | 13+ | - |

## Quick Start

```bash
# Clone
git clone https://github.com/khaphanspace/gonhanh.org
cd gonhanh.org

# Setup (install Rust targets)
make setup

# Test
make test

# Build everything
make build
```

## Makefile Commands

| Command | Description |
|---------|-------------|
| `make help` | Show all commands |
| `make setup` | Setup dev environment |
| `make test` | Run all tests |
| `make core` | Build Rust core only |
| `make macos` | Build macOS app |
| `make build` | Build everything (test → core → macos) |
| `make clean` | Clean all build artifacts |
| `make install` | Install to /Applications |

## Project Structure

```
gonhanh.org/
├── Makefile              # Main build commands
├── core/                 # Rust core library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs        # FFI exports
│   │   ├── data/         # Keys, chars, phonology
│   │   ├── engine/       # Main engine
│   │   └── input/        # Telex, VNI methods
│   └── tests/            # Integration tests
├── platforms/
│   └── macos/            # SwiftUI app
│       ├── App.swift
│       ├── MenuBar.swift
│       ├── SettingsView.swift
│       └── RustBridge.swift
├── scripts/              # Build scripts
│   ├── setup.sh
│   ├── build-core.sh
│   └── build-macos.sh
└── docs/
    ├── architecture.md
    ├── development.md
    └── vietnamese-language-system.md
```

## Testing

```bash
# Run all tests
make test

# Run specific test
cd core && cargo test telex

# Run with output
cd core && cargo test -- --nocapture

# Run single test
cd core && cargo test vni_delayed_d_input
```

### Test Files

| File | Coverage |
|------|----------|
| `tests/basic_test.rs` | Single char transformations |
| `tests/word_test.rs` | Full word typing |
| `tests/sentence_test.rs` | Sentence-level tests |

## Building

### Rust Core Only

```bash
make core
# Output: platforms/macos/libgonhanh_core.a
```

### macOS App

```bash
make macos
# Output: platforms/macos/build/Release/GoNhanh.app
```

### Full Build

```bash
make build
# Runs: test → core → macos
```

## Debugging

### Rust

```bash
cd core
RUST_LOG=debug cargo test -- --nocapture
```

### macOS

1. Build with Xcode for debugging:
   ```bash
   ./scripts/build-macos.sh
   ```

2. Open Console.app, filter by `GoNhanh`

3. Check Accessibility permission:
   - System Settings → Privacy & Security → Accessibility
   - Add GoNhanh.app

## Common Issues

### "Library not found"

```bash
make core  # Build Rust core first
```

### "Keyboard hook not working"

Grant Accessibility permission in System Settings.

### Test failures

```bash
make clean
make test
```

## Release Build

```bash
# Build optimized
make build

# App location
open platforms/macos/build/Release/
```

## Code Style

- **Rust**: `cargo fmt`, `cargo clippy`
- **Swift**: Xcode default formatting
- **Commits**: Conventional commits (`feat:`, `fix:`, `docs:`)
