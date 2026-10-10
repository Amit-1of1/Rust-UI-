//! A fast ARIA check that runs in `cargo test`, without a browser.
//!
//! Every demo is rendered on the server and the HTML is checked for mistakes that
//! break assistive technology but still compile: references to ids that don't exist,
//! duplicate ids, misspelled `aria-*` attributes or roles, controls with no accessible
//! name, and widget roles missing their required state. axe in `scripts/check.sh --a11y`
//! remains the real gate (it sees CSS, colour and the live DOM); this catches the
//! structural half in milliseconds, on every `cargo test`.

use std::collections::{HashMap, HashSet};

use common::{render, DEMOS};

mod common;

// ---------------------------------------------------------------------------------------
// A tiny HTML tree builder. SSR output is machine-written and regular, so this does not
// need to handle the whole HTML spec: tags, attributes (quoted or bare), text, void tags.
// ---------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Node {
    tag: String,
    attrs: Vec<(String, String)>,
    children: Vec<usize>,
    text: String,
    parent: Option<usize>,
}

impl Node {
    fn attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.as_str())
    }
}

const VOID: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

fn parse(html: &str) -> Vec<Node> {
    let mut nodes = vec![Node {
        tag: "#root".into(),
        ..Default::default()
    }];
    let mut stack = vec![0usize];
    let b = html.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'<' {
            if html[i..].starts_with("<!--") {
                i = html[i..].find("-->").map_or(b.len(), |e| i + e + 3);
                continue;
            }
            if b.get(i + 1) == Some(&b'/') {
                let end = html[i..].find('>').map_or(b.len(), |e| i + e);
                let name = html[i + 2..end].trim().to_ascii_lowercase();
                if let Some(pos) = stack.iter().rposition(|&n| nodes[n].tag == name) {
                    stack.truncate(pos.max(1));
                }
                i = end + 1;
                continue;
            }
            // Open tag.
            let mut j = i + 1;
            while j < b.len() && !b[j].is_ascii_whitespace() && b[j] != b'>' && b[j] != b'/' {
                j += 1;
            }
            let tag = html[i + 1..j].to_ascii_lowercase();
            let mut attrs = Vec::new();
            let mut self_closing = false;
            loop {
                while j < b.len() && b[j].is_ascii_whitespace() {
                    j += 1;
                }
                if j >= b.len() {
                    break;
                }
                if b[j] == b'>' {
                    j += 1;
                    break;
                }
                if b[j] == b'/' {
                    self_closing = true;
                    j += 1;
                    continue;
                }
                let start = j;
                while j < b.len() && !b"= />".contains(&b[j]) && !b[j].is_ascii_whitespace() {
                    j += 1;
                }
                let name = html[start..j].to_ascii_lowercase();
                let mut value = String::new();
                if j < b.len() && b[j] == b'=' {
                    j += 1;
                    if j < b.len() && (b[j] == b'"' || b[j] == b'\'') {
                        let q = b[j];
                        let s = j + 1;
                        j = s;
                        while j < b.len() && b[j] != q {
                            j += 1;
                        }
                        value = html[s..j].to_string();
                        j += 1;
                    } else {
                        let s = j;
                        while j < b.len() && !b[j].is_ascii_whitespace() && b[j] != b'>' {
                            j += 1;
                        }
                        value = html[s..j].to_string();
                    }
                }
                attrs.push((name, value));
            }
            let parent = *stack.last().unwrap_or(&0);
            let id = nodes.len();
            nodes.push(Node {
                tag: tag.clone(),
                attrs,
                parent: Some(parent),
                ..Default::default()
            });
            nodes[parent].children.push(id);
            if !self_closing && !VOID.contains(&tag.as_str()) {
                stack.push(id);
            }
            i = j;
        } else {
            let end = html[i..].find('<').map_or(b.len(), |e| i + e);
            let top = *stack.last().unwrap_or(&0);
            nodes[top].text.push_str(&html[i..end]);
            nodes[top].text.push(' ');
            i = end;
        }
    }
    nodes
}

// ---------------------------------------------------------------------------------------
// ARIA 1.2 vocabulary (from the WAI-ARIA 1.2 recommendation, W3C).
// ---------------------------------------------------------------------------------------

