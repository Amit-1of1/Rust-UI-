//! Runs every component contract (`crates/ui/src/components/<name>/contract.toml`) on the
//! headless native renderer. A row marked `native = true` must pass here, and a row marked
//! `native = false` must still fail, so the docs' "Native" column can't drift from what Blitz
//! does in either direction. The same rows run in browsers in `tests/e2e/contracts.mjs`.
//! Format: docs/CONTRACTS.md.
//!
//! `CONTRACT=dialog cargo test -p native --test contracts` runs one component and prints each
//! row's outcome.

mod common;
use common::*;

#[path = "../../../crates/cli/src/contract.rs"]
#[allow(dead_code)]
mod contract;

#[path = "../../docs/src/generated.rs"]
#[rustfmt::skip]
#[allow(dead_code)]
mod generated;

use blitz_dom::LocalName;
use contract::{Contract, PartRef, Step};
use dioxus::prelude::*;
use std::cell::RefCell;
use std::path::Path;

thread_local! {
    /// The demo the next `Harness` renders. Tests run on their own threads, and a harness
    /// renders on the thread that made it.
    static DEMO: RefCell<String> = const { RefCell::new(String::new()) };
}

fn app() -> Element {
    let name = DEMO.with(|d| d.borrow().clone());
    rsx! {
        ui::ThemeStyles {}
        main { style: "padding: 24px; max-width: 760px;",
            div { {generated::demo(&name)} }
        }
    }
}

fn harness(name: &str) -> Harness {
    DEMO.with(|d| *d.borrow_mut() = name.to_string());
    Harness::new(app)
}

fn node(h: &Harness, at: &PartRef) -> Option<usize> {
    h.doc
        .query_selector_all(&at.selector)
        .unwrap_or_else(|e| panic!("bad selector {}: {e:?}", at.selector))
        .get(at.nth - 1)
        .copied()
}

fn rect(h: &Harness, id: usize) -> Rect {
    let node = h.doc.get_node(id).expect("node");
    let pos = node.absolute_position(0.0, 0.0);
    let size = node.final_layout.size;
    let scroll = h.doc.viewport_scroll();
    Rect {
        x: pos.x - scroll.x as f32,
        y: pos.y - scroll.y as f32,
        w: size.width,
        h: size.height,
    }
}

fn visible(h: &Harness, at: &PartRef) -> bool {
    node(h, at).is_some_and(|id| rect(h, id).on_screen())
}

fn contains(h: &Harness, root: usize, mut node: Option<usize>) -> bool {
    while let Some(id) = node {
        if id == root {
            return true;
        }
        node = h.doc.get_node(id).and_then(|n| n.parent);
    }
    false
}

/// Where focus is, for failure messages.
fn focus_text(h: &Harness) -> String {
    match h.focused().and_then(|id| h.doc.get_node(id)) {
        None => "nothing has focus".into(),
        Some(n) => {
            let tag = n
                .element_data()
                .map_or("?".into(), |e| e.name.local.to_string());
            let text: String = n.text_content().trim().chars().take(30).collect();
            format!("focus is on <{tag}> {text:?}")
        }
    }
}

fn key(name: &str) -> (Key, Code, Modifiers) {
    let mut mods = Modifiers::empty();
    let mut parts: Vec<&str> = name.split('+').collect();
    let last = parts.pop().expect("key");
    for m in parts {
        mods |= match m {
            "Shift" => Modifiers::SHIFT,
            "Control" => Modifiers::CONTROL,
            "Alt" => Modifiers::ALT,
            "Meta" => Modifiers::META,
            _ => unreachable!("checked by the contract parser"),
        };
    }
    let (k, c) = match last {
        "Enter" => (Key::Enter, Code::Enter),
        "Space" => (Key::Character(" ".into()), Code::Space),
        "Escape" => (Key::Escape, Code::Escape),
        "Tab" => (Key::Tab, Code::Tab),
        "ArrowUp" => (Key::ArrowUp, Code::ArrowUp),
        "ArrowDown" => (Key::ArrowDown, Code::ArrowDown),
        "ArrowLeft" => (Key::ArrowLeft, Code::ArrowLeft),
        "ArrowRight" => (Key::ArrowRight, Code::ArrowRight),
        "Home" => (Key::Home, Code::Home),
        "End" => (Key::End, Code::End),
        "PageUp" => (Key::PageUp, Code::PageUp),
        "PageDown" => (Key::PageDown, Code::PageDown),
        "Backspace" => (Key::Backspace, Code::Backspace),
        "Delete" => (Key::Delete, Code::Delete),
        c => (Key::Character(c.into()), Code::Unidentified),
    };
    (k, c, mods)
}

