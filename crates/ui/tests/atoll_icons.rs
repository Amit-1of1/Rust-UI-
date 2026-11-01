//! Atoll's own icon set: every icon renders, matches its SVG source, and carries what the
//! design-system rules ask for (a default accessible name, keywords, a category, and reserved
//! meanings that are not shared).

use dioxus::prelude::*;
use std::collections::HashSet;
use std::path::Path;
use ui::icons::{atoll, Icon, IconData, IconNode};

fn render_icon(icon: IconData) -> String {
    #[derive(Clone, PartialEq)]
    struct One(IconData);
    let mut dom = VirtualDom::new_with_props(|p: One| rsx! { Icon { icon: p.0 } }, One(icon));
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn source_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/atoll"))
}

#[test]
fn every_atoll_icon_renders_its_shapes() {
    assert!(atoll::ALL.len() >= 500, "{}", atoll::ALL.len());
    for icon in atoll::ALL.iter() {
        let html = render_icon(*icon);
        assert!(
            html.contains(&format!("data-icon=\"{}\"", icon.name)),
            "{html}"
        );
        assert!(html.contains("viewBox=\"0 0 24 24\""), "{html}");
        assert!(html.contains("aria-hidden=\"true\""), "{html}");
        let shapes = [
            "<path ",
            "<circle ",
            "<ellipse ",
            "<rect ",
            "<line ",
            "<polyline ",
            "<polygon ",
        ]
        .iter()
        .map(|t| html.matches(t).count())
        .sum::<usize>();
        assert_eq!(shapes, icon.nodes.len(), "{}: {html}", icon.name);
    }
}

#[test]
fn solid_shapes_fill_without_a_stroke() {
    let html = render_icon(atoll::THEME);
    assert!(
        html.contains("fill=\"currentColor\" stroke=\"none\""),
        "{html}"
    );
    assert!(atoll::THEME
        .nodes
        .iter()
        .any(|n| matches!(n, IconNode::Fill { .. })));
}

#[test]
fn generated_set_matches_the_svg_sources() {
    let mut files: Vec<String> = std::fs::read_dir(source_dir())
        .unwrap()
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter_map(|n| n.strip_suffix(".svg").map(str::to_owned))
        .collect();
    files.sort();
    let names: Vec<&str> = atoll::ALL.iter().map(|i| i.name).collect();
    assert_eq!(files, names, "re-run node scripts/gen-atoll-icons.mjs");
    for icon in atoll::ALL.iter() {
        let svg = std::fs::read_to_string(source_dir().join(format!("{}.svg", icon.name))).unwrap();
        let shapes = svg.matches("/>").count();
        assert_eq!(
            shapes,
            icon.nodes.len(),
            "{} changed: re-run node scripts/gen-atoll-icons.mjs",
            icon.name
        );
        assert!(svg.contains("viewBox=\"0 0 24 24\""), "{}", icon.name);
    }
}

#[test]
fn index_lines_up_with_the_icons_and_is_complete() {
    assert_eq!(atoll::INDEX.len(), atoll::ALL.len());
    for (meta, icon) in atoll::INDEX.iter().zip(atoll::ALL.iter()) {
        assert_eq!(meta.name, icon.name);
        assert!(
            !meta.label.is_empty(),
            "{}: no default accessible name",
            meta.name
        );
        assert!(meta.keywords.len() >= 2, "{}: needs keywords", meta.name);
        assert!(
            atoll::CATEGORIES.contains(&meta.category),
            "{}: {}",
            meta.name,
            meta.category
        );
        assert_eq!(atoll::meta(meta.name), Some(meta));
    }
    assert_eq!(atoll::meta("not-an-icon"), None);
}

#[test]
fn names_say_what_an_icon_means_and_are_unique() {
    let mut seen = HashSet::new();
    for icon in atoll::ALL.iter() {
        assert!(seen.insert(icon.name), "duplicate {}", icon.name);
        // Purpose, not shape: no icon is named after the geometry it happens to use.
        for shape in ["triangle", "circle", "square", "arrow", "chevron", "line"] {
            assert!(
                !icon.name.split('-').any(|w| w == shape),
                "{} is named after its shape",
                icon.name
            );
        }
    }
}

#[test]
fn reserved_meanings_are_not_shared() {
    let mut meanings = HashSet::new();
    let mut count = 0;
    for meta in atoll::INDEX.iter() {
        if let Some(r) = meta.reserved {
            count += 1;
            assert!(meanings.insert(r), "two icons are reserved for {r}");
        }
    }
    assert!(count >= 20, "{count}");
    // The everyday system actions are all reserved.
    for name in [
        "add", "close", "delete", "edit", "search", "settings", "back", "next",
    ] {
        assert!(atoll::meta(name).unwrap().reserved.is_some(), "{name}");
    }
}

#[test]
fn directional_icons_mirror_in_rtl() {
    for name in ["back", "next", "sign-in", "sign-out", "page-next"] {
        let icon = atoll::ALL.iter().find(|i| i.name == name).unwrap();
        assert!(icon.mirror_rtl, "{name}");
    }
    const { assert!(!atoll::EXPAND.mirror_rtl) };
}

#[test]
fn components_render_like_lucide_ones() {
    let mut dom = VirtualDom::new(|| rsx! { atoll::Crate { label: "Crate" } });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("data-icon=\"crate\""), "{html}");
    assert!(html.contains("role=\"img\""), "{html}");
    assert!(html.contains("aria-label=\"Crate\""), "{html}");
    assert!(html.contains("stroke-linecap=\"round\""), "{html}");
}
