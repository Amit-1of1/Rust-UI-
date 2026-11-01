//! The "Properties" section of a component's docs page: one table per public component
//! (`#[component] pub fn`), listing each prop with its type, whether it is required, its
//! default and its doc comment. It is read from the component's own source, so it can't drift
//! from the code; `atoll registry` puts it in `apps/docs/src/generated.rs`.
//!
//! Required or optional follows the rules of Dioxus' `Props` derive (dioxus `core-macro`,
//! `props/mod.rs`): a prop is optional when it has `#[props(default)]` or `default = ...`,
//! extends an element's attributes, is `children`, or is an `Option<T>` or
//! `ReadSignal<Option<T>>` (unless marked `!optional`). Everything else, event handlers
//! included, is required.
//!
//! Components whose props struct comes from `atoll-primitives` (the props are the primitive's
//! own) get a line naming the struct instead of a table.

/// One prop of a component.
#[derive(Debug, PartialEq)]
pub struct Prop {
    pub name: String,
    pub ty: String,
    pub required: bool,
    /// What the prop is when left out, in words or as Rust (`None`, `false`, an expression).
    pub default: String,
    pub doc: String,
}

/// One public component and where its props come from.
#[derive(Debug, PartialEq)]
pub struct Component {
    pub name: String,
    pub props: Props,
}

#[derive(Debug, PartialEq)]
pub enum Props {
    /// Read from the parameter list or a props struct in the same file.
    Listed(Vec<Prop>),
    /// A props struct defined elsewhere (a primitive's), by name.
    External(String),
}

/// The public components of one source file, in file order.
pub fn components(source: &str) -> Vec<Component> {
    let lines: Vec<&str> = source.lines().collect();
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.starts_with("#[component]") {
            continue;
        }
        // The `pub fn` follows the attribute, possibly after doc lines or other attributes.
        let Some(start) = (i + 1..lines.len()).find(|&j| !lines[j].starts_with("#[")) else {
            continue;
        };
        let Some(rest) = lines[start].strip_prefix("pub fn ") else {
            continue; // private or pub(crate): not part of the API
        };
        let Some(open) = rest.find('(') else { continue };
        let name = rest[..open].trim().to_string();
        let sig = signature(&lines[start..]);
        let params = split_top(&sig);
        let props = match params.as_slice() {
            [one] if one_props_param(one).is_some() => {
                let ty = one_props_param(one).unwrap_or_default();
                let base = ty.split('<').next().unwrap_or(&ty).trim().to_string();
                match struct_fields(&lines, &base) {
                    Some(fields) => Props::Listed(fields),
                    None => Props::External(ty),
                }
            }
            _ => Props::Listed(params.iter().filter_map(|p| prop(p, false)).collect()),
        };
        out.push(Component { name, props });
    }
    out
}

/// The text between the first `(` and its matching `)` of a `pub fn` signature.
fn signature(lines: &[&str]) -> String {
    let text = lines.join("\n");
    let Some(open) = text.find('(') else {
        return String::new();
    };
    let mut depth = 0usize;
    for (k, c) in text[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return text[open + 1..open + k].to_string();
                }
            }
            _ => {}
        }
    }
    String::new()
}

/// Splits a parameter or field list at the commas that are not inside brackets. Doc comments
/// are kept with the item they precede; `//` comments are dropped.
fn split_top(list: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    for line in list.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") && !trimmed.starts_with("///") {
            continue;
        }
        if trimmed.starts_with("///") {
            // Whole lines: their commas never separate items.
            current.push_str(trimmed);
            current.push('\n');
            continue;
        }
        let mut rest = take_attributes(trimmed, &mut current);
        let mut depth = 0i32;
        let mut prev = ' ';
        'line: while !rest.is_empty() {
            for (k, c) in rest.char_indices() {
                // An attribute after a comma on the same line (`a: u8, #[props(default)] b: u8`).
                if c == '#' && depth <= 0 && rest[k..].starts_with("#[") {
                    rest = take_attributes(&rest[k..], &mut current);
                    continue 'line;
                }
                match c {
                    '(' | '[' | '{' | '<' => depth += 1,
                    ')' | ']' | '}' => depth -= 1,
                    '>' if prev != '-' && prev != '=' => depth -= 1,
                    ',' if depth <= 0 => {
                        parts.push(std::mem::take(&mut current));
                        prev = c;
                        continue;
                    }
                    _ => {}
                }
                current.push(c);
                prev = c;
            }
            break;
        }
        current.push('\n');
    }
    parts.push(current);
    parts
        .into_iter()
        .filter(|p| {
            p.lines()
                .any(|l| l.contains(':') && !l.trim().starts_with("///"))
        })
        .collect()
}

