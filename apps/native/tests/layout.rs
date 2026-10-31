//! WCAG 2.2 layout checks over every demo, on the headless native renderer (no browser).
//!
//! - **Target size** (SC 2.5.8): every pointer target is at least 24x24 CSS px, or is spaced
//!   so a 24 px circle around it touches no other target.
//! - **Focus not obscured** (SC 2.4.11): Tab through the demo; each focused element, once
//!   scrolled into view the way a browser does, is not entirely covered by something else
//!   (a fixed bar, a toast, an overlay).
//! - **Reflow** (SC 1.4.10): at 320 CSS px wide nothing makes the page scroll sideways.
//! - **Text spacing** (SC 1.4.12): with the WCAG spacing overrides nothing new overflows the
//!   page or gets clipped.
//! - **Longer text**: every text node and placeholder pseudo-localised, 40% longer with accents
//!   (as German or Finnish runs), with no new page overflow or clipped text.
//! - **Right to left**: the demo inside `dir="rtl"` (logical properties flip), with no page
//!   overflow and nothing clipped that isn't clipped left to right.
//!
//! Findings that are accepted for now sit in `KNOWN` with a reason; a test fails on a new
//! finding and on a `KNOWN` entry that no longer happens, so the list stays true.
//! `LAYOUT_REPORT=1 cargo test -p native --test layout -- --nocapture` prints every finding,
//! known ones included.
//!
//! Blitz is not a browser (docs/NATIVE.md): it has no `position: sticky` (laid out as
//! `relative`) and Taffy has no right-to-left flex or grid order, so the RTL run checks
//! logical properties and overflow, not mirrored item order. docs/notes/layout-checks.md
//! says how findings were cross-checked in Chromium and which ones are Blitz's own.

mod common;
use common::*;

use blitz_dom::{LocalName, Node};
use blitz_traits::shell::ColorScheme;
use dioxus::prelude::*;
use std::cell::Cell;
use std::collections::BTreeSet;

/// Blitz lays text inputs out 300px wide (browsers: 20 characters).
const INPUT_WIDTH: &str = "Blitz lays text inputs and textareas out 300px wide (browsers: 20 characters), which holds narrow columns open; Chromium shows no overflow here";
/// Blitz ignores the size of inline-flex and absolute boxes inside a flex item.
const INLINE_BOX: &str = "Blitz ignores the width and height of an inline-flex or absolute box inside a flex item (pagination links shrink to their digits; the link's screen-reader-only text gets a full-size box)";
/// Blitz has no `::after` hit area and no painting order for positioned elements.
const SIDEBAR_ACTION: &str = "the menu action's ::after hit area is 36x36 in browsers; Blitz builds no box for it and paints positioned elements in DOM order, so the menu button sits on top of it";
const CALENDAR_TABLE: &str = "Blitz's table layout makes the calendar wider than browsers do (Chromium fits it at 320px) and lays out the hidden month and year options";
const FLEX_BUTTON: &str = "Blitz sizes a flex button to its full content, ignoring the `min-width: 0` of the ellipsised summary inside it; Chromium keeps the tool call trigger inside a 320px page";
const NATIVE_SELECT: &str = "Blitz lays out a <select>'s options as one line of inline text, so the field is as wide as all of them; browsers draw one box the width of the longest option (Chromium keeps the demo inside 320px)";
const DATE_RANGE: &str = "Real: the date range field (and the DateRangeField around it) needs about 315px and can't wrap between its two dates without changing the primitive's markup";
const BLITZ_TEXT: &str = "Blitz-only: it falls back to a wider serif font (Inter isn't bundled) and doesn't break these long strings the way Chromium does; Chromium shows no overflow";

