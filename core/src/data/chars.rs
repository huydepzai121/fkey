//! Vietnamese Unicode Character System
//!
//! Provides character conversion between base vowels + modifiers + marks
//! and composed Vietnamese Unicode characters.
//!
//! ## Design Principles
//! - Single lookup table for all vowel combinations (12 bases × 6 marks = 72)
//! - Uses Rust's built-in `to_uppercase()` for case conversion
//! - No hardcoded case-by-case matching
//!
//! ## Character Components
//! - Base vowel: a, ă, â, e, ê, i, o, ô, ơ, u, ư, y
//! - Mark (dấu thanh): none, sắc, huyền, hỏi, ngã, nặng
//! - Case: lowercase, uppercase

use super::keys;

/// Tone modifiers (dấu phụ) - changes base vowel form
pub mod tone {
    pub const NONE: u8 = 0;
    pub const CIRCUMFLEX: u8 = 1; // ^ (mũ): a→â, e→ê, o→ô
    pub const HORN: u8 = 2; // ơ, ư or breve ă
}

/// Marks (dấu thanh) - Vietnamese tone marks
pub mod mark {
    pub const NONE: u8 = 0;
    pub const SAC: u8 = 1; // sắc (á)
    pub const HUYEN: u8 = 2; // huyền (à)
    pub const HOI: u8 = 3; // hỏi (ả)
    pub const NGA: u8 = 4; // ngã (ã)
    pub const NANG: u8 = 5; // nặng (ạ)
}

/// Vietnamese vowel lookup table
/// Each entry: (base_char, [sắc, huyền, hỏi, ngã, nặng])
const VOWEL_TABLE: [(char, [char; 5]); 12] = [
    ('a', ['á', 'à', 'ả', 'ã', 'ạ']),
    ('ă', ['ắ', 'ằ', 'ẳ', 'ẵ', 'ặ']),
    ('â', ['ấ', 'ầ', 'ẩ', 'ẫ', 'ậ']),
    ('e', ['é', 'è', 'ẻ', 'ẽ', 'ẹ']),
    ('ê', ['ế', 'ề', 'ể', 'ễ', 'ệ']),
    ('i', ['í', 'ì', 'ỉ', 'ĩ', 'ị']),
    ('o', ['ó', 'ò', 'ỏ', 'õ', 'ọ']),
    ('ô', ['ố', 'ồ', 'ổ', 'ỗ', 'ộ']),
    ('ơ', ['ớ', 'ờ', 'ở', 'ỡ', 'ợ']),
    ('u', ['ú', 'ù', 'ủ', 'ũ', 'ụ']),
    ('ư', ['ứ', 'ừ', 'ử', 'ữ', 'ự']),
    ('y', ['ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ']),
];

/// Get base character from key + tone modifier
///
/// # Arguments
/// * `key` - Virtual keycode (a, e, i, o, u, y)
/// * `tone` - Tone modifier: 0=none, 1=circumflex(^), 2=horn/breve
///
/// # Returns
/// Base vowel character: a, ă, â, e, ê, i, o, ô, ơ, u, ư, y
fn get_base_char(key: u16, t: u8) -> Option<char> {
    match key {
        keys::A => Some(match t {
            tone::CIRCUMFLEX => 'â',
            tone::HORN => 'ă', // breve for 'a'
            _ => 'a',
        }),
        keys::E => Some(match t {
            tone::CIRCUMFLEX => 'ê',
            _ => 'e',
        }),
        keys::I => Some('i'),
        keys::O => Some(match t {
            tone::CIRCUMFLEX => 'ô',
            tone::HORN => 'ơ',
            _ => 'o',
        }),
        keys::U => Some(match t {
            tone::HORN => 'ư',
            _ => 'u',
        }),
        keys::Y => Some('y'),
        _ => None,
    }
}

