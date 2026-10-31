//! `rust-ui explain`: run `cargo check` and say, in plain words, how to fix the errors that
//! come from using the kit's components.
//!
//! `rsx!` turns every component call into a props builder, so a mistake in one line of markup
//! reaches you as a builder error: "Missing required field `title`" pointing at the macro, or
//! "no method named `colour` found for struct `ButtonPropsBuilder<((), ...)>`". The fix is
//! almost always in the component's own source: which props it has, what each one means, and
//! which enum variants exist. This module reads that source (the components you copied, and
//! the registry for the ones you have not) and turns each such error into a short note with
//! the fix and the documentation of the prop involved. It reads files and compiler output
//! only; it never edits your code.

use crate::{BLOCKS_DIR, COMPONENTS_DIR};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// One prop of a component, as written in its source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Prop {
    pub name: String,
    pub ty: String,
    /// The prop's `///` comment, joined into one paragraph.
    pub doc: String,
    /// Whether leaving it out compiles (`#[props(default)]`, or an `Option`).
    pub optional: bool,
}

/// A component's props, from `pub struct <Name>Props` or a `#[component] fn` argument list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Component {
    pub name: String,
    /// The registry item (folder) it lives in.
    pub item: String,
    pub props: Vec<Prop>,
    /// The props accept any HTML attribute too (`#[props(extends = ...)]`).
    pub extends: bool,
}

/// What the kit's source says about its own API.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Api {
    pub components: BTreeMap<String, Component>,
    /// Enum name to (item, variants).
    pub enums: BTreeMap<String, (String, Vec<String>)>,
    /// Every registry item (component and block folder) name, installed or not.
    pub items: Vec<String>,
    /// The changelog, docs/MIGRATION.md and unreleased `changes/` fragments, as (file, text):
    /// searched for renamed and removed names.
    #[serde(skip)]
    pub history: Vec<(String, String)>,
}

impl Api {
    /// Reads the components and blocks under `kit` (a checkout of this repo), then lets the
    /// copies in `project_dirs` (an app's `src/components`, edited or not) override them.
    pub fn load(kit: &Path, project_dirs: &[PathBuf]) -> Api {
        let mut api = Api::default();
        for dir in [kit.join(COMPONENTS_DIR), kit.join(BLOCKS_DIR)] {
            api.scan_dir(&dir, true);
        }
        for dir in project_dirs {
            api.scan_dir(dir, false);
        }
        api.items.sort();
        api.items.dedup();
        let mut files = vec!["CHANGELOG.md".to_string(), "docs/MIGRATION.md".to_string()];
        if let Ok(entries) = fs::read_dir(kit.join("changes")) {
            let mut fragments: Vec<String> = entries
                .filter_map(|e| e.ok()?.file_name().into_string().ok())
                .filter(|n| n.ends_with(".md") && n != "README.md")
                .map(|n| format!("changes/{n}"))
                .collect();
            fragments.sort();
            files.extend(fragments);
        }
        api.history = files
            .into_iter()
            .filter_map(|f| Some((f.clone(), fs::read_to_string(kit.join(&f)).ok()?)))
            .collect();
        api
    }