/// Accepted findings, as a prefix of `check: demo: element ...`, with the reason they stay.
/// Most are Blitz differing from browsers (each was checked against Chromium with the same
/// markup and CSS); docs/notes/layout-checks.md lists them and what would fix them.
const KNOWN: &[(&str, &str)] = &[
    ("focus-obscured: sidebar: ", SIDEBAR_ACTION),
    ("long-text: calendar: ", CALENDAR_TABLE),
    ("long-text: card: ", INPUT_WIDTH),
    ("long-text: chart: ", BLITZ_TEXT),
    ("long-text: chat: ", INPUT_WIDTH),
    ("long-text: chat_composer: ", INPUT_WIDTH),
    ("long-text: chips: ", INPUT_WIDTH),
    ("long-text: combobox: ", INPUT_WIDTH),
    ("long-text: data_table: ", BLITZ_TEXT),
    ("long-text: date_time_picker: ", INPUT_WIDTH),
    ("long-text: date_picker: ", DATE_RANGE),
    ("long-text: drag_and_drop_list: ", BLITZ_TEXT),
    ("long-text: hooks: ", BLITZ_TEXT),
    ("long-text: input: ", INPUT_WIDTH),
    ("long-text: input_group: ", INPUT_WIDTH),
    ("long-text: label: ", INPUT_WIDTH),
    ("long-text: marquee: ", BLITZ_TEXT),
    ("long-text: native_select: ", NATIVE_SELECT),
    ("long-text: number_field: ", INPUT_WIDTH),
    ("long-text: sidebar: ", BLITZ_TEXT),
    ("long-text: table: ", BLITZ_TEXT),
    ("long-text: theme_customizer: ", INPUT_WIDTH),
    ("long-text: time_picker: ", INPUT_WIDTH),
    ("long-text: tool_call: ", FLEX_BUTTON),
    ("reflow: calendar: ", CALENDAR_TABLE),
    ("reflow: card: ", INPUT_WIDTH),
    ("reflow: date_picker: ", DATE_RANGE),
    ("reflow: field: ", INPUT_WIDTH),
    ("reflow: form: ", INPUT_WIDTH),
    ("reflow: form_derive: ", INPUT_WIDTH),
    ("reflow: icons: ", BLITZ_TEXT),
    ("reflow: input: ", INPUT_WIDTH),
    ("reflow: input_otp: ", BLITZ_TEXT),
    ("reflow: label: ", INPUT_WIDTH),
    ("reflow: link: ", INLINE_BOX),
    ("target-size: pagination: ", INLINE_BOX),
    ("target-size: sidebar: ", SIDEBAR_ACTION),
];

// ---------------------------------------------------------------------------------------------
// The page each demo is rendered in

thread_local! {
    /// Text nodes' text before `pseudo_localise` rewrote it, so findings name elements by the
    /// words a reader of the demo sees.
    static ORIGINAL: std::cell::RefCell<std::collections::HashMap<usize, String>> =
        Default::default();
    static DEMO: Cell<Option<fn() -> Element>> = const { Cell::new(None) };
    static RTL: Cell<bool> = const { Cell::new(false) };
}

/// The same shell as `apps/native` and the interaction tests, with less padding so a 320 px
/// window keeps most of its width for the demo.
fn app() -> Element {
    let demo = DEMO.with(Cell::get).expect("demo set");
    let dir = if RTL.with(Cell::get) { "rtl" } else { "ltr" };
    rsx! {
        ui::ThemeStyles {}
        main { dir, style: "padding: 16px; max-width: 760px;",
            div { {demo()} }
        }
    }
}

#[derive(Clone, Copy)]
struct Page {
    width: u32,
    height: u32,
    rtl: bool,
    css: &'static str,
}

const DESKTOP: Page = Page {
    width: WIDTH,
    height: HEIGHT,
    rtl: false,
    css: "",
};

fn render(demo: fn() -> Element, page: &Page) -> Harness {
    DEMO.with(|d| d.set(Some(demo)));
    ORIGINAL.with(|o| o.borrow_mut().clear());
    RTL.with(|r| r.set(page.rtl));
    Harness::with_viewport(app, page.width, page.height, ColorScheme::Light, page.css)
}

/// The WCAG 1.4.12 text-spacing overrides (the W3C bookmarklet's values).
const TEXT_SPACING: &str = "* { line-height: 1.5 !important; letter-spacing: 0.12em !important; \
     word-spacing: 0.16em !important; } p { margin-bottom: 2em !important; }";

// ---------------------------------------------------------------------------------------------
// Walking the rendered tree

fn node(h: &Harness, id: usize) -> &Node {
    h.doc.get_node(id).expect("node")
}

fn attr<'a>(n: &'a Node, name: &str) -> Option<&'a str> {
    n.attr(LocalName::from(name))
}

fn tag(n: &Node) -> &str {
    n.element_data().map_or("", |e| &e.name.local)
}

