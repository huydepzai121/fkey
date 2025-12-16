# Validation Algorithm

> Thuật toán xác định chuỗi ký tự có phải âm tiết tiếng Việt hợp lệ hay không.

**Liên quan**: [vietnamese-language-system.md](./vietnamese-language-system.md) | [core-engine-algorithm.md](./core-engine-algorithm.md)

---

## 1. Mục đích

```
Validation xảy ra TRƯỚC khi transform:

"duoc" + j → VALID   → transform → "được" ✓
"claus" + s → INVALID → giữ nguyên → "clauss" ✓
"http" + s → INVALID → giữ nguyên → "https" ✓
```

Bảo vệ: code (`function`, `const`), tên riêng (`John`, `Claude`), từ mượn (`pizza`), URL/email.

---

## 2. Cấu trúc âm tiết

```
Syllable = (C₁)(G)V(C₂)

C₁ = Initial consonant (phụ âm đầu)  - optional
G  = Glide (âm đệm: o, u)            - optional
V  = Vowel nucleus (nguyên âm)       - REQUIRED
C₂ = Final consonant (âm cuối)       - optional
```

**Ví dụ:**
| Input | C₁ | G | V | C₂ |
|-------|------|-----|------|------|
| `a` | - | - | a | - |
| `ban` | b | - | a | n |
| `hoa` | h | o | a | - |
| `qua` | qu | - | a | - |
| `giau` | gi | - | au | - |
| `nghieng` | ngh | - | ie | ng |
| `duoc` | d | - | uo | c |

---

## 3. Data Constants

### 3.1 Phụ âm đầu (C₁)

```rust
// 16 phụ âm đơn
VALID_INITIALS_1: [b, c, d, g, h, k, l, m, n, p, q, r, s, t, v, x]

// 10 phụ âm đôi
VALID_INITIALS_2: [ch, gh, gi, kh, ng, nh, ph, qu, th, tr]

// 1 phụ âm ba: ngh
```

### 3.2 Âm cuối (C₂)

```rust
// 9 âm cuối đơn (gồm bán nguyên âm)
VALID_FINALS_1: [c, m, n, p, t, i, y, o, u]

// 3 âm cuối đôi
VALID_FINALS_2: [ch, ng, nh]
```

### 3.3 Quy tắc chính tả

| Consonant | Invalid trước | Nên dùng |
|-----------|---------------|----------|
| `c` | e, i, y | → `k` |
| `k` | a, o, u | → `c` |
| `g` | e | → `gh` |
| `ng` | e, i | → `ngh` |
| `gh` | a, o, u | → `g` |
| `ngh` | a, o, u | → `ng` |

### 3.4 Invalid Vowel Patterns (Exclusion Approach)

```rust
// Pattern nguyên âm KHÔNG tồn tại trong tiếng Việt
INVALID_VOWEL_PATTERNS: [[O, U], [Y, O]]
// ou → you, our, house, about, would
// yo → yoke, York, your, beyond
```

**Tại sao dùng Exclusion thay vì Inclusion?**

| Aspect | Inclusion (valid patterns) | Exclusion (invalid patterns) |
|--------|---------------------------|------------------------------|
| Danh sách | ~30+ diphthongs, ~10 triphthongs | Chỉ 2: `ou`, `yo` |
| Maintenance | Cao - phải cover hết | Thấp - chỉ thêm khi gặp |
| Risk | False negative (miss valid) | False positive (miss invalid) |

Validation hiện tại đã cover: initials (27), finals (12), spelling rules (6).
Chỉ còn rất ít vowel patterns "lọt lưới" → Exclusion hiệu quả hơn.

---

## 4. Thuật toán Parse

### 4.1 Syllable Structure

```rust
pub struct Syllable {
    pub initial: Vec<usize>,    // indices trong buffer
    pub glide: Option<usize>,   // index của âm đệm
    pub vowel: Vec<usize>,      // indices nguyên âm
    pub final_c: Vec<usize>,    // indices âm cuối
}
```

### 4.2 Parse Algorithm

```
parse(buffer_keys: &[u16]) -> Syllable

1. Tìm vị trí nguyên âm đầu tiên
   - Special case: gi + vowel → gi là initial
   - Special case: qu + vowel → qu là initial

2. Xác định glide (âm đệm)
   - o + (a, e) → o là glide
   - u + (y, e) khi không phải qu → u là glide

3. Thu thập nguyên âm liên tiếp

4. Phần còn lại là final consonant
   - Try 2-char: ch, ng, nh
   - Try 1-char: c, m, n, p, t, i, y, o, u
```

---

## 5. Validation Rules

Engine chạy 6 rules tuần tự. Rule đầu tiên fail → trả về lỗi ngay.

```rust
const RULES: &[Rule] = &[
    rule_has_vowel,           // Rule 1
    rule_valid_initial,       // Rule 2
    rule_all_chars_parsed,    // Rule 3
    rule_spelling,            // Rule 4
    rule_valid_final,         // Rule 5
    rule_valid_vowel_pattern, // Rule 6 (NEW)
];
```

### Rule 1: Has Vowel

