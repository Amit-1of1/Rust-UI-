//! Finding elements the way a user would: by role and accessible name, by visible text, by
//! label, by placeholder, by `data-testid`, or by a CSS selector when nothing else fits.
//!
//! Like Testing Library, the role, text, label and placeholder queries skip elements a user
//! cannot perceive (`display: none`, `visibility: hidden`, `hidden`, inside `aria-hidden`);
//! call [`Query::hidden`] to include them. CSS and test-id queries match everything.

use blitz_dom::{BaseDocument, LocalName, Node};
use std::fmt;

/// What to look for. Build one with [`role`], [`text`], [`label`], [`placeholder`],
/// [`test_id`] or [`css`] (a plain `&str` is a CSS selector too), then narrow it with
/// [`name`](Query::name), [`nth`](Query::nth) or [`within`](Query::within).
#[derive(Clone, Debug)]
pub struct Query {
    kind: Kind,
    nth: Option<usize>,
    within: Option<Box<Query>>,
    include_hidden: bool,
}

#[derive(Clone, Debug)]
enum Kind {
    Css(String),
    Role { role: String, name: Option<Match> },
    Text(Match),
    Label(Match),
    Placeholder(Match),
    TestId(String),
    Node(usize),
}

/// How a name or text is compared: whitespace runs count as one space and the ends are
/// trimmed on both sides; `Exact` is case-sensitive, `Contains` is a substring.
#[derive(Clone, Debug)]
enum Match {
    Exact(String),
    Contains(String),
}

impl Match {
    fn test(&self, got: &str) -> bool {
        let got = normalize(got);
        match self {
            Match::Exact(want) => got == normalize(want),
            Match::Contains(want) => got.contains(&normalize(want)),
        }
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Match::Exact(s) => write!(f, "{s:?}"),
            Match::Contains(s) => write!(f, "containing {s:?}"),
        }
    }
}

/// Elements with this ARIA role (explicit `role="..."` or the one the tag implies:
/// `button`, `link`, `textbox`, `checkbox`, `heading`, `dialog`, ...). Narrow it with
/// [`Query::name`].
pub fn role(role: &str) -> Query {
    Query::new(Kind::Role {
        role: role.to_string(),
        name: None,
    })
}

/// The innermost elements whose visible text is exactly `text` (whitespace collapsed).
pub fn text(text: &str) -> Query {
    Query::new(Kind::Text(Match::Exact(text.to_string())))
}

/// The innermost elements whose text contains `part`.
pub fn text_containing(part: &str) -> Query {
    Query::new(Kind::Text(Match::Contains(part.to_string())))
}

/// The form control labelled `label`: by `<label for>`, a wrapping `<label>`,
/// `aria-labelledby` or `aria-label`.
pub fn label(label: &str) -> Query {
    Query::new(Kind::Label(Match::Exact(label.to_string())))
}

/// Controls whose `placeholder` is `placeholder`.
pub fn placeholder(placeholder: &str) -> Query {
    Query::new(Kind::Placeholder(Match::Exact(placeholder.to_string())))
}

/// The element with `data-testid="<id>"`.
pub fn test_id(id: &str) -> Query {
    Query::new(Kind::TestId(id.to_string()))
}

/// Elements matching a CSS selector. A plain `&str` passed to any `Screen` method means the
/// same thing.
pub fn css(selector: &str) -> Query {
    Query::new(Kind::Css(selector.to_string()))
}

impl Query {
    fn new(kind: Kind) -> Self {
        Self {
            kind,
            nth: None,
            within: None,
            include_hidden: false,
        }
    }

    pub(crate) fn node(id: usize) -> Self {
        Self::new(Kind::Node(id))
    }

    /// Only elements whose accessible name is exactly `name` (role queries).
    pub fn name(mut self, name: &str) -> Self {
        if let Kind::Role { name: n, .. } = &mut self.kind {
            *n = Some(Match::Exact(name.to_string()));
        } else {
            panic!("atoll-test: .name() only applies to role(...) queries");
        }
        self
    }

    /// Only elements whose accessible name contains `part` (role queries).
    pub fn name_containing(mut self, part: &str) -> Self {
        if let Kind::Role { name: n, .. } = &mut self.kind {
            *n = Some(Match::Contains(part.to_string()));
        } else {
            panic!("atoll-test: .name_containing() only applies to role(...) queries");
        }
        self
    }

    /// The `n`th match (from 0), for when several elements match on purpose.
    pub fn nth(mut self, n: usize) -> Self {
        self.nth = Some(n);
        self
    }

    /// The first match.
    pub fn first(self) -> Self {
        self.nth(0)
    }