/// Rendered: displayed, visible, not `aria-hidden`/`inert` (closed or decorative), with a size.
fn hidden(n: &Node) -> bool {
    if n.is_element() && n.style.display == taffy::Display::None {
        return true;
    }
    if attr(n, "aria-hidden") == Some("true") || attr(n, "inert").is_some() {
        return true;
    }
    n.primary_styles().is_some_and(|s| {
        use style::computed_values::visibility::T as Visibility;
        s.get_inherited_box().visibility != Visibility::Visible
    })
}

fn rect_of(h: &Harness, id: usize) -> Rect {
    let n = node(h, id);
    let pos = n.absolute_position(0.0, 0.0);
    let scroll = h.doc.viewport_scroll();
    Rect {
        x: pos.x - scroll.x as f32,
        y: pos.y - scroll.y as f32,
        w: n.final_layout.size.width,
        h: n.final_layout.size.height,
    }
}

/// Every element in DOM order with whether an ancestor (or itself) is hidden.
fn elements(h: &Harness) -> Vec<usize> {
    let mut out = Vec::new();
    let mut stack = vec![h.doc.root_element().id];
    while let Some(id) = stack.pop() {
        let n = node(h, id);
        if !n.is_element() || hidden(n) {
            continue;
        }
        out.push(id);
        stack.extend(n.children.iter().rev().copied());
    }
    out
}

/// A short, stable name for an element in a finding: tag, role, and its name or text.
fn describe(h: &Harness, id: usize) -> String {
    let n = node(h, id);
    let mut s = tag(n).to_string();
    if let Some(role) = attr(n, "role") {
        s.push_str(&format!("[role={role}]"));
    }
    if let Some(slot) = attr(n, "data-slot") {
        s.push_str(&format!("[data-slot={slot}]"));
    }
    let name = attr(n, "aria-label")
        .map(str::to_owned)
        .unwrap_or_else(|| original_text(h, id));
    let name: String = name.split_whitespace().collect::<Vec<_>>().join(" ");
    let name: String = name.chars().take(24).collect();
    if !name.is_empty() {
        s.push_str(&format!(" \"{name}\""));
    }
    s
}

/// `text_content`, with pseudo-localised text nodes read back in their original words.
fn original_text(h: &Harness, id: usize) -> String {
    let n = node(h, id);
    if let Some(t) = n.text_data() {
        return ORIGINAL
            .with(|o| o.borrow().get(&id).cloned())
            .unwrap_or_else(|| t.content.clone());
    }
    n.children.iter().map(|&c| original_text(h, c)).collect()
}

const TARGET_ROLES: &[&str] = &[
    "button",
    "checkbox",
    "combobox",
    "link",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "option",
    "radio",
    "slider",
    "spinbutton",
    "switch",
    "tab",
    "treeitem",
];

/// Something a pointer user aims at: a control, a link, or anything with a widget role.
fn is_target(n: &Node) -> bool {
    if attr(n, "disabled").is_some() || attr(n, "aria-disabled") == Some("true") {
        return false;
    }
    let native = match tag(n) {
        "button" | "select" | "textarea" | "summary" => true,
        "a" => attr(n, "href").is_some(),
        "input" => attr(n, "type") != Some("hidden"),
        _ => false,
    };
    native
        || attr(n, "role").is_some_and(|r| TARGET_ROLES.contains(&r))
        || attr(n, "tabindex").is_some_and(|t| t.parse::<i32>().is_ok_and(|t| t >= 0))
}

/// Inline in a sentence (a link in running text): exempt from 2.5.8.
fn inline_in_text(n: &Node) -> bool {
    n.display_constructed_as.is_inline_flow()
}

// ---------------------------------------------------------------------------------------------
// Findings

#[derive(Default)]
struct Report {
    found: BTreeSet<String>,
}

impl Report {
    fn add(&mut self, check: &str, demo: &str, what: String) {
        self.found.insert(format!("{check}: {demo}: {what}"));
    }

