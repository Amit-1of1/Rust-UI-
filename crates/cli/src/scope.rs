//! Scoped theme CSS for apps that already have their own styles.
//!
//! `theme.css` defines its tokens on `:root` and has a few page-wide rules (reduced motion,
//! forced colours, `.sr-only`, `[data-theme]`). Dropped into an existing app those reach the
//! whole page, and the app's own `--radius` or `--primary-color` on `:root` can clash with
//! the kit's. [`scope_css`] rewrites every selector so the stylesheet only applies inside an
//! element marked [`SCOPE`] (what `ThemeScope` renders):
//!
//! | Selector | Scoped |
//! |---|---|
//! | `:root`, `html`, `html[data-theme="dark"]` | `[data-rui-scope]`, `[data-rui-scope][data-theme="dark"]` |
//! | `[data-accent="blue"]` (attributes only, first compound) | on the scope, inside it, or on an ancestor of it (`<html>`) |
//! | `:where(svg[data-icon])` | `:where([data-rui-scope] svg[data-icon])` (still zero specificity) |
//! | anything else (`.sr-only`, `*`) | `[data-rui-scope] .sr-only` |
//!
//! Declarations, comments and at-rule preludes are copied unchanged; `@media`, `@supports`,
//! `@layer` and `@container` blocks are rewritten recursively. `rust-ui registry` writes the
//! kit's `theme.scoped.css` and `reset.scoped.css` with it, and `rust-ui init --scoped`
//! applies it to the theme it copies.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The attribute that marks the element the scoped stylesheets apply inside.
pub const SCOPE: &str = "[data-rui-scope]";

/// The kit's stylesheets that get a generated scoped twin: (source, scoped), relative to the
/// repo root.
pub const SCOPED_FILES: [(&str, &str); 2] = [
    (
        "crates/ui/assets/theme.css",
        "crates/ui/assets/theme.scoped.css",
    ),
    (
        "crates/ui/assets/reset.css",
        "crates/ui/assets/reset.scoped.css",
    ),
];

/// `css` with every selector confined to [`SCOPE`].
pub fn scope_css(css: &str) -> String {
    rewrite_rules(css)
}

/// The generated file's text: a header naming the source, then the scoped CSS.
pub fn scoped_file(source_name: &str, css: &str) -> String {
    format!(
        "/* Generated from {source_name} by `rust-ui registry`; do not edit.\n \
         * The same rules, applied only inside an element with data-rui-scope (ThemeScope\n \
         * renders one), so they never reach the rest of an existing app. */\n{}",
        scope_css(css)
    )
}

/// Write each scoped file from its source. Returns the paths written.
pub fn write(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for (src, dst) in SCOPED_FILES {
        let text = fs::read_to_string(root.join(src))?;
        let name = Path::new(src)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let path = root.join(dst);
        fs::write(&path, scoped_file(&name, &text))?;
        out.push(path);
    }
    Ok(out)
}

/// Scoped files that differ from what [`write`] would produce.
pub fn stale(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for (src, dst) in SCOPED_FILES {
        let text = fs::read_to_string(root.join(src))?;
        let name = Path::new(src)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let path = root.join(dst);
        if fs::read_to_string(&path).ok() != Some(scoped_file(&name, &text)) {
            out.push(path);
        }
    }
    Ok(out)
}

/// At-rules whose block holds rules (rewritten); any other block (`@keyframes`,
/// `@font-face`) is copied as it is.
const NESTING_AT_RULES: [&str; 4] = ["@media", "@supports", "@layer", "@container"];

/// Walk a list of rules, rewriting the selector of each qualified rule.
fn rewrite_rules(css: &str) -> String {
    let b = css.as_bytes();
    let mut out = String::with_capacity(css.len() * 2);
    let mut start = 0; // start of the current prelude
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'*') => i = skip_comment(b, i),
            b'"' | b'\'' => i = skip_string(b, i),
            b';' => {
                // A statement at-rule (`@import ...;`): copy it.
                out.push_str(&css[start..=i]);
                i += 1;
                start = i;
            }
            b'{' => {
                let end = matching_brace(b, i);
                let prelude = &css[start..i];
                let body = &css[i + 1..end.saturating_sub(1).max(i + 1)];
                let (lead, sel) = split_leading(prelude);
                out.push_str(lead);
                let at = sel.trim_start();
                if at.starts_with('@') {
                    out.push_str(sel);
                    out.push('{');
                    if NESTING_AT_RULES.iter().any(|r| at.starts_with(r)) {
                        out.push_str(&rewrite_rules(body));
                    } else {
                        out.push_str(body);
                    }
                    out.push('}');
                } else {
                    let trailing = &sel[sel.trim_end().len()..];
                    out.push_str(&scope_selector_list(sel.trim_end()));
                    out.push_str(trailing);
                    out.push('{');
                    out.push_str(body);
                    out.push('}');
                }
                i = end;
                start = i;
            }
            _ => i += 1,
        }
    }
    out.push_str(&css[start..]);
    out
}

