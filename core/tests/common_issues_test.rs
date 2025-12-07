//! Common Issues Tests
//!
//! Test cases for common issues documented in docs/common-issues.md
//! These tests verify the engine handles edge cases correctly.

mod common;
use common::{run_telex, run_vni};

// ============================================================
// ISSUE 2.1: Dính chữ (aa -> aâ instead of â)
// Engine should return correct backspace count
// ============================================================

#[test]
fn telex_circumflex_not_sticky() {
    // These should produce single character output, not doubled
    run_telex(&[
        ("aa", "â"), // NOT "aâ"
        ("ee", "ê"), // NOT "eê"
        ("oo", "ô"), // NOT "oô"
        ("dd", "đ"), // NOT "dđ"
        ("DD", "Đ"), // NOT "DĐ"
    ]);
}

#[test]
fn vni_modifier_not_sticky() {
    run_vni(&[
        ("a6", "â"), // NOT "a6" or "aâ"
        ("e6", "ê"),
        ("o6", "ô"),
        ("d9", "đ"),
        ("D9", "Đ"),
    ]);
}

// ============================================================
// ISSUE 2.4: Lặp chữ (được -> đđược)
// Engine buffer should handle 'd' correctly
// ============================================================

#[test]
fn telex_no_double_d() {
    // đ should appear once, not twice
    run_telex(&[
        ("dduwowcj", "được"), // NOT "đđược"
        ("ddif", "đì"),       // NOT "đđì"
        ("ddi", "đi"),        // NOT "đđi"
        ("ddang", "đang"),    // NOT "đđang"
        ("ddaauf", "đầu"),    // NOT "đđầu"
    ]);
}

#[test]
fn vni_no_double_d() {
    run_vni(&[
        ("d9u7o7c5", "được"), // NOT "đđược"
        ("d9i", "đi"),
        ("d9ang", "đang"),
    ]);
}

// ============================================================
// ISSUE 2.4: Mất dấu (trường -> trương)
// Tone mark should be preserved on correct vowel
// ============================================================

#[test]
fn telex_preserve_tone_mark() {
    // Mark should appear on correct vowel
    run_telex(&[
        ("truwowngf", "trường"), // NOT "trương"
        ("dduwowngf", "đường"),  // NOT "đương"
        ("nguwowif", "người"),   // NOT "ngươi"
        ("muwowif", "mười"),     // NOT "mươi"
    ]);
}

#[test]
fn vni_preserve_tone_mark() {
    run_vni(&[
        ("tru7o7ng2", "trường"),
        ("d9u7o7ng2", "đường"),
        ("ngu7o7i2", "người"),
    ]);
}

// ============================================================
// ISSUE: Mark repositioning after diacritic
// When adding diacritic changes phonology, mark must move
// e.g., "ua2" → "uà", then "7" → should be "ừa" not "ưà"
// ============================================================

#[test]
fn vni_mark_reposition_after_horn() {
    // ua without q → u is main vowel, mark on u
    // After adding horn to 'u' → ư still main vowel, mark stays on ư
    run_vni(&[
        ("ua27", "ừa"), // mark on u, then horn → ừa
        ("ua2", "ùa"),  // ua without q: u is main vowel, mark on u
        ("ua7", "ưa"),  // Just horn, no mark
    ]);
}

#[test]
fn vni_mark_reposition_oa_pattern() {
    // VNI behavior: '6' applies to last a/e/o found in vowels without tone
    // In 'oa' buffer = [o, a], '6' matches 'a' first (reverse order)
    // So 'oa26' → 'oầ' (â with huyền)
    run_vni(&[
        ("oa26", "oầ"), // 6 applies to a → â, mark stays on â
        ("o6a2", "ồa"), // 6 applies to o first → ô, then a with mark → reposition to ô
        ("oa2", "oà"),  // Just mark, no circumflex
    ]);
}

#[test]
fn telex_mark_reposition_after_horn() {
    // Telex behavior: 'w' applies to last a/o/u found
    // In 'uaf' buffer = [u, a], 'w' matches 'a' first (reverse order)
    // So 'uafw' → 'uằ' (ă with huyền)
    run_telex(&[
        ("uafw", "uằ"), // w applies to a → ă, mark stays
        ("uwaf", "ừa"), // w applies to u first → ư, then mark on ư
        ("oafw", "oằ"), // w applies to a → ă
    ]);
}

