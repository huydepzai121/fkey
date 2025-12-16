//! English Words Protection Test
//!
//! Tests that English words with INVALID Vietnamese structure are NOT transformed.
//!
//! NOTE: Words matching valid Vietnamese patterns (e.g., "as"→"á", "or"→"ỏ") WILL be
//! transformed - this is correct IME behavior. We can only protect words that are
//! structurally invalid in Vietnamese.

use gonhanh_core::engine::Engine;

/// Test that English words are NOT transformed
fn assert_no_transform(words: &[&str]) {
    let mut telex = Engine::new();
    telex.set_method(0); // Telex

    for word in words {
        telex.clear();
        let mut output = String::new();

        for ch in word.chars() {
            let key = char_to_key(ch);
            let result = telex.on_key(key, ch.is_uppercase(), false);

            if result.action == 1 {
                let bs = result.backspace as usize;
                for _ in 0..bs.min(output.len()) {
                    output.pop();
                }
                for i in 0..result.count as usize {
                    if let Some(c) = char::from_u32(result.chars[i]) {
                        output.push(c);
                    }
                }
            } else {
                output.push(ch);
            }
        }

        assert_eq!(
            output, *word,
            "English word '{}' was transformed to '{}'",
            word, output
        );
    }
}

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => 0, 's' => 1, 'd' => 2, 'f' => 3, 'h' => 4, 'g' => 5, 'z' => 6, 'x' => 7,
        'c' => 8, 'v' => 9, 'b' => 11, 'q' => 12, 'w' => 13, 'e' => 14, 'r' => 15,
        'y' => 16, 't' => 17, '1' => 18, '2' => 19, '3' => 20, '4' => 21, '6' => 22,
        '5' => 23, '9' => 25, '7' => 26, '8' => 28, '0' => 29, 'o' => 31, 'u' => 32,
        'i' => 34, 'p' => 35, 'l' => 37, 'j' => 38, 'k' => 40, 'n' => 45, 'm' => 46,
        _ => 255,
    }
}

// =============================================================================
// WORDS WITH INVALID VIETNAMESE STRUCTURE
// These SHOULD NOT be transformed because they violate Vietnamese phonology
// =============================================================================

/// Words with invalid initial consonant clusters
/// Vietnamese only allows: ch, gh, gi, kh, ng, ngh, nh, ph, qu, th, tr
const INVALID_INITIALS: &[&str] = &[
    // bl- cluster
    "black", "blue", "blank", "blast", "blend", "blind", "block", "blood", "blow", "blog",
    // br- cluster
    "brain", "branch", "brand", "break", "bring", "broad", "brown", "brush", "brief",
    // cl- cluster
    "class", "clean", "clear", "click", "client", "climb", "clone", "close", "cloud", "club",
    // cr- cluster
    "crash", "create", "credit", "cross", "crowd", "crypto", "crystal",
    // dr- cluster
    "draft", "dragon", "drain", "draw", "dream", "dress", "drink", "drive", "drop", "drug",
    // fl- cluster
    "flag", "flash", "flat", "flex", "flight", "float", "floor", "flow", "fluid", "flutter",
    // fr- cluster
    "frame", "free", "fresh", "friend", "from", "front", "frozen", "fruit",
    // gl- cluster
    "glass", "global", "glory", "glue", "gmail",
    // gr- cluster
    "grade", "grand", "grant", "graph", "grass", "great", "green", "grid", "group", "grow",
    // pl- cluster
    "place", "plan", "plant", "plate", "play", "please", "plot", "plug", "plus", "pocket",
    // pr- cluster
    "practice", "press", "price", "print", "private", "problem", "process", "product", "program", "project",
    // sc- cluster
    "scale", "scan", "scene", "school", "science", "scope", "score", "screen", "script", "scroll",
    // sk- cluster
    "sketch", "skill", "skip", "sky",
    // sl- cluster
    "slack", "sleep", "slide", "slim", "slot", "slow",
    // sm- cluster
    "small", "smart", "smile", "smooth", "smtp",
    // sn- cluster
    "snake", "snap", "snow",
    // sp- cluster
    "space", "spam", "span", "spark", "speak", "special", "speed", "spell", "spend", "split", "sport", "spot", "spread", "spring", "sql",
    // st- cluster
    "stack", "staff", "stage", "stand", "star", "start", "state", "static", "status", "stay", "step", "stick", "still", "stock", "stop", "store", "story", "strategy", "stream", "street", "stress", "strict", "string", "strip", "strong", "struct", "student", "study", "style", "submit",
    // sw- cluster
    "swap", "sweet", "swift", "swim", "swing", "switch", "symbol", "sync", "syntax", "system",
    // tw- cluster
    "tweet", "twice", "twin", "twist", "type",
    // wr- cluster
    "wrap", "write", "wrong",
];

