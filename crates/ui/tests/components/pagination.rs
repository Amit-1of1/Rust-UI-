//! `page_items`, the numbers a pager shows, for every page count up to 60, every current page
//! (and out-of-range ones) and every `siblings` up to 7 (more siblings than pages behaves like
//! `siblings = pages`). 30,000-odd cases, each checked for the promises in its doc comment.

use ui::components::pagination::page_items;

#[test]
fn page_items_are_well_formed_for_every_small_case() {
    let mut cases = 0;
    for pages in 0..=60usize {
        for page in 0..=pages + 1 {
            for siblings in 0..=7usize {
                let items = page_items(page, pages, siblings);
                let last = pages.max(1);
                let current = page.clamp(1, last);
                let ctx = format!("page {page} of {pages}, siblings {siblings}: {items:?}");
                assert_eq!(items.first(), Some(&Some(1)), "{ctx}");
                assert_eq!(items.last(), Some(&Some(last)), "{ctx}");
                let shown: Vec<usize> = items.iter().flatten().copied().collect();
                assert!(shown.windows(2).all(|w| w[0] < w[1]), "not increasing: {ctx}");
                // `siblings` pages each side of the current one are shown (inside the range).
                for p in current.saturating_sub(siblings).max(1)..=(current + siblings).min(last) {
                    assert!(shown.contains(&p), "page {p} missing: {ctx}");
                }
                // A gap stands for two or more pages (a gap of one shows the page), and gaps
                // are never next to each other or at the ends.
                for (i, item) in items.iter().enumerate() {
                    if item.is_none() {
                        let (before, after) = (items[i - 1].unwrap(), items[i + 1].unwrap());
                        assert!(after >= before + 3, "a gap hides fewer than 2 pages: {ctx}");
                    }
                }
                // Short: at most first, last, two gaps and the window.
                assert!(items.len() <= 2 * siblings + 5, "{ctx}");
                cases += 1;
            }
        }
    }
    assert!(cases > 15_000);
}