const ARIA_ATTRS: &[&str] = &[
    "aria-activedescendant",
    "aria-atomic",
    "aria-autocomplete",
    "aria-braillelabel",
    "aria-brailleroledescription",
    "aria-busy",
    "aria-checked",
    "aria-colcount",
    "aria-colindex",
    "aria-colindextext",
    "aria-colspan",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-description",
    "aria-details",
    "aria-disabled",
    "aria-dropeffect",
    "aria-errormessage",
    "aria-expanded",
    "aria-flowto",
    "aria-grabbed",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-level",
    "aria-live",
    "aria-modal",
    "aria-multiline",
    "aria-multiselectable",
    "aria-orientation",
    "aria-owns",
    "aria-placeholder",
    "aria-posinset",
    "aria-pressed",
    "aria-readonly",
    "aria-relevant",
    "aria-required",
    "aria-roledescription",
    "aria-rowcount",
    "aria-rowindex",
    "aria-rowindextext",
    "aria-rowspan",
    "aria-selected",
    "aria-setsize",
    "aria-sort",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "aria-valuetext",
];

const ROLES: &[&str] = &[
    "alert",
    "alertdialog",
    "application",
    "article",
    "banner",
    "blockquote",
    "button",
    "caption",
    "cell",
    "checkbox",
    "code",
    "columnheader",
    "combobox",
    "complementary",
    "contentinfo",
    "definition",
    "deletion",
    "dialog",
    "directory",
    "document",
    "emphasis",
    "feed",
    "figure",
    "form",
    "generic",
    "grid",
    "gridcell",
    "group",
    "heading",
    "img",
    "insertion",
    "link",
    "list",
    "listbox",
    "listitem",
    "log",
    "main",
    "marquee",
    "math",
    "menu",
    "menubar",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "meter",
    "navigation",
    "none",
    "note",
    "option",
    "paragraph",
    "presentation",
    "progressbar",
    "radio",
    "radiogroup",
    "region",
    "row",
    "rowgroup",
    "rowheader",
    "scrollbar",
    "search",
    "searchbox",
    "separator",
    "slider",
    "spinbutton",
    "status",
    "strong",
    "subscript",
    "superscript",
    "switch",
    "tab",
    "table",
    "tablist",
    "tabpanel",
    "term",
    "textbox",
    "time",
    "timer",
    "toolbar",
    "tooltip",
    "tree",
    "treegrid",
    "treeitem",
];

/// Attributes whose value is a space-separated list of ids in the same document.
const ID_REFS: &[&str] = &[
    "aria-labelledby",
    "aria-describedby",
    "aria-controls",
    "aria-owns",
    "aria-flowto",
    "aria-activedescendant",
    "aria-errormessage",
    "aria-details",
];

/// Roles that must have an accessible name (WAI-ARIA 1.2 "name required").
const NAME_REQUIRED: &[&str] = &[
    "button",
    "checkbox",
    "combobox",
    "dialog",
    "alertdialog",
    "img",
    "link",
    "listbox",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "meter",
    "progressbar",
    "radio",
    "radiogroup",
    "searchbox",
    "slider",
    "spinbutton",
    "switch",
    "tab",
    "tabpanel",
    "textbox",
    "tree",
    "treeitem",
];

/// Roles and the state they must carry (WAI-ARIA 1.2 "required states and properties").
const REQUIRED_STATE: &[(&str, &str)] = &[
    ("checkbox", "aria-checked"),
    ("switch", "aria-checked"),
    ("menuitemcheckbox", "aria-checked"),
    ("menuitemradio", "aria-checked"),
    ("radio", "aria-checked"),
    ("combobox", "aria-expanded"),
    ("slider", "aria-valuenow"),
    ("scrollbar", "aria-valuenow"),
    ("heading", "aria-level"),
];

/// The role a node exposes: explicit `role`, else the implicit one for common tags.
fn role(n: &Node) -> Option<String> {
    if let Some(r) = n.attr("role") {
        return r.split_whitespace().next().map(str::to_string);
    }
    let implicit = match n.tag.as_str() {
        "button" => "button",
        "a" if n.attr("href").is_some() => "link",
        "img" => "img",
        "select" => "combobox",
        "textarea" => "textbox",
        "input" => match n.attr("type").unwrap_or("text") {
            "checkbox" => "checkbox",
            "radio" => "radio",
            "range" => "slider",
            "button" | "submit" | "reset" => "button",
            "hidden" => return None,
            _ => "textbox",
        },
        _ => return None,
    };
    Some(implicit.into())
}

