//! Server-side render smoke tests: every exported component renders without
//! panicking and keeps the accessibility attributes its primitive provides.

use dioxus::prelude::*;
use ui::prelude::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn button_renders_variant_and_size() {
    let html = render(|| {
        rsx! {
            Button { variant: ButtonVariant::Destructive, size: ButtonSize::Sm, "Delete" }
        }
    });
    assert!(html.contains("Delete"));
    assert!(html.contains("<button"));
}

#[test]
fn switch_exposes_switch_role() {
    let html = render(|| rsx! { Switch { aria_label: "Wifi" } });
    assert!(html.contains("role=\"switch\""), "{html}");
}

#[test]
fn checkbox_exposes_checkbox_role() {
    let html = render(|| rsx! { Checkbox { id: "c", name: "c" } });
    assert!(html.contains("role=\"checkbox\""), "{html}");
}

#[test]
fn tabs_expose_tablist_and_tab_roles() {
    let html = render(|| {
        rsx! {
            Tabs { default_value: "a".to_string(),
                TabList {
                    TabTrigger { value: "a".to_string(), index: 0usize, "A" }
                    TabTrigger { value: "b".to_string(), index: 1usize, "B" }
                }
                TabContent { index: 0usize, value: "a".to_string(), "Panel A" }
            }
        }
    });
    assert!(html.contains("role=\"tablist\""), "{html}");
    assert!(html.contains("role=\"tab\""), "{html}");
    assert!(html.contains("Panel A"));
}

#[test]
fn layout_components_render() {
    let html = render(|| {
        rsx! {
            Card {
                CardHeader { CardTitle { "Title" } CardDescription { "Desc" } }
                CardContent {
                    Label { html_for: "x", "Name" }
                    Input { id: "x" }
                    Badge { variant: BadgeVariant::Outline, "New" }
                    Separator { horizontal: true, decorative: true }
                }
                CardFooter { "Footer" }
            }
            Accordion {
                AccordionItem { index: 0usize,
                    AccordionTrigger { "Question" }
                    AccordionContent { "Answer" }
                }
            }
        }
    });
    for text in ["Title", "Desc", "Name", "New", "Footer", "Question"] {
        assert!(html.contains(text), "missing {text} in {html}");
    }
}

#[test]
fn closed_overlays_render_without_panicking() {
    let html = render(|| {
        rsx! {
            ThemeStyles {}
            ToastProvider {
                Dialog { open: false, DialogTitle { "Dialog" } }
                Sheet { open: false, SheetTitle { "Sheet" } }
                PopoverRoot { PopoverTrigger { "Pop" } PopoverContent { "Body" } }
                DropdownMenu {
                    DropdownMenuTrigger { "Menu" }
                    DropdownMenuContent {
                        DropdownMenuItem::<u8> { value: 1, index: 0usize, "One" }
                    }
                }
                Tooltip { TooltipTrigger { "Tip" } TooltipContent { "Hint" } }
            }
        }
    });
    for text in ["Pop", "Menu", "Tip"] {
        assert!(html.contains(text), "missing {text} in {html}");
    }
}

#[test]
fn theme_customizer_offers_labelled_radios_and_a_range() {
    let html = render(|| rsx! { ThemeCustomizer {} });
    assert!(html.contains("<legend>Accent colour</legend>"), "{html}");
    assert_eq!(
        html.matches("type=\"radio\"").count(),
        ACCENTS.len() + 3,
        "{html}"
    );
    assert!(html.contains("for=\"theme-radius\""), "{html}");
}

#[test]
fn callout_is_a_labelled_note() {
    let html =
        render(|| rsx! { Callout { variant: CalloutVariant::Warning, title: "Careful", "Body" } });
    assert!(html.contains("role=\"note\""), "{html}");
    assert!(html.contains("data-variant=\"warning\""), "{html}");
    assert!(html.contains("Careful") && html.contains("Body"), "{html}");
}

#[test]
fn empty_title_is_a_heading_and_media_is_hidden() {
    let html = render(|| rsx! { Empty { EmptyMedia { "i" } EmptyTitle { level: 2, "Nothing" } } });
    assert!(html.contains("role=\"heading\""), "{html}");
    assert!(html.contains("aria-level=\"2\""), "{html}");
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
}

#[test]
fn status_hides_the_dot_and_keeps_the_label() {
    let html = render(|| rsx! { Status { kind: StatusKind::Error, pulse: true, "Down" } });
    assert!(html.contains("data-kind=\"error\""), "{html}");
    assert!(html.contains("data-pulse=\"true\""), "{html}");
    assert!(
        html.contains("aria-hidden=\"true\"") && html.contains("Down"),
        "{html}"
    );
}

#[test]
fn shimmer_is_busy_only_while_loading() {
    let on = render(|| rsx! { Shimmer { loading: true, "x" } });
    let off = render(|| rsx! { Shimmer { loading: false, "x" } });
    assert!(on.contains("aria-busy=\"true\""), "{on}");
    assert!(!off.contains("aria-busy"), "{off}");
}

#[test]
fn marquee_has_a_pause_button_and_hides_its_copy() {
    let html = render(|| rsx! { Marquee { label: "Logos", span { "Acme" } } });
    assert!(html.contains("aria-label=\"Logos\""), "{html}");
    assert!(html.contains("aria-pressed=\"false\""), "{html}");
    assert_eq!(html.matches("Acme").count(), 2, "{html}");
    assert!(html.contains("inert"), "{html}");
}

#[test]
fn button_group_is_a_group() {
    let html =
        render(|| rsx! { ButtonGroup { vertical: true, "aria-label": "Zoom", Button { "+" } } });
    assert!(html.contains("role=\"group\""), "{html}");
    assert!(html.contains("data-orientation=\"vertical\""), "{html}");
}

#[test]
fn stepper_marks_the_current_step() {
    let html = render(|| {
        rsx! {
            Stepper { total: 3, default_step: 1,
                StepperItem { step: 0, title: "A" }
                StepperItem { step: 1, title: "B" }
                StepperItem { step: 2, title: "C" }
            }
        }
    });
    assert_eq!(html.matches("aria-current=\"step\"").count(), 1, "{html}");
    assert!(
        html.contains("data-state=\"done\"") && html.contains("data-state=\"upcoming\""),
        "{html}"
    );
    assert!(html.contains("(completed)"), "{html}");
}

#[test]
fn bottom_nav_marks_the_current_page() {
    let html = render(|| {
        rsx! {
            BottomNav { label: "Tabs",
                BottomNavItem { label: "Home", current: true, href: "/" }
                BottomNavItem { label: "Search" }
            }
        }
    });
    assert!(html.contains("aria-label=\"Tabs\""), "{html}");
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1, "{html}");
    assert!(html.contains("<a ") && html.contains("<button"), "{html}");
}