/// Index just past the comment starting at `i`.
fn skip_comment(b: &[u8], i: usize) -> usize {
    let mut j = i + 2;
    while j + 1 < b.len() && !(b[j] == b'*' && b[j + 1] == b'/') {
        j += 1;
    }
    (j + 2).min(b.len())
}

/// Index just past the string starting at `i`.
fn skip_string(b: &[u8], i: usize) -> usize {
    let q = b[i];
    let mut j = i + 1;
    while j < b.len() && b[j] != q {
        if b[j] == b'\\' {
            j += 1;
        }
        j += 1;
    }
    (j + 1).min(b.len())
}

/// Index just past the `}` matching the `{` at `open`.
fn matching_brace(b: &[u8], open: usize) -> usize {
    let mut depth = 0;
    let mut j = open;
    while j < b.len() {
        match b[j] {
            b'/' if b.get(j + 1) == Some(&b'*') => {
                j = skip_comment(b, j);
                continue;
            }
            b'"' | b'\'' => {
                j = skip_string(b, j);
                continue;
            }
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return j + 1;
                }
            }
            _ => {}
        }
        j += 1;
    }
    b.len()
}

/// Split a prelude into its leading whitespace and comments, and the selector (or at-rule).
fn split_leading(prelude: &str) -> (&str, &str) {
    let b = prelude.as_bytes();
    let mut i = 0;
    loop {
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if b[i..].starts_with(b"/*") {
            i = skip_comment(b, i);
        } else {
            break;
        }
    }
    prelude.split_at(i)
}

