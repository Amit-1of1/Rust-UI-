use super::render;
use dioxus::prelude::*;
use ui::components::breadcrumb::visible_crumbs;
use ui::prelude::*;

#[test]
fn long_trails_keep_the_first_and_last_steps() {
    assert_eq!(visible_crumbs(6, 3), vec![Some(0), None, Some(4), Some(5)]);
    assert_eq!(visible_crumbs(6, 4), vec![Some(0), None, Some(3), Some(4), Some(5)]);
    // Hiding one step would save nothing: show it.
    assert_eq!(visible_crumbs(5, 4).len(), 5);
    assert_eq!(visible_crumbs(4, 3), vec![Some(0), Some(1), Some(2), Some(3)]);
    assert_eq!(visible_crumbs(2, 0), vec![Some(0), Some(1)]);
    assert_eq!(visible_crumbs(0, 3), Vec::<Option<usize>>::new());
}

#[test]
fn a_collapsed_trail_names_its_ellipsis_and_marks_the_current_page() {
    let html = render(|| {
        rsx! {
            BreadcrumbTrail { max_items: 3, items: vec![
                Crumb::link("Home", "/"),
                Crumb::link("Projects", "/p"),
                Crumb::link("Atoll", "/p/a"),
                Crumb::link("Settings", "/p/a/s"),
                Crumb::page("Members"),
            ] }
        }
    });
    assert!(html.contains("aria-label=\"Show 2 more\""), "{html}");
    assert!(!html.contains("Projects"), "{html}");
    assert!(html.contains("aria-current=\"page\""), "{html}");
}

#[test]
fn disabled_steps_and_custom_separators() {
    let html = render(|| {
        rsx! {
            Breadcrumb {
                BreadcrumbItem { BreadcrumbLink { href: "/b", disabled: true, "Billing" } }
                BreadcrumbSeparator { "›" }
                BreadcrumbItem { BreadcrumbPage { "Invoices" } }
                BreadcrumbSeparator {}
            }
        }
    });
    assert!(html.contains("aria-disabled=\"true\""), "{html}");
    assert!(!html.contains("href=\"/b\""), "{html}");
    assert!(html.contains("›"), "{html}");
    assert!(html.contains(">/"), "the default separator: {html}");
}
