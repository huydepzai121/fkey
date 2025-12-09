# Gõ Nhanh: Codebase Summary

## Project Structure

```
gonhanh.org/
├── core/                          # Rust core engine (no dependencies)
│   ├── src/
│   │   ├── lib.rs                # FFI exports (ime_init, ime_key, ime_method, etc.)
│   │   ├── engine/               # Main processing pipeline
│   │   │   ├── mod.rs            # Engine struct + keystroke processing
│   │   │   ├── buffer.rs         # Circular input buffer (64 chars max)
│   │   │   ├── syllable.rs       # Syllable state tracking
│   │   │   ├── validation.rs     # Vietnamese phonology rules
│   │   │   ├── transform.rs      # Diacritic + tone transformations
│   │   │   └── shortcut.rs       # User-defined abbreviation expansion
│   │   ├── input/                # Input method definitions
│   │   │   ├── mod.rs            # ToneType, InputMethod enums
│   │   │   ├── telex.rs          # Telex: a's → á
│   │   │   └── vni.rs            # VNI: a13 → á
│   │   ├── data/                 # Static character/key mappings
│   │   │   ├── chars.rs          # Vietnamese vowel lookup tables
│   │   │   ├── vowel.rs          # Phonology rules (consonant clusters, vowel patterns)
│   │   │   ├── keys.rs           # macOS virtual keycodes
│   │   │   ├── constants.rs      # Telex/VNI key mappings
│   │   │   └── mod.rs            # Module exports
│   │   ├── updater/              # GitHub release checking
│   │   └── utils.rs              # Utility functions
│   ├── tests/                    # 160+ integration tests
│   │   ├── integration_test.rs   # Full pipeline tests
│   │   ├── unit_test.rs          # Individual module tests
│   │   ├── engine_test.rs        # Engine-specific cases
│   │   ├── typing_test.rs        # Real-world typing sequences
│   │   ├── paragraph_test.rs     # Multi-word paragraph tests
│   │   └── common/mod.rs         # Test utilities
│   ├── Cargo.toml                # Package metadata + build settings
│   └── Cargo.lock                # Dependency lock (none for core!)
│
├── platforms/
│   └── macos/                    # SwiftUI native macOS app
│       ├── App.swift             # Main app entry point
│       ├── RustBridge.swift      # FFI bridge (480 lines)
│       │   ├── ImeResult struct  # FFI result struct (#[repr(C)])
│       │   ├── C function imports
│       │   ├── RustBridge class  # Safe Rust API wrapper
│       │   └── KeyboardHookManager# CGEventTap setup + management
│       ├── MenuBar.swift         # Status bar controller (310 lines)
│       ├── SettingsView.swift    # Settings UI
│       ├── OnboardingView.swift  # First-run wizard
│       ├── AboutView.swift       # About window
│       ├── UpdateView.swift      # Update checker UI
│       ├── UpdateManager.swift   # GitHub release API
│       ├── LaunchAtLogin.swift   # Auto-start on login
│       └── AppMetadata.swift     # Constants (version, URLs)
│
├── scripts/                      # Build automation
│   ├── build-core.sh            # Universal Rust library (arm64 + x86_64)
│   ├── build-macos.sh           # xcodebuild for app
│   ├── build-windows.sh         # Windows build stub
│   └── setup.sh                 # Dev environment setup
│
├── .github/workflows/
│   ├── ci.yml                   # Format → Clippy → Tests on push/PR
│   └── release.yml              # Semantic release + DMG building
│
├── Makefile                      # Development targets
├── Cargo.toml                    # Workspace config
├── README.md                     # Project overview
└── LICENSE                       # GPL-3.0-or-later
```

## Key Files & Responsibilities

### FFI Interface (lib.rs)
- **Purpose**: Exports C ABI functions for Swift to call
- **Key Functions**:
  - `ime_init()` - Initialize engine once
  - `ime_key(keycode, caps, ctrl)` - Process keystroke
  - `ime_method(method)` - Set Telex/VNI mode
  - `ime_enabled(enabled)` - Enable/disable
  - `ime_clear()` - Reset buffer (word boundary)
  - `ime_free(result)` - Deallocate result

### Engine Core (engine/mod.rs)
- **Architecture**: Validation-first transformation
- **Pipeline**:
  1. Check if buffer is valid Vietnamese phonology
  2. Find matching transformation patterns
  3. Apply diacritics/tone modifiers (longest-match-first)
  4. Check shortcuts for user-defined abbreviations
  5. Return action (None/Send/Restore) with backspace count + output chars

### Buffer Management (engine/buffer.rs)
- **Type**: Circular buffer with 64-char capacity
- **Tracks**: Characters + their associated marks/tones
- **Used For**: Context for multi-keystroke transforms (e.g., "oa" → "oà")

### Syllable State (engine/syllable.rs)
- **Tracks**: Which position (start/middle/end) for consonant placement
- **Rules**: Vietnamese only allows consonants at specific positions
- **Example**: "bộ" = b(consonant) + ộ(vowel with tone)

### Validation (engine/validation.rs)
- **Purpose**: Reject invalid Vietnamese before transform
- **Checks**:
  - Valid consonant clusters at syllable start
  - Valid vowel patterns + tone combinations
  - Correct consonant position (start/final)