    /// Only look inside the (single) element `outer` finds.
    pub fn within(mut self, outer: impl Into<Query>) -> Self {
        self.within = Some(Box::new(outer.into()));
        self
    }

    /// Also match elements a user cannot see or a screen reader would skip.
    pub fn hidden(mut self) -> Self {
        self.include_hidden = true;
        self
    }

    /// Every matching element, in document order.
    pub(crate) fn all(&self, doc: &BaseDocument) -> Result<Vec<usize>, String> {
        let scope = match &self.within {
            Some(outer) => Some(outer.one(doc)?),
            None => None,
        };
        let root = scope.unwrap_or_else(|| doc.root_node().id);
        let mut found = Vec::new();
        match &self.kind {
            Kind::Node(id) => {
                if doc.get_node(*id).is_some() {
                    found.push(*id);
                }
            }
            Kind::Css(sel) => {
                let ids = doc
                    .query_selector_all(sel)
                    .map_err(|_| format!("invalid CSS selector {sel:?}"))?;
                found.extend(ids.into_iter().filter(|&id| is_inside(doc, root, id)));
            }
            Kind::TestId(id) => {
                walk(doc, root, &mut |n| {
                    if n.attr(LocalName::from("data-testid")) == Some(id.as_str()) {
                        found.push(n.id);
                    }
                });
            }
            Kind::Role { role: want, name } => {
                walk(doc, root, &mut |n| {
                    if role_of(doc, n).as_deref() == Some(want.as_str())
                        && (self.include_hidden || !is_hidden(doc, n))
                        && name
                            .as_ref()
                            .is_none_or(|m| m.test(&accessible_name(doc, n)))
                    {
                        found.push(n.id);
                    }
                });
            }
            Kind::Text(m) => {
                walk(doc, root, &mut |n| {
                    let tag = tag(n);
                    if matches!(tag, "script" | "style" | "head" | "title" | "html") {
                        return;
                    }
                    if !m.test(&n.text_content()) {
                        return;
                    }
                    // Innermost only: skip an element when one of its element children
                    // already holds the whole match.
                    let child_matches = n.children.iter().any(|&c| {
                        doc.get_node(c)
                            .is_some_and(|c| c.is_element() && m.test(&c.text_content()))
                    });
                    if !child_matches && (self.include_hidden || !is_hidden(doc, n)) {
                        found.push(n.id);
                    }
                });
            }
            Kind::Label(m) => {
                walk(doc, root, &mut |n| {
                    if !is_labelable(n) && n.attr(LocalName::from("aria-label")).is_none() {
                        return;
                    }
                    if (self.include_hidden || !is_hidden(doc, n))
                        && labels_of(doc, n).iter().any(|l| m.test(l))
                    {
                        found.push(n.id);
                    }
                });
            }
            Kind::Placeholder(m) => {
                walk(doc, root, &mut |n| {
                    if n.attr(LocalName::from("placeholder"))
                        .is_some_and(|p| m.test(p))
                        && (self.include_hidden || !is_hidden(doc, n))
                    {
                        found.push(n.id);
                    }
                });
            }
        }
        if let Some(i) = self.nth {
            return Ok(found.get(i).copied().into_iter().collect());
        }
        Ok(found)
    }

    /// Exactly one match, or an error that says what was found instead.
    pub(crate) fn one(&self, doc: &BaseDocument) -> Result<usize, String> {
        let all = self.all(doc)?;
        match all.as_slice() {
            [id] => Ok(*id),
            [] => Err(format!(
                "no element matches {self}.\n{}",
                accessible_summary(doc)
            )),
            many => {
                let list: Vec<String> = many.iter().take(8).map(|&id| describe(doc, id)).collect();
                Err(format!(
                    "{} elements match {self}; narrow it (.name(..), .within(..), .nth(..)):\n  {}",
                    many.len(),
                    list.join("\n  ")
                ))
            }
        }
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            Kind::Css(s) => write!(f, "css({s:?})")?,
            Kind::Role { role, name: None } => write!(f, "role({role:?})")?,
            Kind::Role {
                role,
                name: Some(n),
            } => write!(f, "role({role:?}) named {n}")?,
            Kind::Text(m) => write!(f, "text {m}")?,
            Kind::Label(m) => write!(f, "label {m}")?,
            Kind::Placeholder(m) => write!(f, "placeholder {m}")?,
            Kind::TestId(id) => write!(f, "test_id({id:?})")?,
            Kind::Node(id) => write!(f, "element #{id}")?,
        }
        if let Some(n) = self.nth {
            write!(f, " (match {n})")?;
        }
        if let Some(w) = &self.within {
            write!(f, " within {w}")?;
        }
        Ok(())
    }
}

