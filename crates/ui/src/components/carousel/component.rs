use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/carousel/style.css")]
struct Styles;

fn scroll_by(track: &str, direction: i32) {
    // `track` is generated here (`dx-carousel-<n>`), never user text.
    let _ = document::eval(&format!(
        "var t=document.getElementById('{track}');if(t){{var r=matchMedia('(prefers-reduced-motion: reduce)').matches;t.scrollBy({{left:{direction}*t.clientWidth*0.9,behavior:r?'auto':'smooth'}});}}"
    ));
}

/// A row of slides that scrolls horizontally and snaps, with Previous/Next buttons. The track
/// itself is focusable, so arrow keys scroll it. Nothing moves on its own.
#[component]
pub fn Carousel(
    /// Accessible name, for example "Featured products".
    #[props(into)]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let track = use_hook(|| {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        format!("dx-carousel-{}", NEXT.fetch_add(1, Ordering::Relaxed))
    });
    let t = use_ui_strings();
    let slides = fill(&t.slides, &[("label", &label)]);
    rsx! {
        section {
            class: Styles::dx_carousel,
            "aria-roledescription": "carousel",
            "aria-label": "{label}",
            ..attributes,
            div {
                id: "{track}",
                class: Styles::dx_carousel_track,
                tabindex: "0",
                role: "group",
                "aria-label": "{slides}",
                {children}
            }
            div { class: Styles::dx_carousel_controls,
                button {
                    class: Styles::dx_carousel_button,
                    r#type: "button",
                    onclick: {
                        let track = track.clone();
                        move |_| scroll_by(&track, -1)
                    },
                    {t.previous}
                }
                button {
                    class: Styles::dx_carousel_button,
                    r#type: "button",
                    onclick: move |_| scroll_by(&track, 1),
                    {t.next}
                }
            }
        }
    }
}

/// One slide. Give it a `label` such as "1 of 4" so screen readers can place it.
#[component]
pub fn CarouselItem(
    #[props(into, default)] label: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_carousel_item,
            role: "group",
            "aria-roledescription": "slide",
            "aria-label": label,
            ..attributes,
            {children}
        }
    }
}
