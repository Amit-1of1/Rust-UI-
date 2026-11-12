# Pagination

- Copy into your app: `atoll add pagination` or `dx components add pagination --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `icons`, `native_select`, `ui_strings`).
- Import: `use crate::components::pagination::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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
/// Each page button is named "Page N" (translatable), and the current one says so with
/// `aria-current="page"`. With `page_sizes`, a "Rows per page" select comes after the pages.
/// When the number of pages isn't known (a cursor-based API), leave `pages` out and pass
/// `has_next`: the pager shows Previous, "Page N" and Next.
///
/// For pages that are URLs, compose [`Pagination`] with [`PaginationLink`] instead.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn Pager(
    page: usize,
    /// How many pages there are. Leave it out (0) with `has_next` when that isn't known.
    #[props(default)]
    pages: usize,
    on_page_change: EventHandler<usize>,
    #[props(default = 1)] siblings: usize,
    /// For an unknown number of pages: whether there is a page after this one.
    #[props(default)]
    has_next: Option<bool>,
    /// Choices for the page size ("10, 25, 50"); shows a select when not empty.
    #[props(default)]
    page_sizes: Vec<usize>,
    /// The current page size, one of `page_sizes`.
    #[props(default)]
    page_size: usize,
    #[props(default)] on_page_size_change: Option<EventHandler<usize>>,
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
    let mut size = use_signal(|| 25usize);
    let mut feed = use_signal(|| 1usize);
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
            page_sizes: vec![10, 25, 50],
            page_size: size(),
            on_page_size_change: move |n| size.set(n),
        }
        // Unknown total (a cursor-based feed): Previous, "Page N", Next.
        Pager {
            aria_label: "Pages of the activity feed",
            page: feed(),
            has_next: feed() < 4,
            on_page_change: move |p| feed.set(p),
        }
        }
    }
}
```
