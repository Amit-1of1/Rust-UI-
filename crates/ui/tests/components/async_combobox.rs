//! `AsyncCombobox` before anything loads (SSR runs no tasks). The loading, paging, error and
//! announcement behaviour needs the requests to finish: apps/native/tests/async_combobox.rs.

use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

fn app() -> Element {
    let source = use_async_options(|query: String, _page: usize| async move {
        Ok(AsyncPage::last(vec![query]))
    });
    rsx! {
        AsyncCombobox::<String> {
            options: source,
            label: |s: String| s,
            aria_label: "City",
            list_aria_label: "Cities",
            id: "city",
        }
    }
}

#[test]
fn closed_async_combobox_is_a_plain_combobox_with_a_quiet_live_region() {
    let html = render(app);
    assert!(html.contains("role=\"combobox\""), "{html}");
    assert!(html.contains("aria-expanded=false"), "{html}");
    assert!(html.contains("id=\"city\""), "{html}");
    // The live region exists from the start (so later text is announced) and is empty.
    assert!(html.contains("data-slot=\"combobox-announcer\""), "{html}");
    assert!(html.contains("role=\"status\""), "{html}");
    assert!(html.contains("aria-live=\"polite\""), "{html}");
    assert!(!html.contains("aria-busy"), "{html}");
    assert!(!html.contains("Loading"), "{html}");
}

#[test]
fn plain_combobox_has_no_live_region() {
    let html = render(ui::demos::combobox::Demo);
    assert!(!html.contains("combobox-announcer"), "{html}");
    assert!(!html.contains("aria-busy"), "{html}");
}

#[test]
fn async_strings_are_translated_for_every_locale() {
    for lang in ui::components::ui_strings::LOCALES {
        let t = UiStrings::locale(lang).expect("locale");
        let en = UiStrings::default();
        assert_ne!(t.load_more, en.load_more, "{lang}");
        assert!(t.result_count.contains("{count}"), "{lang}");
    }
}