/// Split at top-level occurrences of `sep` (outside brackets, parentheses and strings).
fn split_top(s: &str, sep: u8) -> Vec<&str> {
    let b = s.as_bytes();
    let (mut depth, mut start, mut i) = (0i32, 0, 0);
    let mut parts = Vec::new();
    while i < b.len() {
        match b[i] {
            b'"' | b'\'' => {
                i = skip_string(b, i);
                continue;
            }
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            c if c == sep && depth == 0 => {
                parts.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    parts.push(&s[start..]);
    parts
}

/// Each selector in a comma list, scoped; the list is joined one per line, duplicates dropped.
fn scope_selector_list(list: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    for sel in split_top(list, b',') {
        for s in scope_selector(sel.trim()) {
            if !out.contains(&s) {
                out.push(s);
            }
        }
    }
    out.join(",\n")
}

/// Length of the first compound selector (up to the first top-level combinator).
fn first_compound_len(sel: &str) -> usize {
    let b = sel.as_bytes();
    let (mut depth, mut i) = (0i32, 0);
    while i < b.len() {
        match b[i] {
            b'"' | b'\'' => {
                i = skip_string(b, i);
                continue;
            }
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b' ' | b'\t' | b'\n' | b'\r' | b'>' | b'+' | b'~' if depth == 0 => return i,
            _ => {}
        }
        i += 1;
    }
    b.len()
}

fn scope_selector(sel: &str) -> Vec<String> {
    let n = first_compound_len(sel);
    let (first, rest) = sel.split_at(n);
    // The page root: the scope element takes its place.
    if first.contains(":root") || first.starts_with("html") {
        let mut c = first.replace(":root", SCOPE);
        if let Some(tail) = c.strip_prefix("html") {
            c = format!("{SCOPE}{tail}");
        }
        return vec![format!("{c}{rest}")];
    }
    // `:where(x)` keeps its zero specificity: scope inside it.
    if let Some(inner_rest) = first.strip_prefix(":where(") {
        if let Some(close) = closing_paren(inner_rest) {
            let inner = &inner_rest[..close];
            let after = &inner_rest[close + 1..];
            let scoped: Vec<String> = split_top(inner, b',')
                .into_iter()
                .flat_map(|s| scope_selector(s.trim()))
                .collect();
            return vec![format!(":where({}){after}{rest}", scoped.join(", "))];
        }
    }
    // Attribute switches such as `[data-theme="dark"]` or `[dir="rtl"]` may sit on the scope
    // itself, inside it, or on an ancestor (the app sets them on `<html>`).
    if is_attribute_only(first) {
        return vec![
            format!("{SCOPE}{first}{rest}"),
            format!("{SCOPE} {first}{rest}"),
            format!("{first} {SCOPE}{rest}"),
        ];
    }
    vec![format!("{SCOPE} {sel}")]
}

/// Index of the `)` that closes an already opened parenthesis in `s`.
fn closing_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Whether a compound selector is only attribute selectors (`[a][b="c"]`).
fn is_attribute_only(compound: &str) -> bool {
    let b = compound.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'[' {
            return false;
        }
        i += 1;
        while i < b.len() && b[i] != b']' {
            if b[i] == b'"' || b[i] == b'\'' {
                i = skip_string(b, i);
            } else {
                i += 1;
            }
        }
        i += 1;
    }
    !b.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(sel: &str) -> String {
        scope_selector_list(sel)
    }

    #[test]
    fn root_and_html_become_the_scope() {
        assert_eq!(one(":root"), "[data-rui-scope]");
        assert_eq!(one(":root:root"), "[data-rui-scope][data-rui-scope]");
        assert_eq!(
            one("html[data-theme=\"dark\"]"),
            "[data-rui-scope][data-theme=\"dark\"]"
        );
        assert_eq!(
            one("html:root :is(input, textarea):focus-visible"),
            "[data-rui-scope][data-rui-scope] :is(input, textarea):focus-visible"
        );
    }

    #[test]
    fn attribute_switches_match_on_inside_and_above_the_scope() {
        assert_eq!(
            one("[data-accent=\"blue\"]"),
            "[data-rui-scope][data-accent=\"blue\"],\n\
             [data-rui-scope] [data-accent=\"blue\"],\n\
             [data-accent=\"blue\"] [data-rui-scope]"
        );
        assert_eq!(
            one("[dir=\"rtl\"] svg[data-mirror-rtl]"),
            "[data-rui-scope][dir=\"rtl\"] svg[data-mirror-rtl],\n\
             [data-rui-scope] [dir=\"rtl\"] svg[data-mirror-rtl],\n\
             [dir=\"rtl\"] [data-rui-scope] svg[data-mirror-rtl]"
        );
    }

    #[test]
    fn where_keeps_zero_specificity_and_the_rest_is_a_descendant() {
        assert_eq!(
            one(":where(svg[data-icon][data-size=\"xs\"])"),
            ":where([data-rui-scope] svg[data-icon][data-size=\"xs\"])"
        );
        assert_eq!(
            one("*,\n  *::before"),
            "[data-rui-scope] *,\n[data-rui-scope] *::before"
        );
        assert_eq!(one(".sr-only"), "[data-rui-scope] .sr-only");
    }

    #[test]
    fn declarations_comments_and_at_rules_are_kept() {
        let css = "/* a { b } */\n:root {\n  --x: \"{\";\n}\n@media (prefers-color-scheme: dark) {\n  :root { --y: 1; }\n}\n@keyframes k { from { opacity: 0 } }\n@supports not selector(:dir(rtl)) {\n  [dir=\"rtl\"] svg { scale: -1 1; }\n}\n";
        let out = scope_css(css);
        assert!(out.starts_with("/* a { b } */\n[data-rui-scope] {\n  --x: \"{\";\n}"));
        assert!(out
            .contains("@media (prefers-color-scheme: dark) {\n  [data-rui-scope] { --y: 1; }\n}"));
        assert!(out.contains("@keyframes k { from { opacity: 0 } }"));
        assert!(out.contains("@supports not selector(:dir(rtl)) {"));
        assert!(!out.contains("\n:root"));
    }

    #[test]
    fn the_kit_theme_has_no_unscoped_selector_left() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for (src, _) in SCOPED_FILES {
            let css = fs::read_to_string(root.join(src)).unwrap();
            let out = scope_css(&css);
            let mut rules = Vec::new();
            collect_selectors(&out, &mut rules);
            assert!(!rules.is_empty(), "{src}: no rules found");
            for sel in rules {
                for s in split_top(&sel, b',') {
                    assert!(s.contains(SCOPE), "{src}: `{}` is not scoped", s.trim());
                }
            }
        }
    }

    /// Every qualified rule's selector in `css` (at-rule blocks searched recursively).
    fn collect_selectors(css: &str, out: &mut Vec<String>) {
        let b = css.as_bytes();
        let (mut i, mut start) = (0, 0);
        while i < b.len() {
            match b[i] {
                b'/' if b.get(i + 1) == Some(&b'*') => i = skip_comment(b, i),
                b'"' | b'\'' => i = skip_string(b, i),
                b';' => {
                    i += 1;
                    start = i;
                }
                b'{' => {
                    let end = matching_brace(b, i);
                    let (_, sel) = split_leading(&css[start..i]);
                    let sel = sel.trim();
                    if sel.starts_with('@') {
                        if NESTING_AT_RULES.iter().any(|r| sel.starts_with(r)) {
                            collect_selectors(&css[i + 1..end - 1], out);
                        }
                    } else {
                        out.push(sel.to_string());
                    }
                    i = end;
                    start = i;
                }
                _ => i += 1,
            }
        }
    }
}
