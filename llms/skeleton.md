# Skeleton

- Copy into your app: `atoll add skeleton` or `dx components add skeleton --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::skeleton::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Skeleton component is used to display a placeholder preview of content before the data gets loaded. This helps improve perceived performance and provides users with a visual indication that content is being loaded.

## Component Structure

```rust
Skeleton {
    // Accepts all GlobalAttributes, commonly used with style for sizing
    style: "width: 15rem; height: 1rem;",
}
```

## Shapes and composition

`shape` is `Rect` (the default, with the theme's corner radius), `Circle` (give it a width; the height follows) or `Text` (one line at the current font size, so it lines up with the text it replaces; `lines: 3` draws a paragraph with a shorter last line). Size skeletons like the content they replace so nothing moves when it loads, and compose them freely: a circle and a few text lines make a list row.

Skeletons are hidden from screen readers. Put `aria-busy="true"` on the region that is loading and remove it when the content arrives.

When the system asks for reduced motion the pulse stops (the kit's theme turns animations off).

## API

```rust
/// The shape a [`Skeleton`] stands in for.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum SkeletonShape {
    /// A box with the theme's corner radius (cards, images, buttons).
    #[default]
    Rect,
    /// A circle (avatars, icons): give it a width; the height follows.
    Circle,
    /// A line of text: one line high at the current font size, so it matches the text it
    /// replaces. Set `lines` for a paragraph.
    Text,
}

/// A placeholder shown while content loads. Size it with `style` (or a class) to the size of
/// what it replaces, so nothing moves when the content arrives. Decorative: mark the region
/// that is loading with `aria-busy="true"` instead.
#[component]
pub fn Skeleton(
    #[props(default)] shape: SkeletonShape,
    /// For `SkeletonShape::Text`: how many lines; the last one is shorter, like a paragraph.
    #[props(default = 1)]
    lines: usize,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
use crate::components::skeleton::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; gap: 2rem;",
            SkeletonInfoDemo {}
            SkeletonCardDemo {}
            SkeletonShapesDemo {}
        }
    }
}

#[component]
fn SkeletonInfoDemo() -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 1rem;",
            Skeleton { style: "width: 3rem; height: 3rem; border-radius: 50%;" }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                Skeleton { style: "width: 11.625rem; height: 1rem;" }
                Skeleton { style: "width: 8.5rem; height: 1rem;" }
            }
        }
    }
}

#[component]
fn SkeletonCardDemo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            Skeleton { style: "width: 15rem; height: 8rem; border-radius: 0.75rem;" }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                Skeleton { style: "width: 15.625rem; height: 1rem;" }
                Skeleton { style: "width: 12.5rem; height: 1rem;" }
            }
        }
    }
}

/// The shapes: a circle for an avatar and text lines that follow the font size.
#[component]
fn SkeletonShapesDemo() -> Element {
    rsx! {
        div { style: "display: flex; align-items: flex-start; gap: 1rem; width: 16rem;", aria_busy: "true",
            Skeleton { shape: SkeletonShape::Circle, style: "width: 2.5rem;" }
            div { style: "flex: 1; font-size: 0.875rem;",
                Skeleton { shape: SkeletonShape::Text, lines: 3 }
            }
        }
    }
}
```
