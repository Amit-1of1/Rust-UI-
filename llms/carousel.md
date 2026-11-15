# Carousel

- Copy into your app: `atoll add carousel` or `dx components add carousel --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `ui_strings`).
- Import: `use crate::components::carousel::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A horizontally scrolling row of slides with scroll snapping and Previous/Next buttons. Nothing auto-plays, so there is no motion to pause.

```rust
Carousel { label: "Featured plans",
    CarouselItem { label: "1 of 3", "..." }
    CarouselItem { label: "2 of 3", "..." }
}
```

Slide width and spacing are CSS (`.dx-carousel-item` flex-basis); the buttons scroll by about one viewport. Slides centre their content; `CarouselItem { media: true, img { .. } }` makes a full-bleed photo slide instead (the content fills the slide and an `img` is cropped to cover it).

## Accessibility

- Role and name: a `section` with `aria-roledescription="carousel"` named by `label`; the scrolling track is a focusable group; each slide is `aria-roledescription="slide"` with your `label`.
- Scrolling: focus the track and use the arrow keys (native scrolling).
- Reduced motion: scrolling is instant instead of smooth.

## Source

Original to this repo. The pattern follows the WAI-ARIA carousel practices; Rust/UI's `carousel.rs` was used only to see the feature set.

## Keyboard and accessibility

Follows the WAI-ARIA [Carousel](https://www.w3.org/WAI/ARIA/apg/patterns/carousel/) pattern. Roles: `group`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Tab | On the slide track | Moves to Previous, then Next (the track is one tab stop). | yes |

Known gaps:

- Previous and Next change nothing a screen reader is told: the visible slide is not marked and nothing is announced.
- Arrow keys on the track scroll it natively; there is no slide-by-slide keyboard step.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// A row of slides that scrolls horizontally and snaps, with Previous/Next buttons. The track
/// itself is focusable, so arrow keys scroll it. Nothing moves on its own.
#[component]
pub fn Carousel(
    /// Accessible name, for example "Featured products".
    #[props(into)]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// One slide. Give it a `label` such as "1 of 4" so screen readers can place it.
#[component]
pub fn CarouselItem(
    #[props(into, default)] label: Option<String>,
    /// A full-bleed slide for a photo or video: the content fills the slide edge to edge
    /// (an `img` is cropped to cover it) instead of being centred.
    #[props(default)]
    media: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::carousel::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "max-width: 32rem;",
            Carousel { label: "Featured plans",
                for (i , name) in ["Starter", "Team", "Business", "Enterprise"].into_iter().enumerate() {
                    CarouselItem { label: "{i + 1} of 4", strong { "{name}" } }
                }
            }
        }
    }
}
```
