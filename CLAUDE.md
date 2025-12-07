# CLAUDE.md

## Project
GoNhanh - Vietnamese input method engine. Rust core + native platform apps.

## Structure
```
core/           # Rust FFI library
platforms/
  macos/        # SwiftUI app
  windows/      # WPF (planned)
```

## Commands
```bash
# Build
cd core && cargo build --release
./scripts/build-macos.sh

# Test
cd core && cargo test
```

## Code Style
- Rust: `cargo fmt`, `cargo clippy`
- Swift: SwiftLint defaults
- Commits: conventional commits (feat/fix/docs/refactor)

## Contributing
1. Fork & create feature branch
2. Write tests for new features
3. Ensure `cargo test` passes
4. Open PR with clear description