/// Moves the `#[...]` attributes at the start of `line` into `current`, each on its own line,
/// and returns the rest.
fn take_attributes<'a>(line: &'a str, current: &mut String) -> &'a str {
    let mut rest = line.trim_start();
    while rest.starts_with("#[") {
        let mut depth = 0i32;
        let mut end = rest.len();
        for (k, c) in rest.char_indices() {
            match c {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        end = k + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        current.push_str(&rest[..end]);
        current.push('\n');
        rest = rest[end..].trim_start();
    }
    rest
}

/// `props: DialogProps` (the whole parameter list is one props struct): the struct's type.
fn one_props_param(param: &str) -> Option<String> {
    let line = param
        .lines()
        .map(str::trim)
        .find(|l| !l.starts_with("///") && !l.starts_with("#["))?;
    line.strip_prefix("props:")
        .map(|t| t.trim().trim_end_matches(',').to_string())
}

/// The fields of `pub struct <name>` in the same file, when it is there.
fn struct_fields(lines: &[&str], name: &str) -> Option<Vec<Prop>> {
    let start = lines.iter().position(|l| {
        let l = l.trim_start();
        l.strip_prefix("pub struct ").is_some_and(|rest| {
            rest.strip_prefix(name)
                .is_some_and(|after| after.starts_with([' ', '{', '<']))
        })
    })?;
    let mut body = String::new();
    let mut depth = 0i32;
    let mut started = false;
    'outer: for line in &lines[start..] {
        for c in line.chars() {
            match c {
                '{' => {
                    depth += 1;
                    if !started {
                        started = true;
                        continue;
                    }
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 && started {
                        break 'outer;
                    }
                }
                _ => {}
            }
            if started {
                body.push(c);
            }
        }
        if started {
            body.push('\n');
        }
    }
    Some(
        split_top(&body)
            .iter()
            .filter_map(|f| prop(f, true))
            .collect(),
    )
}

/// One parameter or field (doc lines, attributes, then `name: Type`).
fn prop(item: &str, is_field: bool) -> Option<Prop> {
    let mut doc = Vec::new();
    let mut attrs = String::new();
    let mut decl = String::new();
    for line in item.lines().map(str::trim) {
        if let Some(d) = line.strip_prefix("///") {
            doc.push(d.trim().to_string());
        } else if line.starts_with("#[") {
            attrs.push_str(line);
        } else if !line.is_empty() {
            if !decl.is_empty() {
                decl.push(' ');
            }
            decl.push_str(line);
        }
    }
    let decl = decl.trim().trim_end_matches(',');
    let decl = if is_field {
        decl.strip_prefix("pub ")?
    } else {
        decl
    };
    let (name, ty) = decl.split_once(':')?;
    let name = name.trim().trim_start_matches("mut ").to_string();
    let ty = ty.trim().to_string();
    if name.is_empty() || name.contains(' ') || ty.is_empty() {
        return None;
    }
    let props_attr = props_args(&attrs);
    let extends: Vec<&str> = props_attr
        .iter()
        .filter_map(|a| a.strip_prefix("extends"))
        .map(|a| a.trim_start_matches([' ', '=']).trim())
        .collect();
    let explicit_default = props_attr.iter().find_map(|a| {
        if a == "default" {
            Some(String::new())
        } else {
            a.strip_prefix("default")
                .map(|rest| rest.trim_start())
                .and_then(|rest| rest.strip_prefix('='))
                .map(|e| e.trim().to_string())
        }
    });
    let not_optional = props_attr.iter().any(|a| a == "!optional");
    let optional_type = !not_optional && is_optional_type(&ty);

    let (required, default) = if !extends.is_empty() {
        (
            false,
            format!("none (any `{}` attribute)", extends.join("`/`")),
        )
    } else if let Some(expr) = explicit_default {
        let shown = if expr.is_empty() {
            type_default(&ty)
        } else {
            format!("`{expr}`")
        };
        (false, shown)
    } else if name == "children" && !optional_type {
        (false, "nothing".to_string())
    } else if optional_type {
        (false, "`None`".to_string())
    } else {
        (true, String::new())
    };
    Some(Prop {
        name,
        ty,
        required,
        default,
        doc: doc.join(" ").trim().to_string(),
    })
}