#[test]
fn vni_ua_vs_qua_patterns() {
    // Compare patterns: ua (mua) vs qua
    run_vni(&[
        // Without q: ua → u is main vowel, mark on u
        ("ua1", "úa"), // mark on u (main vowel)
        ("ua2", "ùa"), // mark on u (main vowel)
        // With q: qua → u is medial, mark on a
        ("qua1", "quá"), // mark on a (medial pair)
        ("qua2", "quà"), // mark on a (medial pair)
        // With horn on u: ưa has diacritic on first vowel
        ("u7a1", "ứa"), // ư first, then a, mark on ư
        ("u7a2", "ừa"), // ư first, then a, mark on ư
        // Delayed horn after mark
        ("ua17", "ứa"), // úa → ứa (mark stays on ư)
        ("ua27", "ừa"), // ùa → ừa (mark stays on ư)
    ]);
}

#[test]
fn vni_uo_compound_with_marks() {
    // ươ compound vowel patterns - mark on ơ (2nd vowel with diacritic)
    run_vni(&[
        // uo + horn on both → ươ, then mark on ơ
        ("uo71", "ướ"),   // ươ + sắc → ướ
        ("uo72", "ườ"),   // ươ + huyền → ườ
        ("uo73", "ưở"),   // ươ + hỏi → ưở
        ("uo74", "ưỡ"),   // ươ + ngã → ưỡ
        ("uo75", "ượ"),   // ươ + nặng → ượ
        // Alternate order: mark first, then horn
        ("uo17", "ướ"),   // uó + horn → ướ (mark repositions)
        ("uo27", "ườ"),   // uò + horn → ườ
        // Horn on u first, then o, then mark
        ("u7o71", "ướ"),  // ư + ơ + sắc → ướ
        ("u7o72", "ườ"),  // ư + ơ + huyền → ườ
    ]);
}

// ============================================================
// REAL WORDS: Từ thực tế với VNI
// ============================================================

#[test]
fn vni_real_words_with_uo() {
    // Common words with ươ compound
    run_vni(&[
        ("nu7o7c1", "nước"),   // nước (water)
        ("bu7o7m1", "bướm"),   // bướm (butterfly)
        ("su7o7ng1", "sướng"), // sướng (happy)
        ("lu7o7ng2", "lương"), // lương (salary)
        ("d9u7o7ng2", "đường"), // đường (road/sugar)
        ("tru7o7ng2", "trường"), // trường (school)
        ("thu7o7ng2", "thường"), // thường (usually)
        ("cu7o7ng2", "cường"),   // cường (strong)
        ("hu7o7ng1", "hướng"),   // hướng (direction)
        ("vu7o7n2", "vườn"),     // vườn (garden)
    ]);
}

#[test]
fn vni_real_words_with_ua() {
    // Words with ua (mua type) vs qua
    run_vni(&[
        // mua type: u is main vowel
        ("mua2", "mùa"),   // mùa (season)
        ("chua1", "chúa"), // chúa (lord)
        ("sua4", "sữa"),   // sữa (milk)
        ("rua2", "rùa"),   // rùa (turtle)
        ("lua1", "lúa"),   // lúa (rice plant)
        ("thua1", "thúa"), // thúa (not common, but valid)
        // qua type: u is medial
        ("qua1", "quá"),   // quá (too much)
        ("qua3", "quả"),   // quả (fruit)
        ("que1", "qué"),   // qué (not common)
    ]);
}

#[test]
fn vni_real_words_with_ie() {
    // Words with iê compound
    run_vni(&[
        ("vie65t", "việt"),     // Việt
        ("tie61ng", "tiếng"),   // tiếng (sound/language)
        ("bie63n", "biển"),     // biển (sea)
        ("mie61ng", "miếng"),   // miếng (piece)
        ("chie62n", "chiền"),   // chiền (not common)
        ("die64n", "diễn"),     // diễn (perform)
        ("kie63m", "kiểm"),     // kiểm (check)
        ("tie62n", "tiền"),     // tiền (money)
        ("hie63u", "hiểu"),     // hiểu (understand)
    ]);
}

