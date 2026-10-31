//! Built-in translations keep every placeholder of the English text, and right-to-left
//! languages are detected.

use ui::prelude::*;

/// `(field, text)` pairs, read from the `Debug` output so a new field is covered without
/// editing this test.
fn fields(strings: &UiStrings) -> Vec<(String, String)> {
    let debug = format!("{strings:?}");
    let body = debug
        .trim_start_matches("UiStrings {")
        .trim_end_matches('}')
        .trim();
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(colon) = rest.find(": \"") {
        let name = rest[..colon]
            .trim()
            .trim_start_matches(',')
            .trim()
            .to_string();
        let after = &rest[colon + 3..];
        // Find the closing quote that isn't escaped.
        let mut end = 0;
        let bytes = after.as_bytes();
        while end < bytes.len() {
            if bytes[end] == b'"' && (end == 0 || bytes[end - 1] != b'\\') {
                break;
            }
            end += 1;
        }
        out.push((name, after[..end].to_string()));
        rest = &after[(end + 1).min(after.len())..];
    }
    out
}

fn placeholders(text: &str) -> Vec<String> {
    let mut found: Vec<String> = text
        .split('{')
        .skip(1)
        .filter_map(|part| part.split_once('}').map(|(name, _)| name.to_string()))
        // `Debug` writes some characters as `\u{64d}`; those aren't placeholders.
        .filter(|name| !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
        .collect();
    found.sort();
    found
}

#[test]
fn every_locale_keeps_every_placeholder() {
    let english = fields(&UiStrings::default());
    assert!(english.len() > 60, "parsed only {} fields", english.len());
    for code in LOCALES {
        let strings = UiStrings::locale(code).expect("listed locale exists");
        let translated = fields(&strings);
        assert_eq!(translated.len(), english.len(), "{code}: field count");
        for ((name, en), (_, text)) in english.iter().zip(&translated) {
            assert!(!text.trim().is_empty(), "{code}.{name} is empty");
            assert_eq!(
                placeholders(en),
                placeholders(text),
                "{code}.{name}: {text:?} must keep the placeholders of {en:?}"
            );
        }
    }
}

#[test]
fn locale_matches_the_language_part_of_a_tag() {
    assert_eq!(UiStrings::locale("fr-CA"), Some(UiStrings::french()));
    assert_eq!(UiStrings::locale("pt_BR"), Some(UiStrings::portuguese()));
    assert_eq!(UiStrings::locale("EN"), Some(UiStrings::default()));
    assert_eq!(UiStrings::locale("xx"), None);
}

#[test]
fn right_to_left_languages() {
    for lang in ["ar", "he-IL", "fa", "ur"] {
        assert_eq!(text_direction(lang), "rtl", "{lang}");
    }
    for lang in ["en", "fr", "ja", "zh-Hans", ""] {
        assert_eq!(text_direction(lang), "ltr", "{lang}");
    }
}
