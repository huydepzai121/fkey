//! Behavior Tests - Real-world typing scenarios
//!
//! Tests for common user behaviors:
//! - Typing mistakes and corrections (backspace)
//! - Retyping after errors
//! - Mixed typing patterns
//! - Edge cases encountered in daily use

use gonhanh_core::data::keys;
use gonhanh_core::engine::{Action, Engine};

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => keys::A,
        'b' => keys::B,
        'c' => keys::C,
        'd' => keys::D,
        'e' => keys::E,
        'f' => keys::F,
        'g' => keys::G,
        'h' => keys::H,
        'i' => keys::I,
        'j' => keys::J,
        'k' => keys::K,
        'l' => keys::L,
        'm' => keys::M,
        'n' => keys::N,
        'o' => keys::O,
        'p' => keys::P,
        'q' => keys::Q,
        'r' => keys::R,
        's' => keys::S,
        't' => keys::T,
        'u' => keys::U,
        'v' => keys::V,
        'w' => keys::W,
        'x' => keys::X,
        'y' => keys::Y,
        'z' => keys::Z,
        '0' => keys::N0,
        '1' => keys::N1,
        '2' => keys::N2,
        '3' => keys::N3,
        '4' => keys::N4,
        '5' => keys::N5,
        '6' => keys::N6,
        '7' => keys::N7,
        '8' => keys::N8,
        '9' => keys::N9,
        '<' => keys::DELETE, // Use < to simulate backspace
        _ => 255,
    }
}

/// Simulate typing with support for backspace (<)
fn type_sequence(e: &mut Engine, input: &str) -> String {
    let mut screen = String::new();
    for c in input.chars() {
        let key = char_to_key(c);
        let is_caps = c.is_uppercase();

        if key == keys::DELETE {
            // Backspace: remove from screen and notify engine
            screen.pop();
            e.on_key(key, false, false);
            continue;
        }

        let r = e.on_key(key, is_caps, false);
        if r.action == Action::Send as u8 {
            for _ in 0..r.backspace {
                screen.pop();
            }
            for i in 0..r.count as usize {
                if let Some(ch) = char::from_u32(r.chars[i]) {
                    screen.push(ch);
                }
            }
        } else if keys::is_letter(key) {
            screen.push(if is_caps {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            });
        }
    }
    screen
}

fn test_telex(input: &str, expected: &str) {
    let mut e = Engine::new();
    let result = type_sequence(&mut e, input);
    assert_eq!(
        result, expected,
        "\n[Telex] '{}' → '{}' (expected '{}')",
        input, result, expected
    );
}

fn test_vni(input: &str, expected: &str) {
    let mut e = Engine::new();
    e.set_method(1);
    let result = type_sequence(&mut e, input);
    assert_eq!(
        result, expected,
        "\n[VNI] '{}' → '{}' (expected '{}')",
        input, result, expected
    );
}

// ============================================================
// BACKSPACE: Xóa ký tự và gõ lại
// ============================================================

#[test]
fn telex_backspace_and_retype() {
    // Gõ sai "vieet" -> xóa -> gõ lại "viet"
    test_telex("vieet<s", "việ"); // viê + backspace + s = việ

    // Gõ "xin chaof" -> xóa -> gõ lại
    test_telex("chaof<o", "chào"); // chaò + backspace + o = chào
}

#[test]
fn telex_backspace_mid_word() {
    // Gõ sai giữa từ, xóa và sửa
    test_telex("toi<as", "toá"); // to + i + backspace + á = toá
}

#[test]
fn vni_backspace_and_retype() {
    test_vni("a1<2", "à"); // á + backspace + 2 = à
    test_vni("o6<7", "ơ"); // ô + backspace + 7 = ơ
}

// ============================================================
// TYPO: Gõ nhầm thứ tự phím
// ============================================================

#[test]
fn telex_wrong_order_mark_before_vowel() {
    // Người dùng có thể gõ dấu trước nguyên âm
    // Engine chỉ xử lý khi có nguyên âm trong buffer
    test_telex("sa", "sa"); // s trước, không có vowel -> pass through
    test_telex("as", "á");  // a trước, s sau -> á
}

