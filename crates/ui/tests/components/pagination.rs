use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

#[test]
fn pager_names_each_page_and_offers_page_sizes() {
    let html = render(|| {
        rsx! {
            Pager { page: 2, pages: 5, on_page_change: |_| {},
                page_sizes: vec![10, 25], page_size: 25,
            }
        }
    });
    assert!(html.contains("aria-label=\"Page 3\""), "{html}");
    assert!(html.contains("aria-label=\"Pagination\""), "{html}");
    assert!(html.contains("Rows per page"), "{html}");
}

#[test]
fn pager_without_a_total_shows_the_page_and_ends_at_has_next() {
    let html = render(|| {
        rsx! {
            Pager { page: 3, has_next: false, on_page_change: |_| {} }
        }
    });
    assert!(html.contains("Page 3"), "{html}");
    assert!(!html.contains("aria-label=\"Page 1\""), "{html}");
    // Next is disabled: the second aria-disabled after Previous (which is enabled on page 3).
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 1, "{html}");
}