impl From<&str> for Query {
    fn from(selector: &str) -> Self {
        css(selector)
    }
}

impl From<&String> for Query {
    fn from(selector: &String) -> Self {
        css(selector)
    }
}

impl From<String> for Query {
    fn from(selector: String) -> Self {
        css(&selector)
    }
}

impl From<&Query> for Query {
    fn from(q: &Query) -> Self {
        q.clone()
    }
}

/// Collapse whitespace runs to one space and trim.
pub(crate) fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn tag(n: &Node) -> &str {
    n.element_data().map_or("", |e| &*e.name.local)
}

fn attr<'a>(n: &'a Node, name: &str) -> Option<&'a str> {
    n.attr(LocalName::from(name))
}

/// Every element under `root` (itself included), in document order.
fn walk(doc: &BaseDocument, root: usize, f: &mut dyn FnMut(&Node)) {
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let Some(n) = doc.get_node(id) else { continue };
        if n.is_element() {
            f(n);
        }
        stack.extend(n.children.iter().rev().copied());
    }
}

fn is_inside(doc: &BaseDocument, root: usize, mut id: usize) -> bool {
    loop {
        if id == root {
            return true;
        }
        match doc.get_node(id).and_then(|n| n.parent) {
            Some(p) => id = p,
            None => return false,
        }
    }
}

/// Hidden from a user or a screen reader: `hidden`, `aria-hidden="true"`, `display: none` or
/// `visibility: hidden` on it or an ancestor.
pub(crate) fn is_hidden(doc: &BaseDocument, n: &Node) -> bool {
    if let Some(s) = n.primary_styles() {
        use style::computed_values::visibility::T as Visibility;
        if s.get_inherited_box().visibility != Visibility::Visible {
            return true;
        }
    }
    let mut cur = Some(n);
    while let Some(n) = cur {
        if n.is_element() {
            if attr(n, "hidden").is_some() || attr(n, "aria-hidden") == Some("true") {
                return true;
            }
            if n.primary_styles()
                .is_some_and(|s| s.clone_display().is_none())
            {
                return true;
            }
        }
        cur = n.parent.and_then(|p| doc.get_node(p));
    }
    false
}

/// The element's role: its own `role` attribute (first token), else the one its tag implies.
pub(crate) fn role_of(doc: &BaseDocument, n: &Node) -> Option<String> {
    if let Some(r) = attr(n, "role").and_then(|r| r.split_whitespace().next()) {
        return Some(r.to_string());
    }
    let implied = match tag(n) {
        "button" => "button",
        "a" if attr(n, "href").is_some() => "link",
        "input" => match attr(n, "type").unwrap_or("text") {
            "checkbox" => "checkbox",
            "radio" => "radio",
            "range" => "slider",
            "number" => "spinbutton",
            "search" => "searchbox",
            "button" | "submit" | "reset" | "image" => "button",
            "hidden" | "file" | "color" | "date" | "time" | "datetime-local" | "month" | "week"
            | "password" => return None,
            _ => "textbox",
        },
        "textarea" => "textbox",
        "select" => {
            if attr(n, "multiple").is_some() {
                "listbox"
            } else {
                "combobox"
            }
        }
        "option" => "option",
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => "heading",
        "img" if attr(n, "alt") != Some("") => "img",
        "ul" | "ol" | "menu" => "list",
        "li" => "listitem",
        "nav" => "navigation",
        "main" => "main",
        "dialog" => "dialog",
        "form" => "form",
        "table" => "table",
        "tr" => "row",
        "td" => "cell",
        "th" => "columnheader",
        "hr" => "separator",
        "progress" => "progressbar",
        "meter" => "meter",
        "aside" => "complementary",
        "article" => "article",
        "fieldset" => "group",
        "details" => "group",
        "summary" => "button",
        "output" => "status",
        "header" if !inside_sectioning(doc, n) => "banner",
        "footer" if !inside_sectioning(doc, n) => "contentinfo",
        _ => return None,
    };
    Some(implied.to_string())
}

fn inside_sectioning(doc: &BaseDocument, n: &Node) -> bool {
    let mut cur = n.parent.and_then(|p| doc.get_node(p));
    while let Some(p) = cur {
        if matches!(tag(p), "article" | "aside" | "main" | "nav" | "section") {
            return true;
        }
        cur = p.parent.and_then(|q| doc.get_node(q));
    }
    false
}

fn is_labelable(n: &Node) -> bool {
    matches!(
        tag(n),
        "input" | "textarea" | "select" | "button" | "meter" | "output" | "progress"
    ) || attr(n, "role").is_some()
}