    fn scan_dir(&mut self, dir: &Path, registry: bool) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        let mut folders: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_dir())
            .collect();
        folders.sort();
        for folder in folders {
            let Some(item) = folder.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if registry {
                self.items.push(item.to_string());
            }
            let Ok(files) = fs::read_dir(&folder) else {
                continue;
            };
            let mut files: Vec<PathBuf> = files
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|e| e == "rs"))
                .filter(|p| !p.to_string_lossy().ends_with("_demo.rs"))
                .collect();
            files.sort();
            for file in files {
                if let Ok(src) = fs::read_to_string(&file) {
                    self.scan_source(item, &src);
                }
            }
        }
    }

    /// Adds the props structs, `#[component]` functions and enums found in one file.
    pub fn scan_source(&mut self, item: &str, src: &str) {
        let src = strip_line_comments(src);
        let mut structs: BTreeMap<String, (Vec<Prop>, bool)> = BTreeMap::new();
        let mut props_of: Vec<(String, String)> = Vec::new();

        let mut rest = src.as_str();
        while let Some(at) = next_item(rest) {
            let (kind, name, body, after) = at;
            match kind {
                ItemKind::Struct if name.ends_with("Props") => {
                    structs.insert(name.clone(), parse_fields(body, true));
                }
                ItemKind::Enum => {
                    let variants = split_top(body)
                        .iter()
                        .filter_map(|v| leading_ident(&strip_attrs_and_docs(v).1))
                        .collect::<Vec<_>>();
                    if !variants.is_empty() {
                        self.enums.insert(name, (item.to_string(), variants));
                    }
                }
                ItemKind::Component => {
                    let args = split_top(body);
                    // `fn Badge(props: BadgeProps)`: the struct holds the props.
                    if let [only] = args.as_slice() {
                        let (_, code) = strip_attrs_and_docs(only);
                        if let Some((arg, ty)) = code.split_once(':') {
                            if arg.trim() == "props" {
                                props_of.push((name.clone(), ty.trim().to_string()));
                                rest = after;
                                continue;
                            }
                        }
                    }
                    let (props, extends) = parse_fields(body, false);
                    self.components.insert(
                        name.clone(),
                        Component {
                            name,
                            item: item.to_string(),
                            props,
                            extends,
                        },
                    );
                }
                ItemKind::Struct => {}
            }
            rest = after;
        }

        for (component, ty) in props_of {
            let ty = ty.split('<').next().unwrap_or(&ty).trim().to_string();
            if let Some((props, extends)) = structs.get(&ty) {
                self.components.insert(
                    component.clone(),
                    Component {
                        name: component,
                        item: item.to_string(),
                        props: props.clone(),
                        extends: *extends,
                    },
                );
            }
        }
        // A props struct used by a component in another file (or by a primitive re-export):
        // still worth knowing under its component's name.
        for (ty, (props, extends)) in structs {
            let component = ty.trim_end_matches("Props").to_string();
            self.components
                .entry(component.clone())
                .or_insert(Component {
                    name: component,
                    item: item.to_string(),
                    props,
                    extends,
                });
        }
    }
}

#[derive(Debug, PartialEq)]
enum ItemKind {
    Struct,
    Enum,
    Component,
}

/// Finds the next `pub struct X {`, `pub enum X {` or `#[component] ... fn X(` and returns its
/// kind, name, the text between its brackets, and the source after it.
fn next_item(src: &str) -> Option<(ItemKind, String, &str, &str)> {
    let candidates = [
        ("pub struct ", ItemKind::Struct, b'{'),
        ("pub enum ", ItemKind::Enum, b'{'),
        ("#[component]", ItemKind::Component, b'('),
    ];
    let (pos, kind, open) = candidates
        .into_iter()
        .filter_map(|(pat, kind, open)| src.find(pat).map(|p| (p, pat, kind, open)))
        .min_by_key(|(p, ..)| *p)
        .map(|(p, pat, kind, open)| (p + pat.len(), kind, open))?;
    let tail = &src[pos..];
    let (name, from) = if kind == ItemKind::Component {
        let fn_at = tail.find("fn ")?;
        let after_fn = &tail[fn_at + 3..];
        (leading_ident(after_fn)?, pos + fn_at + 3)
    } else {
        (leading_ident(tail)?, pos)
    };
    let rest = &src[from..];
    // A tuple or unit struct has `;` or `(` before any `{`: skip it.
    let open_at = rest.find(open as char)?;
    if kind != ItemKind::Component && rest[..open_at].contains([';', '(']) {
        return Some((
            ItemKind::Struct,
            String::new(),
            "",
            &rest[rest.find(';').unwrap_or(0) + 1..],
        ));
    }
    let close = matching(rest.as_bytes(), open_at)?;
    Some((kind, name, &rest[open_at + 1..close], &rest[close + 1..]))
}

