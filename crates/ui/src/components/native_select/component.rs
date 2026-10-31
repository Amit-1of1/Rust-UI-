use crate::components::icons::ChevronDown;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/native_select/style.css")]
struct Styles;

/// The browser's own `<select>`, styled like [`Input`](crate::components::input::Input).
///
/// Use it where the platform picker is the better choice: phones (the OS wheel or sheet),
/// long forms, very long option lists, and server-rendered pages, where it works before the
/// app has loaded. Use `Select` when the list itself needs custom styling.
///
/// ```rust,ignore
/// NativeSelect { aria_label: "Country", onchange: move |e: FormEvent| country.set(e.value()),
///     NativeSelectOption { value: "", "Choose a country" }
///     NativeSelectGroup { label: "Europe",
///         NativeSelectOption { value: "fr", "France" }
///     }
/// }
/// ```
///
/// Any `select` attribute (`name`, `value`, `required`, `disabled`, `multiple`, `id`,
/// `aria-*`) goes to the `<select>`; `class` is added to the kit's.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn NativeSelect(
    onchange: Option<EventHandler<FormEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=select)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(select { class: Styles::dx_native_select });
    rsx! {
        span { class: Styles::dx_native_select_wrapper,
            select {
                onchange: move |e| _ = onchange.map(|f| f(e)),
                oninput: move |e| _ = oninput.map(|f| f(e)),
                onfocus: move |e| _ = onfocus.map(|f| f(e)),
                onblur: move |e| _ = onblur.map(|f| f(e)),
                onmounted: move |e| _ = onmounted.map(|f| f(e)),
                ..merge_attributes(vec![base, attributes]),
                {children}
            }
            ChevronDown { "aria-hidden": "true", class: Styles::dx_native_select_icon.to_string(), size: "16px" }
        }
    }
}

/// One `<option>` of a [`NativeSelect`]. Takes `value`, `selected` and `disabled`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn NativeSelectOption(
    #[props(extends=GlobalAttributes)]
    #[props(extends=option)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        option { ..attributes, {children} }
    }
}

/// A labelled group of options (`<optgroup>`), read by screen readers with its options.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn NativeSelectGroup(
    #[props(into)] label: String,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    rsx! {
        optgroup { label, disabled, {children} }
    }
}
