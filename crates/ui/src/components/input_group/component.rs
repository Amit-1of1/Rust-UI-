use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/input_group/style.css")]
struct Styles;

/// An input with text, icons or buttons attached inside one bordered box ("https://", "USD",
/// a search button). Use `InputGroupInput` for the control and `InputGroupAddon` for the rest.
/// Give the control a visible `<label>` or `aria-label`.
#[component]
pub fn InputGroup(
    #[props(default)] invalid: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            role: "group",
            "data-invalid": invalid.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_input_group }), attributes]),
            {children}
        }
    }
}

/// Static content or a button before (default) or after (`end`) the control.
#[component]
pub fn InputGroupAddon(
    #[props(default)] end: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-align": if end { "end" } else { "start" },
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_input_group_addon }), attributes]),
            {children}
        }
    }
}

/// The text control. It has no border of its own; the group draws it.
#[component]
pub fn InputGroupInput(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    oninput: Option<EventHandler<FormEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
) -> Element {
    rsx! {
        input {
            oninput: move |e| _ = oninput.map(|callback| callback(e)),
            onblur: move |e| _ = onblur.map(|callback| callback(e)),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_input_group_input }), attributes]),
        }
    }
}
