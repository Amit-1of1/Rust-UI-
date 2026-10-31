//! Motion audit: what every animation and transition in a set of stylesheets costs the browser.
//!
//! Each animated property falls in one of three classes, the order browsers do the work in:
//!
//! - **composite**: `transform` (and `translate`, `scale`, `rotate`), `opacity`, `filter`.
//!   The compositor runs these off the main thread; a busy main thread doesn't drop frames.
//! - **paint**: colours, shadows, `visibility` and the like. Repainted every frame, no layout.
//! - **layout**: sizes, positions, spacing, `all`, or anything the audit doesn't know. Each
//!   frame lays the page out again, which is what drops frames on slow devices.
//!
//! A layout animation is an error unless the rule (or its `@keyframes` block) says why it needs
//! one, in a comment naming the properties:
//!
//! ```css
//! .dx-sidebar-gap {
//!   /* atoll-motion: layout width (the gap is what makes room for the sidebar) */
//!   transition: width 200ms ease-out;
//! }
//! ```
//!
//! `atoll motion` prints the audit; `crates/cli/tests/motion.rs` runs it on the kit, and
//! `tests/e2e/frames.mjs` checks the same thing in the browser (what actually ran, Chromium's
//! own compositor verdict, and frame times).

use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cost {
    Composite,
    Paint,
    Layout,
}

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Cost::Composite => "composite",
            Cost::Paint => "paint",
            Cost::Layout => "layout",
        })
    }
}

/// What animating `property` costs. Unknown properties count as layout, so the audit can only
/// err on the safe side.
pub fn cost(property: &str) -> Cost {
    let p = property.trim().to_ascii_lowercase();
    match p.as_str() {
        "transform" | "translate" | "scale" | "rotate" | "opacity" | "filter"
        | "backdrop-filter" => Cost::Composite,
        "color"
        | "background"
        | "background-color"
        | "background-position"
        | "border-color"
        | "border-top-color"
        | "border-right-color"
        | "border-bottom-color"
        | "border-left-color"
        | "border-inline-color"
        | "border-block-color"
        | "outline-color"
        | "outline-offset"
        | "text-decoration-color"
        | "box-shadow"
        | "fill"
        | "stroke"
        | "stroke-dashoffset"
        | "stroke-dasharray"
        | "visibility"
        | "clip-path"
        | "caret-color"
        | "accent-color"
        | "text-shadow"
        | "border-radius" => Cost::Paint,
        _ => Cost::Layout,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Transition,
    Keyframes,
}

/// One animated property in one rule.
#[derive(Clone, Debug)]
pub struct Finding {
    pub file: String,
    /// The rule's selector, or `@keyframes <name>`.
    pub selector: String,
    pub kind: Kind,
    pub property: String,
    pub cost: Cost,
    /// The reason from an `atoll-motion: layout ...` comment, when one covers this property.
    pub allowed: Option<String>,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.kind {
            Kind::Transition => "transition",
            Kind::Keyframes => "animation",
        };
        write!(
            f,
            "{} `{}`: {kind} of `{}` ({})",
            self.file, self.selector, self.property, self.cost
        )?;
        if let Some(why) = &self.allowed {
            write!(f, ": {why}")?;
        }
        Ok(())
    }
}

/// One block of a stylesheet while scanning: its prelude, declarations and the motion
/// allowances its comments give.
struct Block {
    prelude: String,
    allow: Vec<(String, String)>,
}

/// `atoll-motion: layout width height (reason)` → [("width", reason), ("height", reason)].
fn allowances(comment: &str) -> Vec<(String, String)> {
    let Some(rest) = comment.split("atoll-motion:").nth(1) else {
        return Vec::new();
    };
    let rest = rest.trim();
    let Some(rest) = rest.strip_prefix("layout") else {
        return Vec::new();
    };
    let (props, reason) = match rest.find('(') {
        Some(i) => (&rest[..i], rest[i + 1..].trim_end().trim_end_matches(')')),
        None => (rest, ""),
    };
    props
        .split([' ', ','])
        .filter(|p| !p.is_empty())
        .map(|p| (p.to_ascii_lowercase(), reason.trim().to_string()))
        .collect()
}

