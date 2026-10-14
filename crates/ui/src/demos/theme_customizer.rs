use crate::components::theme_customizer::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        // ThemeScope carries the choices where `<html>` can't be reached (native).
        ThemeScope { ThemeCustomizer {} }
    }
}
