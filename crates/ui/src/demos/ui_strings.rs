use crate::components::button::{Button, ButtonVariant};
use crate::components::carousel::{Carousel, CarouselItem};
use crate::components::spinner::Spinner;
use crate::components::ui_strings::*;
use dioxus::prelude::*;

fn french() -> UiStrings {
    UiStrings {
        previous: "Précédent".into(),
        next: "Suivant".into(),
        loading: "Chargement".into(),
        ..UiStrings::default()
    }
}

#[component]
pub fn Demo() -> Element {
    let mut french_on = use_signal(|| false);
    let strings = use_memo(move || {
        if french_on() {
            french()
        } else {
            UiStrings::default()
        }
    });
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; max-width: 32rem;",
            div { role: "group", "aria-label": "Language", style: "display: flex; gap: 0.5rem;",
                Button {
                    variant: if french_on() { ButtonVariant::Outline } else { ButtonVariant::Primary },
                    "aria-pressed": "{!french_on()}",
                    onclick: move |_| french_on.set(false),
                    "English"
                }
                Button {
                    variant: if french_on() { ButtonVariant::Primary } else { ButtonVariant::Outline },
                    "aria-pressed": "{french_on()}",
                    onclick: move |_| french_on.set(true),
                    "Français"
                }
            }
            UiStringsProvider { strings,
                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                    Spinner {}
                    span { "The spinner's name and the carousel buttons follow the language." }
                }
                Carousel { label: "Plans",
                    for (i , name) in ["Starter", "Team", "Business"].into_iter().enumerate() {
                        CarouselItem { label: "{i + 1} of 3", strong { "{name}" } }
                    }
                }
            }
        }
    }
}