fn describe(at: &PartRef) -> String {
    if at.nth == 1 {
        at.part.clone()
    } else {
        format!("{}@{}", at.part, at.nth)
    }
}

/// Run one row's steps; `Err` names the first step that did not hold.
fn run(name: &str, steps: &[Step]) -> Result<(), String> {
    let mut h = harness(name);
    for step in steps {
        let missing = |at: &PartRef| format!("no element for {} ({})", describe(at), at.selector);
        match step {
            Step::Focus { at } => {
                let id = node(&h, at).ok_or_else(|| missing(at))?;
                h.doc.set_focus_to(id);
                // Blitz moved focus without an event; a no-op key release makes the renderer
                // report it (focus/focusin), as a real focus change would.
                h.key(Key::Unidentified, Code::Unidentified, Modifiers::empty());
            }
            Step::Click { at } => {
                let id = node(&h, at).ok_or_else(|| missing(at))?;
                let (x, y) = rect(&h, id).center();
                h.click_at(x, y);
            }
            Step::Press { key: k, times } => {
                let (k, c, m) = key(k);
                for _ in 0..*times {
                    h.key(k.clone(), c, m);
                }
            }
            Step::Type { text } => h.type_text(text),
            Step::Visible { at } => {
                if !visible(&h, at) {
                    return Err(format!("{} is not visible", describe(at)));
                }
            }
            Step::Hidden { at } => {
                if visible(&h, at) {
                    return Err(format!("{} is still visible", describe(at)));
                }
            }
            Step::Focused { at } => {
                let id = node(&h, at).ok_or_else(|| missing(at))?;
                if h.focused() != Some(id) {
                    return Err(format!(
                        "{} does not have focus ({})",
                        describe(at),
                        focus_text(&h)
                    ));
                }
            }
            Step::FocusIn { at } => {
                let id = node(&h, at).ok_or_else(|| missing(at))?;
                if !contains(&h, id, h.focused()) {
                    return Err(format!(
                        "focus is not inside {} ({})",
                        describe(at),
                        focus_text(&h)
                    ));
                }
            }
            Step::Text { at, text } => {
                let id = node(&h, at).ok_or_else(|| missing(at))?;
                let got = h
                    .doc
                    .get_node(id)
                    .map(|n| n.text_content())
                    .unwrap_or_default();
                if !got.contains(text.as_str()) {
                    return Err(format!("{} says {got:?}, not {text:?}", describe(at)));
                }
            }
            Step::Attr {
                at,
                name: attr,
                value,
                negate,
            } => {
                let id = node(&h, at).ok_or_else(|| missing(at))?;
                let got = h
                    .doc
                    .get_node(id)
                    .and_then(|n| n.attr(LocalName::from(attr.as_str())))
                    .map(str::to_string);
                if (got.as_deref() == Some(value.as_str())) == *negate {
                    return Err(format!(
                        "{} has {attr}={} (wanted {}{value})",
                        describe(at),
                        got.as_deref().unwrap_or("<none>"),
                        if *negate { "not " } else { "" }
                    ));
                }
            }
        }
    }
    Ok(())
}

#[test]
fn contracts_hold_on_native() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src/components");
    let only = std::env::var("CONTRACT").ok();
    let contracts = Contract::load_all(&dir).expect("contracts");
    assert!(!contracts.is_empty(), "no contract.toml found");
    let mut wrong = Vec::new();
    for c in contracts
        .iter()
        .filter(|c| only.as_deref().is_none_or(|o| o == c.name))
    {
        let runnable = c.runnable().unwrap_or_else(|e| panic!("{e}"));
        for row in &runnable.rows {
            let outcome = run(&c.name, &row.steps);
            if only.is_some() {
                println!("{} {}: {:?}", c.name, row.key, outcome);
            }
            match (row.native, outcome) {
                (true, Err(e)) => wrong.push(format!(
                    "{} {}: marked native = true but fails on Blitz: {e}",
                    c.name, row.key
                )),
                (false, Ok(())) => wrong.push(format!(
                    "{} {}: passes on Blitz now; set native = true in its contract.toml",
                    c.name, row.key
                )),
                _ => {}
            }
        }
    }
    assert!(wrong.is_empty(), "\n{}\n", wrong.join("\n"));
}