/// Index of the bracket that closes the one at `open`.
fn matching(b: &[u8], open: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut i = open;
    let mut in_str = false;
    while i < b.len() {
        let c = b[i];
        if in_str {
            if c == b'\\' {
                i += 1;
            } else if c == b'"' {
                in_str = false;
            }
        } else {
            match c {
                b'/' if b.get(i + 1) == Some(&b'/') => {
                    while i < b.len() && b[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                }
                b'"' => in_str = true,
                b'(' | b'{' | b'[' => depth += 1,
                b')' | b'}' | b']' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    None
}

/// Splits on commas that are not inside brackets, generics or strings.
fn split_top(body: &str) -> Vec<String> {
    let b = body.as_bytes();
    let mut out = Vec::new();
    let (mut depth, mut start, mut in_str) = (0i32, 0usize, false);
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if in_str {
            if c == b'\\' {
                i += 1;
            } else if c == b'"' {
                in_str = false;
            }
        } else {
            match c {
                // Doc comments hold commas and brackets of their own.
                b'/' if b.get(i + 1) == Some(&b'/') => {
                    i = body[i..].find('\n').map_or(b.len(), |n| i + n);
                    continue;
                }
                b'"' => in_str = true,
                b'(' | b'{' | b'[' | b'<' => depth += 1,
                b'>' if i > 0 && b[i - 1] == b'-' => {}
                b')' | b'}' | b']' | b'>' => depth -= 1,
                b',' if depth == 0 => {
                    out.push(body[start..i].to_string());
                    start = i + 1;
                }
                _ => {}
            }
        }
        i += 1;
    }
    out.push(body[start..].to_string());
    out.into_iter().filter(|s| !s.trim().is_empty()).collect()
}

/// Separates `///` lines and `#[...]` attributes from the code that follows them.
/// Returns (doc text, attributes joined, code).
fn strip_attrs_and_docs(item: &str) -> (String, String) {
    let (doc, _attrs, code) = split_item(item);
    (doc, code)
}

fn split_item(item: &str) -> (String, String, String) {
    let mut doc = Vec::new();
    let mut attrs = String::new();
    let mut s = item.trim_start();
    loop {
        if let Some(r) = s.strip_prefix("///") {
            let end = r.find('\n').unwrap_or(r.len());
            doc.push(r[..end].trim().to_string());
            s = r[end..].trim_start();
        } else if s.starts_with("#[") {
            let end = matching(s.as_bytes(), 1).unwrap_or(s.len() - 1);
            attrs.push_str(&s[..=end]);
            s = s[end + 1..].trim_start();
        } else {
            break;
        }
    }
    let doc = doc
        .iter()
        .map(|l| l.as_str())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    (doc, attrs, s.trim().to_string())
}

/// Reads `name: Type` items (struct fields, or `#[component]` arguments).
fn parse_fields(body: &str, is_struct: bool) -> (Vec<Prop>, bool) {
    let mut props = Vec::new();
    let mut extends = false;
    for item in split_top(body) {
        let (doc, attrs, code) = split_item(&item);
        let code = code.strip_prefix("pub ").unwrap_or(&code).trim();
        let code = code.strip_prefix("mut ").unwrap_or(code);
        let Some((name, ty)) = code.split_once(':') else {
            continue;
        };
        let name = name.trim().trim_start_matches("r#").to_string();
        let ty = squash(ty);
        if attrs.contains("extends") {
            extends = true;
            continue;
        }
        if is_struct && name == "attributes" && ty.starts_with("Vec<Attribute") {
            continue;
        }
        let optional = attrs.contains("default")
            || ty.starts_with("Option<")
            || ty.starts_with("ReadSignal<Option<")
            || ty == "Element";
        props.push(Prop {
            name,
            ty,
            doc: unlink(&doc),
            optional,
        });
    }
    (props, extends)
}

fn squash(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// "[`DialogDescription`]" reads as "`DialogDescription`" in a terminal.
fn unlink(doc: &str) -> String {
    let mut out = String::with_capacity(doc.len());
    let mut s = doc;
    while let Some(i) = s.find("[`") {
        out.push_str(&s[..i]);
        let r = &s[i + 1..];
        match r.find("`]") {
            Some(j) => {
                out.push_str(&r[..=j]);
                let mut after = &r[j + 2..];
                // [`X`](link) and [`X`][link]
                for (open, close) in [('(', ')'), ('[', ']')] {
                    if after.starts_with(open) {
                        after = after.find(close).map_or(after, |k| &after[k + 1..]);
                    }
                }
                s = after;
            }
            None => {
                out.push('[');
                s = r;
            }
        }
    }
    out.push_str(s);
    out
}

fn leading_ident(s: &str) -> Option<String> {
    let s = s.trim_start();
    let end = s
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(s.len());
    (end > 0 && s[..1].chars().all(|c| c.is_alphabetic() || c == '_')).then(|| s[..end].to_string())
}

/// Drops `// ...` comments that are not `///` docs, outside strings.
fn strip_line_comments(src: &str) -> String {
    src.lines()
        .map(|line| {
            let b = line.as_bytes();
            let mut in_str = false;
            let mut i = 0;
            while i + 1 < b.len() {
                match b[i] {
                    b'\\' if in_str => i += 1,
                    b'"' => in_str = !in_str,
                    b'/' if !in_str && b[i + 1] == b'/' => {
                        if b.get(i + 2) == Some(&b'/') && b.get(i + 3) != Some(&b'/') {
                            return line;
                        }
                        return &line[..i];
                    }
                    _ => {}
                }
                i += 1;
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------------------
// Compiler output
// ---------------------------------------------------------------------------------------

/// One explained error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Hint {
    /// `file:line:column` of the code to change.
    pub at: String,
    /// One line: what is wrong.
    pub title: String,
    /// What to do, and the docs of the prop or enum involved.
    pub fix: Vec<String>,
    /// rustc's error code (`E0061`).
    pub code: String,
}

/// What `explain` found in one `cargo check` run.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Report {
    pub hints: Vec<Hint>,
    /// Errors that are not about the kit, as rustc's first line.
    pub other: Vec<String>,
}

#[derive(Debug, Clone)]
struct Span {
    file: String,
    line: usize,
    line_end: usize,
    col: usize,
    label: String,
    primary: bool,
}

#[derive(Debug, Clone)]
struct Diag {
    message: String,
    code: String,
    spans: Vec<Span>,
    /// Labels and messages of the error and its children, for pattern matching.
    text: String,
}

fn diag_of(v: &serde_json::Value) -> Option<Diag> {
    if v.get("reason")?.as_str()? != "compiler-message" {
        return None;
    }
    let m = v.get("message")?;
    if m.get("level")?.as_str()? != "error" {
        return None;
    }
    let spans_of = |m: &serde_json::Value| -> Vec<Span> {
        m.get("spans")
            .and_then(|s| s.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|s| {
                        Some(Span {
                            file: s.get("file_name")?.as_str()?.to_string(),
                            line: s.get("line_start")?.as_u64()? as usize,
                            line_end: s.get("line_end")?.as_u64()? as usize,
                            col: s.get("column_start")?.as_u64()? as usize,
                            label: s
                                .get("label")
                                .and_then(|l| l.as_str())
                                .unwrap_or_default()
                                .to_string(),
                            primary: s.get("is_primary")?.as_bool()?,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let spans = spans_of(m);
    let message = m.get("message")?.as_str()?.to_string();
    let mut text = message.clone();
    for s in &spans {
        text.push('\n');
        text.push_str(&s.label);
    }
    for c in m
        .get("children")
        .and_then(|c| c.as_array())
        .into_iter()
        .flatten()
    {
        if let Some(msg) = c.get("message").and_then(|m| m.as_str()) {
            text.push('\n');
            text.push_str(msg);
        }
        for s in spans_of(c) {
            text.push('\n');
            text.push_str(&s.label);
        }
    }
    let code = m
        .get("code")
        .and_then(|c| c.get("code"))
        .and_then(|c| c.as_str())
        .unwrap_or_default()
        .to_string();
    Some(Diag {
        message,
        code,
        spans,
        text,
    })
}

/// The text between the first backtick pair after `after`.
fn ticked<'a>(text: &'a str, after: &str) -> Option<&'a str> {
    let i = text.find(after)? + after.len();
    let r = &text[i..];
    let s = r.find('`')? + 1;
    let e = r[s..].find('`')? + s;
    Some(&r[s..e])
}

/// `ui::prelude::ButtonPropsBuilder<TypedBuilderFields>` -> `Button`.
fn component_of_builder(path: &str) -> Option<String> {
    let last = path.split('<').next()?.rsplit("::").next()?;
    last.split("PropsBuilder")
        .next()
        .filter(|n| !n.is_empty() && last.contains("PropsBuilder"))
        .map(str::to_string)
}

/// Levenshtein distance, for "did you mean".
fn distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.to_lowercase().chars().collect();
    let b: Vec<char> = b.to_lowercase().chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.iter().enumerate() {
        let mut cur = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            let sub = prev[j] + usize::from(ca != cb);
            cur.push(sub.min(prev[j + 1] + 1).min(cur[j] + 1));
        }
        prev = cur;
    }
    prev[b.len()]
}

fn nearest<'a>(word: &str, options: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    options
        .into_iter()
        .map(|o| (distance(word, o), o))
        .filter(|(d, o)| *d <= (o.len().max(word.len()) / 3).max(1))
        .min_by_key(|(d, _)| *d)
        .map(|(_, o)| o)
}

/// "`a`, `b` and `c`"
fn ticks(list: &[&str]) -> String {
    let t: Vec<String> = list.iter().map(|s| format!("`{s}`")).collect();
    match t.len() {
        0 => String::new(),
        1 => t[0].clone(),
        n => format!("{} and {}", t[..n - 1].join(", "), t[n - 1]),
    }
}

impl Api {
    /// Explains every error in `cargo check --message-format=json` output. `workspace` is
    /// where the relative paths in that output start (the directory cargo ran in).
    pub fn explain(&self, cargo_json: &str, workspace: &Path) -> Report {
        let mut report = Report::default();
        let mut seen = std::collections::HashSet::new();
        for line in cargo_json.lines() {
            let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
                continue;
            };
            let Some(d) = diag_of(&v) else {
                continue;
            };
            let key = format!(
                "{}{:?}",
                d.message,
                d.spans.iter().map(|s| (s.line, s.col)).collect::<Vec<_>>()
            );
            if !seen.insert(key) {
                continue;
            }
            match self.hint(&d, workspace) {
                Some(h) => report.hints.push(h),
                None => report.other.push(match d.spans.iter().find(|s| s.primary) {
                    Some(s) => format!("{}:{}:{}: {}", s.file, s.line, s.col, d.message),
                    None => d.message.clone(),
                }),
            }
        }
        report
    }

    fn hint(&self, d: &Diag, workspace: &Path) -> Option<Hint> {
        let primary = d.spans.iter().find(|s| s.primary)?;
        let at = |s: &Span| format!("{}:{}:{}", s.file, s.line, s.col);
        let hint = |title: String, fix: Vec<String>| Hint {
            at: at(primary),
            title,
            fix,
            code: d.code.clone(),
        };

        // A required prop left out: rustc says "this method takes 1 argument but 0 arguments
        // were supplied" about a type named `<Name>PropsBuilder_Error_Missing_required_field_<prop>`.
        if let Some(i) = d.text.find("PropsBuilder_Error_Missing_required_field_") {
            let before = &d.text[..i];
            let name = before
                .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
                .next()?
                .to_string();
            let field: String = d.text[i + "PropsBuilder_Error_Missing_required_field_".len()..]
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            let p = self.prop(&name, &field);
            let value = match p.map(|p| p.ty.as_str()) {
                Some("String") => "\"...\"".to_string(),
                Some(ty) if self.enums.contains_key(ty) => {
                    format!(
                        "{ty}::{}",
                        self.enums[ty].1.first().map_or("..", String::as_str)
                    )
                }
                _ => "...".to_string(),
            };
            let mut fix = vec![format!("Add it: `{name} {{ {field}: {value}, ... }}`.")];
            if let Some(p) = p {
                fix.insert(0, format!("`{field}` ({}): {}", p.ty, or_none(&p.doc)));
            }
            return Some(hint(
                format!("`{name}` needs the `{field}` prop, which has no default"),
                fix,
            ));
        }

        // An unknown prop: "no method named `colour` found for struct `ButtonPropsBuilder<..>`".
        if d.message.starts_with("no method named") && d.message.contains("PropsBuilder") {
            let prop = ticked(&d.message, "no method named")?;
            let name = component_of_builder(ticked(&d.message, "found for struct")?)?;
            let c = self.components.get(&name)?;
            let names: Vec<&str> = c.props.iter().map(|p| p.name.as_str()).collect();
            let mut fix = Vec::new();
            if let Some(near) = nearest(prop, names.iter().copied()) {
                let p = self.prop(&name, near)?;
                fix.push(format!(
                    "Did you mean `{near}` ({})? {}",
                    p.ty,
                    or_none(&p.doc)
                ));
            }
            fix.push(format!("`{name}` props: {}.", ticks(&names)));
            if c.extends {
                fix.push(
                    "It also takes HTML attributes (`class`, `id`, `aria-*`, ...). rustc's own \
                     suggestion may be one of those, not what you meant."
                        .into(),
                );
            }
            return Some(hint(format!("`{name}` has no prop `{prop}`"), fix));
        }

        // A misspelt or removed enum variant.
        if d.message.starts_with("no variant") {
            let variant = ticked(&d.message, "named")?;
            let ty = ticked(&d.message, "found for enum")?;
            let (item, variants) = self.enums.get(ty)?;
            let names: Vec<&str> = variants.iter().map(String::as_str).collect();
            let mut fix = Vec::new();
            if let Some(near) = nearest(variant, names.iter().copied()) {
                fix.push(format!("Did you mean `{ty}::{near}`?"));
            }
            fix.push(format!("`{ty}` ({item}) has {}.", ticks(&names)));
            fix.extend(self.migration_notes(&format!("{ty}::{variant}")));
            return Some(hint(format!("`{ty}::{variant}` does not exist"), fix));
        }

        // A string (or other value) where a kit enum goes. rustc points at the whole `rsx!`
        // block; find the line by the props that take that enum.
        if d.code == "E0308" {
            let expected = ticked(&d.text, "expected")?;
            let (item, variants) = self.enums.get(expected)?;
            let found = ticked(&d.text, "found").unwrap_or("another type");
            let props: Vec<(&str, &str)> = self
                .components
                .values()
                .flat_map(|c| {
                    c.props
                        .iter()
                        .filter(|p| p.ty == expected)
                        .map(move |p| (c.name.as_str(), p.name.as_str()))
                })
                .collect();
            let span = d
                .spans
                .iter()
                .find(|s| s.primary && s.label.contains("expected"))
                .unwrap_or(primary);
            let located = find_prop_line(workspace, span, expected, &props);
            let names: Vec<&str> = variants.iter().map(String::as_str).collect();
            let example = names.first().copied().unwrap_or("..");
            let mut h = hint(
                format!("a `{found}` is given where `{expected}` goes"),
                vec![
                    format!("Write the variant, not a string: `{expected}::{example}`."),
                    format!("`{expected}` ({item}) has {}.", ticks(&names)),
                ],
            );
            if let Some((line, col, prop)) = located {
                h.at = format!("{}:{line}:{col}", span.file);
                h.title = format!("`{prop}` takes a `{expected}`, not a `{found}`");
            }
            return Some(h);
        }

        // A component module that is not there: either not copied yet, or not a kit item.
        if matches!(d.code.as_str(), "E0432" | "E0433" | "E0583") {
            let module = ticked(&d.text, "could not find")
                .or_else(|| ticked(&d.text, "no"))
                .or_else(|| ticked(&d.text, "file not found for module"))
                .or_else(|| ticked(&d.text, "unresolved import"))?;
            let module = module.rsplit("::").next()?;
            if !d.text.contains("components") && !primary.file.contains("components") {
                return None;
            }
            if self.items.iter().any(|i| i == module) {
                let fix = if d.code == "E0583" {
                    format!("Copy it: `rust-ui add {module}`.")
                } else {
                    format!(
                        "Copy it: `rust-ui add {module}`, then add `pub mod {module};` to your components `mod.rs`."
                    )
                };
                return Some(hint(
                    format!("`{module}` is a kit component that is not in this project yet"),
                    vec![fix],
                ));
            }
            let names: Vec<&str> = self.items.iter().map(String::as_str).collect();
            let mut fix = Vec::new();
            if let Some(near) = nearest(module, names.iter().copied()) {
                fix.push(format!(
                    "Did you mean `{near}`? `rust-ui add {near}` copies it."
                ));
            }
            fix.push("`rust-ui list` shows every component and block.".into());
            return Some(hint(format!("there is no kit component `{module}`"), fix));
        }
        None
    }

    fn prop(&self, component: &str, prop: &str) -> Option<&Prop> {
        self.components
            .get(component)?
            .props
            .iter()
            .find(|p| p.name == prop)
    }

    /// Changelog and migration lines that name `needle` (`ButtonSize::Icon`), so a removed or
    /// renamed item points at what replaced it.
    fn migration_notes(&self, needle: &str) -> Vec<String> {
        let mut out = Vec::new();
        for (file, text) in &self.history {
            for line in text.lines().filter(|l| l.contains(needle)) {
                let line = line.replace("**", "");
                let line = line.trim().trim_start_matches(['-', '#', ' ']);
                let note = format!("{file}: {line}");
                if !out.contains(&note) && out.len() < 2 {
                    out.push(note);
                }
            }
        }
        out
    }
}

fn or_none(doc: &str) -> &str {
    if doc.is_empty() {
        "(no docs)"
    } else {
        doc
    }
}

/// Inside the lines a span covers, the first `prop: value` whose value is not `Expected::...`.
fn find_prop_line(
    workspace: &Path,
    span: &Span,
    expected: &str,
    props: &[(&str, &str)],
) -> Option<(usize, usize, String)> {
    let src = fs::read_to_string(workspace.join(&span.file)).ok()?;
    for (n, line) in src.lines().enumerate() {
        let number = n + 1;
        if number < span.line || number > span.line_end {
            continue;
        }
        for (_, prop) in props {
            let pat = format!("{prop}:");
            let mut from = 0;
            while let Some(i) = line[from..].find(&pat) {
                let at = from + i;
                let boundary = at == 0
                    || !line[..at]
                        .chars()
                        .next_back()
                        .is_some_and(|c| c.is_alphanumeric() || c == '_');
                let value = line[at + pat.len()..].trim_start();
                if boundary
                    && !value.starts_with(expected)
                    && !value.starts_with(char::is_lowercase)
                {
                    return Some((number, at + 1, prop.to_string()));
                }
                from = at + pat.len();
            }
        }
    }
    None
}

/// The report as terminal text.
pub fn render(report: &Report) -> String {
    let mut out = String::new();
    for h in &report.hints {
        out.push_str(&format!("{}  {}\n", h.at, h.title));
        for f in &h.fix {
            out.push_str(&format!("    {f}\n"));
        }
        out.push('\n');
    }
    if !report.other.is_empty() {
        out.push_str(&format!(
            "{} other error(s) are not about kit components; `cargo check` shows them in full:\n",
            report.other.len()
        ));
        for o in &report.other {
            out.push_str(&format!("    {o}\n"));
        }
    }
    out
}