    /// Fail on findings no `KNOWN` entry covers, and on `KNOWN` entries for this check that
    /// cover nothing any more. An entry covers every finding it is a prefix of, so
    /// `"reflow: form: "` accepts all of one demo's reflow findings.
    fn finish(self, check: &str) {
        let verbose = std::env::var_os("LAYOUT_REPORT").is_some();
        let known: Vec<&str> = KNOWN
            .iter()
            .map(|(k, _)| *k)
            .filter(|k| k.starts_with(&format!("{check}: ")))
            .collect();
        let covered = |f: &str| known.iter().any(|k| f.starts_with(k));
        let new: Vec<&str> = self
            .found
            .iter()
            .map(String::as_str)
            .filter(|f| !covered(f))
            .collect();
        let gone: Vec<&str> = known
            .iter()
            .copied()
            .filter(|k| !self.found.iter().any(|f| f.starts_with(k)))
            .collect();
        if verbose {
            for f in &self.found {
                let mark = if covered(f) { "known" } else { "NEW" };
                println!("[{mark}] {f}");
            }
        }
        assert!(
            new.is_empty() && gone.is_empty(),
            "{check}: {} new finding(s):\n  {}\n{} KNOWN entr(ies) that match nothing now (remove them):\n  {}",
            new.len(),
            new.join("\n  "),
            gone.len(),
            gone.join("\n  "),
        );
    }
}

fn demos() -> impl Iterator<Item = (&'static str, fn() -> Element)> {
    ui::demos::ALL.iter().copied()
}

// ---------------------------------------------------------------------------------------------
// SC 2.5.8 Target Size (Minimum)

const MIN_TARGET: f32 = 24.0;

fn rect_meets_circle(r: Rect, (cx, cy): (f32, f32), radius: f32) -> bool {
    let nx = cx.clamp(r.x, r.x + r.w);
    let ny = cy.clamp(r.y, r.y + r.h);
    (nx - cx).powi(2) + (ny - cy).powi(2) < radius.powi(2) - 0.01
}

/// Targets below 24x24 that also fail the spacing exception, in one rendered page.
fn small_targets(h: &Harness) -> Vec<(usize, Rect)> {
    let targets: Vec<(usize, Rect)> = elements(h)
        .into_iter()
        .filter(|&id| is_target(node(h, id)) && !inline_in_text(node(h, id)))
        .map(|id| (id, hit_area(h, id)))
        .filter(|(_, r)| r.w > 0.0 && r.h > 0.0)
        .collect();
    let small = |r: &Rect| r.w < MIN_TARGET - 0.01 || r.h < MIN_TARGET - 0.01;
    let mut out = Vec::new();
    for &(id, r) in &targets {
        if !small(&r) {
            continue;
        }
        // A target inside a larger target (an icon button's inner span with a role) is aimed
        // at through its parent.
        let c = r.center();
        let crowded = targets.iter().any(|&(other, o)| {
            if other == id || contains(h, other, id) || contains(h, id, other) {
                return false;
            }
            if small(&o) {
                let oc = o.center();
                (oc.0 - c.0).hypot(oc.1 - c.1) < MIN_TARGET - 0.01
            } else {
                rect_meets_circle(o, c, MIN_TARGET / 2.0)
            }
        });
        if crowded {
            out.push((id, r));
        }
    }
    out
}

/// The area that takes the pointer: the element's box, grown by a `::before`/`::after` box
/// (the usual way to give a small icon button a bigger hit area).
fn hit_area(h: &Harness, id: usize) -> Rect {
    let n = node(h, id);
    let mut r = rect_of(h, id);
    for pseudo in [n.before, n.after].into_iter().flatten() {
        let p = rect_of(h, pseudo);
        if p.w <= 0.0 || p.h <= 0.0 || hidden(node(h, pseudo)) {
            continue;
        }
        let (x1, y1) = (r.x.min(p.x), r.y.min(p.y));
        let (x2, y2) = ((r.x + r.w).max(p.x + p.w), (r.y + r.h).max(p.y + p.h));
        r = Rect {
            x: x1,
            y: y1,
            w: x2 - x1,
            h: y2 - y1,
        };
    }
    r
}

fn contains(h: &Harness, ancestor: usize, mut id: usize) -> bool {
    while let Some(p) = node(h, id).parent {
        if p == ancestor {
            return true;
        }
        id = p;
    }
    false
}