/// Apply mark to base vowel character
///
/// Uses lookup table to find the marked variant.
///
/// # Arguments
/// * `base` - Base vowel character (a, ă, â, e, ê, i, o, ô, ơ, u, ư, y)
/// * `mark` - Mark: 0=none, 1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng
fn apply_mark(base: char, m: u8) -> char {
    if m == mark::NONE || m > mark::NANG {
        return base;
    }

    VOWEL_TABLE
        .iter()
        .find(|(b, _)| *b == base)
        .map(|(_, marks)| marks[(m - 1) as usize])
        .unwrap_or(base)
}

/// Convert to uppercase using Rust's Unicode-aware method
///
/// This handles all Vietnamese characters correctly without
/// explicit character mapping.
fn to_upper(ch: char) -> char {
    ch.to_uppercase().next().unwrap_or(ch)
}

/// Convert key + modifiers to Vietnamese character
///
/// # Arguments
/// * `key` - Virtual keycode
/// * `caps` - Uppercase flag
/// * `tone` - Tone modifier: 0=none, 1=circumflex(^), 2=horn/breve
/// * `mark` - Mark: 0=none, 1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng
pub fn to_char(key: u16, caps: bool, tone: u8, mark: u8) -> Option<char> {
    // Handle D specially (not a vowel but needs conversion)
    if key == keys::D {
        return Some(if caps { 'D' } else { 'd' });
    }

    let base = get_base_char(key, tone)?;
    let marked = apply_mark(base, mark);
    Some(if caps { to_upper(marked) } else { marked })
}

/// Get đ/Đ character
pub fn get_d(caps: bool) -> char {
    if caps {
        'Đ'
    } else {
        'đ'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_vowels() {
        // Basic vowels without modifiers
        assert_eq!(to_char(keys::A, false, 0, 0), Some('a'));
        assert_eq!(to_char(keys::E, false, 0, 0), Some('e'));
        assert_eq!(to_char(keys::I, false, 0, 0), Some('i'));
        assert_eq!(to_char(keys::O, false, 0, 0), Some('o'));
        assert_eq!(to_char(keys::U, false, 0, 0), Some('u'));
        assert_eq!(to_char(keys::Y, false, 0, 0), Some('y'));
    }

    #[test]
    fn test_tone_modifiers() {
        // Circumflex (^)
        assert_eq!(to_char(keys::A, false, 1, 0), Some('â'));
        assert_eq!(to_char(keys::E, false, 1, 0), Some('ê'));
        assert_eq!(to_char(keys::O, false, 1, 0), Some('ô'));

        // Horn/Breve
        assert_eq!(to_char(keys::A, false, 2, 0), Some('ă'));
        assert_eq!(to_char(keys::O, false, 2, 0), Some('ơ'));
        assert_eq!(to_char(keys::U, false, 2, 0), Some('ư'));
    }

    #[test]
    fn test_marks() {
        // All 5 marks on 'a'
        assert_eq!(to_char(keys::A, false, 0, 1), Some('á')); // sắc
        assert_eq!(to_char(keys::A, false, 0, 2), Some('à')); // huyền
        assert_eq!(to_char(keys::A, false, 0, 3), Some('ả')); // hỏi
        assert_eq!(to_char(keys::A, false, 0, 4), Some('ã')); // ngã
        assert_eq!(to_char(keys::A, false, 0, 5), Some('ạ')); // nặng
    }

    #[test]
    fn test_combined_tone_and_mark() {
        // â + sắc = ấ
        assert_eq!(to_char(keys::A, false, 1, 1), Some('ấ'));
        // ơ + huyền = ờ
        assert_eq!(to_char(keys::O, false, 2, 2), Some('ờ'));
        // ư + nặng = ự
        assert_eq!(to_char(keys::U, false, 2, 5), Some('ự'));
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(to_char(keys::A, true, 0, 0), Some('A'));
        assert_eq!(to_char(keys::A, true, 0, 1), Some('Á'));
        assert_eq!(to_char(keys::A, true, 1, 1), Some('Ấ'));
        assert_eq!(to_char(keys::O, true, 2, 2), Some('Ờ'));
        assert_eq!(to_char(keys::U, true, 2, 5), Some('Ự'));
    }

    #[test]
    fn test_d() {
        assert_eq!(get_d(false), 'đ');
        assert_eq!(get_d(true), 'Đ');
    }
}
