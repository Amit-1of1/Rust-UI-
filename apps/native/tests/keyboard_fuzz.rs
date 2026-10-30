//! Keyboard fuzzing on the headless native renderer: every demo gets random key sequences and
//! must keep these invariants after every key:
//!
//! - **No panic** anywhere in the component or the renderer.
//! - **Focus is never lost**: something has focus, it is still in the document, and it is
//!   rendered (not inside `display: none`). Losing focus drops a keyboard user back to the
//!   top of the page.
//! - **A modal keeps focus**: while a modal dialog is open, focus stays inside it.
//! - **No keyboard trap** (WCAG 2.1.2): at the end of every sequence, at most three Escapes
//!   close any modal, and Tab then leaves the component.
//!
//! The keys are Tab, Shift+Tab, Escape, Enter, Space, the arrows, Home/End, PageUp/PageDown,
//! plus every key the component's `contract.toml` lists, so components get pressed the keys they
//! claim to handle. A failing sequence is shrunk to the shortest one that still fails and
//! printed with its seed. Sequences are deterministic: `FUZZ_SEED=n` picks another set,
//! `FUZZ_CASES=n` (default 3) and `FUZZ_LEN=n` (default 20) make the run longer, `FUZZ=dialog`
//! runs one demo and prints every sequence.
//!
//! Known native gaps (also `native = false` rows in the contracts) are listed in [`KNOWN`]
//! with the shortest sequence that shows them; `known_gaps_still_reproduce` fails once one is
//! fixed, so the list can't go stale. For those demos a lost focus is put back at the start
//! and the sequence goes on, so the other invariants are still checked there.

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
use dioxus::prelude::*;
use std::cell::RefCell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;

thread_local! {
    static DEMO: RefCell<String> = const { RefCell::new(String::new()) };
}

/// The demo between two plain buttons, so Tab has somewhere to go on both sides.
fn app() -> Element {
    let name = DEMO.with(|d| d.borrow().clone());
    rsx! {
        ui::ThemeStyles {}
        main { style: "padding: 24px; max-width: 760px;",
            button { id: "fuzz-before", "before" }
            div { id: "fuzz-demo", {generated::demo(&name)} }
            button { id: "fuzz-after", "after" }
        }
    }
}

const BASE_KEYS: &[&str] = &[
    "Tab",
    "Shift+Tab",
    "Escape",
    "Enter",
    "Space",
    "ArrowUp",
    "ArrowDown",
    "ArrowLeft",
    "ArrowRight",
    "Home",
    "End",
    "PageUp",
    "PageDown",
];

/// (demo, invariant, shortest sequence that breaks it, why). Each must still reproduce.
/// Empty since overlays return focus to their opener through element handles
/// (`hooks::ReturnFocus`), which also works without a script.
const KNOWN: &[(&str, Invariant, &[&str], &str)] = &[];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Invariant {
    Panic,
    FocusLost,
    ModalEscaped,
    KeyboardTrap,
}

#[derive(Debug, Clone)]
struct Failure {
    invariant: Invariant,
    /// Keys pressed up to and including the one that broke it.
    at: usize,
    detail: String,
}