#[test]
fn targets_are_at_least_24px_or_spaced() {
    let mut report = Report::default();
    for (name, demo) in demos() {
        let h = render(demo, &DESKTOP);
        for (id, r) in small_targets(&h) {
            report.add(
                "target-size",
                name,
                format!("{} is {:.0}x{:.0}", describe(&h, id), r.w, r.h),
            );
        }
    }
    report.finish("target-size");
}

// ---------------------------------------------------------------------------------------------
// SC 2.4.11 Focus Not Obscured (Minimum)

/// Scroll the window the least amount that brings `r` inside it, as a browser does when focus
/// moves (`scrollIntoView({ block: "nearest" })`). Sticky and fixed bars are not taken into
/// account, which is exactly what makes focus end up underneath them, unless the page sets
/// `scroll-padding-bottom` (Blitz's style engine has no `scroll-padding`, so a test passes the
/// app's value).
fn scroll_nearest_with(h: &mut Harness, id: usize, padding_bottom: f32) {
    let r = rect_of(h, id);
    let height = h.height as f32 - padding_bottom;
    let dy = if r.y < 0.0 {
        r.y
    } else if r.y + r.h > height {
        (r.y + r.h - height).min(r.y)
    } else {
        0.0
    };
    if dy != 0.0 {
        h.doc.scroll_viewport_by(0.0, -dy as f64);
        h.settle();
    }
}

/// How much of the focused element shows: the share of sample points where it (or something
/// inside it) is what a click would land on.
fn visible_share(h: &Harness, id: usize) -> Option<f32> {
    let r = rect_of(h, id);
    let scroll = h.doc.viewport_scroll();
    let mut seen = 0;
    let mut total = 0;
    for i in 0..5 {
        for j in 0..5 {
            let x = r.x + r.w * (0.1 + 0.2 * i as f32);
            let y = r.y + r.h * (0.1 + 0.2 * j as f32);
            if x < 0.0 || y < 0.0 || x >= h.width as f32 || y >= h.height as f32 {
                continue;
            }
            total += 1;
            let hit = h.doc.hit(x + scroll.x as f32, y + scroll.y as f32);
            if hit.is_some_and(|hit| {
                hit.node_id == id || contains(h, id, hit.node_id) || contains(h, hit.node_id, id)
            }) {
                seen += 1;
            }
        }
    }
    // `None`: no sample point is inside the window (off the side; reflow reports that).
    (total > 0).then(|| seen as f32 / total as f32)
}

/// Tab through `h` (at most `limit` stops) and report each stop that is entirely covered.
fn obscured_focus(h: &mut Harness, limit: usize) -> Vec<usize> {
    obscured_focus_with(h, limit, 0.0)
}

fn obscured_focus_with(h: &mut Harness, limit: usize, padding_bottom: f32) -> Vec<usize> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for _ in 0..limit {
        h.key(Key::Tab, Code::Tab, Modifiers::empty());
        let Some(id) = h.focused() else { break };
        if !seen.insert(id) {
            break;
        }
        // Blitz still Tabs to `tabindex="-1"` elements (the primitives' hidden checkbox
        // inputs); a browser never lands there, so neither does this check.
        let negative = attr(node(h, id), "tabindex")
            .and_then(|t| t.trim().parse::<i32>().ok())
            .is_some_and(|t| t < 0);
        let r = rect_of(h, id);
        if negative || r.w <= 0.0 || r.h <= 0.0 {
            continue;
        }
        scroll_nearest_with(h, id, padding_bottom);
        if visible_share(h, id) == Some(0.0) {
            out.push(id);
        }
    }
    out
}

#[test]
fn focus_is_never_hidden_behind_other_content() {
    let mut report = Report::default();
    for (name, demo) in demos() {
        for page in [&DESKTOP, &PHONE] {
            let mut h = render(demo, page);
            for id in obscured_focus(&mut h, 60) {
                report.add(
                    "focus-obscured",
                    name,
                    format!("{} at {}px", describe(&h, id), page.width),
                );
            }
        }
    }
    report.finish("focus-obscured");
}

// ---------------------------------------------------------------------------------------------
// Overflow and clipping (reflow, text spacing, longer text, RTL)

/// The smallest window WCAG reflow asks for (1280 px at 400% zoom).
const PHONE: Page = Page {
    width: 320,
    height: 640,
    rtl: false,
    css: "",
};

