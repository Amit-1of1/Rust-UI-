use crate::components::theme_customizer::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ThemeCustomizer {}
    }
}