/// Splits on `sep` outside parentheses.
fn split_top(s: &str, sep: char) -> Vec<&str> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0i32, 0);
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

const NOT_A_PROPERTY: &[&str] = &[
    "ease",
    "ease-in",
    "ease-out",
    "ease-in-out",
    "linear",
    "step-start",
    "step-end",
    "allow-discrete",
    "normal",
    "infinite",
    "initial",
    "inherit",
    "unset",
];

/// The properties a `transition` shorthand or `transition-property` value names.
pub fn transition_properties(value: &str) -> Vec<String> {
    let mut out = Vec::new();
    for item in split_top(value, ',') {
        for token in split_top(item.trim(), ' ') {
            let t = token.trim().to_ascii_lowercase();
            let is_time =
                t.ends_with("ms") || t.ends_with('s') && t[..t.len() - 1].parse::<f64>().is_ok();
            if t.is_empty() || t.contains('(') || is_time || NOT_A_PROPERTY.contains(&t.as_str()) {
                continue;
            }
            if t.parse::<f64>().is_ok() || t == "none" {
                continue;
            }
            out.push(t);
            break;
        }
    }
    out
}

/// Every animated property in one stylesheet.
pub fn audit_css(file: &str, css: &str) -> Vec<Finding> {
    let chars: Vec<char> = css.chars().collect();
    let mut stack: Vec<Block> = vec![Block {
        prelude: String::new(),
        allow: Vec::new(),
    }];
    let mut findings = Vec::new();
    let mut buf = String::new();
    let mut depth = 0i32;
    let mut i = 0;

    let declaration = |buf: &str, stack: &[Block], findings: &mut Vec<Finding>| {
        let Some((prop, value)) = buf.split_once(':') else {
            return;
        };
        let prop = prop.trim().to_ascii_lowercase();
        let keyframes = stack
            .iter()
            .find(|b| b.prelude.starts_with("@keyframes"))
            .map(|b| b.prelude.clone());
        let (kind, props, selector) = match (&keyframes, prop.as_str()) {
            (Some(name), p) if !p.starts_with("--") && p != "offset" => {
                (Kind::Keyframes, vec![p.to_string()], name.clone())
            }
            (None, "transition" | "transition-property") => {
                let selector = stack
                    .iter()
                    .filter(|b| !b.prelude.is_empty() && !b.prelude.starts_with('@'))
                    .map(|b| b.prelude.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                (Kind::Transition, transition_properties(value), selector)
            }
            _ => return,
        };
        for property in props {
            let cost = cost(&property);
            let allowed = (cost == Cost::Layout)
                .then(|| {
                    stack
                        .iter()
                        .rev()
                        .flat_map(|b| &b.allow)
                        .find(|(p, _)| *p == property)
                        .map(|(_, why)| why.clone())
                })
                .flatten();
            findings.push(Finding {
                file: file.to_string(),
                selector: selector.clone(),
                kind,
                property,
                cost,
                allowed,
            });
        }
    };

    while i < chars.len() {
        let c = chars[i];
        if c == '/' && chars.get(i + 1) == Some(&'*') {
            let start = i + 2;
            let mut end = start;
            while end + 1 < chars.len() && !(chars[end] == '*' && chars[end + 1] == '/') {
                end += 1;
            }
            let comment: String = chars[start..end.min(chars.len())].iter().collect();
            if let Some(top) = stack.last_mut() {
                top.allow.extend(allowances(&comment));
            }
            i = end + 2;
            continue;
        }
        i += 1;
        match c {
            '(' => {
                depth += 1;
                buf.push(c);
            }
            ')' => {
                depth -= 1;
                buf.push(c);
            }
            ';' if depth == 0 => {
                declaration(&buf, &stack, &mut findings);
                buf.clear();
            }
            '{' if depth == 0 => {
                stack.push(Block {
                    prelude: buf.split_whitespace().collect::<Vec<_>>().join(" "),
                    allow: Vec::new(),
                });
                buf.clear();
            }
            '}' if depth == 0 => {
                declaration(&buf, &stack, &mut findings);
                buf.clear();
                if stack.len() > 1 {
                    stack.pop();
                }
            }
            _ => buf.push(c),
        }
    }
    findings
}

/// The kit's stylesheets: the theme files and every component's and block's `style.css`.
pub fn kit_sheets(root: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        let mut entries: Vec<_> = entries.flatten().map(|e| e.path()).collect();
        entries.sort();
        for p in entries {
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|e| e == "css") {
                out.push(p);
            }
        }
    }
    let mut out = Vec::new();
    walk(&root.join("crates/ui/assets"), &mut out);
    walk(&root.join("crates/ui/src"), &mut out);
    out
}

