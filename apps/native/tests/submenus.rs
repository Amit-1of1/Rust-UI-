//! Nested submenus on the headless native renderer (Blitz): the pointer side that the keyboard
//! contracts don't cover. Hover intent runs on real timers (`dioxus-sdk-time`, tokio off the
//! web), so each test drives a current-thread tokio runtime and waits through the delays.
//!
//! Run with `cargo test -p native --test submenus`.

mod common;
use common::*;

#[path = "../../docs/src/generated.rs"]
#[rustfmt::skip]
#[allow(dead_code)]
mod generated;

use std::time::Duration;

use dioxus::prelude::*;

macro_rules! demo {
    ($name:literal) => {{
        fn app() -> Element {
            rsx! {
                ui::ThemeStyles {}
                main { style: "padding: 24px; max-width: 760px;",
                    div { {generated::demo($name)} }
                }
            }
        }
        app as fn() -> Element
    }};
}

/// A runtime for the submenu timers. Keep the guard alive for the whole test.
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("tokio runtime")
}

/// Let `ms` of real time pass (timers fire), then run what they woke.
fn wait(rt: &tokio::runtime::Runtime, h: &mut Harness, ms: u64) {
    rt.block_on(tokio::time::sleep(Duration::from_millis(ms)));
    h.settle();
}

fn attr(h: &Harness, selector: &str, name: &str) -> String {
    let id = h.find(selector).unwrap_or_else(|| panic!("no {selector}"));
    h.doc
        .get_node(id)
        .and_then(|node| node.attr(blitz_dom::LocalName::from(name)))
        .unwrap_or_default()
        .to_owned()
}

fn text(h: &Harness, selector: &str) -> String {
    let id = h.find(selector).unwrap_or_else(|| panic!("no {selector}"));
    h.doc.get_node(id).expect("node").text_content()
}

const SUB_TRIGGER: &str = "[role=menuitem][aria-haspopup=menu]";
const SUB_MENU: &str = "[role=menu] [role=menu]";

fn open_context_menu(h: &mut Harness) {
    let (x, y) = h.rect("[aria-haspopup=menu]").expect("trigger").center();
    h.right_click_at(x, y);
    assert!(h.rect("[role=menu]").is_some(), "context menu did not open");
}

#[test]
fn hovering_a_submenu_item_opens_it_after_a_pause() {
    let rt = runtime();
    let _guard = rt.enter();
    let mut h = Harness::new(demo!("context_menu"));
    open_context_menu(&mut h);
    h.hover(SUB_TRIGGER);
    assert!(h.rect(SUB_MENU).is_none(), "opened before the hover delay");
    wait(&rt, &mut h, 200);
    let sub = h.rect(SUB_MENU).expect("submenu did not open on hover");
    let trigger = h.rect(SUB_TRIGGER).expect("trigger");
    assert!(sub.on_screen(), "{sub:?}");
    // Beside its trigger, at the inline end.
    assert!(
        sub.x >= trigger.x + trigger.w - 1.0,
        "sub {sub:?} trigger {trigger:?}"
    );
    assert!(
        (sub.y - trigger.y).abs() < 16.0,
        "sub {sub:?} trigger {trigger:?}"
    );
    assert_eq!(attr(&h, SUB_TRIGGER, "aria-expanded"), "true");
}

#[test]
fn the_pointer_can_cross_to_the_submenu_but_leaving_closes_it() {
    let rt = runtime();
    let _guard = rt.enter();
    let mut h = Harness::new(demo!("context_menu"));
    open_context_menu(&mut h);
    h.hover(SUB_TRIGGER);
    wait(&rt, &mut h, 200);
    // Over the item below on the way, then into the submenu inside the grace period.
    h.hover("[role=menu] > [role=menuitem]:last-child");
    wait(&rt, &mut h, 100);
    h.hover(&format!("{SUB_MENU} [role=menuitem]"));
    wait(&rt, &mut h, 400);
    assert!(
        h.rect(SUB_MENU).is_some(),
        "submenu closed while the pointer was on it"
    );
    // Away from both: closed after the grace period, the menu itself stays.
    h.hover("[role=menu] > [role=menuitem]");
    wait(&rt, &mut h, 400);
    assert!(
        h.rect(SUB_MENU).is_none(),
        "submenu stayed open after the pointer left"
    );
    assert!(h.rect("[role=menu]").is_some(), "the whole menu closed");
}

#[test]
fn choosing_a_submenu_item_closes_the_whole_menu() {
    let rt = runtime();
    let _guard = rt.enter();
    let mut h = Harness::new(demo!("context_menu"));
    open_context_menu(&mut h);
    h.click(SUB_TRIGGER);
    assert!(h.rect(SUB_MENU).is_some(), "click did not open the submenu");
    h.click(&format!("{SUB_MENU} [role=menuitem]"));
    wait(&rt, &mut h, 50);
    assert!(
        h.rect("[role=menu]").is_none(),
        "menu still open after choosing"
    );
    let text = text(&h, "main");
    assert!(text.contains("Selected: copy-link"), "{text}");
}

#[test]
fn dropdown_submenus_nest_and_close_one_level_at_a_time() {
    let rt = runtime();
    let _guard = rt.enter();
    let mut h = Harness::new(demo!("dropdown_menu"));
    h.click("button");
    let sub_trigger = "[role=option][data-state]";
    h.click(sub_trigger);
    let level2 = "[role=listbox] [role=group]";
    assert!(h.rect(level2).is_some(), "first submenu did not open");
    // "Project" is the nested submenu's trigger.
    h.click(&format!("{level2} [role=option][data-state]"));
    let level3 = "[role=listbox] [role=group] [role=group]";
    assert!(h.rect(level3).is_some(), "nested submenu did not open");
    // ArrowRight on an open submenu's item moves focus into it.
    h.key(Key::ArrowRight, Code::ArrowRight, Modifiers::empty());
    let focused = h.focused().expect("focus");
    assert_eq!(Some(focused), h.find(&format!("{level3} [role=option]")));
    h.key(Key::Escape, Code::Escape, Modifiers::empty());
    assert!(
        h.rect(level3).is_none(),
        "Escape did not close the nested submenu"
    );
    assert!(
        h.rect(level2).is_some(),
        "Escape closed more than one level"
    );
    h.key(Key::ArrowLeft, Code::ArrowLeft, Modifiers::empty());
    assert!(
        h.rect(level2).is_none(),
        "ArrowLeft did not close the submenu"
    );
    assert!(h.rect("[role=listbox]").is_some(), "the menu itself closed");
}

#[test]
fn menubar_submenu_opens_from_its_menu() {
    let rt = runtime();
    let _guard = rt.enter();
    let mut h = Harness::new(demo!("menubar"));
    h.click("[role=menubar] button");
    h.hover(SUB_TRIGGER);
    wait(&rt, &mut h, 200);
    let sub = h
        .rect(SUB_MENU)
        .expect("menubar submenu did not open on hover");
    assert!(sub.on_screen(), "{sub:?}");
    h.click(&format!("{SUB_MENU} [role=menuitem]"));
    assert!(
        h.rect("[role=menubar] [role=menu] [role=menu]").is_none(),
        "menubar menu still open after choosing"
    );
}
