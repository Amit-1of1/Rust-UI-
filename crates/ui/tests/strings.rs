//! Kit strings come from `UiStringsProvider` when there is one, English otherwise.

mod common;

use common::render;
use dioxus::prelude::*;
use ui::components::ui_strings::fill;
use ui::prelude::*;

fn french() -> UiStrings {
    UiStrings {
        previous: "Précédent".into(),
        next: "Suivant".into(),
        loading: "Chargement".into(),
        breadcrumb: "Fil d’Ariane".into(),
        otp_label: "Code à usage unique".into(),
        otp_character: "{label}, caractère {n} sur {total}".into(),
        ..UiStrings::default()
    }
}

#[test]
fn provider_translates_kit_strings() {
    fn app() -> Element {
        let strings = use_signal(french);
        rsx! {
            UiStringsProvider { strings,
                Spinner {}
                Breadcrumb {}
                InputOtp { length: 2 }
                Carousel { label: "Produits", div { "1" } }
            }
        }
    }
    let html = render(app);
    for text in [
        "aria-label=\"Chargement\"",
        "aria-label=\"Fil d’Ariane\"",
        "Code à usage unique, caractère 2 sur 2",
        ">Précédent<",
        ">Suivant<",
    ] {
        assert!(html.contains(text), "missing {text} in {html}");
    }
    assert!(
        !html.contains("Loading") && !html.contains("Next"),
        "{html}"
    );
}

#[test]
fn english_without_a_provider_and_props_win() {
    fn app() -> Element {
        rsx! {
            Spinner {}
            Spinner { label: "Saving" }
            Pagination { PaginationNext {} }
        }
    }
    let html = render(app);
    assert!(html.contains("aria-label=\"Loading\""), "{html}");
    assert!(html.contains("aria-label=\"Saving\""), "{html}");
    assert!(html.contains("Go to next page"), "{html}");
}

#[test]
fn fill_replaces_every_placeholder() {
    assert_eq!(
        fill(
            "Showing {first}–{last} of {total} rows",
            &[("first", "1"), ("last", "10"), ("total", "42")]
        ),
        "Showing 1–10 of 42 rows"
    );
    assert_eq!(fill("No placeholders", &[("x", "y")]), "No placeholders");
}
