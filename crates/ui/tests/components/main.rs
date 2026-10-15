//! Per-component tests: one file per component, `tests/components/<name>.rs`.
//!
//! New component tests go here instead of the end of `render.rs`, where every branch used to
//! append and conflict. The kit's build.rs lists the files, so adding one needs no other edit.
//! Each file can `use super::render;` and `use ui::prelude::*;`.

#![allow(dead_code)]

use dioxus::prelude::*;

pub fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

include!(concat!(env!("OUT_DIR"), "/component_tests.rs"));
