//! `rust-ui doctor`: static checks that keep components consistent with the guardrails.
//!
//! Errors fail the command; warnings are reported but do not. Everything here reads
//! files only, so it is fast enough to run in the gate.

use crate::{COMPONENTS_DIR, DEMOS_DIR, THEME_FILE};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub level: Level,
    pub component: String,
    pub message: String,
}

/// CSS properties whose value is a colour. Shadows and gradients are left alone because
/// a black shadow with alpha is the same in both themes.
const COLOUR_PROPS: &[&str] = &[
    "color",
    "background",
    "background-color",
    "border",
    "border-color",
    "border-top",
    "border-bottom",
    "border-left",
    "border-right",
    "outline",
    "outline-color",
    "fill",
    "stroke",
];

fn has_colour_literal(value: &str) -> bool {
    if value.contains("gradient") {
        return false;
    }
    let bytes = value.as_bytes();
    let hex = bytes.iter().enumerate().any(|(i, &b)| {
        b == b'#'
            && bytes[i + 1..]
                .iter()
                .take_while(|c| c.is_ascii_hexdigit())
                .count()
                >= 3
    });
    hex || value.contains("rgb(") || value.contains("rgba(") || value.contains("hsl(")
}

/// Hard-coded colours in colour properties, as `(line, text)`. A literal used only as the
/// fallback inside `var(--token, #fff)` is allowed because the token still wins.
pub fn hard_coded_colours(css: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (i, line) in css.lines().enumerate() {
        let line = line.trim();
        let Some((prop, value)) = line.split_once(':') else {
            continue;
        };
        if !COLOUR_PROPS.contains(&prop.trim()) {
            continue;
        }
        let value = value.trim().trim_end_matches(';');
        // Drop `var(--x, fallback)` groups so only bare literals remain.
        let mut bare = String::new();
        let mut depth = 0;
        for ch in value.chars() {
            match ch {
                '(' if bare.ends_with("var") => depth += 1,
                '(' if depth > 0 => depth += 1,
                ')' if depth > 0 => depth -= 1,
                _ if depth > 0 => {}
                _ => bare.push(ch),
            }
        }
        if has_colour_literal(&bare) {
            out.push((i + 1, line.to_string()));
        }
    }
    out
}

/// Physical properties that flip wrongly in right-to-left layouts, as `(line, text)`.
///
/// Left/right offsets are allowed where they are deliberate: percentage or `calc` values
/// (centring), lines marked `rtl-ok`, and rules for `data-side="left|right"` placement, which name a physical side.
pub fn physical_properties(css: &str) -> Vec<(usize, String)> {
    const PHYSICAL: &[&str] = &[
        "margin-left",
        "margin-right",
        "padding-left",
        "padding-right",
        "border-left",
        "border-right",
        "border-top-left-radius",
        "border-top-right-radius",
        "border-bottom-left-radius",
        "border-bottom-right-radius",
    ];
    let mut out = Vec::new();
    let mut physical_side = false;
    for (i, line) in css.lines().enumerate() {
        let line = line.trim();
        if line.ends_with('{') {
            physical_side =
                line.contains("data-side=\"left\"") || line.contains("data-side=\"right\"");
            continue;
        }
        let Some((prop, value)) = line.split_once(':') else {
            continue;
        };
        if line.contains("rtl-ok") {
            continue;
        }
        let prop = prop.trim();
        let value = value.trim().trim_end_matches(';').trim();
        let offset = matches!(prop, "left" | "right")
            && !value.contains('%')
            && !value.contains("calc")
            && !physical_side;
        let physical = (PHYSICAL.contains(&prop) && !physical_side)
            || offset
            || (prop == "text-align" && matches!(value, "left" | "right"))
            || (prop == "float" && matches!(value, "left" | "right"));
        if physical {
            out.push((i + 1, line.to_string()));
        }
    }
    out
}

/// A component counts as tested when some test function is named after it.
fn has_test(tests: &str, name: &str) -> bool {
    let stem = name.strip_suffix("_group").unwrap_or(name);
    tests.contains(&format!("fn {name}")) || tests.contains(&format!("fn {stem}"))
}

/// Custom properties that dioxus-primitives sets inline at runtime, so no stylesheet
/// declares them.
const PRIMITIVE_VARS: &[&str] = &["--toast-count", "--toast-index", "--area-color"];

/// Custom properties declared as `--name:` anywhere in `text` (CSS, or inline styles in Rust).
pub fn declared_vars(text: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let mut rest = text;
    while let Some(i) = rest.find("--") {
        let tail = &rest[i..];
        let len = tail[2..]
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
            .map_or(tail.len(), |n| n + 2);
        if tail[len..].trim_start().starts_with(':') && len > 2 {
            out.insert(tail[..len].to_string());
        }
        rest = &tail[len.max(2)..];
    }
    out
}

/// `var(--name)` uses without a fallback, as `(line, name)`. With a fallback the value is
/// still valid when the property is missing, so those are fine.
pub fn bare_var_uses(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let mut rest = line;
        while let Some(at) = rest.find("var(") {
            let inner = rest[at + 4..].trim_start();
            let len = inner
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
                .unwrap_or(inner.len());
            let name = &inner[..len];
            if name.starts_with("--") && inner[len..].trim_start().starts_with(')') {
                out.push((i + 1, name.to_string()));
            }
            rest = &rest[at + 4..];
        }
    }
    out
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

