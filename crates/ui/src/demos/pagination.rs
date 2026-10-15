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
