//! Drives the kit's demos on the headless native renderer (Blitz, the engine behind
//! dioxus-native) and checks what a user would see: overlays open where they should,
//! controls change state when clicked, and the dark theme applies. No window or GPU is
//! needed; `common::Harness` explains how.
//!
//! Run it with `cargo test -p native`, not as part of `cargo test --workspace`: there the
//! gallery's `web` feature reaches the motion crate, which then calls JavaScript and panics.
//!
//! Set `NATIVE_SHOTS=1` to also save a CPU-painted PNG of every case to
//! `target/native-headless/` for a human to look at.

mod common;
use common::*;

#[path = "../../docs/src/generated.rs"]
#[rustfmt::skip]
#[allow(dead_code)]
mod generated;

use blitz_traits::shell::ColorScheme;
use dioxus::prelude::*;

/// A demo inside the same shell as `apps/native`.
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

fn shot(h: &Harness, name: &str) {
    if std::env::var_os("NATIVE_SHOTS").is_some() {
        let dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/native-headless");
        h.screenshot(dir.join(format!("{name}.png")));
    }
}

/// Click `trigger`, return the rect of `content` (which must not exist before).
fn open(name: &str, app: fn() -> Element, trigger: &str, content: &str) -> Rect {
    let mut h = Harness::new(app);
    assert!(
        h.rect(content).is_none_or(|r| !r.on_screen()),
        "{name}: {content} visible before the click"
    );
    h.click(trigger);
    shot(&h, name);
    let r = h
        .rect(content)
        .unwrap_or_else(|| panic!("{name}: {content} did not open"));
    assert!(r.on_screen(), "{name}: {content} opened off screen: {r:?}");
    r
}

fn centred(r: Rect) -> bool {
    let (cx, cy) = r.center();
    (cx - WIDTH as f32 / 2.0).abs() < 2.0 && (cy - HEIGHT as f32 / 2.0).abs() < 2.0
}

#[test]
fn modals_are_centred_in_the_window() {
    for (name, app, content) in [
        ("dialog", demo!("dialog"), "[role=dialog]"),
        ("alert_dialog", demo!("alert_dialog"), "[role=alertdialog]"),
        ("command", demo!("command"), "[role=dialog]"),
    ] {
        let r = open(name, app, "button", content);
        assert!(r.inside_window() && centred(r), "{name}: {r:?}");
    }
}

#[test]
fn sheet_spans_the_window_height_at_its_edge() {
    let r = open("sheet", demo!("sheet"), "button", "[role=dialog]");
    assert!(r.inside_window(), "{r:?}");
    assert_eq!((r.x, r.y, r.h), (0.0, 0.0, HEIGHT as f32), "{r:?}");
}

#[test]
fn drawer_sits_on_the_bottom_edge() {
    let r = open("drawer", demo!("drawer"), "button", "[role=dialog]");
    assert!(r.inside_window(), "{r:?}");
    assert_eq!(r.y + r.h, HEIGHT as f32, "{r:?}");
}

#[test]
fn modal_backdrop_covers_the_window() {
    let mut h = Harness::new(demo!("dialog"));
    h.click("button");
    let backdrop = h.rect("[data-state=open]").expect("backdrop");
    assert_eq!(
        backdrop,
        Rect {
            x: 0.0,
            y: 0.0,
            w: WIDTH as f32,
            h: HEIGHT as f32
        }
    );
    // Painted, not just laid out: the backdrop dims the corner of the window.
    let [r, g, b, _] = h.pixel(WIDTH - 4, HEIGHT - 4);
    assert!(
        r < 230 && g < 230 && b < 230,
        "corner not dimmed: {r},{g},{b}"
    );
}

#[test]
fn closing_a_modal_leaves_the_page_clickable() {
    let mut h = Harness::new(demo!("dialog"));
    h.click("button");
    h.click("[role=dialog] button"); // Cancel
    assert!(h.rect("[role=dialog]").is_none(), "dialog still open");
    h.click("button");
    assert!(
        h.rect("[role=dialog]").is_some(),
        "trigger no longer reachable"
    );
}

