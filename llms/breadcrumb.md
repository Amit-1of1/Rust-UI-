# Breadcrumb

- Copy into your app: `atoll add breadcrumb` or `dx components add breadcrumb --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `icons`, `ui_strings`).
- Import: `use crate::components::breadcrumb::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A trail of links to the current page.

```rust
Breadcrumb {
    BreadcrumbItem { BreadcrumbLink { href: "/", House { "aria-hidden": "true" } "Home" } }
    BreadcrumbSeparator {}
    BreadcrumbItem { BreadcrumbPage { "Settings" } }
}
```

- Icons go inside `BreadcrumbLink`, decorative (`aria-hidden`); use them for every step or for none.
- `BreadcrumbSeparator` draws "/" by default; pass your own: `BreadcrumbSeparator { ChevronRight { "aria-hidden": "true" } }`.
- `BreadcrumbLink { disabled: true }` keeps a step in the trail but can't be followed (no `href`, `aria-disabled`, skipped by Tab).

## Collapsing long trails

`BreadcrumbTrail` builds the trail from a list and, when there are more than `max_items` steps, shows the first and the last `max_items - 1` with a "…" button between. Pressing it shows every step and moves focus to the first one it revealed. `visible_crumbs` is the rule on its own.

```rust
BreadcrumbTrail { max_items: 3, items: vec![
    Crumb::link("Home", "/"),
    Crumb::link("Projects", "/projects"),
    Crumb::link("Atoll", "/projects/atoll"),
    Crumb::page("Members"),
] }
```

For hand-built trails, `BreadcrumbEllipsis { label, on_click }` is the same button.

## Accessibility

- Role and name: a `nav` named "Breadcrumb" (translatable); the current page is not a link and has `aria-current="page"`; separators are `aria-hidden`; the "…" button is named "Show N more"
- Keyboard: links are tabbable in order; Enter or Space on "…" expands the trail and focuses the first revealed step
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
    /// Can't be followed for now: no `href`, `aria-disabled="true"`, out of the tab order and
    /// drawn dimmed. Still read as a link in the trail.
    #[props(default)]
    disabled: bool,
    /// For in-app navigation without a router: call `prevent_default()` and switch the view.
    /// With the kit's `router` feature, a path `href` goes through the router by itself.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The current page: not a link, and marked with `aria-current="page"`.
#[component]
pub fn BreadcrumbPage(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The divider between items: "/" unless you pass your own, such as a `ChevronRight` icon or
/// "›". Hidden from assistive technology.
#[component]
pub fn BreadcrumbSeparator(#[props(default)] children: Element) -> Element {
    // An empty `children` is a lone placeholder node, not an empty template.
    let custom = children.as_ref().is_ok_and(|node|

/// Stands in for the items a collapsed trail hides. A button: pressing it calls `on_click`
/// (show the hidden items, or open a menu of them). [`BreadcrumbTrail`] uses it for you.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn BreadcrumbEllipsis(
    /// Its name, such as "Show 3 more"; read instead of the dots.
    #[props(into)]
    label: String,
    on_click: EventHandler<MouseEvent>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element

/// One step of a [`BreadcrumbTrail`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct Crumb {
    pub label: String,
    /// `None` for the current page (the last step).
    pub href: Option<String>,
    pub disabled: bool,
}

/// Which crumbs a trail of `count` shows when at most `max` fit: the first one, then the last
/// `max - 1`, with a gap (`None`) between. `max` below 2 counts as 2, and a gap never hides
/// just one crumb (that crumb is shown instead).
///
/// ```
/// use ui::components::breadcrumb::visible_crumbs;
/// assert_eq!(visible_crumbs(6, 3), vec![Some(0), None, Some(4), Some(5)]);
/// assert_eq!(visible_crumbs(3, 3), vec![Some(0), Some(1), Some(2)]);
/// ```
pub fn visible_crumbs(count: usize, max: usize) -> Vec<Option<usize>>

/// A breadcrumb trail built from a list, which collapses the middle into a "…" button when
/// there are more than `max_items` steps. Pressing the button shows every step and moves
/// focus to the first one it revealed.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     BreadcrumbTrail { max_items: 3, items: vec![
///         Crumb::link("Home", "/"),
///         Crumb::link("Projects", "/projects"),
///         Crumb::link("Atoll", "/projects/atoll"),
///         Crumb::link("Settings", "/projects/atoll/settings"),
///         Crumb::page("Members"),
///     ] }
/// }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn BreadcrumbTrail(
    items: Vec<Crumb>,
    /// How many steps to show before collapsing (the first and the last `max_items - 1`).
    #[props(default = usize::MAX)]
    max_items: usize,
    /// A custom separator, drawn between every two steps.
    #[props(default)]
    separator: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
use crate::components::breadcrumb::*;
use crate::components::icons::{ChevronRight, House};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1rem;",
            Breadcrumb {
                BreadcrumbItem {
                    BreadcrumbLink { href: "#",
                        House { "aria-hidden": "true", size: "0.875rem" }
                        "Home"
                    }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem { BreadcrumbLink { href: "#", "Components" } }
                BreadcrumbSeparator {}
                BreadcrumbItem { BreadcrumbPage { "Breadcrumb" } }
            }
            // A custom separator and a step you can't open yet.
            Breadcrumb { aria_label: "Billing breadcrumb",
                BreadcrumbItem { BreadcrumbLink { href: "#", "Workspace" } }
                BreadcrumbSeparator { ChevronRight { "aria-hidden": "true" } }
                BreadcrumbItem { BreadcrumbLink { href: "#", disabled: true, "Billing (owners only)" } }
                BreadcrumbSeparator { ChevronRight { "aria-hidden": "true" } }
                BreadcrumbItem { BreadcrumbPage { "Invoices" } }
            }
            // A long trail collapses its middle; "…" shows the rest.
            BreadcrumbTrail { aria_label: "Project breadcrumb", max_items: 3,
                items: vec![
                    Crumb::link("Home", "#"),
                    Crumb::link("Projects", "#"),
                    Crumb::link("Atoll", "#"),
                    Crumb::link("Settings", "#"),
                    Crumb::page("Members"),
                ],
            }
        }
    }
}
```
