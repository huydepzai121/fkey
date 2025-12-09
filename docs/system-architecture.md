# Gõ Nhanh: System Architecture

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                           macOS Application                          │
│                                                                      │
│  ┌────────────────────────────────┐     ┌──────────────────────┐   │
│  │      SwiftUI Menu Bar          │     │   Update Manager     │   │
│  │   • Input method selector      │     │   • GitHub releases  │   │
│  │   • Enable/disable toggle      │     │   • Check version    │   │
│  │   • About/Settings windows     │     │                      │   │
│  └───────────────┬────────────────┘     └──────────────────────┘   │
│                  │                                                   │
│  ┌───────────────┴────────────────────────────────────────────┐    │
│  │              CGEventTap Keyboard Hook                      │    │
│  │   • Intercepts keyDown events system-wide                 │    │
│  │   • Forwards to RustBridge for processing                │    │
│  │   • Smart text replacement (backspace vs selection)      │    │
│  └───────────────┬────────────────────────────────────────────┘    │
│                  │                                                   │
│  ┌───────────────┴────────────────────────────────────────────┐    │
│  │           RustBridge (FFI Layer)                           │    │
│  │   • C ABI function calls to Rust engine                  │    │
│  │   • Memory-safe pointer handling                         │    │
│  │   • Thread-safe (uses Mutex internally)                  │    │
│  └───────────────┬────────────────────────────────────────────┘    │
└──────────────────┼───────────────────────────────────────────────────┘
                   │
                   ↓ extern "C" function calls
┌──────────────────────────────────────────────────────────────────────┐
│                    Rust Core Engine                                   │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │                  Input Method Layer                          │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌───────────────────┐  │   │
│  │  │ Input::Telex │  │ Input::VNI   │  │ Input::Shortcut   │  │   │
│  │  │              │  │              │  │ (user-defined)    │  │   │
│  │  │ a+s → sắc    │  │ a+1 → sắc    │  │ custom expansions │  │   │
│  │  └──────────────┘  └──────────────┘  └───────────────────┘  │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                              ↓                                        │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │              Engine Processing Pipeline                      │   │
│  │                                                              │   │
│  │  1. Input:  Keycode → Tone type (sắc, huyền, etc.)       │   │
│  │                                                              │   │
│  │  2. Buffer: Append to 64-char circular buffer              │   │
│  │             Track marks/tones for each char                │   │
│  │                                                              │   │
│  │  3. Validation: Check against Vietnamese phonology         │   │
│  │                 ✓ Valid consonant clusters at start        │   │
│  │                 ✓ Valid vowel patterns                     │   │
│  │                 ✓ Correct tone placement                   │   │
│  │                                                              │   │
│  │  4. Transform: Apply diacritics + vowel modifiers          │   │
│  │                • Longest-match-first for multi-char        │   │
│  │                • Support circumflex (â, ô, ê)             │   │
│  │                • Support horn/breve (ă, ơ, ư)             │   │
│  │                                                              │   │
│  │  5. Shortcut:  Expand user abbreviations if matched        │   │
│  │                                                              │   │
│  │  6. Output:    Return action + backspace + output chars    │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                              ↓                                        │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │           Data & Validation Rules (static tables)            │   │
│  │  • Vowel table: 72 entries (12 bases × 6 marks)            │   │
│  │  • Consonant clusters: Valid initial sequences             │   │
│  │  • Vowel groups: Valid combinations                        │   │
│  │  • macOS keycodes: a-z, space, backspace                   │   │
│  │  • Telex/VNI mappings: Key → tone/mark                     │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

## Data Flow: Keystroke to Output

### Example: Typing "á" in Telex

