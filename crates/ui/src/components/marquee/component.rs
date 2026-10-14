use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/marquee/style.css")]
struct Styles;

/// A looping, auto-scrolling strip (logos, quotes). It has a pause button, pauses while
/// hovered or focused, and under reduced motion stops and becomes a scrollable row. The
/// repeated copy is hidden from assistive tech.
#[component]
pub fn Marquee(
    /// Seconds for one full loop.
    #[props(default = 20)]
    duration: u32,
    /// Accessible name for the strip. Defaults to "Scrolling content" ([`UiStrings`](crate::components::ui_strings::UiStrings)).
    #[props(into)]
    label: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut paused = use_signal(|| false);
    let t = use_ui_strings();
    let label = label.unwrap_or(t.scrolling_content);
    rsx! {
        div {
            role: "group",
            "aria-label": "{label}",
            "data-paused": paused().then_some("true"),
            style: "--dx-marquee-duration: {duration}s;",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_marquee }), attributes]),
            div { class: Styles::dx_marquee_viewport,
                div { class: Styles::dx_marquee_track,
                    div { class: Styles::dx_marquee_row, {children.clone()} }
                    div { class: Styles::dx_marquee_row, "aria-hidden": "true", "inert": "true", {children} }
                }
            }
            button {
                class: Styles::dx_marquee_toggle,
                r#type: "button",
                "aria-pressed": "{paused()}",
                onclick: move |_| paused.toggle(),
                if paused() { {t.play} } else { {t.pause} }
                span { class: Styles::dx_marquee_sr, {t.scrolling} }
            }
        }
    }
}
