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

// ---- needs the patched renderer (docs/FORKS.md) ---------------------------------------------

fn attr(h: &Harness, selector: &str, name: &str) -> Option<String> {
    let id = h.find(selector)?;
    h.doc
        .get_node(id)?
        .attr(blitz_dom::LocalName::from(name))
        .map(str::to_owned)
}

#[test]
fn tooltip_opens_on_hover() {
    let mut h = Harness::new(demo!("tooltip"));
    assert!(h.rect("[role=tooltip]").is_none_or(|r| !r.on_screen()));
    h.move_to(1000.0, 700.0);
    h.hover("[tabindex]");
    shot(&h, "tooltip");
    let r = h.rect("[role=tooltip]").expect("tooltip did not open");
    assert!(r.on_screen(), "{r:?}");
    // Centred on its trigger: needs `translate(-50%)`, which upstream Blitz dropped.
    let t = h.rect("[tabindex]").unwrap();
    let painted = h.doc.get_node(h.find("[role=tooltip]").unwrap()).unwrap();
    let shift = painted.css_transform_2d().map_or(0.0, |m| m[4] as f32);
    assert!(
        ((r.x + shift + r.w / 2.0) - (t.x + t.w / 2.0)).abs() < 2.0,
        "tooltip {r:?} (shift {shift}) not centred on {t:?}"
    );
    h.move_to(1000.0, 700.0);
    assert!(
        h.rect("[role=tooltip]").is_none_or(|r| !r.on_screen()),
        "tooltip stayed open"
    );
}

#[test]
fn hover_card_opens_on_hover() {
    let mut h = Harness::new(demo!("hover_card"));
    h.move_to(1000.0, 700.0);
    h.hover("[role=button]");
    h.settle();
    shot(&h, "hover_card");
    let r = h.rect("[role=tooltip]");
    assert!(r.is_some_and(|r| r.on_screen()), "{r:?}");
}

#[test]
fn slider_jumps_to_a_click_on_its_track() {
    let mut h = Harness::new(demo!("slider"));
    let before = attr(&h, "[role=slider]", "aria-valuenow");
    let track = h.rect("[role=group]").expect("slider root");
    h.click_at(track.x + track.w * 0.9, track.y + track.h / 2.0);
    h.settle();
    shot(&h, "slider");
    let after = attr(&h, "[role=slider]", "aria-valuenow");
    assert_ne!(before, after);
}

#[test]
fn menubar_opens_on_click() {
    let r = open(
        "menubar",
        demo!("menubar"),
        "[role=menubar] button",
        "[role=menu][id]",
    );
    assert!(r.h > 30.0, "{r:?}");
}

#[test]
fn tab_key_moves_focus() {
    let mut h = Harness::new(demo!("button_group"));
    h.key(Key::Tab, Code::Tab, Modifiers::empty());
    let first = h.focused().expect("nothing focused after Tab");
    h.key(Key::Tab, Code::Tab, Modifiers::empty());
    let second = h.focused().expect("focus lost");
    assert_ne!(first, second);
    h.key(Key::Tab, Code::Tab, Modifiers::SHIFT);
    assert_eq!(h.focused(), Some(first));
}

#[test]
fn arrow_keys_move_focus_between_tabs() {
    let mut h = Harness::new(demo!("tabs"));
    h.click("[role=tab]");
    let first = h.focused().expect("tab not focused by click");
    h.key(Key::ArrowRight, Code::ArrowRight, Modifiers::empty());
    let next = h.focused().expect("focus lost");
    assert_ne!(first, next, "ArrowRight did not move focus");
    assert_eq!(
        h.doc
            .get_node(next)
            .and_then(|n| n.attr(blitz_dom::LocalName::from("role"))),
        Some("tab")
    );
}

#[test]
fn enter_activates_a_focused_button() {
    let mut h = Harness::new(demo!("dialog"));
    h.key(Key::Tab, Code::Tab, Modifiers::empty());
    h.key(Key::Enter, Code::Enter, Modifiers::empty());
    assert!(
        h.rect("[role=dialog]").is_some(),
        "Enter did not open the dialog"
    );
}

// ---- needs the patched primitives (docs/FORKS.md, no-script fallbacks) ----------------------