#[test]
fn vni_real_words_mixed() {
    // Mixed common words
    run_vni(&[
        ("co1", "có"),         // có (have)
        ("kho6ng", "không"),   // không (no/not)
        ("la2", "là"),         // là (is)
        ("d9i", "đi"),         // đi (go)
        ("ve62", "về"),        // về (return)
        ("a6n", "ăn"),         // ăn (eat)
        ("u6ng1", "ống"),      // ống (tube) - circumflex then mark
        ("ba2n", "bàn"),       // bàn (table)
        ("nha2", "nhà"),       // nhà (house)
        ("ho65c", "học"),      // học (study)
    ]);
}

// ============================================================
// Edge case: Rapid typing patterns
// User types faster than normal, keys arrive in quick succession
// ============================================================

#[test]
fn telex_rapid_compound_vowels() {
    // Common words typed rapidly
    run_telex(&[
        // Full ươ compound with various marks
        ("truwowngf", "trường"),
        ("dduwowcj", "được"),
        ("suwowngs", "sướng"),
        ("buwowms", "bướm"),
        // iê compound
        ("vieetj", "việt"),
        ("tieengs", "tiếng"),
        ("bieenr", "biển"),
        // uô compound
        ("muoons", "muốn"),
        ("cuoocj", "cuộc"),
        ("thuoocj", "thuộc"),
    ]);
}

// ============================================================
// Edge case: Mixed order typing
// User types marks/tones at different positions
// ============================================================

#[test]
fn telex_delayed_all_patterns() {
    // Delayed mode: tone key after consonants
    run_telex(&[
        // w after whole syllable
        ("tungw", "tưng"),
        ("tongw", "tơng"),
        ("tangw", "tăng"),
        // Multiple w for ươ
        ("tuoww", "tươ"),
        ("nguoiw", "ngưoi"),  // first w on u
        ("nguoiww", "ngươi"), // second w on o
    ]);
}

#[test]
fn vni_delayed_all_patterns() {
    run_vni(&[
        // Delayed modifier
        ("tung7", "tưng"),
        ("tong7", "tơng"),
        ("tang8", "tăng"),
        // Delayed đ
        ("dung9", "đung"),
        ("Dung9", "Đung"),
    ]);
}

// ============================================================
// REAL WORDS: Từ thực tế với Telex
// ============================================================

#[test]
fn telex_real_words_with_uo() {
    // Common words with ươ compound
    run_telex(&[
        ("nuwowcs", "nước"),     // nước (water)
        ("buwowms", "bướm"),     // bướm (butterfly)
        ("suwowngs", "sướng"),   // sướng (happy)
        ("luwowngf", "lương"),   // lương (salary)
        ("dduwowngf", "đường"),  // đường (road/sugar)
        ("truwowngf", "trường"), // trường (school)
        ("thuwowngf", "thường"), // thường (usually)
        ("cuwowngf", "cường"),   // cường (strong)
        ("huwowngs", "hướng"),   // hướng (direction)
        ("vuwownf", "vườn"),     // vườn (garden)
    ]);
}

#[test]
fn telex_real_words_with_ua() {
    // Words with ua (mua type) vs qua
    run_telex(&[
        // mua type: u is main vowel, mark on u
        ("muaf", "mùa"),   // mùa (season)
        ("chuas", "chúa"), // chúa (lord)
        ("suax", "sữa"),   // sữa (milk)
        ("ruaf", "rùa"),   // rùa (turtle)
        ("luas", "lúa"),   // lúa (rice plant)
        // qua type: u is medial, mark on a
        ("quas", "quá"),   // quá (too much)
        ("quar", "quả"),   // quả (fruit)
        ("quaf", "quà"),   // quà (gift)
    ]);
}