/// Run every check against a repo checkout.
pub fn run(root: &Path) -> Result<Vec<Finding>, String> {
    let dir = root.join(COMPONENTS_DIR);
    let mut names: Vec<String> = fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();

    let tests = root.join("crates/ui/tests");
    let render: String = ["render.rs", "form.rs", "data_table.rs", "pickers.rs"]
        .iter()
        .map(|f| read(&tests.join(f)))
        .collect();
    let demos = read(&tests.join("demos.rs"));
    let gallery = read(&root.join("apps/gallery/src/main.rs"));

    let mut out = Vec::new();
    let mut add = |level, c: &str, m: String| {
        out.push(Finding {
            level,
            component: c.to_string(),
            message: m,
        })
    };

    let theme = read(&root.join(THEME_FILE));
    if !theme.contains("prefers-reduced-motion") {
        add(
            Level::Error,
            "theme",
            "theme.css has no prefers-reduced-motion rule".into(),
        );
    }

    // Every custom property declared anywhere in the kit, for the undefined-var check.
    let mut sources: Vec<(String, String, String)> = Vec::new(); // (component, file, text)
    for name in &names {
        if let Ok(files) = fs::read_dir(dir.join(name)) {
            for f in files.filter_map(Result::ok) {
                let file = f.file_name().to_string_lossy().into_owned();
                if file.ends_with(".css") || file.ends_with(".rs") {
                    sources.push((name.clone(), file, read(&f.path())));
                }
            }
        }
    }
    let mut declared = declared_vars(&theme);
    for (_, _, text) in &sources {
        declared.extend(declared_vars(text));
    }
    sources.sort();
    for (name, file, text) in &sources {
        for (line, var) in bare_var_uses(text) {
            if !declared.contains(&var) && !PRIMITIVE_VARS.contains(&var.as_str()) {
                add(
                    Level::Error,
                    name,
                    format!("{file}:{line} uses var({var}), which nothing defines"),
                );
            }
        }
    }

    for name in &names {
        let folder = dir.join(name);
        let demo = root.join(DEMOS_DIR).join(format!("{name}.rs"));
        for file in ["component.rs", "style.css", "docs.md"] {
            if !folder.join(file).exists() {
                add(Level::Error, name, format!("missing {file}"));
            }
        }
        let has_demo = demo.exists();
        if !has_demo {
            add(Level::Warning, name, format!("no demo in {DEMOS_DIR}"));
        } else {
            if !demos.contains(&format!("{name}_demo")) {
                add(Level::Error, name, "demo is not in tests/demos.rs".into());
            }
            if !gallery.contains(&format!("demos::{name}::Demo")) {
                add(
                    Level::Error,
                    name,
                    "demo is not shown in the gallery".into(),
                );
            }
        }
        if !has_test(&render, name) {
            add(Level::Warning, name, "no render test".into());
        }
        let css = read(&folder.join("style.css"));
        for (line, text) in hard_coded_colours(&css) {
            add(
                Level::Warning,
                name,
                format!("style.css:{line} hard-coded colour `{text}` (use a theme token)"),
            );
        }
        let physical = physical_properties(&css);
        if !physical.is_empty() {
            add(
                Level::Error,
                name,
                format!(
                    "{} physical CSS properties break right-to-left (first: style.css:{} `{}`)",
                    physical.len(),
                    physical[0].0,
                    physical[0].1
                ),
            );
        }
    }
    out.sort_by_key(|a| a.level == Level::Warning);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_bare_colours_but_not_token_fallbacks() {
        let css = "a {\n  color: #fff;\n  background: var(--x, #000);\n  border: 1px solid rgb(0 0 0);\n  box-shadow: 0 0 rgb(0 0 0);\n  background: linear-gradient(red, #fff);\n}";
        let lines: Vec<_> = hard_coded_colours(css).iter().map(|f| f.0).collect();
        assert_eq!(lines, [2, 4]);
    }

    #[test]
    fn flags_physical_properties_only() {
        let css = "a {\n  margin-left: 1px;\n  margin-inline-start: 1px;\n  text-align: left;\n  text-align: center;\n  left: 0;\n  left: 50%;\n}\na[data-side=\"right\"] {\n  margin-left: 4px;\n  left: 100%;\n}";
        let lines: Vec<_> = physical_properties(css).iter().map(|f| f.0).collect();
        assert_eq!(lines, [2, 4, 6]);
    }

    #[test]
    fn finds_declared_and_bare_vars() {
        let css = "a {\n  --gap: 1px;\n  margin: var(--gap);\n  color: var(--fg, #000);\n  width: calc(var( --w ) * 2);\n}";
        let declared = declared_vars(css);
        assert!(
            declared.contains("--gap") && declared.len() == 1,
            "{declared:?}"
        );
        assert_eq!(
            bare_var_uses(css),
            [(3, "--gap".to_string()), (5, "--w".to_string())]
        );
    }

    #[test]
    fn the_repo_has_no_doctor_errors() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let errors: Vec<_> = run(&root)
            .unwrap()
            .into_iter()
            .filter(|f| f.level == Level::Error)
            .collect();
        assert!(errors.is_empty(), "{errors:#?}");
    }
}