### Transformation (engine/transform.rs)
- **Operations**:
  - Apply tone marks (sắc, huyền, hỏi, ngã, nặng)
  - Apply vowel modifiers (circumflex: â, ô, ê; horn: ă, ơ, ư)
  - Longest-match-first for multi-character sequences

### Input Methods (input/telex.rs, input/vni.rs)
- **Telex Mapping**: Key sequence to tone/mark (e.g., "a" + "s" → sắc mark)
- **VNI Mapping**: Numeric codes (e.g., "a" + "1" → sắc mark)
- **Extensible**: New input methods just need ToneType implementation

### Data Tables (data/)
- **chars.rs**: 72-entry vowel table (12 bases × 6 marks)
- **vowel.rs**: Vietnamese phonology rules (initial consonant clusters, vowel groups)
- **keys.rs**: macOS virtual keycodes (constants for a-z, space, backspace)
- **constants.rs**: Telex/VNI key mappings

### Swift Bridge (platforms/macos/RustBridge.swift)
- **Role**: Safe wrapper around raw C FFI
- **Struct Alignment**: ImeResult matches Rust #[repr(C)] layout exactly
- **Features**:
  - Initialize once, thread-safe
  - Extract UTF-32 codepoints from fixed-size tuple
  - Convert to Swift Character array

### Keyboard Hook (platforms/macos/RustBridge.swift)
- **Method**: CGEventTap with `listenOnly` option
- **Advantages**: Works in more apps than alternatives
- **Fallback**: Triple fallback (HID → Session → AnnotatedSession)
- **Smart Replacement**:
  - Backspace method: Most apps (Terminal, VSCode)
  - Selection method: Autocomplete apps (Chrome, Excel) to fix "dính chữ"

### Menu Bar UI (platforms/macos/MenuBar.swift)
- **Status Icon**: "V" (Vietnamese) or "E" (English) badge
- **Menu Items**:
  - Toggle switch (Ctrl+Space hotkey)
  - Input method selector (Cmd+1/Cmd+2)
  - Settings, About, Update check, Help
- **Auto-launch**: Opt-in during onboarding

## Module Dependencies

```
lib.rs (FFI Public API)
  ├─ engine::Engine ✓ (public types)
  │  ├─ buffer::Buffer, Char
  │  ├─ syllable::Syllable
  │  ├─ validation::is_valid
  │  ├─ transform::Transform
  │  ├─ shortcut::ShortcutTable
  │  └─ input::{ToneType, InputMethod}
  │
  ├─ input (Telex/VNI)
  │  ├─ data::keys (keycodes)
  │  └─ data::constants (tone mappings)
  │
  └─ data (all static)
     ├─ chars::* (vowel tables)
     ├─ vowel::Phonology (rules)
     ├─ keys::* (keycodes)
     └─ constants::* (mappings)
```

## Entry Points for Development

### Adding a New Input Method
1. Create `core/src/input/your_method.rs`
2. Implement tone mapping (reference: telex.rs, vni.rs)
3. Update `engine/mod.rs` to handle new method
4. Add tests in `core/tests/`
5. Update Swift UI in MenuBar.swift

### Fixing a Vietnamese Phonology Issue
1. Check failing test in `core/tests/`
2. Review `engine/validation.rs` for rule check
3. May need to update `data/vowel.rs` for consonant/vowel rules
4. Add regression test + fix

### Improving Performance
1. Profile with `make build` (cargo release mode)
2. Check `engine/buffer.rs` (circular buffer efficiency)
3. Review `engine/validation.rs` (early rejection patterns)
4. Benchmark specific test cases

### Cross-Platform Port
1. **Windows**: Create `platforms/windows/` with C# WPF or similar
2. **Linux**: Create `platforms/linux/` with Qt or GTK
3. **Core**: Reuse `core/` unchanged (zero platform deps)
4. **FFI**: Adapt platform bindings (C ABI remains consistent)

## Build System

### Makefile Targets
```bash
make test       # Run Rust tests (160+ tests)
make format     # cargo fmt + clippy check
make build      # Full release build (core + macos)
make clean      # Clean build artifacts
make install    # Install to /Applications
make release    # Patch version tag + push
```

### CI/CD Pipeline
- **ci.yml**: Format, clippy, tests on every push/PR (blocks merge if fails)
- **release.yml**: On version tag, builds DMG and creates GitHub release

## Testing Strategy

### Test Structure
- **Unit Tests** (inline): `#[test]` in each module
- **Integration Tests** (`core/tests/`):
  - `integration_test.rs` - Full FFI pipeline
  - `unit_test.rs` - Component-specific
  - `engine_test.rs` - Engine logic edge cases
  - `typing_test.rs` - Real typing sequences
  - `paragraph_test.rs` - Multi-word Vietnamese text

### Test Coverage
- 160+ tests total
- Examples: "Telex á → a+s", "VNI ạ → a+6", "ư vowel variants", "consonant cluster validation"
- Parametrized with `rstest` macro for thoroughness

---

**Total Code**: ~3,500 lines Rust + ~765 lines Swift
**Repomix Analysis**: 50,314 tokens, 199,893 chars
**Last Updated**: 2025-12-09