#[test]
fn telex_double_mark() {
    // Gõ dấu 2 lần -> revert
    test_telex("ass", "as");
    test_telex("aff", "af");
    test_telex("arr", "ar");
}

#[test]
fn telex_double_tone() {
    // Gõ tone 2 lần -> revert
    test_telex("aaa", "aa");
    test_telex("ooo", "oo");
    test_telex("aww", "aw");
}

// ============================================================
// MIXED: Kết hợp nhiều thao tác
// ============================================================

#[test]
fn telex_change_mark_mid_word() {
    // Đổi dấu giữa chừng: gõ sắc rồi đổi sang huyền
    test_telex("asf", "às"); // á + f -> à + s (revert á, add huyền, append s)
}

#[test]
fn telex_tone_then_mark() {
    // Thêm tone (^) rồi thêm mark (sắc)
    test_telex("aas", "ấ");
    test_telex("ees", "ế");
    test_telex("oos", "ố");
}

#[test]
fn telex_mark_then_tone() {
    // Thêm mark trước, tone sau
    test_telex("asa", "ấ"); // á + a = ấ
    test_telex("oso", "ố"); // ó + o = ố (nếu engine hỗ trợ)
}

// ============================================================
// COMMON TYPOS: Lỗi thường gặp khi gõ nhanh
// ============================================================

#[test]
fn telex_common_words_with_typos() {
    // "việt" - gõ đúng
    test_telex("vieets", "việt");

    // "được" - gõ đúng
    test_telex("dduowwcj", "được");
}

#[test]
fn vni_common_words() {
    // "việt" với VNI
    test_vni("vie65t", "việt");

    // "được" với VNI
    test_vni("d9u7o7c5", "được");
}

// ============================================================
// EDGE CASES: Các trường hợp biên
// ============================================================

#[test]
fn telex_only_consonants() {
    // Chỉ gõ phụ âm, không có nguyên âm
    test_telex("bcd", "bcd");
    test_telex("xyz", "xyz");
}

#[test]
fn telex_mark_without_vowel() {
    // Gõ dấu khi không có nguyên âm trong buffer
    test_telex("bs", "bs"); // không có vowel, s là letter thường
    test_telex("ts", "ts");
}

#[test]
fn telex_multiple_backspace() {
    // Xóa nhiều ký tự liên tiếp
    test_telex("abcd<<<", "a");
    test_telex("vieets<<<ng", "ving");
}

#[test]
fn telex_empty_after_backspace() {
    // Xóa hết rồi gõ lại
    test_telex("a<b", "b");
    test_telex("ab<<cd", "cd");
}

// ============================================================
// CONTINUOUS TYPING: Gõ liên tục nhiều từ
// ============================================================

#[test]
fn telex_word_boundary() {
    // Sau khi gõ xong một từ, buffer nên được clear
    // khi gặp ký tự không phải letter (space, punctuation)
    // Hiện tại test với single word
    test_telex("xin", "xin");
    test_telex("chaof", "chào");
}

// ============================================================
// CAPITALIZATION: Chữ hoa
// ============================================================

#[test]
fn telex_caps_mid_word() {
    // Caps ở giữa từ (ít gặp nhưng có thể xảy ra)
    test_telex("viEets", "viỆt");
}

#[test]
fn telex_all_caps() {
    test_telex("VIEETS", "VIỆT");
    test_telex("DDUWOWNGF", "ĐƯỜNG");
}

#[test]
fn vni_all_caps() {
    test_vni("VIE65T", "VIỆT");
    test_vni("D9U7O7NG2", "ĐƯỜNG");
}

// ============================================================
// RAPID TYPING: Gõ nhanh, có thể nhầm
// ============================================================

#[test]
fn telex_rapid_typing_patterns() {
    // Patterns thường gặp khi gõ nhanh
    test_telex("ngoafif", "ngoàif"); // gõ f 2 lần
    test_telex("nguwowif", "người");
}

#[test]
fn vni_rapid_typing() {
    test_vni("ngu7o72i2", "người");
    test_vni("to6i1", "tối");
}
