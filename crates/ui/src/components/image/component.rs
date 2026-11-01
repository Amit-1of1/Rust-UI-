use crate::components::icons::ImageOff;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/image/style.css")]
struct Styles;

/// How an [`Image`] fills its box when the box and the picture have different shapes.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ImageFit {
    /// Fills the box, cropping the edges.
    #[default]
    Cover,
    /// Shows the whole picture, leaving bars.
    Contain,
}

impl ImageFit {
    fn class(self) -> &'static str {
        match self {
            ImageFit::Cover => "cover",
            ImageFit::Contain => "contain",
        }
    }
}

/// A picture with a fixed shape, sharp on every screen, and a fallback when it fails.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     Image {
///         src: "/cabin-800.jpg",
///         srcset: "/cabin-800.jpg 800w, /cabin-1600.jpg 1600w",
///         sizes: "(min-width: 40rem) 40rem, 100vw",
///         alt: "The cabin from the lake",
///         ratio: 16.0 / 9.0,
///     }
/// }
/// # }
/// ```
///
/// `alt` is required, so a picture with no text alternative doesn't compile; pass `alt: ""`
/// on purpose for a decorative one.
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Image { src: "/cabin.jpg" } }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn Image(
    /// The picture's URL. Empty shows the fallback.
    #[props(into)]
    src: String,
    /// What the picture shows, for people who can't see it; `""` for a decorative picture.
    #[props(into)]
    alt: String,
    /// Other sizes of the same picture (`"a.jpg 1x, a@2x.jpg 2x"` or widths with `sizes`), so
    /// the browser loads the one that fits the screen's density and the layout.
    #[props(into, default)]
    srcset: Option<String>,
    /// How wide the picture is drawn at each breakpoint, for `srcset` widths.
    #[props(into, default)]
    sizes: Option<String>,
    /// Width divided by height (`16.0 / 9.0`). The box keeps this shape at any width, so the
    /// page doesn't jump when the picture arrives. Without it the picture keeps its own shape.
    #[props(default)]
    ratio: Option<f64>,
    #[props(default)] fit: ImageFit,
    /// Load only when near the screen (`loading="lazy"`). On by default; turn it off for the
    /// first picture a visitor sees.
    #[props(default = true)]
    lazy: bool,
    /// Shown instead of the picture when `src` is empty or fails to load. Defaults to a muted
    /// box with an "image missing" icon, named by `alt`.
    #[props(default)]
    fallback: Option<Element>,
    /// Attributes for the wrapper: `style` for a width, `class`, ...
    #[props(extends=GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    // The source that failed, so a new `src` gets a new chance.
    let mut failed = use_signal(|| None::<String>);
    let broken = src.is_empty() || failed().as_deref() == Some(src.as_str());
    let style = ratio
        .filter(|r| r.is_finite() && *r > 0.0)
        .map(|r| format!("aspect-ratio: {r};"));
    let base = attributes!(div {
        class: Styles::dx_image,
        "data-fit": fit.class(),
        "data-ratio": ratio.is_some().then_some("true"),
        "data-state": if broken { "error" } else { "ok" },
        style,
    });
    rsx! {
        div { ..merge_attributes(vec![base, attributes]),
            if broken {
                if let Some(fallback) = fallback {
                    {fallback}
                } else {
                    span {
                        class: Styles::dx_image_fallback,
                        role: (!alt.is_empty()).then_some("img"),
                        aria_label: (!alt.is_empty()).then_some(alt.clone()),
                        aria_hidden: alt.is_empty().then_some("true"),
                        ImageOff { "aria-hidden": "true", size: "1.5rem" }
                    }
                }
            } else {
                img {
                    class: Styles::dx_image_img,
                    src: src.clone(),
                    srcset,
                    sizes,
                    alt: alt.clone(),
                    loading: lazy.then_some("lazy"),
                    decoding: "async",
                    onerror: {
                        let src = src.clone();
                        move |_| failed.set(Some(src.clone()))
                    },
                }
            }
        }
    }
}