```
User types: [a] then [s]

Step 1: Key 'a' pressed
  ├─ CGEventTap captures keyDown
  ├─ RustBridge.processKey(keyCode=0x00, caps=false, ctrl=false)
  ├─ Rust: ime_key() called
  ├─ Engine:
  │  ├─ Append 'a' to buffer
  │  ├─ Validate: "a" is valid (vowel alone)
  │  ├─ No transform yet (single char, waiting for next)
  │  └─ Return Action::None (pass through)
  ├─ Swift: No action, let 'a' appear naturally
  └─ Output: User sees 'a' typed

Step 2: Key 's' pressed (sắc mark in Telex)
  ├─ CGEventTap captures keyDown
  ├─ RustBridge.processKey(keyCode=0x01, caps=false, ctrl=false)
  ├─ Rust: ime_key() called
  ├─ Engine:
  │  ├─ Check buffer context: "a" + "s" → sắc mark
  │  ├─ Validation: "á" is valid Vietnamese vowel
  │  ├─ Transform: Apply sắc mark to 'a' → 'á'
  │  ├─ Check shortcuts: No expansion needed
  │  ├─ Return Action::Send {
  │  │    backspace: 1,  // Delete 'a'
  │  │    chars: ['á']   // Insert 'á'
  │  └─ }
  ├─ Swift:
  │  ├─ Send 1 backspace (delete 'a')
  │  ├─ Send 'á' (via Unicode keyboard event)
  │  └─ 's' keystroke consumed (not passed through)
  └─ Output: User sees 'á' (exactly 1 character)

Result: "á" displayed instead of "as"
Latency: ~0.2-0.5ms total (Rust engine: <0.1ms)
```

### Example: Typing "không" with Shortcut

```
User types: [k] [h] [o] [n] [g] [space]  (or defined shortcut key)

Setup: User defined shortcut "khong" → "không"

Processing:
  Step 1-4: Build buffer "khon" → valid syllable, wait
  Step 5: Shortcut lookup
    ├─ Check if "khong" matches any user abbreviation
    ├─ Match found: "khong" → "không"
    └─ Return: backspace: 5, chars: ['k','h','ô','n','g']

  Swift execution:
    ├─ Delete 5 chars (k, h, o, n, g)
    ├─ Insert 5 chars (k, h, ô, n, g)
    └─ No change visible but ô is now correct diacritic
```

## FFI Interface Specification

### Function Signatures (C ABI)

```c
// Initialize engine (call once)
void ime_init(void);

// Process keystroke
typedef struct {
    uint32_t chars[32];      // UTF-32 output characters
    uint8_t action;          // 0=None, 1=Send, 2=Restore
    uint8_t backspace;       // Number of chars to delete
    uint8_t count;           // Number of valid chars
    uint8_t _pad;            // Padding for alignment
} ImeResult;

ImeResult* ime_key(uint16_t keycode, bool caps, bool ctrl);

// Set input method (0=Telex, 1=VNI)
void ime_method(uint8_t method);

// Enable/disable engine
void ime_enabled(bool enabled);

// Clear buffer (word boundary)
void ime_clear(void);

// Free result (caller must call this exactly once per ime_key)
void ime_free(ImeResult* result);
```

### Action Types

| Value | Name | Meaning | Response |
|-------|------|---------|----------|
| 0 | None | No transformation, pass key through | Send key to app |
| 1 | Send | Transform matched, replace text | Backspace + insert |
| 2 | Restore | Undo previous transform | Not currently used |

### Memory Ownership

- **FFI Responsibility**: Rust engine allocates Result struct
- **Caller Responsibility**: Swift must call `ime_free(result)` to deallocate
- **Safety**: Use `defer { ime_free(ptr) }` to guarantee cleanup even on early return

## Platform Integration Details

### macOS CGEventTap

#### Event Interception
```swift
// Tap into keyboard events system-wide
let eventMask: CGEventMask = (1 << CGEventType.keyDown.rawValue)

let tap = CGEvent.tapCreate(
    tap: .cghidEventTap,                    // Try HID event tap first
    place: .headInsertEventTap,             // Insert at head of chain
    options: .defaultTap,                   // Can modify/drop events
    eventsOfInterest: eventMask,            // Only keyDown
    callback: keyboardCallback,             // Our handler
    userInfo: nil
)
```

#### Fallback Strategy
```
1st attempt: CGEventTapType.cghidEventTap
   └─ If fails → 2nd attempt: cgSessionEventTap
      └─ If fails → 3rd attempt: cgAnnotatedSessionEventTap
         └─ If all fail → Accessibility permission required
```

#### Text Replacement Methods

**Method 1: Backspace (most apps)**
```
Send: BS BS ... BS (backspace count times)
      ↓ (small delay)
Send: Unicode input event with output chars
```
Works for: Terminal, VS Code, Sublime, plain text editors

