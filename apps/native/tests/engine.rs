//! Renderer-level checks for the patched Blitz / dioxus-native-dom crates on our forks
//! (`docs/FORKS.md`).
//! Each test is a few lines of markup that failed on the upstream releases (see
//! `docs/FORKS.md` for the fix). Run with `cargo test -p native`.

mod common;
use common::*;

use dioxus::prelude::*;

fn attr(h: &Harness, selector: &str, name: &str) -> Option<String> {
    let id = h.find(selector)?;
    h.doc
        .get_node(id)?
        .attr(blitz_dom::LocalName::from(name))
        .map(str::to_owned)
}

fn rgb(h: &Harness, x: f32, y: f32) -> [u8; 3] {
    let [r, g, b, _] = h.pixel(x as u32, y as u32);
    [r, g, b]
}

const RED: [u8; 3] = [255, 0, 0];
const BLUE: [u8; 3] = [0, 0, 255];

// ---- events ---------------------------------------------------------------------------

#[test]
fn pointer_events_reach_dioxus_handlers() {
    fn app() -> Element {
        let mut log = use_signal(Vec::<String>::new);
        rsx! {
            div {
                id: "target",
                style: "margin: 40px; width: 200px; height: 100px;",
                "data-log": log.read().join(","),
                onpointerdown: move |e| log.write().push(format!("down@{}", e.element_coordinates().x as i32)),
                onpointermove: move |_| {
                    if log.read().last().map(String::as_str) != Some("move") {
                        log.write().push("move".into())
                    }
                },
                onpointerup: move |_| log.write().push("up".into()),
            }
        }
    }
    let mut h = Harness::new(app);
    let r = h.rect("#target").unwrap();
    h.click_at(r.x + 10.0, r.y + 10.0);
    let log = attr(&h, "#target", "data-log").unwrap();
    assert_eq!(log, "move,down@10,up");
}

