# Gõ Nhanh: Project Overview & Product Development Requirements

## Project Vision

Gõ Nhanh is a **high-performance Vietnamese input method engine** (IME) for macOS that enables fast, accurate Vietnamese text input with minimal system overhead. The project demonstrates production-grade system software design: Rust core for performance and safety, SwiftUI for native macOS integration, and validated patterns for Vietnamese phonology.

## Product Goals

1. **Performance**: Sub-millisecond keystroke latency (<1ms)
2. **Reliability**: Comprehensive validation-first transformation pipeline
3. **User Experience**: Seamless platform integration via CGEventTap keyboard hook
4. **Memory Efficiency**: ~5MB memory footprint with optimized binary packaging

## Target Users

- **Primary**: Vietnamese professionals and students on macOS who type Vietnamese daily
- **Secondary**: Vietnamese diaspora, bilingual professionals
- **Requirement**: macOS 10.15+ with Accessibility permissions enabled

## Core Functional Requirements

### Input Methods
- **Telex**: Vietnamese keyboard layout (VIQR-style: a's → á)
- **VNI**: Alternative numeric layout support
- **Shortcuts**: User-defined abbreviations with priority matching

### Keystroke Processing
1. Buffer management: Maintain context for multi-keystroke transforms
2. Validation: Check syllable against Vietnamese phonology rules
3. Transformation: Apply diacritics (sắc, huyền, hỏi, ngã, nặng) and tone modifiers (circumflex, horn)
4. Output: Send backspace + replacement characters or pass through

### Platform Integration
- CGEventTap keyboard hook intercepts keyDown events system-wide
- Smart text replacement: Backspace method (Terminal) + Selection method (Chrome/Excel)
- Ctrl+Space global hotkey for Vietnamese/English toggle
- Application detection: Specialized handling for autocomplete apps

## Non-Functional Requirements

### Performance
- Keystroke latency: <1ms measured end-to-end
- CPU usage: <2% during normal typing
- Memory: ~5MB resident set size
- No input delay under sustained high-speed typing

### Reliability
- 160+ integration tests covering edge cases
- Validation-first pattern: Reject invalid Vietnamese before transforming
- Graceful fallback: Pass through on disable or invalid input
- Thread-safe global engine instance via Mutex

### Compatibility
- macOS 10.15 Catalina and later
- Apple Silicon (arm64) and Intel (x86_64) universal binaries
- Works with all major applications: Terminal, VS Code, Chrome, Safari, Office

### Security
- No internet access required (offline-first)
- GPL-3.0-or-later license (free and open source)
- Accessibility permission: Required for keyboard hook (transparent to user)
- No telemetry or analytics

## Architecture Overview

```
User Keystroke (CGEventTap)
        ↓
   RustBridge (FFI Bridge)
        ↓
   Rust Engine (ime_key)
    ├─ Buffer Management (circular 64-char)
    ├─ Validation (Vietnamese syllable rules)
    ├─ Transform (diacritics + tone modifiers)
    └─ Shortcut Lookup (user-defined abbreviations)
        ↓
   Result (action, backspace count, output chars)
        ↓
   SwiftUI (Send text or pass through)
```

## Success Metrics

| Metric | Target | Current |
|--------|--------|---------|
| Keystroke latency | <1ms | ~0.2-0.5ms |
| Memory usage | <10MB | ~5MB |
| Test coverage | >90% | 160+ tests |
| macOS compatibility | 10.15+ | Validated |
| User satisfaction | 4.5/5 stars | Active community |

## Roadmap

### Phase 1: macOS (Complete)
- Telex + VNI input methods
- Menu bar app with settings
- Auto-launch on login
- Update checker via GitHub releases

### Phase 2: Cross-Platform (Planned)
- **Windows 10/11**: DirectX keyboard hook + C# WPF UI
- **Linux**: X11/Wayland event hook + Qt UI
- Feature parity with macOS version

### Phase 3: Enhanced Features (Future)
- Cloud sync for user preferences
- Machine learning for shortcut suggestions
- Dictionary lookup integration
- Advanced diacritics editor

## Development Standards

### Code Organization
- **Core** (`core/src/`): Rust engine, pure logic, zero platform dependencies
- **Platform** (`platforms/macos/`): SwiftUI UI, platform integration, FFI bridge
- **Scripts** (`scripts/`): Build automation for universal binaries
- **Tests** (`core/tests/`): Integration tests + unit tests

### Quality Gates
- Format: `cargo fmt` (automatic formatting)
- Lint: `cargo clippy -- -D warnings` (no warnings allowed)
- Tests: `cargo test` (160+ tests must pass)
- Build: Universal binary creation (arm64 + x86_64)

### Commit Message Format
Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): subject

body

footer
```

Examples:
- `feat(engine): add shortcut expansion for common abbreviations`
- `fix(transform): correct diacritic placement for ư vowel`
- `docs(ffi): update RustBridge interface documentation`
- `test(validation): add edge cases for invalid syllables`

## Dependencies

### Rust
- Zero production dependencies (pure stdlib)
- Dev: `rstest` for parametrized tests

### Swift/macOS
- Foundation: URLSession, UserDefaults, FileHandle
- AppKit: NSApplication, NSStatusBar, CGEventTap
- SwiftUI: Standard UI components, macOS 11+ features

### Build Tools
- `cargo` (Rust toolchain)
- `xcodebuild` (macOS app build)
- GNU Make (build automation)

## Maintenance & Support

### Release Schedule
- Patch releases: Bug fixes and small improvements (monthly)
- Minor releases: New features (quarterly)
- Major releases: Breaking changes (annually or as needed)

### Community
- GitHub Issues: Bug reports and feature requests
- GitHub Discussions: Questions and community support
- Contributing: GPL-3.0 requires contributor agreement

## Success Criteria for Milestones

**v1.0 Release**
- All core input methods working reliably
- Sub-1ms latency confirmed
- 160+ tests passing
- macOS app in official release

**v1.1+ Releases**
- Cross-platform support (Windows/Linux)
- User-customizable shortcuts
- Enhanced documentation
- Community contribution guidelines

---

**Last Updated**: 2025-12-09
**Status**: Active Development
**Repository**: https://github.com/khaphanspace/gonhanh.org
