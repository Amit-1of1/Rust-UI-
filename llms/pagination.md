# Pagination

- Copy into your app: `rust-ui add pagination` or `dx components add pagination --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `icons`, `ui_strings`).
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
pub fn PaginationLink(mut props: PaginationLinkProps) -> Element

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

/// Client-side paging: Previous, numbered pages and Next as buttons, for a list that pages
/// in place (no URL per page). `page` is 1-based. Previous and Next are `aria-disabled` on
/// the first and last page, so they keep focus but do nothing; long ranges collapse to
/// "1 … 4 5 6 … 20" (`siblings` pages each side of the current one).
///
/// For pages that are URLs, compose [`Pagination`] with [`PaginationLink`] instead.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn Pager(
    page: usize,
    pages: usize,
    on_page_change: EventHandler<usize>,
    #[props(default = 1)] siblings: usize,
    /// Goes on the `<nav>`: give it an `aria-label` ("Pages of results").
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element

/// The page numbers [`Pager`] shows, `None` for a gap: the first and last page, and
/// `siblings` pages each side of `page`. A gap of one page shows that page instead.
pub fn page_items(page: usize, pages: usize, siblings: usize) -> Vec<Option<usize>>
```

## Example

```rust
use crate::components::pagination::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut page = use_signal(|| 6usize);
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
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
        // Client-side paging: buttons, and the ends do nothing on the first and last page.
        Pager {
            aria_label: "Pages of results",
            page: page(),
            pages: 20,
            on_page_change: move |p| page.set(p),
        }
        }
    }
}
```