/// The comma-separated arguments of every `#[props(...)]` attribute.
fn props_args(attrs: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = attrs;
    while let Some(at) = rest.find("#[props(") {
        let inner = &rest[at + "#[props(".len()..];
        // The attribute ends at the `)]` that closes it; arguments may hold parentheses.
        let mut depth = 1i32;
        let mut end = inner.len();
        for (k, c) in inner.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = k;
                        break;
                    }
                }
                _ => {}
            }
        }
        let mut depth = 0i32;
        let mut arg = String::new();
        for c in inner[..end].chars() {
            match c {
                '(' | '[' | '{' | '<' => depth += 1,
                ')' | ']' | '}' | '>' => depth -= 1,
                ',' if depth == 0 => {
                    out.push(std::mem::take(&mut arg).trim().to_string());
                    continue;
                }
                _ => {}
            }
            arg.push(c);
        }
        if !arg.trim().is_empty() {
            out.push(arg.trim().to_string());
        }
        rest = &inner[end..];
    }
    out
}

/// `Option<T>` or `ReadSignal<Option<T>>` (also `ReadOnlySignal`): optional in Dioxus props.
fn is_optional_type(ty: &str) -> bool {
    let ty = ty.replace(' ', "");
    let ty = ty
        .strip_prefix("std::option::")
        .or_else(|| ty.strip_prefix("core::option::"))
        .unwrap_or(&ty);
    if ty.starts_with("Option<") {
        return true;
    }
    ["ReadSignal<", "ReadOnlySignal<"].iter().any(|p| {
        ty.strip_prefix(p)
            .is_some_and(|inner| inner.starts_with("Option<"))
    })
}

/// What `#[props(default)]` gives for common types.
fn type_default(ty: &str) -> String {
    let inner = ty
        .strip_prefix("ReadSignal<")
        .or_else(|| ty.strip_prefix("ReadOnlySignal<"))
        .and_then(|t| t.strip_suffix('>'))
        .unwrap_or(ty)
        .trim();
    match inner {
        "bool" => "`false`".into(),
        "String" => "`\"\"`".into(),
        "u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32" | "i64" | "isize" => {
            "`0`".into()
        }
        "f32" | "f64" => "`0.0`".into(),
        t if t.starts_with("Option<") => "`None`".into(),
        t if t.starts_with("Vec<") => "empty".into(),
        t if t.starts_with("Callback") || t.starts_with("EventHandler") => "does nothing".into(),
        t if t.contains('<') => "`Default::default()`".into(),
        _ => format!("`{inner}::default()`"),
    }
}

/// The `#[default]` variant of `pub enum <name>` in `source`.
fn default_variant(source: &str, name: &str) -> Option<String> {
    let mut lines = source.lines().skip_while(|l| {
        l.trim_start()
            .strip_prefix("pub enum ")
            .is_none_or(|rest| rest.trim_end_matches(['{', ' ']) != name)
    });
    lines.next()?;
    let mut marked = false;
    for line in lines {
        let t = line.trim();
        if t.starts_with('}') {
            return None;
        }
        if t == "#[default]" {
            marked = true;
        } else if marked && !t.starts_with("///") && !t.starts_with("#[") {
            let variant: String = t
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            return (!variant.is_empty()).then_some(variant);
        }
    }
    None
}

/// Table cells can't hold `|` or line breaks.
fn cell(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}