#[test]
fn telex_real_words_with_ie() {
    // Words with iê compound
    run_telex(&[
        ("vieetj", "việt"),     // Việt
        ("tieengs", "tiếng"),   // tiếng (sound/language)
        ("bieenr", "biển"),     // biển (sea)
        ("mieengs", "miếng"),   // miếng (piece)
        ("dieenx", "diễn"),     // diễn (perform)
        ("kieemr", "kiểm"),     // kiểm (check)
        ("tieenf", "tiền"),     // tiền (money)
        ("hieeur", "hiểu"),     // hiểu (understand)
    ]);
}

#[test]
fn telex_real_words_mixed() {
    // Mixed common words
    run_telex(&[
        ("cos", "có"),         // có (have)
        ("khoong", "không"),   // không (no/not)
        ("laf", "là"),         // là (is)
        ("ddi", "đi"),         // đi (go)
        ("veef", "về"),        // về (return)
        ("awn", "ăn"),         // ăn (eat)
        ("oongs", "ống"),      // ống (tube)
        ("banf", "bàn"),       // bàn (table)
        ("nhaf", "nhà"),       // nhà (house)
        ("hocj", "học"),       // học (study)
    ]);
}

#[test]
fn telex_uo_compound_with_marks() {
    // ươ compound vowel patterns - mark on ơ (2nd vowel with diacritic)
    run_telex(&[
        // Full ươ with all marks
        ("uwows", "ướ"),   // ươ + sắc → ướ
        ("uwowf", "ườ"),   // ươ + huyền → ườ
        ("uwowr", "ưở"),   // ươ + hỏi → ưở
        ("uwowx", "ưỡ"),   // ươ + ngã → ưỡ
        ("uwowj", "ượ"),   // ươ + nặng → ượ
        // Alternate typing patterns
        ("uowws", "ướ"),   // uo + w (on o) + w (on u) + s
        ("uowwf", "ườ"),   // uo + ww + huyền
    ]);
}

// ============================================================
// Edge case: Backspace and retype
// User corrects mistakes mid-word
// ============================================================

#[test]
fn telex_correction_patterns() {
    // Common correction scenarios
    run_telex(&[
        // Type wrong mark, then correct (mark replacement)
        ("asf", "à"), // á then f replaces sắc with huyền → à
        ("afs", "á"), // à then s replaces huyền with sắc → á
        // Simple letter replacement mid-word
        ("ab<c", "ac"), // a + b + backspace + c = ac
        // Backspace mid-word then apply mark
        ("toi<as", "toá"), // to + i + backspace + á = toá
    ]);
}

// ============================================================
// Edge case: All caps typing
// User types in ALL CAPS mode
// ============================================================

#[test]
fn telex_all_caps_words() {
    run_telex(&[
        ("VIEETJ", "VIỆT"),
        ("DDUWOWCJ", "ĐƯỢC"),
        ("TRUWOWNGF", "TRƯỜNG"),
        ("NGUWOWIF", "NGƯỜI"),
        ("DDUWOWNGF", "ĐƯỜNG"),
    ]);
}

#[test]
fn vni_all_caps_words() {
    run_vni(&[
        ("VIE65T", "VIỆT"),
        ("D9U7O7C5", "ĐƯỢC"),
        ("TRU7O7NG2", "TRƯỜNG"),
    ]);
}

// ============================================================
// Edge case: Words ending with mark/tone keys
// Keys that are both letters and modifiers
// ============================================================

#[test]
fn telex_letter_vs_modifier() {
    // 's' as letter vs 's' as sắc mark
    run_telex(&[
        ("sa", "sa"),    // s is consonant
        ("as", "á"),     // s is sắc mark
        ("sas", "sá"),   // first s consonant, second s mark
        ("sass", "sas"), // revert: sá + s = sas
    ]);

    // 'f' as letter vs 'f' as huyền mark
    run_telex(&[
        ("fa", "fa"), // f is consonant (borrowed words)
        ("af", "à"),  // f is huyền mark
    ]);
}

// ============================================================
// Edge case: Buffer boundary
// Long words that might overflow buffer
// ============================================================

#[test]
fn telex_long_words() {
    run_telex(&[
        // Long compound words
        ("nghieeng", "nghiêng"),
        ("khuyeens", "khuyến"),
        ("truwowngf", "trường"),
        ("nguoongf", "nguồng"), // unusual but valid
    ]);
}
