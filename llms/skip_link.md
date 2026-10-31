# SkipLink

- Copy into your app: `atoll add skip_link` or `dx components add skip_link --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::skip_link::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

`SkipLink` is a "Skip to content" link that stays hidden until it has keyboard focus, then appears in the corner, so keyboard users can jump past the header and navigation. `MainContent` is the page's `<main>` landmark it points at.

```rust
rsx! {
    SkipLink {}
    header { /* logo, nav */ }
    MainContent { Outlet::<Route> {} }
}
```

`SkipLink` points at `#main-content` by default (`target` changes it) and reads `UiStrings::skip_to_content` unless it has children. It is positioned against the nearest positioned ancestor, usually the page. `MainContent` renders `main id="main-content" tabindex="-1"`, so following the skip link moves focus into it.

With the kit's `router` feature, `MainContent` inside a `Router` also takes focus after every in-app navigation (not on the first load). Keyboard and screen reader users then start at the new page, and a screen reader reads the main landmark, instead of staying on the link they used. Set the page's title (`document::Title`) and give each page a heading.

## Accessibility

- Role and name: `SkipLink` is a native link named by its text; `MainContent` is the `main` landmark
- Keyboard: the skip link is the first Tab stop; Enter moves focus to the main content
- Focus: shown with the kit's focus ring when focused; `MainContent` shows no ring, since it is not a control
- Reduced motion: nothing animates

## Source

Original to this repo (booking app gaps 11 and 21).

## Stability

`SkipLink` and `MainContent` are stable; the focus move after a route change (the `router` feature) is experimental (docs/STABILITY.md).

## API

```rust
/// The id [`SkipLink`] points at and [`MainContent`] carries unless told otherwise.
pub const MAIN_CONTENT_ID: &str;

/// A "Skip to content" link for keyboard users: hidden until it has focus, then shown at
/// the top corner of the nearest positioned ancestor. Put it first in the page, before the
/// header and navigation, and point it at the page's [`MainContent`].
#[component]
pub fn SkipLink(
    /// The id of the element to skip to, without `#`.
    #[props(into, default = MAIN_CONTENT_ID.to_string())]
    target: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    /// The link text. Defaults to `UiStrings::skip_to_content`.
    children: Option<Element>,
) -> Element

/// The page's `<main>` landmark: the target of [`SkipLink`] (`id="main-content"`,
/// focusable from script with `tabindex="-1"`).
///
/// With the kit's `router` feature, inside a `Router`, it also takes focus after an in-app
/// navigation (not on the first load), so keyboard and screen reader users start at the new
/// page instead of on the link they used. Give the page a heading inside it.
#[component]
pub fn MainContent(
    #[props(into, default = MAIN_CONTENT_ID.to_string())] id: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::skip_link::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "position: relative; display: flex; flex-direction: column; gap: 0.5rem; padding-top: 2.75rem;",
            SkipLink { target: "skip-link-demo-main" }
            p { style: "margin: 0;", "Tab into this demo: the skip link appears above, and Enter jumps past the links below." }
            nav { "aria-label": "Skip link demo",
                a { href: "#skip-link", "One" }
                " · "
                a { href: "#skip-link", "Two" }
            }
            // A page has one `MainContent` (`<main>`); the gallery already has its own, so the
            // demo skips to a plain focusable block.
            div { id: "skip-link-demo-main", tabindex: "-1", style: "outline: none;",
                p { style: "margin: 0;", "The main content." }
            }
        }
    }
}
```
