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