fn inside(h: &Harness, node: Option<usize>, selector: &str) -> bool {
    let Some(root) = h.find(selector) else {
        return false;
    };
    let mut node = node;
    while let Some(id) = node {
        if id == root {
            return true;
        }
        node = h.doc.get_node(id).and_then(|n| n.parent);
    }
    false
}

#[test]
fn escape_closes_a_dialog() {
    let mut h = Harness::new(demo!("dialog"));
    h.click("button");
    assert!(h.rect("[role=dialog]").is_some(), "dialog did not open");
    assert!(
        inside(&h, h.focused(), "[role=dialog]"),
        "focus did not move into the dialog"
    );
    h.key(Key::Escape, Code::Escape, Modifiers::empty());
    assert!(h.rect("[role=dialog]").is_none(), "Escape did not close it");
}

#[test]
fn clicking_outside_closes_a_dialog() {
    let mut h = Harness::new(demo!("dialog"));
    h.click("button");
    h.click_at(10.0, 10.0); // on the backdrop
    assert!(h.rect("[role=dialog]").is_none(), "dialog still open");
}

#[test]
fn clicking_inside_keeps_a_dialog_open() {
    let mut h = Harness::new(demo!("dialog"));
    h.click("button");
    let d = h.rect("[role=dialog]").unwrap();
    h.click_at(d.x + d.w / 2.0, d.y + 8.0);
    assert!(h.rect("[role=dialog]").is_some(), "dialog closed");
}

#[test]
fn tab_stays_inside_a_modal_dialog() {
    let mut h = Harness::new(demo!("dialog"));
    h.click("button");
    for i in 0..8 {
        h.key(Key::Tab, Code::Tab, Modifiers::empty());
        assert!(
            h.rect("[role=dialog]").is_some(),
            "Tab {i} closed the dialog"
        );
        assert!(
            inside(&h, h.focused(), "[role=dialog]"),
            "Tab {i} moved focus out of the dialog"
        );
    }
}

#[test]
fn escape_and_outside_clicks_close_a_popover() {
    let mut h = Harness::new(demo!("popover"));
    h.click("button");
    assert!(h.rect("[role=dialog]").is_some(), "popover did not open");
    h.key(Key::Escape, Code::Escape, Modifiers::empty());
    assert!(h.rect("[role=dialog]").is_none(), "Escape did not close it");
    h.click("button");
    assert!(h.rect("[role=dialog]").is_some(), "popover did not reopen");
    h.click_at(WIDTH as f32 - 10.0, HEIGHT as f32 - 10.0);
    assert!(
        h.rect("[role=dialog]").is_none(),
        "outside click did not close it"
    );
}

#[test]
fn slider_follows_a_drag() {
    let mut h = Harness::new(demo!("slider"));
    let track = h.rect("[role=group]").expect("slider root");
    let y = track.y + track.h / 2.0;
    let value = |h: &Harness| -> f64 {
        attr(h, "[role=slider]", "aria-valuenow")
            .unwrap()
            .parse()
            .unwrap()
    };
    let start = (track.x + track.w * 0.2, y);
    // The pointer leaves the track on the way; the drag keeps following it.
    h.drag(
        start,
        &[
            (track.x + track.w * 0.5, y + 80.0),
            (track.x + track.w * 0.8, y),
        ],
    );
    shot(&h, "slider_drag");
    let v = value(&h);
    assert!((70.0..=90.0).contains(&v), "value {v} after drag");
    assert!(
        h.find("[data-pointer-shield]").is_none(),
        "drag did not end"
    );
}

#[test]
fn context_menu_opens_on_right_click_and_closes() {
    let mut h = Harness::new(demo!("context_menu"));
    let (x, y) = h.rect("[aria-haspopup=menu]").expect("trigger").center();
    let open_menu = |h: &mut Harness| {
        h.right_click_at(x, y);
        let m = h.rect("[role=menu]").expect("menu did not open");
        assert!(m.on_screen(), "{m:?}");
        assert!(
            (m.x - x).abs() < 2.0 && (m.y - y).abs() < 2.0,
            "menu at {m:?}, click at {x},{y}"
        );
    };
    open_menu(&mut h);
    shot(&h, "context_menu");
    h.key(Key::Escape, Code::Escape, Modifiers::empty());
    assert!(h.rect("[role=menu]").is_none(), "Escape did not close it");
    open_menu(&mut h);
    h.click_at(WIDTH as f32 - 10.0, HEIGHT as f32 - 10.0);
    assert!(
        h.rect("[role=menu]").is_none(),
        "outside click did not close it"
    );
}

