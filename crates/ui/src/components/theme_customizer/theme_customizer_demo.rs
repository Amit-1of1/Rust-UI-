use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ThemeCustomizer {}
    }
}
