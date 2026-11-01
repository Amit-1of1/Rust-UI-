//! The foundations (docs/FOUNDATIONS.md) hold: component stylesheets take layers, motion,
//! shadows and type sizes from the theme tokens instead of literal values, and every token
//! they name exists in theme.css.

use std::collections::BTreeSet;
use std::path::Path;

const THEME: &str = include_str!("../assets/theme.css");

/// Every component stylesheet, as (path, text without comments).
fn sheets() -> Vec<(String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/components");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path().join("style.css");
        if let Ok(css) = std::fs::read_to_string(&path) {
            out.push((path.display().to_string(), strip_comments(&css)));
        }
    }
    assert!(out.len() > 60, "found only {} stylesheets", out.len());
    out
}

fn strip_comments(css: &str) -> String {
    let mut out = String::new();
    let mut rest = css;
    while let Some(i) = rest.find("/*") {
        out.push_str(&rest[..i]);
        rest = rest[i..].find("*/").map_or("", |j| &rest[i + j + 2..]);
    }
    out.push_str(rest);
    out
}

/// `(property, value)` for every declaration of `prop` (and its longhands with `prefix`).
fn declarations<'a>(css: &'a str, prefix: &str) -> Vec<(&'a str, &'a str)> {
    css.split(['{', '}', ';'])
        .filter_map(|d| d.split_once(':'))
        .map(|(p, v)| (p.trim(), v.trim()))
        .filter(|(p, _)| p.starts_with(prefix))
        .collect()
}

#[test]
fn layers_come_from_the_z_index_scale() {
    for (path, css) in sheets() {
        for (_, v) in declarations(&css, "z-index") {
            assert!(
                v.contains("var(--z-index-") || v.contains("var(--toast-"),
                "{path}: z-index {v} is not on the scale (--z-index-raised ... --z-index-top)"
            );
        }
    }
}

/// A duration written as a number between 50 ms and 450 ms: the range the duration tokens
/// cover. Shorter ones are "instant", longer ones are loops (spinners, marquees).
fn literal_duration(value: &str) -> Option<String> {
    for word in value.split(|c: char| c.is_whitespace() || c == ',' || c == '(' || c == ')') {
        let ms = if let Some(n) = word.strip_suffix("ms") {
            n.parse::<f64>().ok()
        } else if let Some(n) = word.strip_suffix('s') {
            n.parse::<f64>().ok().map(|s| s * 1000.0)
        } else {
            None
        };
        if ms.is_some_and(|ms| (50.0..=450.0).contains(&ms)) {
            return Some(word.to_string());
        }
    }
    None
}

#[test]
fn transitions_use_the_duration_tokens() {
    for (path, css) in sheets() {
        for prefix in ["transition", "animation"] {
            for (p, v) in declarations(&css, prefix) {
                // Loops run on their own clock; delays stagger, they are not a speed.
                if v.contains("infinite") || p.ends_with("-delay") {
                    continue;
                }
                if let Some(d) = literal_duration(v) {
                    panic!("{path}: {p}: {v} has the literal duration {d}; use --duration-rapid/fast/medium/slow");
                }
            }
        }
    }
}

#[test]
fn drop_shadows_come_from_the_elevation_tokens() {
    for (path, css) in sheets() {
        // The colour picker's thumbs draw a black ring on any colour, on purpose.
        if path.contains("color_picker") {
            continue;
        }
        for (_, v) in declarations(&css, "box-shadow") {
            assert!(
                !v.contains("rgb(0 0 0") && !v.contains("#000"),
                "{path}: box-shadow {v} has a literal shadow colour; use --shadow-raised, --shadow-overlay or --color-shadow-*"
            );
        }
    }
}

#[test]
fn type_sizes_come_from_the_type_scale() {
    for (path, css) in sheets() {
        for (_, v) in declarations(&css, "font-size") {
            let literal = v.ends_with("rem") || v.ends_with("px");
            // Chart tick labels are drawn in SVG at a fixed small size.
            if literal && !(path.contains("chart") && v == "10px") {
                panic!("{path}: font-size {v} is not on the type scale; use --font-size-*");
            }
        }
    }
}

#[test]
fn every_foundation_token_a_component_names_exists() {
    let families = [
        "--unit-x",
        "--font-size-",
        "--line-height-",
        "--font-weight-",
        "--z-index-",
        "--duration-",
        "--easing-",
        "--shadow-",
        "--radius-",
        "--color-",
        "--layout-",
    ];
    let mut missing = BTreeSet::new();
    for (path, css) in sheets() {
        let mut rest = css.as_str();
        while let Some(i) = rest.find("var(--") {
            rest = &rest[i + 4..];
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
                .collect();
            if families.iter().any(|f| name.starts_with(f)) && !THEME.contains(&format!("{name}:"))
            {
                missing.insert(format!("{path}: {name}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "tokens used but not defined in theme.css: {missing:#?}"
    );
}

#[test]
fn the_type_scale_sits_on_the_4px_grid() {
    for line in THEME.lines() {
        let line = line.trim();
        if let Some(v) = line
            .strip_prefix("--line-height-")
            .and_then(|l| l.split_once(':'))
            .map(|(_, v)| v.trim().trim_end_matches(';'))
        {
            let rem: f64 = v.trim_end_matches("rem").parse().unwrap();
            let px = rem * 16.0;
            assert!(px % 4.0 == 0.0, "{line}: {px}px is off the 4 px grid");
        }
        if let Some(v) = line
            .strip_prefix("--unit-x")
            .and_then(|l| l.split_once(':'))
            .map(|(_, v)| v.trim().trim_end_matches(';'))
        {
            let rem: f64 = v.trim_end_matches("rem").parse().unwrap();
            assert!((rem * 16.0) % 4.0 == 0.0, "{line} is off the 4 px grid");
        }
    }
}

#[test]
fn reduced_motion_zeroes_the_duration_tokens() {
    let at = THEME
        .find("@media (prefers-reduced-motion: reduce) {\n  :root {\n    --duration-rapid")
        .expect("a reduced-motion block for the duration tokens");
    let block = &THEME[at..at + 300];
    for d in ["rapid", "fast", "medium", "slow"] {
        assert!(
            block.contains(&format!("--duration-{d}: 0.01ms")),
            "{block}"
        );
    }
}
