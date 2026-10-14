use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/carousel/style.css")]
struct Styles;

fn scroll_by(track: &str, element: Option<Rc<MountedData>>, direction: f64) {
    // `track` is generated here (`dx-carousel-<n>`), never user text.
    let script = document::eval(&format!(
        "var t=document.getElementById('{track}');if(t){{var r=matchMedia('(prefers-reduced-motion: reduce)').matches;t.scrollBy({{left:{direction}*t.clientWidth*0.9,behavior:r?'auto':'smooth'}});}}"
    ));
    // Without scripts (native) `send` fails; scroll through the element handle instead.
    if script.send(()).is_ok() {
        return;
    }
    let Some(element) = element else { return };
    spawn(async move {
        let (Ok(offset), Ok(rect)) = (
            element.get_scroll_offset().await,
            element.get_client_rect().await,
        ) else {
            return;
        };
        let to = dioxus::html::geometry::PixelsVector2D::new(
            offset.x + direction * rect.width() * 0.9,
            offset.y,
        );
        _ = element.scroll(to, ScrollBehavior::Instant).await;
    });
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
    let mut track_el = use_signal(|| None::<Rc<MountedData>>);
    rsx! {
        section {
            "aria-roledescription": "carousel",
            "aria-label": "{label}",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_carousel }), attributes]),
            div {
                id: "{track}",
                class: Styles::dx_carousel_track,
                tabindex: "0",
                role: "group",
                onmounted: move |e: MountedEvent| track_el.set(Some(e.data())),
                "aria-label": "{slides}",
                {children}
            }
            div { class: Styles::dx_carousel_controls,
                button {
                    class: Styles::dx_carousel_button,
                    r#type: "button",
                    onclick: {
                        let track = track.clone();
                        move |_| scroll_by(&track, track_el(), -1.0)
                    },
                    {t.previous}
                }
                button {
                    class: Styles::dx_carousel_button,
                    r#type: "button",
                    onclick: move |_| scroll_by(&track, track_el(), 1.0),
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
            role: "group",
            "aria-roledescription": "slide",
            "aria-label": label,
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_carousel_item }), attributes]),
            {children}
        }
    }
}