pub struct Report {
    pub files: usize,
    pub findings: Vec<Finding>,
}

impl Report {
    pub fn count(&self, cost: Cost) -> usize {
        self.findings.iter().filter(|f| f.cost == cost).count()
    }

    /// Layout animations no comment explains: the audit's errors.
    pub fn errors(&self) -> Vec<&Finding> {
        self.findings
            .iter()
            .filter(|f| f.cost == Cost::Layout && f.allowed.is_none())
            .collect()
    }
}

/// Audit `files`, or the kit's stylesheets under `root` when `files` is empty.
pub fn run(root: &Path, files: &[PathBuf]) -> Result<Report, String> {
    let files = if files.is_empty() {
        kit_sheets(root)
    } else {
        files.to_vec()
    };
    let mut findings = Vec::new();
    for f in &files {
        let css =
            std::fs::read_to_string(f).map_err(|e| format!("reading {}: {e}", f.display()))?;
        let name = f
            .strip_prefix(root)
            .unwrap_or(f)
            .to_string_lossy()
            .into_owned();
        findings.extend(audit_css(&name, &css));
    }
    Ok(Report {
        files: files.len(),
        findings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_shorthand_properties() {
        assert_eq!(
            transition_properties(
                "transform 150ms ease-out, opacity 200ms var(--spring-snappy, ease-out), visibility 0ms 200ms"
            ),
            ["transform", "opacity", "visibility"]
        );
        assert_eq!(transition_properties("none"), Vec::<String>::new());
        assert_eq!(transition_properties("all 0.2s"), ["all"]);
        assert_eq!(transition_properties("width, height"), ["width", "height"]);
    }

    #[test]
    fn keyframes_and_allowances() {
        let css = r#"
            .a { transition: width 1s, opacity 1s; }
            .b {
              /* atoll-motion: layout height (it reveals content) */
              transition: height 1s;
              &:hover { transition: margin 1s; }
            }
            @keyframes spin { from { transform: rotate(0) } to { transform: rotate(1turn) } }
            /* not for the next block */
            @keyframes grow {
              /* atoll-motion: layout height (accordion) */
              from { height: 0 } to { height: var(--h) }
            }
        "#;
        let f = audit_css("x.css", css);
        let show: Vec<String> = f
            .iter()
            .map(|f| {
                format!(
                    "{}|{}|{}|{}",
                    f.selector,
                    f.property,
                    f.cost,
                    f.allowed.is_some()
                )
            })
            .collect();
        assert_eq!(
            show,
            [
                ".a|width|layout|false",
                ".a|opacity|composite|false",
                ".b|height|layout|true",
                ".b &:hover|margin|layout|false",
                "@keyframes spin|transform|composite|false",
                "@keyframes spin|transform|composite|false",
                "@keyframes grow|height|layout|true",
                "@keyframes grow|height|layout|true",
            ]
        );
    }
}