fn is_hidden(nodes: &[Node], mut i: usize) -> bool {
    loop {
        let n = &nodes[i];
        if n.attr("aria-hidden") == Some("true") || n.attr("hidden").is_some() {
            return true;
        }
        match n.parent {
            Some(p) => i = p,
            None => return false,
        }
    }
}

/// Visible text inside a node, counting `alt` on images and `<title>` in SVG.
fn text_of(nodes: &[Node], i: usize) -> String {
    let n = &nodes[i];
    if n.attr("aria-hidden") == Some("true") {
        return String::new();
    }
    let mut s = n.text.clone();
    if let Some(alt) = n.attr("alt") {
        s.push_str(alt);
    }
    if let Some(label) = n.attr("aria-label") {
        s.push_str(label);
    }
    for &c in &n.children {
        s.push(' ');
        s.push_str(&text_of(nodes, c));
    }
    s
}

fn has_name(
    nodes: &[Node],
    i: usize,
    ids: &HashMap<String, usize>,
    labels: &HashSet<String>,
) -> bool {
    let n = &nodes[i];
    let nonempty = |s: Option<&str>| s.is_some_and(|s| !s.trim().is_empty());
    if nonempty(n.attr("aria-label")) || nonempty(n.attr("title")) {
        return true;
    }
    if let Some(refs) = n.attr("aria-labelledby") {
        if refs.split_whitespace().any(|r| {
            ids.get(r)
                .is_some_and(|&t| !text_of(nodes, t).trim().is_empty())
        }) {
            return true;
        }
    }
    if matches!(n.tag.as_str(), "input" | "select" | "textarea") {
        if n.attr("id").is_some_and(|id| labels.contains(id)) {
            return true;
        }
        // Wrapped in a <label>.
        let mut p = n.parent;
        while let Some(pi) = p {
            if nodes[pi].tag == "label" {
                return true;
            }
            p = nodes[pi].parent;
        }
        // A bare placeholder is a weak name, but browsers and axe accept it.
        return nonempty(n.attr("placeholder"));
    }
    if n.tag == "img" {
        return n.attr("alt").is_some();
    }
    // Roles that take their name from content.
    let from_content = matches!(
        role(n).as_deref(),
        Some(
            "button"
                | "link"
                | "checkbox"
                | "switch"
                | "radio"
                | "tab"
                | "menuitem"
                | "menuitemcheckbox"
                | "menuitemradio"
                | "treeitem"
                | "option"
        )
    );
    from_content && !text_of(nodes, i).trim().is_empty()
}

fn is_focusable(n: &Node) -> bool {
    if n.attr("disabled").is_some_and(|v| v != "false") {
        return false;
    }
    match n.attr("tabindex") {
        Some(t) => t.parse::<i32>().is_ok_and(|t| t >= 0),
        None => {
            matches!(n.tag.as_str(), "button" | "select" | "textarea")
                || (n.tag == "a" && n.attr("href").is_some())
                || (n.tag == "input" && n.attr("type") != Some("hidden"))
        }
    }
}

/// All problems found in one rendered document.
fn check(html: &str) -> Vec<String> {
    let nodes = parse(html);
    let mut problems = Vec::new();
    let mut ids: HashMap<String, usize> = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if let Some(id) = n.attr("id") {
            if ids.insert(id.to_string(), i).is_some() {
                problems.push(format!("duplicate id \"{id}\""));
            }
        }
    }
    let labels: HashSet<String> = nodes
        .iter()
        .filter(|n| n.tag == "label")
        .filter_map(|n| n.attr("for").map(str::to_string))
        .collect();
    for (i, n) in nodes.iter().enumerate() {
        let what = || match n.attr("class") {
            Some(c) => format!("<{} class=\"{c}\">", n.tag),
            None => format!("<{}>", n.tag),
        };
        for (name, value) in &n.attrs {
            if name.starts_with("aria-") && !ARIA_ATTRS.contains(&name.as_str()) {
                problems.push(format!("{}: unknown attribute {name}", what()));
            }
            // Like axe: a collapsed control may point aria-controls at a popup that is not
            // rendered until it opens.
            let collapsed_popup =
                name == "aria-controls" && n.attr("aria-expanded") == Some("false");
            if (ID_REFS.contains(&name.as_str()) || (n.tag == "label" && name == "for"))
                && !collapsed_popup
            {
                for r in value.split_whitespace() {
                    if !ids.contains_key(r) {
                        problems.push(format!("{}: {name} points at missing id \"{r}\"", what()));
                    }
                }
            }
        }
        if let Some(r) = n.attr("role") {
            for r in r.split_whitespace() {
                if !ROLES.contains(&r) {
                    problems.push(format!("{}: unknown role \"{r}\"", what()));
                }
            }
        }
        if is_hidden(&nodes, i) {
            if is_focusable(n) {
                problems.push(format!("{}: focusable inside aria-hidden", what()));
            }
            continue;
        }
        let Some(role) = role(n) else { continue };
        if role == "none" || role == "presentation" {
            continue;
        }
        if NAME_REQUIRED.contains(&role.as_str()) && !has_name(&nodes, i, &ids, &labels) {
            problems.push(format!("{}: role {role} has no accessible name", what()));
        }
        if n.attr("role").is_some() {
            for &(r, state) in REQUIRED_STATE {
                if r == role && n.attr(state).is_none() {
                    problems.push(format!("{}: role {role} is missing {state}", what()));
                }
            }
        }
    }
    problems
}