#[test]
fn hover_fires_enter_and_leave() {
    fn app() -> Element {
        let mut log = use_signal(Vec::<&'static str>::new);
        rsx! {
            div { id: "log", "data-log": log.read().join(",") }
            div {
                id: "outer",
                style: "margin: 40px; width: 200px; height: 100px; padding: 20px;",
                onmouseenter: move |_| log.write().push("enter"),
                onmouseleave: move |_| log.write().push("leave"),
                onpointerenter: move |_| log.write().push("penter"),
                div { id: "inner", style: "width: 50px; height: 50px;" }
            }
        }
    }
    let mut h = Harness::new(app);
    let r = h.rect("#outer").unwrap();
    let inner = h.rect("#inner").unwrap();
    h.move_to(r.x + 5.0, r.y + 5.0);
    // Moving into a child does not leave the parent (mouseenter/leave don't bubble).
    h.move_to(inner.x + 5.0, inner.y + 5.0);
    h.move_to(700.0, 700.0);
    assert_eq!(attr(&h, "#log", "data-log").unwrap(), "penter,enter,leave");
}

#[test]
fn bubbling_handlers_run_once() {
    fn app() -> Element {
        let mut n = use_signal(|| 0);
        rsx! {
            div { id: "outer", "data-n": "{n}", onclick: move |_| n += 1,
                div { onclick: move |_| {},
                    button { id: "b", onclick: move |_| {}, "x" }
                }
            }
        }
    }
    let mut h = Harness::new(app);
    h.click("#b");
    assert_eq!(attr(&h, "#outer", "data-n").as_deref(), Some("1"));
}

#[test]
fn onmounted_gives_rects_and_focus() {
    fn app() -> Element {
        let mut el = use_signal(|| None::<std::rc::Rc<MountedData>>);
        let mut width = use_signal(|| 0.0);
        let mut focused = use_signal(|| false);
        rsx! {
            input {
                id: "field",
                style: "width: 180px; box-sizing: border-box;",
                onmounted: move |e| el.set(Some(e.data())),
                onfocus: move |_| focused.set(true),
            }
            button {
                id: "go",
                "data-width": "{width}",
                "data-focused": "{focused}",
                onclick: move |_| async move {
                    let Some(el) = el() else { return };
                    if let Ok(r) = el.get_client_rect().await {
                        width.set(r.width());
                    }
                    let _ = el.set_focus(true).await;
                },
                "go"
            }
        }
    }
    let mut h = Harness::new(app);
    h.click("#go");
    h.settle();
    let w: f64 = attr(&h, "#go", "data-width").unwrap().parse().unwrap();
    assert!((w - 180.0).abs() < 1.0, "width {w}");
    assert_eq!(h.doc.get_focussed_node_id(), h.find("#field"));
    assert_eq!(attr(&h, "#go", "data-focused").as_deref(), Some("true"));
}

#[test]
fn focus_and_blur_fire_on_click() {
    fn app() -> Element {
        let mut log = use_signal(Vec::<&'static str>::new);
        rsx! {
            div { id: "log", "data-log": log.read().join(",") }
            input { id: "a", onfocus: move |_| log.write().push("focus-a"), onblur: move |_| log.write().push("blur-a") }
            div { style: "height: 20px;" }
            input { id: "b", onfocusin: move |_| log.write().push("focusin-b") }
        }
    }
    let mut h = Harness::new(app);
    h.click("#a");
    h.click("#b");
    assert_eq!(
        attr(&h, "#log", "data-log").unwrap(),
        "focus-a,blur-a,focusin-b"
    );
}

#[test]
fn clicks_on_inline_children_of_a_padded_block_hit_them() {
    fn app() -> Element {
        let mut n = use_signal(|| 0);
        rsx! {
            p { id: "p", style: "padding: 40px; border: 4px solid black;",
                "Some text "
                button { id: "b", "data-n": "{n}", onclick: move |_| n += 1, "Press" }
                " more text"
            }
        }
    }
    let mut h = Harness::new(app);
    h.click("#b");
    assert_eq!(attr(&h, "#b", "data-n").as_deref(), Some("1"));
}

#[test]
fn right_click_fires_contextmenu() {
    fn app() -> Element {
        let mut at = use_signal(|| None::<i32>);
        rsx! {
            div {
                id: "t",
                style: "margin: 40px; width: 200px; height: 100px;",
                "data-at": at().map(|x| x.to_string()),
                oncontextmenu: move |e| at.set(Some(e.element_coordinates().x as i32)),
            }
        }
    }
    let mut h = Harness::new(app);
    let r = h.rect("#t").unwrap();
    h.click_at(r.x + 10.0, r.y + 10.0);
    assert_eq!(
        attr(&h, "#t", "data-at"),
        None,
        "left click fired contextmenu"
    );
    h.right_click_at(r.x + 12.0, r.y + 10.0);
    assert_eq!(attr(&h, "#t", "data-at").as_deref(), Some("12"));
}

#[test]
fn images_fire_load() {
    use blitz_dom::net::Resource;
    use blitz_dom::util::ImageType;
    fn app() -> Element {
        let mut loaded = use_signal(|| false);
        rsx! {
            img { id: "i", src: "x.png", "data-loaded": "{loaded}", onload: move |_| loaded.set(true) }
        }
    }
    let mut h = Harness::new(app);
    let img = h.find("#i").unwrap();
    assert_eq!(attr(&h, "#i", "data-loaded").as_deref(), Some("false"));
    h.doc.load_resource(Resource::Image(
        img,
        ImageType::Image,
        1,
        1,
        std::sync::Arc::new(vec![0; 4]),
    ));
    h.settle();
    assert_eq!(attr(&h, "#i", "data-loaded").as_deref(), Some("true"));
}

#[test]
fn images_fire_error_when_they_fail() {
    use blitz_dom::net::{ImageHandler, Resource};
    use blitz_dom::util::ImageType;
    use blitz_traits::net::NetHandler;
    use std::sync::{Arc, Mutex};
    fn app() -> Element {
        let mut failed = use_signal(|| 0);
        rsx! {
            img { id: "missing", src: "missing.png", "data-failed": "{failed}", onerror: move |_| failed += 1 }
            img { id: "garbage", src: "garbage.png", "data-failed": "{failed}", onerror: move |_| failed += 1 }
        }
    }
    let mut h = Harness::new(app);
    // What the network provider does: a fetch that fails reaches the handler's `failed`,
    // bytes that don't decode reach `bytes`. Both now report the node.
    let got = Arc::new(Mutex::new(Vec::new()));
    let sink = {
        let got = got.clone();
        Arc::new(move |_doc: usize, r: Result<Resource, Option<String>>| {
            got.lock().unwrap().push(r.ok());
        })
    };
    let missing = h.find("#missing").unwrap();
    let garbage = h.find("#garbage").unwrap();
    Box::new(ImageHandler::new(missing, ImageType::Image)).failed(0, sink.clone());
    Box::new(ImageHandler::new(garbage, ImageType::Image)).bytes(
        0,
        b"not an image"[..].into(),
        sink,
    );
    for resource in got.lock().unwrap().drain(..) {
        h.doc
            .load_resource(resource.expect("a resource, not an error"));
    }
    h.settle();
    assert_eq!(attr(&h, "#missing", "data-failed").as_deref(), Some("2"));
}

#[test]
fn submit_buttons_and_enter_fire_submit() {
    fn app() -> Element {
        let mut submits = use_signal(|| 0);
        rsx! {
            form {
                id: "f",
                "data-submits": "{submits}",
                onsubmit: move |e| {
                    e.prevent_default();
                    submits += 1;
                },
                input { id: "field", r#type: "text" }
                button { id: "go", "Send" }
                button { id: "other", r#type: "button", "Not a submit" }
            }
        }
    }
    let mut h = Harness::new(app);
    h.click("#go");
    assert_eq!(attr(&h, "#f", "data-submits").as_deref(), Some("1"));
    h.click("#other");
    assert_eq!(attr(&h, "#f", "data-submits").as_deref(), Some("1"));
    h.click("#field");
    h.key(Key::Enter, Code::Enter, Modifiers::empty());
    assert_eq!(attr(&h, "#f", "data-submits").as_deref(), Some("2"));
}

#[test]
fn mounted_scroll_goes_to_a_position() {
    use dioxus::html::geometry::PixelsVector2D;
    fn app() -> Element {
        let mut el = use_signal(|| None::<std::rc::Rc<MountedData>>);
        let go = move |x: f64| {
            if let Some(el) = el() {
                spawn(async move {
                    _ = el
                        .scroll(PixelsVector2D::new(x, 0.0), ScrollBehavior::Instant)
                        .await;
                });
            }
        };
        rsx! {
            button { id: "to100", onclick: move |_| go(100.0), "100" }
            button { id: "to40", onclick: move |_| go(40.0), "40" }
            button { id: "far", onclick: move |_| go(1.0e6), "far" }
            div {
                id: "track",
                style: "display: flex; width: 200px; height: 50px; overflow-x: auto;",
                onmounted: move |e| el.set(Some(e.data())),
                div { style: "flex: none; width: 600px; height: 40px;" }
            }
        }
    }
    let mut h = Harness::new(app);
    let track = h.find("#track").unwrap();
    let x = |h: &Harness| h.doc.get_node(track).unwrap().scroll_offset.x;
    h.click("#to100");
    assert_eq!(x(&h), 100.0);
    // A position, like the web's `Element.scroll()`, not a step.
    h.click("#to40");
    assert_eq!(x(&h), 40.0);
    h.click("#far");
    assert!(x(&h) > 300.0 && x(&h) <= 400.0, "not clamped: {}", x(&h));
}

#[test]
fn keyed_lists_reorder_without_losing_items() {
    fn app() -> Element {
        let mut items = use_signal(|| vec![1, 2, 3, 4]);
        rsx! {
            button { id: "rotate", onclick: move |_| items.write().rotate_left(1), "rotate" }
            ul {
                for i in items() {
                    li { key: "{i}", class: "item", "{i}" }
                }
            }
        }
    }
    let mut h = Harness::new(app);
    h.click("#rotate");
    let order: Vec<String> = h
        .doc
        .query_selector_all(".item")
        .unwrap()
        .into_iter()
        .map(|id| h.doc.get_node(id).unwrap().text_content())
        .collect();
    assert_eq!(order, ["2", "3", "4", "1"]);
}

#[test]
fn checkboxes_and_radios_fire_change() {
    fn app() -> Element {
        let mut n = use_signal(|| 0);
        rsx! {
            div { id: "n", "data-n": "{n}" }
            input { id: "c", r#type: "checkbox", onchange: move |_| n += 1 }
            input { id: "r", r#type: "radio", name: "g", onchange: move |_| n += 10 }
        }
    }
    let mut h = Harness::new(app);
    h.click("#c");
    h.click("#r");
    assert_eq!(attr(&h, "#n", "data-n").as_deref(), Some("11"));
}

// ---- positioning -----------------------------------------------------------------------

#[test]
fn position_fixed_is_relative_to_the_window() {
    fn app() -> Element {
        rsx! {
            div { style: "margin: 100px; padding: 50px; position: relative; width: 300px; height: 300px;",
                div { style: "padding: 30px;",
                    div { id: "fixed", style: "position: fixed; top: 10px; right: 20px; width: 100px; height: 40px; background: red;" }
                    div { id: "fill", style: "position: fixed; inset: 0; pointer-events: none;" }
                }
            }
        }
    }
    let h = Harness::new(app);
    let r = h.rect("#fixed").unwrap();
    assert_eq!(
        (r.x, r.y, r.w, r.h),
        (WIDTH as f32 - 120.0, 10.0, 100.0, 40.0)
    );
    let fill = h.rect("#fill").unwrap();
    assert_eq!(
        (fill.x, fill.y, fill.w, fill.h),
        (0.0, 0.0, WIDTH as f32, HEIGHT as f32)
    );
    assert_eq!(rgb(&h, r.x + 50.0, r.y + 20.0), RED);
}

#[test]
fn percentage_transforms_apply_to_the_box_and_its_content() {
    fn app() -> Element {
        rsx! {
            div {
                id: "t",
                style: "position: absolute; left: 400px; top: 100px; width: 200px; height: 100px; transform: translate(-50%, -50%); background: red;",
                div { style: "width: 50px; height: 50px; background: blue;" }
            }
        }
    }
    let h = Harness::new(app);
    // Box spans x 300..500, y 50..150; its child (blue) sits in its top-left corner.
    assert_eq!(rgb(&h, 310.0, 60.0), BLUE, "child not moved with parent");
    assert_eq!(rgb(&h, 450.0, 120.0), RED, "box not moved");
    assert_ne!(
        rgb(&h, 550.0, 170.0),
        RED,
        "untransformed box still painted"
    );
}

#[test]
fn nested_transforms_compose() {
    fn app() -> Element {
        rsx! {
            div { style: "position: absolute; left: 0; top: 0; transform: translateX(100px);",
                div { style: "transform: translateY(100px); width: 40px; height: 40px; background: red;" }
            }
        }
    }
    let h = Harness::new(app);
    assert_eq!(rgb(&h, 120.0, 120.0), RED);
    assert_ne!(rgb(&h, 20.0, 120.0), RED);
}

#[test]
fn clicks_follow_transformed_boxes() {
    fn app() -> Element {
        let mut n = use_signal(|| 0);
        rsx! {
            button {
                id: "b",
                "data-n": "{n}",
                style: "position: absolute; left: 400px; top: 100px; width: 200px; height: 100px; transform: translateX(-50%);",
                onclick: move |_| n += 1,
            }
        }
    }
    let mut h = Harness::new(app);
    h.click_at(320.0, 150.0); // only inside the box after the transform
    assert_eq!(attr(&h, "#b", "data-n").as_deref(), Some("1"));
    h.click_at(550.0, 150.0); // only inside the untransformed box
    assert_eq!(attr(&h, "#b", "data-n").as_deref(), Some("1"));
}

#[test]
fn pointer_events_none_lets_clicks_through() {
    fn app() -> Element {
        let mut n = use_signal(|| 0);
        rsx! {
            div { style: "position: relative; width: 300px; height: 100px;",
                button { id: "b", "data-n": "{n}", style: "width: 300px; height: 100px;", onclick: move |_| n += 1 }
                div { id: "cover", style: "position: absolute; inset: 0; pointer-events: none;",
                    span { id: "auto", style: "pointer-events: auto; position: absolute; left: 0; top: 0; width: 20px; height: 20px;" }
                }
            }
        }
    }
    let mut h = Harness::new(app);
    h.click("#b");
    assert_eq!(attr(&h, "#b", "data-n").as_deref(), Some("1"));
    // A descendant can opt back in.
    let a = h.rect("#auto").unwrap();
    h.move_to(a.x + 5.0, a.y + 5.0);
    assert_eq!(h.doc.get_hover_node_id(), h.find("#auto"));
}

#[test]
fn z_index_escapes_its_parent() {
    fn app() -> Element {
        let mut n = use_signal(|| 0);
        rsx! {
            div { style: "position: relative; height: 50px;",
                div {
                    id: "menu",
                    "data-n": "{n}",
                    style: "position: absolute; top: 20px; left: 0; width: 200px; height: 100px; background: red; z-index: 50;",
                    onclick: move |_| n += 1,
                }
            }
            div { style: "position: relative; height: 200px; background: blue;" }
        }
    }
    let mut h = Harness::new(app);
    let m = h.rect("#menu").unwrap();
    // The lower half of the menu overlaps the later blue sibling of its parent.
    assert_eq!(rgb(&h, m.x + 100.0, m.y + 80.0), RED);
    h.click_at(m.x + 100.0, m.y + 80.0);
    assert_eq!(attr(&h, "#menu", "data-n").as_deref(), Some("1"));
}