/// The "Properties" section for one component's source files, or an empty string when none
/// of them has a public component.
pub fn markdown(sources: &[String]) -> String {
    let mut all: Vec<Component> = sources.iter().flat_map(|s| components(s)).collect();
    // `Variant::default()` reads better as the variant itself when the enum is in these files.
    for c in &mut all {
        if let Props::Listed(props) = &mut c.props {
            for p in props {
                let Some(ty) = p
                    .default
                    .strip_prefix('`')
                    .and_then(|d| d.strip_suffix("::default()`"))
                else {
                    continue;
                };
                if let Some(variant) = sources.iter().find_map(|s| default_variant(s, ty)) {
                    p.default = format!("`{ty}::{variant}`");
                }
            }
        }
    }
    if all.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "## Properties\n\nRead from the component's source by `atoll registry`. A required prop \
         left out is a compile error. Props marked `ReadSignal` take a plain value or a signal.\n",
    );
    for c in &all {
        out.push_str(&format!("\n### `{}`\n\n", c.name));
        match &c.props {
            Props::External(ty) => out.push_str(&format!(
                "Takes `{ty}`, the props of the `atoll-primitives` component it styles; its fields \
                 are in that crate's API docs (`cargo doc -p atoll-primitives --open`).\n"
            )),
            Props::Listed(props) if props.is_empty() => out.push_str("No props.\n"),
            Props::Listed(props) => {
                out.push_str("| Prop | Type | Default | Description |\n|---|---|---|---|\n");
                for p in props {
                    let default = if p.required {
                        "**required**".to_string()
                    } else {
                        p.default.clone()
                    };
                    out.push_str(&format!(
                        "| `{}` | `{}` | {} | {} |\n",
                        p.name,
                        cell(&p.ty),
                        cell(&default),
                        cell(&p.doc)
                    ));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn listed(c: &Component) -> &[Prop] {
        match &c.props {
            Props::Listed(p) => p,
            Props::External(t) => panic!("external {t}"),
        }
    }

    #[test]
    fn reads_parameters() {
        let src = "#[component]\npub fn Button(\n    /// How it looks.\n    #[props(default)] variant: ButtonVariant,\n    #[props(extends=GlobalAttributes)]\n    #[props(extends=button)]\n    attributes: Vec<Attribute>,\n    onclick: Option<EventHandler<MouseEvent>>,\n    on_done: EventHandler<()>,\n    children: Element,\n) -> Element {\n    rsx! {}\n}\n";
        let c = &components(src)[0];
        assert_eq!(c.name, "Button");
        let p = listed(c);
        assert_eq!(p.len(), 5);
        assert_eq!(p[0].name, "variant");
        assert_eq!(p[0].doc, "How it looks.");
        assert_eq!(p[0].default, "`ButtonVariant::default()`");
        let enum_src = format!("pub enum ButtonVariant {{\n    /// Filled.\n    #[default]\n    Primary,\n    Ghost,\n}}\n{src}");
        assert!(markdown(&[enum_src])
            .contains("| `variant` | `ButtonVariant` | `ButtonVariant::Primary` |"));
        assert!(!p[1].required);
        assert!(p[1].default.contains("GlobalAttributes"));
        assert_eq!(p[2].default, "`None`");
        assert!(p[3].required, "an event handler without Option is required");
        assert!(!p[4].required, "children default to nothing");
    }

    #[test]
    fn reads_a_props_struct_in_the_same_file() {
        let src = "#[derive(Props, Clone, PartialEq)]\npub struct DialogProps {\n    /// The heading.\n    #[props(into)]\n    pub title: String,\n\n    #[props(default, into)]\n    pub description: Option<String>,\n\n    #[props(default = ReadSignal::new(Signal::new(true)))]\n    pub is_modal: ReadSignal<bool>,\n\n    pub open: ReadSignal<Option<bool>>,\n    #[props(default)]\n    pub hide: bool,\n}\n\n#[component]\npub fn Dialog(props: DialogProps) -> Element {\n    rsx! {}\n}\n";
        let p = listed(&components(src)[0]).to_vec_names();
        assert_eq!(p, ["title", "description", "is_modal", "open", "hide"]);
        let c = &components(src)[0];
        let p = listed(c);
        assert!(p[0].required);
        assert_eq!(p[1].default, "`None`");
        assert_eq!(p[2].default, "`ReadSignal::new(Signal::new(true))`");
        assert!(!p[3].required);
        assert_eq!(p[4].default, "`false`");
    }

    #[test]
    fn names_an_external_props_struct() {
        let src = "#[component]\npub fn Accordion(props: AccordionProps) -> Element {\n    rsx! {}\n}\n#[component]\npub(crate) fn Hidden(x: u8) -> Element { rsx! {} }\n";
        let all = components(src);
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].props, Props::External("AccordionProps".into()));
    }

    #[test]
    fn one_line_signature_and_not_optional() {
        let src = "#[component]\npub fn Kbd(#[props(!optional)] key: Option<String>, children: Element) -> Element {\n    rsx! {}\n}\n";
        let p = listed(&components(src)[0]).to_vec_names();
        assert_eq!(p, ["key", "children"]);
        assert!(listed(&components(src)[0])[0].required);
    }

    trait Names {
        fn to_vec_names(&self) -> Vec<String>;
    }
    impl Names for [Prop] {
        fn to_vec_names(&self) -> Vec<String> {
            self.iter().map(|p| p.name.clone()).collect()
        }
    }
}