// ---------------------------------------------------------------------------------------
// The checker checks itself, then every demo.
// ---------------------------------------------------------------------------------------

#[test]
fn checker_catches_the_mistakes_it_claims_to() {
    let bad = r#"<div id="a"></div><div id="a"></div>
        <button aria-labeledby="x"></button>
        <input aria-describedby="nope"/>
        <div role="buton">x</div>
        <div role="switch" aria-label="Wifi"></div>
        <div aria-hidden="true"><button>Hidden</button></div>
        <img src="a.png"/>"#;
    let found = check(bad).join("\n");
    for expected in [
        "duplicate id \"a\"",
        "unknown attribute aria-labeledby",
        "points at missing id \"nope\"",
        "unknown role \"buton\"",
        "role switch is missing aria-checked",
        "focusable inside aria-hidden",
        "<img>: role img has no accessible name",
        "<button>: role button has no accessible name",
        "<input>: role textbox has no accessible name",
    ] {
        assert!(found.contains(expected), "missed {expected:?} in:\n{found}");
    }
    let good = r#"<label for="e">Email</label><input id="e" type="email"/>
        <button aria-label="Close"><svg aria-hidden="true"></svg></button>
        <div role="switch" aria-checked=false>Wifi</div>
        <h2 id="t">Title</h2><div role="dialog" aria-labelledby="t"></div>
        <img src="a.png" alt=""/>"#;
    assert_eq!(check(good), Vec::<String>::new());
}

/// Problems found on the first run, kept visible instead of silenced. Each entry is a
/// substring of the reported problem and why it is not fixed yet. The test fails if one
/// of these goes away, so the list can't go stale.
const KNOWN: &[(&str, &str)] = &[
    (
        "date_picker: <span class=\"dx-date-segment",
        "dioxus-primitives: DateSegment sets aria-labelledby=\"{id}-label\" but never renders \
         that element. Browsers fall back to its aria-label (\"day\" etc.), so the name \
         still works, but the reference is invalid. Report upstream.",
    ),
    (
        "sidebar: <button class=\"dx-tooltip-trigger",
        "Tooltip trigger points aria-describedby at tooltip content that only renders \
         while open. Primitive behaviour; check with a screen reader.",
    ),
];

#[test]
fn every_demo_passes_the_aria_check() {
    let mut found = Vec::new();
    for (name, demo) in DEMOS {
        for p in check(&render(*demo)) {
            found.push(format!("{name}: {p}"));
        }
    }
    let new: Vec<&String> = found
        .iter()
        .filter(|p| !KNOWN.iter().any(|(k, _)| p.contains(k)))
        .collect();
    let fixed: Vec<&str> = KNOWN
        .iter()
        .filter(|(k, _)| !found.iter().any(|p| p.contains(k)))
        .map(|(k, _)| *k)
        .collect();
    assert!(
        new.is_empty(),
        "{} new ARIA problems:\n{}",
        new.len(),
        new.iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(fixed.is_empty(), "fixed, remove from KNOWN: {fixed:?}");
}

#[test]
fn every_demo_file_is_checked() {
    common::assert_demos_complete();
}