```rust
// Phải có ít nhất 1 nguyên âm
if syllable.vowel.is_empty() → NoVowel
```

### Rule 2: Valid Initial

```rust
// Phụ âm đầu phải thuộc danh sách hợp lệ
match initial.len() {
    1 → check VALID_INITIALS_1
    2 → check VALID_INITIALS_2
    3 → chỉ cho phép "ngh"
    _ → InvalidInitial
}
```

### Rule 3: All Chars Parsed

```rust
// Mọi ký tự phải được parse vào cấu trúc
parsed_count = initial.len + glide(0|1) + vowel.len + final.len
if parsed_count != buffer.len → InvalidFinal
```

### Rule 4: Spelling

```rust
// Kiểm tra quy tắc chính tả c/k, g/gh, ng/ngh
for (consonant, invalid_vowels) in SPELLING_RULES {
    if initial == consonant && first_vowel in invalid_vowels {
        → InvalidSpelling
    }
}
```

### Rule 5: Valid Final

```rust
// Âm cuối phải thuộc danh sách hợp lệ
match final.len() {
    1 → check VALID_FINALS_1
    2 → check VALID_FINALS_2
    _ → InvalidFinal
}
```

### Rule 6: Valid Vowel Pattern

```rust
// Check vowel pairs không thuộc INVALID_VOWEL_PATTERNS
for pair in consecutive_vowel_pairs {
    if pair in INVALID_VOWEL_PATTERNS → InvalidVowelPattern
}
// Ví dụ: "you" có pair [O,U] → Invalid
// Ví dụ: "yeu" có pair [E,U] → Valid (không trong list)
```

---

## 6. Foreign Word Detection

Ngoài validation, engine còn có hàm `is_foreign_word_pattern()` để detect foreign words:

```rust
pub fn is_foreign_word_pattern(buffer_keys: &[u16], modifier_key: u16) -> bool
```

**Patterns detected:**
1. Invalid vowel patterns (ou, yo) trong buffer
2. Consonant clusters sau finals: T+R, P+R, C+R (metric, spectrum)
3. English prefix: "de" + 's' (describe, design)

**Đặc biệt:** Skip check khi đã có horn transforms (ư, ơ, ươ) → user đang gõ tiếng Việt có chủ đích (vd: "rượu").

---

## 7. API

```rust
/// Validate và trả về kết quả chi tiết
pub fn validate(buffer_keys: &[u16]) -> ValidationResult

/// Quick check
pub fn is_valid(buffer_keys: &[u16]) -> bool

/// Check foreign word pattern (for modifier skipping)
pub fn is_foreign_word_pattern(buffer_keys: &[u16], modifier_key: u16) -> bool

pub enum ValidationResult {
    Valid,
    InvalidInitial,
    InvalidFinal,
    InvalidSpelling,
    InvalidVowelPattern,  // NEW
    NoVowel,
}
```

---

## 8. Test Cases

### Valid

```
ba, ca, an, em, gi, gia, giau, ke, ki, ky
nghe, nghi, nghieng, truong, nguoi, duoc
```

### Invalid - No Vowel

```
bcd, bcdfgh
```

### Invalid - Bad Initial

```
clau, john, bla, string, chrome
```

### Invalid - Spelling

```
ci, ce, cy     → nên dùng ki, ke, ky
ka, ko, ku     → nên dùng ca, co, cu
ngi, nge       → nên dùng nghi, nghe
ge             → nên dùng ghe
```

### Invalid - Foreign Words

```
exp, expect, test, claudeco, claus
```

### Invalid - Vowel Patterns (NEW)

```
you, your, house, about, would, south  → "ou" pattern
yoke, York, beyond                      → "yo" pattern
metric, spectrum, matrix                → T+R, C+R clusters
describe, design                        → "de" + 's' prefix
```

---

## 9. Integration với Engine

```
on_key(key)
│
├─ [is_modifier(key)?]
│  │
│  ├─ ★ VALIDATION: Trước khi transform
│  │   └─ is_valid(buffer)?
│  │       ├─ NO  → return NONE (không transform)
│  │       └─ YES → tiếp tục transform
│  │
│  └─ Apply transformation
│
└─ [is_letter(key)?] → push to buffer
```

---

## Changelog

- **2025-12-16**: Thêm Rule 6 (Vowel Pattern Validation)
  - Thêm `INVALID_VOWEL_PATTERNS` (ou, yo) vào constants
  - Thêm `rule_valid_vowel_pattern` - Rule 6
  - Thêm `is_foreign_word_pattern()` cho foreign word detection
  - Thêm `InvalidVowelPattern` vào ValidationResult
  - Document approach: Exclusion vs Inclusion (chọn Exclusion)
  - Fix issue #15: "metric" không còn bị transform thành "mẻtic"

- **2025-12-11**: Viết lại document theo code thực tế
  - Cập nhật Syllable struct với `Vec<usize>` và `glide` field
  - Chỉnh lại 5 validation rules theo code
  - Loại bỏ pseudo-code sai, thay bằng code snippets chính xác
  - Rút gọn từ ~800 dòng xuống ~200 dòng