/// The left and right edge of what `id` draws itself: its box, and the lines of text it
/// lays out (a long word can stick out of its box). Descendants are checked on their own.
fn extent(h: &Harness, id: usize) -> (f32, f32) {
    let n = node(h, id);
    let r = rect_of(h, id);
    let mut right = r.x + r.w;
    if !is_control(n) {
        if let Some(text) = n.element_data().and_then(|e| e.inline_layout_data.as_ref()) {
            let pad = n.final_layout.padding.left + n.final_layout.border.left;
            right = right.max(r.x + pad + text.layout.width());
        }
    }
    (r.x, right)
}

/// Form controls scroll their own text (and a `select` lists its options elsewhere): only
/// their box counts.
fn is_control(n: &Node) -> bool {
    matches!(tag(n), "input" | "textarea" | "select")
}

fn clips_x(n: &Node) -> bool {
    n.style.overflow.x != taffy::Overflow::Visible
}

/// Elements whose box or text reaches past the window's left or right edge outside any
/// container that clips or scrolls sideways: each one makes the page scroll horizontally.
/// Only the outermost offender of a branch is reported.
fn page_overflow(h: &Harness) -> Vec<usize> {
    let width = h.width as f32;
    let mut out = Vec::new();
    let mut stack = vec![(h.doc.root_element().id, false)];
    while let Some((id, inside_offender)) = stack.pop() {
        let n = node(h, id);
        if !n.is_element() || hidden(n) {
            continue;
        }
        let r = rect_of(h, id);
        let (left, right) = extent(h, id);
        let offends = r.w > 0.0 && (right > width + 1.0 || left < -1.0);
        if offends && !inside_offender {
            out.push(id);
        }
        if clips_x(n) || is_control(n) {
            continue;
        }
        stack.extend(
            n.children
                .iter()
                .rev()
                .map(|&c| (c, inside_offender || offends)),
        );
    }
    out
}

/// Elements that cut off their own content (`overflow: hidden | clip` with content larger
/// than the box). Scroll containers are fine: the rest is a scroll away.
fn clipped(h: &Harness) -> Vec<usize> {
    elements(h)
        .into_iter()
        .filter(|&id| {
            let n = node(h, id);
            let clips =
                |o: taffy::Overflow| matches!(o, taffy::Overflow::Hidden | taffy::Overflow::Clip);
            if n.is_scroll_container() || is_control(n) || ellipsis(h, id) {
                return false;
            }
            let size = n.final_layout.size;
            let content = n.final_layout.content_size;
            let mut wide = content.width;
            if let Some(text) = n.element_data().and_then(|e| e.inline_layout_data.as_ref()) {
                let pad = n.final_layout.padding.left
                    + n.final_layout.border.left
                    + n.final_layout.padding.right
                    + n.final_layout.border.right;
                wide = wide.max(text.layout.width() + pad);
            }
            size.width > 0.0
                && size.height > 0.0
                && ((clips(n.style.overflow.x) && wide > size.width + 1.0)
                    || (clips(n.style.overflow.y) && content.height > size.height + 1.0))
        })
        .collect()
}

/// Cut off on purpose with an ellipsis (`text-overflow` on it or on the text inside it), as
/// long names in a sidebar are.
fn ellipsis(h: &Harness, id: usize) -> bool {
    use style::values::specified::text::TextOverflowSide;
    let n = node(h, id);
    n.primary_styles()
        .is_some_and(|s| !matches!(s.clone_text_overflow().second, TextOverflowSide::Clip))
        || n.children
            .iter()
            .any(|&c| node(h, c).is_element() && ellipsis(h, c))
}

fn overflow_findings(h: &Harness) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in page_overflow(h) {
        let (l, r) = extent(h, id);
        out.insert(format!(
            "{} overflows the page ({:.0}..{:.0} of {})",
            describe(h, id),
            l,
            r,
            h.width
        ));
    }
    out
}

fn clip_findings(h: &Harness) -> BTreeSet<String> {
    clipped(h)
        .into_iter()
        .map(|id| format!("{} clips its content", describe(h, id)))
        .collect()
}

#[test]
fn nothing_scrolls_sideways_at_320px() {
    let mut report = Report::default();
    for (name, demo) in demos() {
        let h = render(demo, &PHONE);
        for f in overflow_findings(&h) {
            report.add("reflow", name, f);
        }
    }
    report.finish("reflow");
}

