//! The showcase app (`apps/showcase`) on the headless native renderer, at a desktop window
//! size and at a phone size: the same code must lay out, navigate, validate a form, open
//! overlays and switch to dark on both. `common::Harness` explains the setup.
//!
//! `NATIVE_SHOTS=1 cargo test -p native --test showcase` saves PNGs of each case to
//! `target/native-headless/showcase-*.png`.

mod common;
use blitz_traits::shell::ColorScheme;
use common::*;

/// A phone in portrait (CSS pixels), below the showcase's 640px breakpoint.
const PHONE: (u32, u32) = (390, 844);

fn showcase_css() -> String {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../showcase/assets/showcase.css");
    std::fs::read_to_string(path).expect("showcase.css")
}

fn desktop() -> Harness {
    Harness::with_viewport(
        showcase::App,
        WIDTH,
        HEIGHT,
        ColorScheme::Light,
        &showcase_css(),
    )
}

fn phone() -> Harness {
    Harness::with_viewport(
        showcase::App,
        PHONE.0,
        PHONE.1,
        ColorScheme::Light,
        &showcase_css(),
    )
}

fn shot(h: &Harness, name: &str) {
    if std::env::var_os("NATIVE_SHOTS").is_some() {
        let dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/native-headless");
        h.screenshot(dir.join(format!("showcase-{name}.png")));
    }
}

fn visible(h: &Harness, selector: &str) -> bool {
    h.rect(selector).is_some_and(|r| r.w > 0.0 && r.h > 0.0)
}

fn text(h: &Harness, selector: &str) -> String {
    let id = h.find(selector).unwrap_or_else(|| panic!("no {selector}"));
    h.doc.get_node(id).expect("node").text_content()
}

/// Scroll `selector` into the window, clear of the phone's tab bar, then click it.
fn press(h: &mut Harness, selector: &str) {
    h.scroll_into_view(selector);
    if let (Some(r), Some(bar)) = (h.rect(selector), h.rect(".sc-bottom")) {
        if bar.h > 0.0 && r.y + r.h > bar.y {
            h.doc
                .scroll_viewport_by(0.0, -(r.y + r.h - bar.y + 16.0) as f64);
            h.settle();
        }
    }
    h.click(selector);
}

/// Open a page through whichever navigation the layout shows.
fn go(h: &mut Harness, page: &str) {
    if visible(h, ".sc-rail") {
        h.click(&format!("#rail-{page}"));
    } else {
        let n = ["home", "forms", "overlays", "settings"]
            .iter()
            .position(|p| *p == page)
            .expect("page")
            + 1;
        h.click(&format!(".sc-bottom li:nth-child({n}) button"));
    }
}

#[test]
fn says_it_runs_on_the_native_renderer() {
    let h = desktop();
    assert_eq!(text(&h, "#renderer").trim(), "Native (Blitz)");
}

#[test]
fn desktop_gets_a_side_rail_and_phone_a_bottom_tab_bar() {
    let h = desktop();
    shot(&h, "desktop-home");
    assert!(visible(&h, ".sc-rail"), "no rail on desktop");
    assert!(!visible(&h, ".sc-bottom"), "tab bar shown on desktop");
    // Two stat cards side by side.
    let cards: Vec<_> = (1..=3)
        .map(|i| {
            h.rect(&format!(".sc-stats > :nth-child({i})"))
                .expect("stat")
        })
        .collect();
    assert!((cards[0].y - cards[1].y).abs() < 1.0 && cards[1].x > cards[0].x);

    let h = phone();
    shot(&h, "phone-home");
    assert!(!visible(&h, ".sc-rail"), "rail shown on a phone");
    let bar = h.rect(".sc-bottom").expect("tab bar");
    assert!(bar.w > 0.0 && bar.h > 40.0, "tab bar collapsed: {bar:?}");
    assert!(
        (bar.y + bar.h - PHONE.1 as f32).abs() < 1.5,
        "tab bar not on the bottom edge: {bar:?}"
    );
    assert!(
        (bar.w - PHONE.0 as f32).abs() < 1.5,
        "tab bar not full width: {bar:?}"
    );
    // Stat cards stack.
    let a = h.rect(".sc-stats > :nth-child(1)").expect("stat");
    let b = h.rect(".sc-stats > :nth-child(2)").expect("stat");
    assert!(
        b.y > a.y + a.h - 1.0,
        "stat cards did not stack: {a:?} {b:?}"
    );
    assert!(
        a.x + a.w <= PHONE.0 as f32 + 0.5,
        "card wider than the phone: {a:?}"
    );
}

