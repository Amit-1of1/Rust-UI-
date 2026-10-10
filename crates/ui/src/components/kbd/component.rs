use dioxus::prelude::*;

#[css_module("/src/components/kbd/style.css")]
struct Styles;

/// A keyboard key, e.g. `Kbd { "Ctrl" }`.
#[component]
pub fn Kbd(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        kbd { class: Styles::dx_kbd, ..attributes, {children} }
    }
}

/// A row of keys that make up one shortcut.
#[component]
pub fn KbdGroup(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span { class: Styles::dx_kbd_group, ..attributes, {children} }
    }
}