#[test]
fn anchored_overlays_open_below_their_trigger() {
    for (name, app, trigger, content) in [
        ("popover", demo!("popover"), "button", "[role=dialog]"),
        (
            "dropdown_menu",
            demo!("dropdown_menu"),
            "button",
            "[role=listbox], [role=menu]",
        ),
        ("select", demo!("select"), "button", "[role=listbox]"),
        (
            "date_picker",
            demo!("date_picker"),
            "button",
            "[role=dialog], table",
        ),
        (
            "combobox",
            demo!("combobox"),
            "button, input",
            "[role=listbox]",
        ),
        (
            "multi_select",
            demo!("multi_select"),
            "button",
            "[role=group] ul",
        ),
    ] {
        let h = Harness::new(app);
        let t = h.rect(trigger).expect("trigger");
        drop(h);
        let r = open(name, app, trigger, content);
        assert!(
            r.y >= t.y + t.h - 1.0,
            "{name}: opened above its trigger: {r:?} vs {t:?}"
        );
        assert!(r.h > 30.0, "{name}: collapsed: {r:?}");
    }
}

fn states(h: &Harness) -> Vec<String> {
    let names = [
        "data-state",
        "aria-expanded",
        "aria-checked",
        "aria-pressed",
    ];
    h.doc
        .tree()
        .iter()
        .flat_map(|(_, n)| {
            names
                .iter()
                .filter_map(|a| n.attr(blitz_dom::LocalName::from(*a)).map(str::to_owned))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn controls_change_state_when_clicked() {
    for (name, app, target) in [
        ("checkbox", demo!("checkbox"), "button"),
        ("switch", demo!("switch"), "button"),
        ("radio_group", demo!("radio_group"), "[role=radio]"),
        ("toggle", demo!("toggle"), "button"),
        ("toggle_group", demo!("toggle_group"), "button"),
        ("accordion", demo!("accordion"), "button"),
        ("collapsible", demo!("collapsible"), "button"),
        ("tabs", demo!("tabs"), "[role=tab][data-state=inactive]"),
    ] {
        let mut h = Harness::new(app);
        let before = states(&h);
        h.click(target);
        assert_ne!(
            before,
            states(&h),
            "{name}: clicking {target} changed nothing"
        );
    }
}

#[test]
fn progress_demo_advances_without_javascript() {
    let mut h = Harness::new(demo!("progress"));
    let before = h.find("[role=progressbar]").and_then(|id| {
        h.doc
            .get_node(id)?
            .attr(blitz_dom::LocalName::from("aria-valuenow"))
            .map(str::to_owned)
    });
    h.click("button");
    let after = h.find("[role=progressbar]").and_then(|id| {
        h.doc
            .get_node(id)?
            .attr(blitz_dom::LocalName::from("aria-valuenow"))
            .map(str::to_owned)
    });
    assert_ne!(before, after);
}

fn card_background(h: &Harness) -> [u8; 4] {
    let r = h.rect("[data-slot=card], main div div").expect("card");
    h.pixel((r.x + 12.0) as u32, (r.y + r.h / 2.0) as u32)
}

fn is_dark([r, g, b, _]: [u8; 4]) -> bool {
    (r as u32 + g as u32 + b as u32) < 3 * 64
}

#[test]
fn dark_theme_follows_the_window_colour_scheme() {
    let light = Harness::with_scheme(demo!("card"), ColorScheme::Light);
    let dark = Harness::with_scheme(demo!("card"), ColorScheme::Dark);
    shot(&dark, "card_dark");
    assert!(
        !is_dark(card_background(&light)),
        "{:?}",
        card_background(&light)
    );
    assert!(
        is_dark(card_background(&dark)),
        "{:?}",
        card_background(&dark)
    );
}

#[test]
fn data_theme_on_any_element_switches_its_subtree() {
    fn app() -> Element {
        rsx! {
            ui::ThemeStyles {}
            main { style: "padding: 24px; max-width: 760px;",
                div { "data-theme": "dark", {generated::demo("card")} }
            }
        }
    }
    let h = Harness::with_scheme(app, ColorScheme::Light);
    shot(&h, "card_data_theme");
    assert!(is_dark(card_background(&h)), "{:?}", card_background(&h));
}