fn parse_key(name: &str) -> (Key, Code, Modifiers) {
    let mut mods = Modifiers::empty();
    let mut parts: Vec<&str> = name.split('+').collect();
    let last = parts.pop().unwrap_or(name);
    for m in parts {
        mods |= match m {
            "Shift" => Modifiers::SHIFT,
            "Control" => Modifiers::CONTROL,
            "Alt" => Modifiers::ALT,
            "Meta" => Modifiers::META,
            _ => Modifiers::empty(),
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

fn press(h: &mut Harness, name: &str) {
    let (k, c, m) = parse_key(name);
    h.key(k, c, m);
}

fn by_id(h: &Harness, id: &str) -> Option<usize> {
    h.find(&format!("#{id}"))
}

fn is_inside(h: &Harness, root: usize, mut node: Option<usize>) -> bool {
    while let Some(id) = node {
        if id == root {
            return true;
        }
        node = h.doc.get_node(id).and_then(|n| n.parent);
    }
    false
}

fn describe(h: &Harness, id: Option<usize>) -> String {
    match id.and_then(|id| h.doc.get_node(id)) {
        None => "nothing".into(),
        Some(n) => {
            let tag = n
                .element_data()
                .map_or("#text".into(), |e| e.name.local.to_string());
            let attr = |a: &str| n.attr(LocalName::from(a)).map(str::to_string);
            let label = attr("aria-label")
                .or_else(|| attr("role").map(|r| format!("role={r}")))
                .unwrap_or_else(|| n.text_content().trim().chars().take(24).collect());
            format!("<{tag}> {label:?}")
        }
    }
}

/// Why focus counts as lost, if it does.
fn focus_problem(h: &Harness) -> Option<String> {
    let Some(id) = h.focused() else {
        return Some("nothing has focus".into());
    };
    let Some(node) = h.doc.get_node(id) else {
        return Some("focus is on a node that no longer exists".into());
    };
    let tag = node
        .element_data()
        .map(|e| e.name.local.to_string())
        .unwrap_or_default();
    if tag == "body" || tag == "html" {
        return Some(format!("focus fell back to <{tag}>"));
    }
    // Still attached: its parents lead to the root.
    let root = h.doc.root_node().id;
    if !is_inside(h, root, Some(id)) {
        return Some(format!(
            "focus is on {}, which was removed",
            describe(h, Some(id))
        ));
    }
    // Rendered: neither it nor an ancestor is `display: none`.
    let mut cur = Some(id);
    while let Some(n) = cur.and_then(|i| h.doc.get_node(i)) {
        // taffy's `Display` (not a dependency here) prints as `None` for display: none.
        if n.is_element() && format!("{:?}", n.style.display) == "None" {
            return Some(format!(
                "focus is on {}, which is not displayed",
                describe(h, Some(id))
            ));
        }
        cur = n.parent;
    }
    None
}

/// The open modal dialog, if any: an `aria-modal` dialog or an alert dialog with a box on
/// screen.
fn open_modal(h: &Harness) -> Option<usize> {
    let found = h
        .doc
        .query_selector_all("[role=dialog][aria-modal=true], [role=alertdialog]")
        .ok()?;
    found.into_iter().find(|&id| {
        h.doc.get_node(id).is_some_and(|n| {
            let s = n.final_layout.size;
            s.width > 0.0 && s.height > 0.0
        })
    })
}

/// Runs one sequence from a fresh render. `Err` is the first invariant that broke.
fn run(demo: &str, keys: &[&str]) -> Result<(), Failure> {
    run_with(demo, keys, false)
}

/// With `tolerate_lost_focus`, a lost focus (a known gap) is repaired by putting focus back
/// at the start, and the sequence goes on, so the other invariants are still checked.
fn run_with(demo: &str, keys: &[&str], tolerate_lost_focus: bool) -> Result<(), Failure> {
    let demo = demo.to_string();
    let keys: Vec<String> = keys.iter().map(|k| k.to_string()).collect();
    let n = keys.len();
    let result = catch_unwind(AssertUnwindSafe(|| -> Result<(), Failure> {
        DEMO.with(|d| *d.borrow_mut() = demo.clone());
        // Toasts start a timer, which needs a Tokio runtime (the windowed shell has one).
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("runtime");
        let _rt = rt.enter();
        let mut h = Harness::new(app);
        let before = by_id(&h, "fuzz-before").expect("sentinel");
        let after = by_id(&h, "fuzz-after").expect("sentinel");
        h.doc.set_focus_to(before);
        press(&mut h, "Tab");
        for (i, key) in keys.iter().enumerate() {
            let fail = |invariant, detail| Failure {
                invariant,
                at: i + 1,
                detail,
            };
            // Tabbed out to a sentinel: step back in, so the keys keep reaching the demo.
            match h.focused() {
                Some(f) if f == before => press(&mut h, "Tab"),
                Some(f) if f == after => press(&mut h, "Shift+Tab"),
                _ => {}
            }
            let modal = open_modal(&h);
            let from = describe(&h, h.focused());
            press(&mut h, key);
            if let Some(p) = focus_problem(&h) {
                if !tolerate_lost_focus {
                    return Err(fail(
                        Invariant::FocusLost,
                        format!("{p} (after {key} on {from})"),
                    ));
                }
                h.doc.set_focus_to(before);
                press(&mut h, "Tab");
            }
            if let Some(m) = modal.filter(|&m| open_modal(&h) == Some(m)) {
                if !is_inside(&h, m, h.focused()) {
                    return Err(fail(
                        Invariant::ModalEscaped,
                        format!(
                            "{key} moved focus out of the open dialog to {}",
                            describe(&h, h.focused())
                        ),
                    ));
                }
            }
        }
        // Escape hatch: Escape closes what's open, then Tab leaves the component.
        for _ in 0..3 {
            if open_modal(&h).is_none() {
                break;
            }
            press(&mut h, "Escape");
        }
        let trap = |detail: String| Failure {
            invariant: Invariant::KeyboardTrap,
            at: n,
            detail,
        };
        if open_modal(&h).is_some() {
            return Err(trap("three Escapes did not close the dialog".into()));
        }
        if tolerate_lost_focus && focus_problem(&h).is_some() {
            h.doc.set_focus_to(before);
            press(&mut h, "Tab");
        }
        if focus_problem(&h).is_some() {
            // Escape lost focus; Tab starts from the top of the page again, which is the
            // lost-focus failure, not a trap.
            return Err(Failure {
                invariant: Invariant::FocusLost,
                at: n,
                detail: format!(
                    "{} (after Escape closed an overlay)",
                    focus_problem(&h).unwrap_or_default()
                ),
            });
        }
        let mut seen = Vec::new();
        for _ in 0..120 {
            if h.focused() == Some(after) {
                return Ok(());
            }
            seen.push(describe(&h, h.focused()));
            press(&mut h, "Tab");
        }
        seen.dedup();
        let tail: Vec<_> = seen.iter().rev().take(4).rev().cloned().collect();
        Err(trap(format!(
            "120 Tabs did not leave the component; focus cycles through {}",
            tail.join(", ")
        )))
    }));
    match result {
        Ok(r) => r,
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "panic".into());
            Err(Failure {
                invariant: Invariant::Panic,
                at: n,
                detail: msg,
            })
        }
    }
}

/// Shortest sub-sequence (by removing chunks, then single keys) that still breaks the same
/// invariant.
fn shrink(demo: &str, keys: Vec<&'static str>, invariant: Invariant) -> Vec<&'static str> {
    let tolerate = known(demo, Invariant::FocusLost);
    let run = |ks: &[&str]| run_with(demo, ks, tolerate);
    let fails = |ks: &[&str]| run(ks).err().is_some_and(|f| f.invariant == invariant);
    let mut keys = match run(&keys) {
        Err(f) if f.invariant != Invariant::KeyboardTrap && f.invariant != Invariant::Panic => {
            keys[..f.at.min(keys.len())].to_vec()
        }
        _ => keys,
    };
    let mut chunk = keys.len() / 2;
    while chunk >= 1 {
        let mut i = 0;
        while i + chunk <= keys.len() {
            let mut shorter = keys.clone();
            shorter.drain(i..i + chunk);
            if fails(&shorter) {
                keys = shorter;
            } else {
                i += chunk;
            }
        }
        chunk /= 2;
    }
    keys
}

/// xorshift64*: deterministic, no crate needed for a few dozen lines' worth of randomness.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        items[(self.next() % items.len() as u64) as usize]
    }
}