#[test]
fn every_page_opens_from_the_rail_and_the_tab_bar() {
    for (size, mut h) in [("desktop", desktop()), ("phone", phone())] {
        for (page, marker) in [
            ("forms", "#workspace-form"),
            ("overlays", "#open-dialog"),
            ("settings", "[role=tablist]"),
            ("home", "#progress-add"),
        ] {
            go(&mut h, page);
            shot(&h, &format!("{size}-{page}"));
            assert!(
                visible(&h, marker),
                "{size}: {page} did not open ({marker})"
            );
        }
    }
}

#[test]
fn the_form_validates_then_saves_with_a_toast() {
    // The toast's timer needs a Tokio runtime to register with, as in the windowed shell.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("runtime");
    let _rt = rt.enter();
    for (size, mut h) in [("desktop", desktop()), ("phone", phone())] {
        go(&mut h, "forms");
        press(&mut h, "#save");
        assert!(
            h.find("input[aria-invalid=true]").is_some(),
            "{size}: empty form not marked invalid"
        );
        assert!(h.find("#saved").is_none());

        press(&mut h, "#workspace-form input");
        h.type_text("Acme");
        press(&mut h, "#workspace-form input[type=email]");
        h.type_text("ops@acme.dev");
        press(&mut h, "#save");
        shot(&h, &format!("{size}-form-saved"));
        assert!(
            h.find("input[aria-invalid=true]").is_none(),
            "{size}: still invalid"
        );
        let saved = text(&h, "#saved");
        assert!(saved.contains("Acme on Team, 5 seats"), "{size}: {saved}");
        assert!(
            h.find("[data-type=success], .toast, [role=region] li")
                .is_some(),
            "{size}: no toast"
        );
    }
}

#[test]
fn overlays_open_inside_the_window_at_both_sizes() {
    for (size, make) in [("desktop", desktop as fn() -> Harness), ("phone", phone)] {
        for (trigger, content) in [
            ("#open-dialog", "[role=dialog]"),
            ("#open-sheet", "[role=dialog]"),
            ("#open-popover", "[role=dialog]"),
            ("#open-menu", "[role=listbox], [role=menu]"),
        ] {
            let mut h = make();
            go(&mut h, "overlays");
            h.click(trigger);
            shot(
                &h,
                &format!("{size}-{}", trigger.trim_start_matches("#open-")),
            );
            let r = h
                .rect(content)
                .unwrap_or_else(|| panic!("{size}: {trigger} opened nothing"));
            assert!(r.w > 0.0 && r.h > 0.0, "{size}: {trigger}: {r:?}");
            assert!(
                r.x >= -0.5 && r.x + r.w <= h.width as f32 + 0.5 && r.y >= -0.5,
                "{size}: {trigger} spills out of the window: {r:?}"
            );
        }
    }
}

fn is_dark([r, g, b, _]: [u8; 4]) -> bool {
    (r as u32 + g as u32 + b as u32) < 3 * 64
}

#[test]
fn the_dark_mode_switch_works_without_javascript() {
    for (size, mut h) in [("desktop", desktop()), ("phone", phone())] {
        // A point on the page background, below the header.
        let (x, y) = (h.width - 8, 80);
        assert!(!is_dark(h.pixel(x, y)), "{size}: dark before the switch");
        h.click(".sc-theme button");
        shot(&h, &format!("{size}-dark"));
        assert!(
            is_dark(h.pixel(x, y)),
            "{size}: switch did not turn the page dark"
        );
    }
}