/// Findings in `after` that `before` didn't have, compared by element (the pseudo-localised
/// text changes the names, so match on everything before the quote).
fn new_findings(before: &BTreeSet<String>, after: BTreeSet<String>) -> Vec<String> {
    let key = |s: &str| {
        let (head, tail) = s.split_once(" \"").unwrap_or((s, ""));
        let rest = tail.split_once("\" ").map_or("", |(_, r)| r);
        let rest = rest.split(" (").next().unwrap_or(rest);
        format!("{head} {rest}")
    };
    let before: BTreeSet<String> = before.iter().map(|s| key(s)).collect();
    after
        .into_iter()
        .filter(|s| !before.contains(&key(s)))
        .collect()
}

#[test]
fn text_spacing_overrides_break_nothing() {
    let spaced = Page {
        css: TEXT_SPACING,
        ..DESKTOP
    };
    let mut report = Report::default();
    for (name, demo) in demos() {
        let base = render(demo, &DESKTOP);
        let mut before = overflow_findings(&base);
        before.extend(clip_findings(&base));
        let h = render(demo, &spaced);
        let mut after = overflow_findings(&h);
        after.extend(clip_findings(&h));
        for f in new_findings(&before, after) {
            report.add("text-spacing", name, f);
        }
    }
    report.finish("text-spacing");
}

/// Pseudo-localise: accent every letter and make each word 40% longer (rounded up), the way
/// translations into German or Finnish run long. Words stay unbreakable, as compounds are.
fn pseudo(text: &str) -> String {
    const FROM: &str = "aceinosuyACEINOSUY";
    const TO: [char; 18] = [
        'á', 'ç', 'é', 'î', 'ñ', 'ö', 'š', 'ü', 'ý', 'Á', 'Ç', 'É', 'Î', 'Ñ', 'Ö', 'Š', 'Ü', 'Ý',
    ];
    let mut out = String::new();
    let mut word = 0usize;
    let flush = |out: &mut String, word: &mut usize| {
        for _ in 0..(*word * 2).div_ceil(5) {
            out.push('ẍ');
        }
        *word = 0;
    };
    for c in text.chars() {
        if c.is_alphanumeric() {
            out.push(FROM.find(c).map_or(c, |i| TO[i]));
            word += 1;
        } else {
            flush(&mut out, &mut word);
            out.push(c);
        }
    }
    flush(&mut out, &mut word);
    out
}

/// Rewrite every text node and placeholder in the rendered page with `pseudo`.
fn pseudo_localise(h: &mut Harness) {
    let mut texts = Vec::new();
    let mut placeholders = Vec::new();
    ORIGINAL.with(|o| o.borrow_mut().clear());
    for (id, n) in h.doc.tree().iter() {
        if let Some(t) = n.text_data() {
            if !t.content.trim().is_empty() {
                ORIGINAL.with(|o| o.borrow_mut().insert(id, t.content.clone()));
                texts.push((id, pseudo(&t.content)));
            }
        } else if let Some(p) = attr(n, "placeholder") {
            placeholders.push((id, pseudo(p)));
        }
    }
    {
        let mut m = h.doc.mutate();
        for (id, t) in &texts {
            m.set_node_text(*id, t);
        }
        for (id, p) in &placeholders {
            m.set_attribute(
                *id,
                blitz_dom::QualName::new(None, blitz_dom::ns!(), LocalName::from("placeholder")),
                p,
            );
        }
    }
    h.settle();
}

#[test]
fn text_40_percent_longer_breaks_nothing() {
    let mut report = Report::default();
    for (name, demo) in demos() {
        for page in [&DESKTOP, &PHONE] {
            let base = render(demo, page);
            let mut before = overflow_findings(&base);
            before.extend(clip_findings(&base));
            let mut h = render(demo, page);
            pseudo_localise(&mut h);
            let mut after = overflow_findings(&h);
            after.extend(clip_findings(&h));
            for f in new_findings(&before, after) {
                report.add("long-text", name, format!("{f} at {}px", page.width));
            }
        }
    }
    report.finish("long-text");
}

