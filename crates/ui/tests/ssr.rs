//! Server rendering and hydration: the ids the kit generates must not depend on what the
//! process rendered before.
//!
//! A fullstack server renders every request in a new `VirtualDom`, and the browser then
//! hydrates the page with its own fresh one. If ids came from a process-wide counter, the
//! second request's HTML would say `dx-field-7` while the browser computes `dx-field-1`, and
//! labels, `aria-describedby` and focus-on-error would point at elements that don't exist.
//! Rendering every demo twice in one process stands in for "request 2 vs. hydration".
//!
//! (`dxc-<n>` ids belong to dioxus-primitives, which sends them from the server to the
//! browser itself when the app turns on Dioxus' `fullstack` feature.)

use common::{render, BLOCKS, DEMOS};

mod common;

/// Id prefixes the kit itself generates (`hooks::use_unique_id`).
const PREFIXES: &[&str] = &[
    "dx-field-",
    "dx-multi-",
    "dx-carousel-",
    "rui-select-",
    "rui-tabs-",
];

fn kit_ids(html: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for prefix in PREFIXES {
        let mut rest = html;
        while let Some(at) = rest.find(prefix) {
            let tail = &rest[at + prefix.len()..];
            let digits: String = tail.chars().take_while(char::is_ascii_digit).collect();
            if !digits.is_empty() {
                ids.push(format!("{prefix}{digits}"));
            }
            rest = tail;
        }
    }
    ids
}

#[test]
fn generated_ids_are_the_same_for_every_render() {
    let mut checked = 0;
    for (name, demo) in DEMOS.iter().chain(BLOCKS) {
        let first = kit_ids(&render(*demo));
        let second = kit_ids(&render(*demo));
        assert_eq!(
            first, second,
            "{name}: ids changed between two renders in one process, so a server-rendered \
             page would not match what the browser hydrates"
        );
        checked += usize::from(!first.is_empty());
    }
    assert!(
        checked >= 5,
        "expected demos with generated ids, found {checked}"
    );
}

#[test]
fn ids_count_from_one_in_each_app() {
    let html = render(|| {
        use dioxus::prelude::*;
        use ui::prelude::*;
        let form = use_form();
        let email = form.field(vec![required()]);
        rsx! {
            Form { state: form, on_valid: |_| {},
                TextField { field: email, label: "Email" }
            }
        }
    });
    assert!(
        html.contains("id=\"dx-field-1\""),
        "first field should be dx-field-1:\n{html}"
    );
}

#[test]
fn a_saved_theme_round_trips_and_bad_values_are_ignored() {
    use dioxus::prelude::*;
    use ui::prelude::*;
    fn app() -> Element {
        let theme = use_theme();
        let first = use_hook(move || {
            // The radius before the preset: picking the preset must not drop it.
            apply_saved_theme(
                &theme,
                "mode=dark;radius=12;accent=blue;preset=reef;density=compact",
            );
            encode_theme(&theme)
        });
        apply_saved_theme(
            &theme,
            "mode=<script>;preset=evil;accent=pink;radius=x;junk",
        );
        let second = encode_theme(&theme);
        rsx! { p { "{first}|{second}" } }
    }
    let html = render(app);
    let expected = "mode=dark;preset=reef;accent=blue;density=compact;radius=12";
    assert!(
        html.contains(&format!("{expected}|{expected}")),
        "unexpected: {html}"
    );
}
