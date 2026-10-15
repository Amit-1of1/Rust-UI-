# Breadcrumb

- Copy into your app: `rust-ui add breadcrumb` or `dx components add breadcrumb --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::breadcrumb::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A trail of links to the current page.

## Accessibility

- Role and name: the current page is not a link and has `aria-current="page"`; separators are `aria-hidden`
- Keyboard: links are tabbable in order
- Reduced motion: none

## API

```rust
#[component]
pub fn Breadcrumb(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn BreadcrumbItem(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn BreadcrumbLink(
    #[props(into, default = "#".to_string())] href: String,
    /// For in-app navigation without a router: call `prevent_default()` and switch the view.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The current page: not a link, and marked with `aria-current="page"`.
#[component]
pub fn BreadcrumbPage(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The divider between items. Hidden from assistive technology.
#[component]
pub fn BreadcrumbSeparator() -> Element
```

## Example

```rust
use crate::components::breadcrumb::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbItem { BreadcrumbLink { href: "#", "Home" } }
            BreadcrumbSeparator {}
            BreadcrumbItem { BreadcrumbLink { href: "#", "Components" } }
            BreadcrumbSeparator {}
            BreadcrumbItem { BreadcrumbPage { "Breadcrumb" } }
        }
    }
}
```