/// Words with 'ou' vowel pattern (invalid in Vietnamese)
const INVALID_OU_PATTERN: &[&str] = &[
    // Common words
    "you", "your", "out", "our", "hour", "four", "pour", "tour", "soup", "soul",
    "loud", "cloud", "proud", "sound", "round", "found", "bound", "pound", "ground",
    "about", "count", "mount", "amount", "house", "mouse", "south", "mouth", "route",
    "could", "would", "should", "through", "enough", "though", "thought", "brought",
    "touch", "couch", "source", "course", "account", "without", "throughout",
    // Tech words
    "cloud", "route", "router", "mount", "output", "layout", "logout", "timeout",
    "checkout", "throughout", "workout", "lookout", "roundup", "soundex",
];

/// Words with 'yo' vowel pattern (invalid in Vietnamese)
const INVALID_YO_PATTERN: &[&str] = &[
    "you", "your", "york", "yoga", "young", "youth", "beyond", "layout", "anyone",
];

/// Words with consonant cluster after final (T+R, P+R, C+R patterns)
/// These are detected by is_foreign_word_pattern()
const INVALID_FINAL_CLUSTERS: &[&str] = &[
    // -tric, -trix, -try patterns
    "metric", "matrix", "electric", "entric", "entric", "entric",
    // -pec, -per patterns
    "spectrum", "expect", "aspect", "respect", "suspect", "inspect",
    // -scr patterns
    "describe", "escript", "escript",
    // Other patterns
    "control", "central", "abstract", "contract", "extract", "subtract",
    "compress", "express", "impress", "suppress", "progress",
    "construct", "destruct", "instruct", "obstruct",
];

/// Words with 'de' + 's' pattern (describe, design, etc.)
const INVALID_DE_S_PATTERN: &[&str] = &[
    "describe", "design", "desk", "desktop", "destroy", "desperate",
];

/// Tech/programming terms with invalid Vietnamese structure
const TECH_INVALID: &[&str] = &[
    // Languages & tools with invalid initials
    "string", "struct", "stream", "script", "scroll", "spring", "sprite",
    "flutter", "flask", "graphql", "grpc", "gradle", "grunt", "gulp",
    "blockchain", "bluetooth", "broadcast", "browser",
    "chrome", "chromium", "crypto", "crystal",
    "docker", "dropbox", "drupal", "django",
    "electron", "express",
    "flask", "flutter", "framework", "frontend",
    "github", "gitlab", "gradle", "graphql", "grpc",
    "playwright", "postgres", "prisma", "prometheus",
    "react", "redux", "redis",
    "scala", "sklearn", "slack", "smtp", "socket", "spark", "splunk", "spring", "sql", "sqlite", "ssh", "ssl", "stack", "stripe", "swagger", "swift", "symfony",
    "terraform", "travis", "trello", "typescript",
    "webpack", "websocket", "wordpress",
];

// =============================================================================
// TESTS
// =============================================================================

#[test]
fn protect_invalid_initials() {
    assert_no_transform(INVALID_INITIALS);
}

#[test]
fn protect_ou_pattern() {
    assert_no_transform(INVALID_OU_PATTERN);
}

#[test]
fn protect_yo_pattern() {
    assert_no_transform(INVALID_YO_PATTERN);
}

#[test]
fn protect_final_clusters() {
    assert_no_transform(INVALID_FINAL_CLUSTERS);
}

#[test]
fn protect_de_s_pattern() {
    assert_no_transform(INVALID_DE_S_PATTERN);
}

#[test]
fn protect_tech_terms() {
    assert_no_transform(TECH_INVALID);
}

/// Summary test - all structurally invalid words
#[test]
fn all_invalid_structure_protected() {
    let all_words: Vec<&str> = [
        INVALID_INITIALS,
        INVALID_OU_PATTERN,
        INVALID_YO_PATTERN,
        INVALID_FINAL_CLUSTERS,
        INVALID_DE_S_PATTERN,
        TECH_INVALID,
    ]
    .concat();

    // Deduplicate
    let mut unique: Vec<&str> = all_words.clone();
    unique.sort();
    unique.dedup();

    println!(
        "Testing {} unique English words with invalid Vietnamese structure",
        unique.len()
    );
    assert_no_transform(&unique);
}