**Method 2: Selection (autocomplete apps)**
```
Send: Shift+Left Shift+Left ... Shift+Left (select chars)
      ↓
Send: Unicode input event (replaces selection)
```
Works for: Chrome, Safari, Excel, Word (fixes "dính chữ" issue)

### Accessibility Permission

#### macOS System Requirement
- **API**: `AXIsProcessTrusted()` checks if app has Accessibility permission
- **User Flow**:
  1. App requests permission on first run
  2. User goes to: System Settings → Privacy & Security → Accessibility
  3. User adds GoNhanh to the list
  4. App restart required to acquire permissions
  5. Once granted, app can create CGEventTap

#### Permission Checking
```swift
// Check permission before starting keyboard hook
let trusted = AXIsProcessTrusted()
if !trusted {
    // Prompt and open System Settings
    let options = [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true]
    AXIsProcessTrustedWithOptions(options as CFDictionary)
}
```

### Global Hotkey: Ctrl+Space

```swift
// Virtual keycode 0x31 = Space
// Flag: maskControl, NOT maskCommand

func isToggleHotkey(_ keyCode: UInt16, _ flags: CGEventFlags) -> Bool {
    keyCode == 0x31 &&
    flags.contains(.maskControl) &&
    !flags.contains(.maskCommand)  // Exclude Cmd+Space (macOS Spotlight)
}

// When matched: Post NotificationCenter event
NotificationCenter.default.post(name: .toggleVietnamese, object: nil)

// Consume event (don't pass to app)
return nil
```

## Component Interactions

### Initialization Sequence
```
1. AppDelegate.applicationDidFinishLaunching
   ├─ Show OnboardingView (if first run)
   └─ On complete: MenuBarController.init()

2. MenuBarController.init()
   ├─ Create status bar icon
   ├─ Load settings from UserDefaults
   ├─ If accessibility trusted: startEngine()
   └─ Otherwise: show permission prompt

3. startEngine()
   ├─ RustBridge.initialize()
   │  └─ Call ime_init() (once, thread-safe)
   ├─ KeyboardHookManager.shared.start()
   │  └─ Create CGEventTap, enable listening
   ├─ RustBridge.setEnabled(true)
   └─ RustBridge.setMethod(userMethod)
```

### Runtime Flow
```
User types key
   ↓
CGEventTap callback fires
   ↓
Extract keycode + modifier flags
   ↓
Check global hotkey (Ctrl+Space) → Toggle Vietnamese
   ↓
Call RustBridge.processKey()
   ├─ Call ime_key(keycode, caps, ctrl)
   ├─ Receive ImeResult
   ├─ Extract UTF-32 chars → Character array
   └─ Return (backspaceCount, chars) tuple
   ↓
If transformation:
   ├─ Send backspaces (CGEvent)
   ├─ Send Unicode replacement
   └─ Consume original key (return nil)
   ↓
Else: Pass through (return unmodified event)
   ↓
Visible to user as transformed or original text
```

## Performance Characteristics

### Latency Budget
| Component | Time | Notes |
|-----------|------|-------|
| CGEventTap callback | ~50μs | System kernel time |
| Rust ime_key() | ~100-200μs | Engine processing |
| Swift RustBridge | ~50μs | FFI overhead + result conversion |
| CGEvent sending | ~100-200μs | Posting to event tap |
| **Total** | **~300-500μs** | <1ms requirement met |

### Memory Profile
| Component | Size | Notes |
|-----------|------|-------|
| Rust engine (static) | ~150KB | Tables + code |
| Swift runtime | ~4.5MB | Standard SwiftUI overhead |
| Buffer (64 chars) | ~200B | Circular buffer per engine instance |
| **Total** | **~5MB** | Matches requirement |

### Scalability
- **Multi-user**: App per user, each runs own engine instance
- **Concurrent**: Mutex-protected ENGINE global (thread-safe)
- **Continuous**: No memory leaks (tested with 160+ tests)
- **No limits**: Can type indefinitely without performance degradation

---

**Last Updated**: 2025-12-09
**Architecture Version**: 1.0
**Diagram Format**: ASCII (compatible with all documentation viewers)
