# Pagination

- Copy into your app: `rust-ui add pagination` or `dx components add pagination --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`, `ui_strings`).
- Import: `use crate::components::pagination::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The pagination component provides navigational controls for paged content. It exposes a consistent structure for previous/next actions, individual page links, and an optional ellipsis for truncated ranges.

## Component Structure

```rust
// The Pagination component wraps the entire control.
Pagination {
    // PaginationContent groups all items in a horizontal list.
    PaginationContent {
        // PaginationItem is the container for a single pagination element.
        // Use one item at a time and swap the inner component as needed.
        PaginationItem {
            // PaginationPrevious renders a previous-page link.
            // - Set href to your previous page url.
            PaginationPrevious { href: "#" }

            // PaginationLink renders a numbered page link.
            // - is_active marks the current page.
            // - href sets the target page.
            PaginationLink { href: "#", is_active: true, "2" }

            // PaginationEllipsis indicates truncated pages.
            PaginationEllipsis {}

            // PaginationNext renders a next-page link.
            // - Set href to your next page url.
            PaginationNext { href: "#" }
        }
    }
}
```

## Notes

- `PaginationLink` uses `is_active` to indicate the current page.
- `PaginationPrevious` and `PaginationNext` show labels on larger (non-mobile) screens; labels are hidden on smaller screens to keep the control compact.

## API

```rust
pub enum PaginationLinkSize {
    #[default]
    Icon,
    Default,
}

pub enum PaginationLinkKind {
    Previous,
    Next,
}

#[component]
pub fn Pagination(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn PaginationContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn PaginationItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

pub struct PaginationLinkProps {
    #[props(default)]
    pub is_active: bool,
    #[props(default)]
    pub size: PaginationLinkSize,
    #[props(default)]
    pub data_kind: Option<PaginationLinkKind>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn PaginationLink(props: PaginationLinkProps) -> Element

#[component]
pub fn PaginationPrevious(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element

#[component]
pub fn PaginationNext(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element

#[component]
pub fn PaginationEllipsis(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
use crate::components::pagination::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Pagination {
            PaginationContent {
                PaginationItem {
                    PaginationPrevious { href: "#" }
                }
                PaginationItem {
                    PaginationLink { href: "#", "1" }
                }
                PaginationItem {
                    PaginationLink { href: "#", is_active: true, "2" }
                }
                PaginationItem {
                    PaginationLink { href: "#", "3" }
                }
                PaginationItem {
                    PaginationEllipsis {}
                }
                PaginationItem {
                    PaginationNext { href: "#" }
                }
            }
        }
    }
}
```