#[test]
fn right_to_left_breaks_nothing() {
    let mut report = Report::default();
    for (name, demo) in demos() {
        for page in [&DESKTOP, &PHONE] {
            let base = render(demo, page);
            let mut before = overflow_findings(&base);
            before.extend(clip_findings(&base));
            let h = render(demo, &Page { rtl: true, ..*page });
            let mut after = overflow_findings(&h);
            after.extend(clip_findings(&h));
            for f in new_findings(&before, after) {
                report.add("rtl", name, format!("{f} at {}px", page.width));
            }
        }
    }
    report.finish("rtl");
}

#[test]
fn pseudo_localisation_is_40_percent_longer() {
    assert_eq!(pseudo("Save"), "Šávéẍẍ");
    assert_eq!(pseudo("Go to page"), "Göẍ töẍ págéẍẍ");
    let s = "Select a fruit";
    assert!(pseudo(s).chars().count() as f32 >= s.chars().count() as f32 * 1.35);
}

/// The showcase app at phone size has a fixed bottom tab bar: tabbing through a long page
/// must never leave focus underneath it.
#[test]
fn focus_clears_the_showcase_tab_bar_on_a_phone() {
    let css = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../showcase/assets/showcase.css"),
    )
    .expect("showcase.css");
    // The phone layout's `scroll-padding-bottom` on `html`, which browsers honour when they
    // scroll focus into view.
    let padding = css
        .split("scroll-padding-bottom:")
        .nth(1)
        .and_then(|v| v.trim().split("rem").next()?.parse::<f32>().ok())
        .map_or(0.0, |rem| rem * 16.0);
    let mut h = Harness::with_viewport(showcase::App, 390, 844, ColorScheme::Light, &css);
    let hidden: Vec<String> = obscured_focus_with(&mut h, 80, padding)
        .into_iter()
        .map(|id| describe(&h, id))
        .collect();
    assert!(
        hidden.is_empty(),
        "focus hidden behind the tab bar: {hidden:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// The checks themselves: each one catches a page built to fail it.

thread_local! {
    static BAD: Cell<&'static str> = const { Cell::new("") };
}

fn bad_app() -> Element {
    let which = BAD.with(Cell::get);
    rsx! {
        ui::ThemeStyles {}
        match which {
            "small" => rsx! {
                div { style: "display: flex; gap: 2px;",
                    button { style: "width: 16px; height: 16px; padding: 0;", "a" }
                    button { style: "width: 16px; height: 16px; padding: 0;", "b" }
                }
            },
            "spaced" => rsx! {
                div { style: "display: flex; gap: 12px;",
                    button { style: "width: 16px; height: 16px; padding: 0;", "a" }
                    button { style: "width: 16px; height: 16px; padding: 0;", "b" }
                }
            },
            "wide" => rsx! { div { style: "width: 400px; height: 10px;" } },
            "scrolls" => rsx! {
                div { style: "overflow-x: auto;", div { style: "width: 400px; height: 10px;" } }
            },
            "clipped" => rsx! {
                div { style: "width: 60px; overflow: hidden; white-space: nowrap;",
                    "A sentence far longer than its box"
                }
            },
            _ => rsx! {
                for i in 0..30 {
                    p { button { "Item {i}" } }
                }
                div { style: "position: fixed; left: 0; right: 0; bottom: 0; height: 120px; background: white;",
                    "A fixed bar"
                }
            },
        }
    }
}

fn bad(which: &'static str, width: u32) -> Harness {
    BAD.with(|b| b.set(which));
    Harness::with_viewport(bad_app, width, 400, ColorScheme::Light, "")
}

#[test]
fn the_checks_catch_what_they_are_for() {
    assert_eq!(
        small_targets(&bad("small", 1024)).len(),
        2,
        "crowded 16px targets"
    );
    assert!(
        small_targets(&bad("spaced", 1024)).is_empty(),
        "spacing exception"
    );
    assert_eq!(
        page_overflow(&bad("wide", 320)).len(),
        1,
        "a 400px box at 320px"
    );
    assert!(
        page_overflow(&bad("scrolls", 320)).is_empty(),
        "inside a scroll container"
    );
    assert_eq!(
        clipped(&bad("clipped", 1024)).len(),
        1,
        "text cut off by overflow: hidden"
    );
    let mut h = bad("fixed-bar", 1024);
    assert!(
        !obscured_focus(&mut h, 40).is_empty(),
        "focus under a fixed bar"
    );
}
