use dioxus::prelude::*;

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
    /// Accessible name for the strip.
    #[props(into, default = "Scrolling content".to_string())]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut paused = use_signal(|| false);
    rsx! {
        div {
            class: Styles::dx_marquee,
            role: "group",
            "aria-label": "{label}",
            "data-paused": paused().then_some("true"),
            style: "--dx-marquee-duration: {duration}s;",
            ..attributes,
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
                if paused() { "Play" } else { "Pause" }
                span { class: Styles::dx_marquee_sr, " scrolling" }
            }
        }
    }
}