fn text_of_ids(doc: &BaseDocument, ids: &str) -> String {
    let parts: Vec<String> = ids
        .split_whitespace()
        .filter_map(|id| doc.get_element_by_id(id))
        .filter_map(|id| doc.get_node(id))
        .map(|n| normalize(&n.text_content()))
        .collect();
    parts.join(" ")
}

/// Every label text that names a control: `aria-labelledby`, `aria-label`, `<label for>` and
/// a wrapping `<label>`.
fn labels_of(doc: &BaseDocument, n: &Node) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(ids) = attr(n, "aria-labelledby") {
        out.push(text_of_ids(doc, ids));
    }
    if let Some(l) = attr(n, "aria-label") {
        out.push(l.to_string());
    }
    if let Some(id) = attr(n, "id").filter(|id| !id.is_empty()) {
        walk(doc, doc.root_node().id, &mut |l| {
            if tag(l) == "label" && attr(l, "for") == Some(id) {
                out.push(l.text_content());
            }
        });
    }
    let mut cur = n.parent.and_then(|p| doc.get_node(p));
    while let Some(p) = cur {
        if tag(p) == "label" {
            out.push(p.text_content());
            break;
        }
        cur = p.parent.and_then(|q| doc.get_node(q));
    }
    out
}

/// A simplified accessible name: `aria-labelledby`, `aria-label`, a label (form controls),
/// `alt` (images), the text inside (buttons, links, headings, options, tabs, ...), then
/// `title` and `placeholder`.
pub(crate) fn accessible_name(doc: &BaseDocument, n: &Node) -> String {
    if let Some(ids) = attr(n, "aria-labelledby") {
        let t = text_of_ids(doc, ids);
        if !t.is_empty() {
            return t;
        }
    }
    if let Some(l) = attr(n, "aria-label").filter(|l| !l.trim().is_empty()) {
        return normalize(l);
    }
    let t = tag(n);
    if matches!(t, "input" | "textarea" | "select") {
        if let Some(l) = labels_of(doc, n).into_iter().find(|l| !l.trim().is_empty()) {
            return normalize(&l);
        }
        if matches!(attr(n, "type"), Some("submit" | "button" | "reset")) {
            if let Some(v) = attr(n, "value") {
                return normalize(v);
            }
        }
    }
    if t == "img" {
        if let Some(alt) = attr(n, "alt") {
            return normalize(alt);
        }
    }
    let from_content = matches!(
        role_of(doc, n).as_deref(),
        Some(
            "button"
                | "link"
                | "heading"
                | "option"
                | "tab"
                | "menuitem"
                | "menuitemcheckbox"
                | "menuitemradio"
                | "checkbox"
                | "radio"
                | "switch"
                | "cell"
                | "columnheader"
                | "rowheader"
                | "row"
                | "listitem"
                | "treeitem"
                | "tooltip"
                | "status"
        )
    );
    if from_content {
        let text = normalize(&n.text_content());
        if !text.is_empty() {
            return text;
        }
    }
    attr(n, "title")
        .or_else(|| attr(n, "placeholder"))
        .map(normalize)
        .unwrap_or_default()
}

/// One line about an element, for failure messages.
pub(crate) fn describe(doc: &BaseDocument, id: usize) -> String {
    let Some(n) = doc.get_node(id) else {
        return format!("#{id} (gone)");
    };
    let mut s = format!("<{}", tag(n));
    for a in ["id", "role", "type", "data-testid"] {
        if let Some(v) = attr(n, a) {
            s.push_str(&format!(" {a}={v:?}"));
        }
    }
    s.push('>');
    let name = accessible_name(doc, n);
    let text = if name.is_empty() {
        normalize(&n.text_content())
    } else {
        name
    };
    if !text.is_empty() {
        let short: String = text.chars().take(50).collect();
        s.push_str(&format!(" {short:?}"));
    }
    s
}

/// What a user could find on the screen, for "no element matches" messages.
fn accessible_summary(doc: &BaseDocument) -> String {
    let mut lines = Vec::new();
    walk(doc, doc.root_node().id, &mut |n| {
        if lines.len() >= 40 || is_hidden(doc, n) {
            return;
        }
        if let Some(r) = role_of(doc, n) {
            if matches!(r.as_str(), "generic" | "presentation" | "none" | "main") {
                return;
            }
            lines.push(format!("  {r} {:?}", accessible_name(doc, n)));
        }
    });
    if lines.is_empty() {
        "The screen has no elements with a role.".into()
    } else {
        format!("On the screen (role, name):\n{}", lines.join("\n"))
    }
}
