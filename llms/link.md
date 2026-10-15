# Link

- Copy into your app: `rust-ui add link` or `dx components add link --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`, `ui_strings`).
- Import: `use crate::components::link::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

`TextLink`: an inline text link, a plain `<a href>` with an underline, the kit's text colour and a visible focus ring. `external: true` opens it in a new tab safely and says so; `current: true` marks the page you are on.

```rust
p { "Read the " TextLink { href: "/docs/theming", "theming guide" } "." }
TextLink { href: "https://lucide.dev", external: true, "Lucide" }
```

It is named `TextLink` so it never clashes with the router's `Link`. For buttons that look like links, use `Button { variant: ButtonVariant::Link }`; for actions that do not navigate, use a button.

## Accessibility

- Role and name: a native link named by its text; external links add "(opens in a new tab)" (from `UiStrings`) and a hidden icon
- Keyboard: Tab reaches it, Enter follows it
- State: `aria-current="page"` when `current`, shown with a thicker underline, not colour alone
- Reduced motion: the hover underline changes without transition

## Source

Idea from Rust/UI `link.rs` (rust-ui/ui@43ddc65, MIT). Not copied: its `force_reload` mode renders a `<button>` that sets `window.location` through web-sys, which breaks middle-click, "open in new tab" and the link role, so it is left out; active-route matching (`PathMatchType`) is replaced by the explicit `current` prop.

## API

```rust
/// An inline text link: a plain `<a href>` with the kit's colours and focus ring.
///
/// Named `TextLink` so it does not clash with the router's `Link` when both preludes are
/// imported. For in-app routes, pass the router's `Link` the same class, or use this with
/// a path `href`.
#[component]
pub fn TextLink(
    #[props(into)] href: String,
    /// Opens in a new tab, with `rel="noopener noreferrer"`, an icon, and
    /// "(opens in a new tab)" for screen readers.
    #[props(default)]
    external: bool,
    /// Marks the link for the page you are on (`aria-current="page"`), e.g. in a nav list.
    #[props(default)]
    current: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::link::TextLink;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            p { style: "margin: 0;",
                "Read the "
                TextLink { href: "#theming", "theming guide" }
                " or browse the "
                TextLink { href: "https://lucide.dev/icons", external: true, "Lucide icons" }
                "."
            }
            nav { "aria-label": "Link demo",
                ul { style: "display: flex; gap: 1rem; list-style: none; margin: 0; padding: 0;",
                    li { TextLink { href: "#link", current: true, "Overview" } }
                    li { TextLink { href: "#chips", "Chips" } }
                    li { TextLink { href: "#icons", "Icons" } }
                }
            }
        }
    }
}
```