/// FNV-1a, so each demo has its own stable seed.
fn hash(s: &str) -> u64 {
    s.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn env_num(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// The keys to press on `demo`: the base set plus its contract's keys.
fn alphabet(demo: &str) -> Vec<&'static str> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src/components");
    let mut keys: Vec<&'static str> = BASE_KEYS.to_vec();
    if let Ok(Some(c)) = contract::Contract::load(&dir, demo) {
        for row in c.keys {
            // A row can name several keys ("Enter / Space").
            for k in row
                .key
                .split(['/', ','])
                .map(str::trim)
                .filter(|k| !k.is_empty())
            {
                if !keys.contains(&k) {
                    keys.push(Box::leak(k.to_string().into_boxed_str()));
                }
            }
        }
    }
    keys
}

fn demos() -> Vec<&'static str> {
    generated::ENTRIES
        .iter()
        .filter(|e| e.has_demo)
        .map(|e| e.name)
        .collect()
}

fn known(demo: &str, invariant: Invariant) -> bool {
    KNOWN
        .iter()
        .any(|(d, i, _, _)| *d == demo && *i == invariant)
}

#[test]
fn keyboard_fuzz_keeps_focus_sane_in_every_demo() {
    let seed = env_num("FUZZ_SEED", 0x5eed);
    let cases = env_num("FUZZ_CASES", 3);
    let len = env_num("FUZZ_LEN", 20) as usize;
    let only = std::env::var("FUZZ").ok();
    let mut failures = Vec::new();
    let mut presses = 0;
    let mut reported = Vec::new();
    for demo in demos() {
        if only.as_deref().is_some_and(|o| o != demo) {
            continue;
        }
        let keys = alphabet(demo);
        let mut rng = Rng::new(seed ^ hash(demo));
        for case in 0..cases {
            let seq: Vec<&'static str> = (0..len).map(|_| rng.pick(&keys)).collect();
            presses += seq.len();
            let tolerate = known(demo, Invariant::FocusLost);
            let outcome = run_with(demo, &seq, tolerate);
            if only.is_some() {
                println!("{demo} #{case}: {} -> {outcome:?}", seq.join(" "));
            }
            let Err(f) = outcome else { continue };
            // One report per demo and invariant: shrinking every case costs minutes.
            if known(demo, f.invariant) || reported.contains(&(demo, f.invariant)) {
                continue;
            }
            reported.push((demo, f.invariant));
            let min = shrink(demo, seq.clone(), f.invariant);
            let detail = run_with(demo, &min, tolerate)
                .err()
                .map_or(f.detail.clone(), |f| f.detail);
            failures.push(format!(
                "{demo}: {:?}: {detail}\n    keys: {}\n    (FUZZ_SEED={seed:#x}, case {case})",
                f.invariant,
                min.join(" ")
            ));
        }
    }
    assert!(presses > 0, "no demo was fuzzed");
    assert!(
        failures.is_empty(),
        "{} keyboard invariant(s) broke:\n{}\n",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn known_gaps_still_reproduce() {
    let mut fixed = Vec::new();
    for (demo, invariant, keys, why) in KNOWN {
        match run(demo, keys) {
            Err(f) if f.invariant == *invariant => {}
            other => fixed.push(format!(
                "{demo} {invariant:?} ({why}): `{}` now gives {other:?}; remove it from KNOWN",
                keys.join(" ")
            )),
        }
    }
    assert!(fixed.is_empty(), "\n{}\n", fixed.join("\n"));
}

/// The harness itself: a demo with nothing focusable is left by one Tab, and the invariants
/// do fire (a sequence that tabs out and back is fine).
#[test]
fn harness_sanity() {
    let r = run("badge", &["Tab", "Shift+Tab", "Escape"]);
    assert!(r.is_ok(), "{r:?}");
    assert!(alphabet("tabs").contains(&"ArrowRight"));
    let mut a = Rng::new(7);
    let mut b = Rng::new(7);
    assert_eq!(a.next(), b.next());
}
