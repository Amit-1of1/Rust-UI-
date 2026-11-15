//! `fill`, which puts names, file names and message text into announcements: a value is
//! inserted as written, never searched for placeholders itself.

use proptest::prelude::*;
use ui::components::ui_strings::fill;

#[test]
fn values_are_not_filled_again() {
    assert_eq!(
        fill("{author} said {text}", &[("author", "{text}"), ("text", "hi")]),
        "{text} said hi"
    );
    assert_eq!(
        fill("{name} is larger than {size}", &[("name", "a{size}.png"), ("size", "1.0 MB")]),
        "a{size}.png is larger than 1.0 MB"
    );
    assert_eq!(fill("{missing} {", &[("x", "y")]), "{missing} {");
    assert_eq!(fill("{{x}}", &[("x", "1")]), "{1}");
}

/// The template's pieces, in order: plain text (no braces) or a placeholder name.
#[derive(Debug, Clone)]
enum Piece {
    Text(String),
    Slot(usize),
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 4_096, ..ProptestConfig::default() })]

    /// Any values, including ones that look like placeholders: the result is the template with
    /// each placeholder replaced by its value, exactly once.
    #[test]
    fn each_placeholder_becomes_its_value(
        pieces in prop::collection::vec(prop_oneof![
            "[a-z .]{0,4}".prop_map(Piece::Text),
            (0usize..3).prop_map(Piece::Slot),
        ], 0..8),
        values in prop::collection::vec("[a-z{}]{0,6}", 3),
    ) {
        let names = ["a", "bb", "count"];
        let mut template = String::new();
        let mut want = String::new();
        for p in &pieces {
            match p {
                Piece::Text(t) => { template.push_str(t); want.push_str(t); }
                Piece::Slot(i) => { template.push_str(&format!("{{{}}}", names[*i])); want.push_str(&values[*i]); }
            }
        }
        let vals: Vec<(&str, &str)> = names.iter().copied().zip(values.iter().map(String::as_str)).collect();
        prop_assert_eq!(fill(&template, &vals), want);
    }
}
