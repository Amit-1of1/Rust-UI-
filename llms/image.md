# Image

- Copy into your app: `atoll add image` or `dx components add image --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::image::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A picture with a fixed shape, sharp on every screen, and a fallback when it can't load.

```rust
Image {
    src: "/cabin-800.jpg",
    srcset: "/cabin-800.jpg 800w, /cabin-1600.jpg 1600w",
    sizes: "(min-width: 40rem) 40rem, 100vw",
    alt: "The cabin from the lake",
    ratio: 16.0 / 9.0,
}
```

- `ratio` (width / height) keeps the box's shape at any width, so the page doesn't jump when the picture arrives; `fit` is `Cover` (crop, the default) or `Contain` (show it all). Set the width with `style` or a class; the box never grows past its container.
- `srcset` and `sizes` let the browser pick the file for the screen's density and the layout width.
- `alt` is required: describe what the picture shows, or pass `alt: ""` for a decorative picture.
- When `src` is empty or fails to load, a muted box with an "image missing" icon takes its place, named by `alt`. Pass `fallback` for your own (initials, a placeholder picture).
- Pictures load lazily (`loading="lazy"`); set `lazy: false` for the first one a visitor sees.

## Component Structure

```rust
// A box that keeps the shape and holds the picture, or the fallback when it fails.
Image { src, alt, srcset, sizes, ratio, fit, lazy,
    fallback: rsx! { /* shown instead when src is empty or fails to load */ },
}
```

## Accessibility

- Role and name: a native `img` named by `alt`; the fallback is `role="img"` with the same name, or hidden when `alt` is empty
- Keyboard: none (not interactive). For a clickable picture, wrap it in a link or button with its own name
- Reduced motion: none

## Source

Original to this repo.

## API

```rust
/// How an [`Image`] fills its box when the box and the picture have different shapes.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum ImageFit {
    /// Fills the box, cropping the edges.
    #[default]
    Cover,
    /// Shows the whole picture, leaving bars.
    Contain,
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
) -> Element
```

## Example

```rust
use crate::components::image::*;
use dioxus::prelude::*;

/// A landscape drawn inline (no network in demos), at two densities via `srcset`.
const SCENE: &str = concat!(
    "data:image/svg+xml;utf8,",
    "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 160 90'>",
    "<rect width='160' height='90' fill='%23bfdbfe'/>",
    "<circle cx='122' cy='24' r='10' fill='%23fde68a'/>",
    "<path d='M0 70 40 36l26 22 22-16 72 48H0z' fill='%2394a3b8'/></svg>"
);

#[component]
pub fn Demo() -> Element {
    let srcset = format!("{SCENE} 1x, {SCENE} 2x");
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 1rem; align-items: flex-start;",
            Image { style: "width: 12rem;", src: SCENE, srcset, alt: "Hills under a pale sun", ratio: 16.0 / 9.0 }
            Image { style: "width: 6rem;", src: SCENE, alt: "Hills, cropped square", ratio: 1.0 }
            Image { style: "width: 9rem;", src: SCENE, alt: "Hills, letterboxed", ratio: 1.0, fit: ImageFit::Contain }
            // Empty source: the fallback, named by `alt`.
            Image { style: "width: 9rem;", src: "", alt: "Profile banner (missing)", ratio: 16.0 / 9.0 }
        }
    }
}
```