#[test]
fn avatar_shows_its_image_once_loaded() {
    use blitz_dom::net::Resource;
    use blitz_dom::util::ImageType;
    let mut h = Harness::new(demo!("avatar"));
    let img = h.find("img").expect("avatar image");
    let state = |h: &Harness| {
        let mut node = h.doc.get_node(img).and_then(|n| n.parent);
        while let Some(id) = node {
            let n = h.doc.get_node(id).unwrap();
            if let Some(state) = n.attr(blitz_dom::LocalName::from("data-state")) {
                return state.to_owned();
            }
            node = n.parent;
        }
        panic!("no avatar root");
    };
    assert_eq!(state(&h), "loading");
    // What the window's network provider does once the image has been fetched.
    h.doc.load_resource(Resource::Image(
        img,
        ImageType::Image,
        1,
        1,
        std::sync::Arc::new(vec![255; 4]),
    ));
    h.settle();
    assert_eq!(state(&h), "loaded");
}

#[test]
fn drag_and_drop_list_reorders_with_the_mouse() {
    let mut h = Harness::new(demo!("drag_and_drop_list"));
    let items = |h: &Harness| -> Vec<usize> {
        h.doc
            .query_selector_all("[aria-roledescription='sortable item']")
            .unwrap()
            .into_iter()
            .collect()
    };
    let before = items(&h);
    assert!(before.len() >= 3, "{before:?}");
    let rect = |h: &Harness, id: usize| {
        let n = h.doc.get_node(id).unwrap();
        let p = n.absolute_position(0.0, 0.0);
        let s = n.final_layout.size;
        (p.x + s.width / 2.0, p.y, s.height)
    };
    let (x, y0, height) = rect(&h, before[0]);
    let (_, y2, h2) = rect(&h, before[2]);
    // Press the first item, drag it into the lower half of the third, release.
    h.drag(
        (x, y0 + height / 2.0),
        &[(x, y0 + height / 2.0 + 10.0), (x, y2 + h2 * 0.75)],
    );
    shot(&h, "drag_and_drop_list");
    let text = |h: &Harness, ids: &[usize]| -> Vec<String> {
        ids.iter()
            .map(|&id| h.doc.get_node(id).unwrap().text_content())
            .collect()
    };
    let (was, now) = (text(&h, &before), text(&h, &items(&h)));
    assert_eq!(
        now[..3],
        [was[1].clone(), was[2].clone(), was[0].clone()],
        "order {now:?}"
    );
}

#[test]
fn drag_and_drop_list_reorders_with_the_keyboard() {
    let mut h = Harness::new(demo!("drag_and_drop_list"));
    let texts = |h: &Harness| -> Vec<String> {
        h.doc
            .query_selector_all("[aria-roledescription='sortable item']")
            .unwrap()
            .into_iter()
            .map(|id| h.doc.get_node(id).unwrap().text_content())
            .collect()
    };
    let was = texts(&h);
    h.click("[aria-roledescription='sortable item']");
    h.key(Key::Enter, Code::Enter, Modifiers::empty());
    h.key(Key::ArrowDown, Code::ArrowDown, Modifiers::empty());
    h.key(Key::Enter, Code::Enter, Modifiers::empty());
    let now = texts(&h);
    assert_eq!(now.len(), was.len(), "an item went missing: {now:?}");
    assert_eq!(now[..2], [was[1].clone(), was[0].clone()]);
}

#[test]
fn theme_customizer_switches_the_theme_without_javascript() {
    let mut h = Harness::new(demo!("theme_customizer"));
    h.click("input[name=theme-mode][value=dark]");
    assert_eq!(
        attr(&h, "[data-theme]", "data-theme").as_deref(),
        Some("dark"),
        "no ThemeScope picked up the mode"
    );
    h.click("input[name=theme-accent][value=blue]");
    assert!(
        h.find("[style*=contents][data-accent=blue]").is_some(),
        "accent not applied"
    );
}
