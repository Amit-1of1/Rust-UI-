//! Server-side render smoke tests: every exported component renders without
//! panicking and keeps the accessibility attributes its primitive provides.

use dioxus::prelude::*;
use ui::components::drag_and_drop_list::DragAndDropList;
use ui::components::tag_group as tg;
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
        ACCENTS.len() + DENSITIES.len() + 3,
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

#[test]
fn form_labels_fields_and_turns_native_validation_off() {
    let html = render(|| {
        rsx! { ui::demos::form::Demo {} }
    });
    assert!(html.contains("novalidate"), "{html}");
    assert_eq!(html.matches("<label").count(), 2, "{html}");
    assert!(html.contains("autocomplete=\"email\""), "{html}");
    // Pristine fields show no error and are not marked invalid.
    assert!(!html.contains("aria-invalid"), "{html}");
    assert!(!html.contains("role=\"alert\""), "{html}");
}

#[test]
fn data_table_renders_a_captioned_page_with_a_status() {
    let html = render(|| {
        rsx! { ui::demos::data_table::Demo {} }
    });
    assert!(html.contains("<caption"), "{html}");
    assert!(html.contains("Team directory"), "{html}");
    assert!(html.contains("aria-sort=\"none\""), "{html}");
    assert!(html.contains("Showing 1–5 of 14 rows"), "{html}");
    assert!(html.contains("Page 1 of 3"), "{html}");
    assert!(html.contains("aria-label=\"Filter rows\""), "{html}");
    assert_eq!(html.matches("type=\"checkbox\"").count(), 6, "{html}");
}

#[test]
fn input_otp_has_a_named_group_and_one_input_per_character() {
    let html = render(|| rsx! { InputOtp { label: "Code", length: 4, group: 2 } });
    assert!(
        html.contains("role=\"group\"") && html.contains("aria-label=\"Code\""),
        "{html}"
    );
    assert_eq!(html.matches("<input").count(), 4, "{html}");
    assert!(html.contains("Code, character 3 of 4"), "{html}");
    assert_eq!(
        html.matches("autocomplete=\"one-time-code\"").count(),
        1,
        "{html}"
    );
    assert_eq!(
        html.matches("aria-hidden=\"true\"").count(),
        1,
        "one dash between the two groups: {html}"
    );
}

#[test]
fn input_group_wraps_a_control_and_marks_invalid() {
    let html = render(|| {
        rsx! {
            InputGroup { invalid: true,
                InputGroupAddon { "https://" }
                InputGroupInput { "aria-label": "Site" }
                InputGroupAddon { end: true, "go" }
            }
        }
    });
    assert!(
        html.contains("role=\"group\"") && html.contains("data-invalid=\"true\""),
        "{html}"
    );
    assert!(
        html.contains("data-align=\"start\"") && html.contains("data-align=\"end\""),
        "{html}"
    );
    assert!(html.contains("aria-label=\"Site\""), "{html}");
}

#[test]
fn carousel_is_a_labelled_carousel_of_labelled_slides() {
    let html = render(|| {
        rsx! {
            Carousel { label: "Plans",
                CarouselItem { label: "1 of 2", "A" }
                CarouselItem { label: "2 of 2", "B" }
            }
        }
    });
    assert!(html.contains("aria-roledescription=\"carousel\""), "{html}");
    assert_eq!(
        html.matches("aria-roledescription=\"slide\"").count(),
        2,
        "{html}"
    );
    assert!(html.contains("tabindex=\"0\""), "{html}");
    assert!(html.contains("Previous") && html.contains("Next"), "{html}");
}

#[test]
fn dropzone_wraps_a_native_file_input_in_a_label() {
    let html = render(|| rsx! { Dropzone { accept: "image/*", multiple: true } });
    assert!(html.contains("<label"), "{html}");
    assert!(html.contains("type=\"file\""), "{html}");
    assert!(
        html.contains("accept=\"image/*\"") && html.contains("multiple"),
        "{html}"
    );
    assert!(html.contains("Drop files here"), "{html}");
    assert!(
        !html.contains("role=\"alert\""),
        "no error before any drop: {html}"
    );
}

#[test]
fn multi_select_starts_closed_with_a_named_button() {
    let html = render(|| {
        rsx! {
            MultiSelect {
                label: "Teams",
                default_selected: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                options: vec![
                    MultiOption::new("a", "Alpha"),
                    MultiOption::new("b", "Beta"),
                    MultiOption::new("c", "Gamma"),
                ],
            }
        }
    });
    assert!(html.contains("aria-expanded=\"false\""), "{html}");
    assert!(html.contains("Teams: Alpha, Beta +1"), "{html}");
    assert!(
        !html.contains("type=\"checkbox\""),
        "panel is closed: {html}"
    );
}

#[test]
fn drawer_renders_nothing_visible_while_closed() {
    // Open dialogs mount after the first render, so the open behaviour is covered by the
    // browser test in tests/e2e/drawer.mjs; here we only check closed is quiet.
    let html = render(|| {
        rsx! {
            Drawer { open: false,
                DrawerTitle { "Move goal" }
            }
        }
    });
    assert!(!html.contains("Move goal"), "{html}");
}

#[test]
fn textarea_autosize_is_opt_in() {
    let plain = render(|| rsx! { Textarea { aria_label: "Notes" } });
    assert!(!plain.contains("data-autosize"), "{plain}");
    let grows = render(|| rsx! { Textarea { aria_label: "Notes", autosize: true } });
    assert!(grows.contains("data-autosize=\"true\""), "{grows}");
}

#[test]
fn ui_strings_provider_renders_its_children() {
    let html = render(|| {
        let strings = use_signal(UiStrings::default);
        rsx! { UiStringsProvider { strings, "hello" } }
    });
    assert!(html.contains("hello"), "{html}");
}

// ---- found by apps/dashboard (see apps/dashboard/GAPS.md) ----

#[test]
fn card_keeps_its_own_class_next_to_the_callers() {
    let html = render(|| rsx! { Card { class: "kpi", CardTitle { class: "big", "7" } } });
    assert!(
        html.contains("dx-card-") && html.contains(" kpi\""),
        "{html}"
    );
    assert!(
        html.contains("dx-card-title-") && html.contains(" big\""),
        "{html}"
    );
}

#[test]
fn data_table_render_cell_replaces_the_text() {
    let html = render(|| {
        rsx! {
            DataTable {
                columns: vec![DataColumn::new("Name"), DataColumn::new("State")],
                rows: vec![vec!["Ada".to_string(), "Paid".to_string()]],
                caption: "Orders",
                render_cell: |(row, col): (usize, usize)| {
                    if col == 1 {
                        rsx! { Badge { "badge-{row}" } }
                    } else {
                        rsx! { "plain" }
                    }
                },
            }
        }
    });
    assert!(html.contains("badge-0") && html.contains("plain"), "{html}");
    assert!(!html.contains(">Paid<"), "{html}");
}

#[test]
fn active_sidebar_item_is_the_current_page() {
    let html = render(|| {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarContent {
                        SidebarMenu {
                            SidebarMenuItem { SidebarMenuButton { is_active: true, "Home" } }
                            SidebarMenuItem { SidebarMenuButton { "Other" } }
                        }
                    }
                }
            }
        }
    });
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1, "{html}");
}

#[test]
fn select_label_names_the_trigger_not_the_root() {
    let html = render(|| {
        rsx! {
            Select::<String> { aria_label: "Fruit",
                SelectOption::<String> { index: 0usize, value: "a", "Apple" }
            }
        }
    });
    // The root keeps no label; the trigger is named by the label and its own value.
    assert!(!html.contains("aria-label=\"Fruit\""), "{html}");
    let trigger = html.split("<button").nth(1).unwrap_or_default();
    let by = trigger
        .split("aria-labelledby=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap_or_else(|| panic!("trigger has no aria-labelledby: {html}"));
    let ids: Vec<&str> = by.split_whitespace().collect();
    assert_eq!(ids.len(), 2, "{by}");
    assert!(
        html.contains(&format!("id=\"{}\" hidden=true>Fruit<", ids[0])),
        "{html}"
    );
    assert!(trigger.contains(&format!("id=\"{}\"", ids[1])), "{html}");

    let html = render(|| {
        rsx! {
            label { id: "fruit-label", "Fruit" }
            SelectMulti::<String> { aria_labelledby: "fruit-label",
                SelectOption::<String> { index: 0usize, value: "a", "Apple" }
            }
        }
    });
    assert!(
        html.contains("aria-labelledby=\"fruit-label rui-select-"),
        "{html}"
    );
    assert!(
        !html.contains("hidden=true>"),
        "no label span needed: {html}"
    );
}

#[test]
fn tab_panels_are_labelled_by_their_tab() {
    let html = render(|| {
        rsx! {
            Tabs { default_value: "a".to_string(),
                TabList {
                    TabTrigger { value: "a".to_string(), index: 0usize, "A" }
                    TabTrigger { value: "b".to_string(), index: 1usize, id: "own", "B" }
                }
                TabContent { value: "a".to_string(), index: 0usize, "Panel A" }
                TabContent { value: "b".to_string(), index: 1usize, aria_labelledby: "own", "Panel B" }
            }
        }
    });
    let tab = html
        .split("role=\"tab\"")
        .nth(1)
        .and_then(|s| s.split("id=\"").nth(1))
        .and_then(|s| s.split('"').next())
        .unwrap_or_else(|| panic!("first tab has no id: {html}"));
    assert!(
        html.contains(&format!("aria-labelledby=\"{tab}\"")),
        "{html}"
    );
    assert!(html.contains("aria-labelledby=\"own\""), "{html}");
}

#[test]
fn tooltip_trigger_describes_only_while_open() {
    let closed = render(|| rsx! { Tooltip { TooltipTrigger { "Tip" } TooltipContent { "Hint" } } });
    assert!(!closed.contains("aria-describedby"), "{closed}");
    let open = render(|| {
        rsx! { Tooltip { default_open: true, TooltipTrigger { "Tip" } TooltipContent { "Hint" } } }
    });
    assert!(open.contains("aria-describedby"), "{open}");
}

#[test]
fn accordion_trigger_is_collapsed_button() {
    let html = render(|| {
        rsx! {
            Accordion {
                AccordionItem { index: 0usize,
                    AccordionTrigger { "Question" }
                    AccordionContent { "Answer" }
                }
            }
        }
    });
    assert!(html.contains("<button"), "{html}");
    assert!(html.contains("aria-expanded=false"), "{html}");
    assert!(html.contains("aria-controls="), "{html}");
    assert!(html.contains("data-open=false"), "{html}");
    assert!(html.contains("Question"), "{html}");
}

#[test]
fn alert_has_alert_role_and_variant() {
    let html = render(|| {
        rsx! {
            Alert { variant: AlertVariant::Destructive,
                AlertTitle { "Heads up" }
                AlertDescription { "Session expired" }
            }
        }
    });
    assert!(html.contains("role=\"alert\""), "{html}");
    assert!(html.contains("data-variant=\"destructive\""), "{html}");
    assert!(html.contains("Heads up"), "{html}");
    assert!(html.contains("Session expired"), "{html}");
}

#[test]
fn alert_dialog_closed_renders_no_dialog() {
    // The dialog content only appears after mount, even when open; assert the closed state.
    let html = render(|| {
        rsx! {
            p { "outside" }
            AlertDialog { open: false,
                AlertDialogTitle { "Delete item" }
                AlertDialogDescription { "Are you sure?" }
                AlertDialogActions {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction { "Delete" }
                }
            }
        }
    });
    assert!(html.contains("outside"), "{html}");
    assert!(!html.contains("alertdialog"), "{html}");
    assert!(!html.contains("Delete item"), "{html}");
}

#[test]
fn aspect_ratio_pads_to_ratio() {
    let html = render(|| rsx! { AspectRatio { ratio: 4.0 / 3.0, div { "Media" } } });
    assert!(html.contains("padding-bottom: 75%"), "{html}");
    assert!(html.contains("position: absolute"), "{html}");
    assert!(html.contains("Media"), "{html}");
}

#[test]
fn avatar_exposes_img_role_and_fallback() {
    let html = render(|| {
        rsx! {
            ImageAvatar { src: "data:image/png;base64,", alt: "User avatar", aria_label: "Basic avatar", "EA" }
        }
    });
    assert!(html.contains("role=\"img\""), "{html}");
    assert!(html.contains("aria-label=\"Basic avatar\""), "{html}");
    assert!(html.contains("alt=\"User avatar\""), "{html}");
    assert!(html.contains("data-state=\"empty\""), "{html}");
    assert!(html.contains(">EA</span>"), "{html}");
}

#[test]
fn badge_renders_variant_and_text() {
    let html = render(|| rsx! { Badge { variant: BadgeVariant::Secondary, "Secondary" } });
    assert!(html.contains("<span"), "{html}");
    assert!(html.contains("data-style=\"secondary\""), "{html}");
    assert!(html.contains("Secondary"), "{html}");
}

#[test]
fn breadcrumb_is_labelled_nav_with_current_page() {
    let html = render(|| {
        rsx! {
            Breadcrumb {
                BreadcrumbItem { BreadcrumbLink { href: "#", "Home" } }
                BreadcrumbSeparator {}
                BreadcrumbItem { BreadcrumbPage { "Breadcrumb" } }
            }
        }
    });
    assert!(html.contains("<nav aria-label=\"Breadcrumb\""), "{html}");
    assert!(html.contains("<ol"), "{html}");
    assert!(
        html.contains("role=\"presentation\" aria-hidden=\"true\""),
        "{html}"
    );
    assert!(html.contains("aria-current=\"page\">Breadcrumb"), "{html}");
    assert!(html.contains("href=\"#\">Home"), "{html}");
}

#[test]
fn calendar_renders_grid_for_view_month() {
    let html = render(|| {
        let day = time::macros::date!(2026 - 05 - 15);
        rsx! { Calendar { selected_date: None, view_date: day, today: day } }
    });
    assert!(html.contains("role=\"application\""), "{html}");
    assert!(html.contains("role=\"grid\""), "{html}");
    assert!(html.contains("aria-label=\"Previous month\""), "{html}");
    assert!(
        html.contains("aria-label=\"Friday, May 15, 2026\""),
        "{html}"
    );
    assert!(html.contains("data-today=true"), "{html}");
}

#[test]
fn card_renders_all_slots() {
    let html = render(|| {
        rsx! {
            Card {
                CardHeader { CardTitle { "Create project" } CardDescription { "One click" } }
                CardContent { "Body" }
                CardFooter { "Footer" }
            }
        }
    });
    for slot in [
        "card",
        "card-header",
        "card-title",
        "card-description",
        "card-content",
        "card-footer",
    ] {
        assert!(
            html.contains(&format!("data-slot=\"{slot}\"")),
            "missing {slot}: {html}"
        );
    }
    assert!(html.contains("Create project"), "{html}");
}

#[test]
fn chart_svgs_are_labelled_images() {
    let html = render(|| {
        let data = vec![ChartDatum::new("Jan", 186.0), ChartDatum::new("Feb", 305.0)];
        rsx! {
            BarChart { data: data.clone(), label: "Visitors" }
            LineChart { data, label: "Trend" }
        }
    });
    assert_eq!(html.matches("role=\"img\"").count(), 2, "{html}");
    assert!(html.contains("aria-label=\"Visitors\""), "{html}");
    assert!(html.contains("<title>Trend</title>"), "{html}");
    assert!(html.contains("<desc>Jan: 186, Feb: 305</desc>"), "{html}");
}

#[test]
fn collapsible_starts_closed() {
    let html = render(|| {
        rsx! {
            Collapsible {
                CollapsibleTrigger { "Recent Activity" }
                CollapsibleList {
                    CollapsibleItem { "Visible" }
                    CollapsibleContent { CollapsibleItem { "Hidden" } }
                }
            }
        }
    });
    assert!(html.contains("aria-expanded=false"), "{html}");
    assert!(html.contains("aria-controls="), "{html}");
    assert!(html.contains("data-open=false"), "{html}");
    assert!(
        html.contains("Recent Activity") && html.contains("Visible"),
        "{html}"
    );
    assert!(!html.contains("Hidden"), "{html}");
}

#[test]
fn color_picker_trigger_is_closed_and_labelled() {
    // The popover only renders when open; assert the closed trigger.
    let html = render(|| rsx! { ColorPicker { label: "Pick a color" } });
    assert!(
        html.contains("role=\"group\" aria-label=\"Color picker\""),
        "{html}"
    );
    assert!(html.contains("data-state=\"closed\""), "{html}");
    assert!(html.contains("aria-expanded=false"), "{html}");
    assert!(
        html.contains("aria-label=\"Selected color #000000\""),
        "{html}"
    );
    assert!(html.contains("Pick a color"), "{html}");
}

#[test]
fn combobox_input_has_combobox_role() {
    // The listbox and its options only render when open; assert the closed input.
    let html = render(|| {
        rsx! {
            Combobox::<String> {
                placeholder: "Select framework...",
                aria_label: "Select framework",
                ComboboxEmpty { "No framework found." }
                ComboboxOption::<String> { index: 0usize, value: "dioxus".to_string(), text_value: "Dioxus".to_string(), "Dioxus" }
            }
        }
    });
    assert!(html.contains("role=\"combobox\""), "{html}");
    assert!(html.contains("aria-haspopup=\"listbox\""), "{html}");
    assert!(html.contains("aria-expanded=false"), "{html}");
    assert!(html.contains("aria-label=\"Select framework\""), "{html}");
    assert!(
        html.contains("placeholder=\"Select framework...\""),
        "{html}"
    );
}

#[test]
fn command_dialog_closed_renders_nothing() {
    // The palette lives in a Dialog whose content only appears after mount; assert closed.
    let html = render(|| {
        rsx! {
            p { "page" }
            CommandDialog {
                open: false,
                on_open_change: |_| {},
                items: vec![CommandItem::new("new-file", "New file").group("File")],
                on_select: |_| {},
            }
        }
    });
    assert!(html.contains("page"), "{html}");
    assert!(!html.contains("role=\"listbox\""), "{html}");
    assert!(!html.contains("New file"), "{html}");
}

#[test]
fn context_menu_trigger_has_menu_popup() {
    // Menu content only renders once opened; assert the closed trigger.
    let html = render(|| {
        rsx! {
            ContextMenu {
                ContextMenuTrigger { "right click here" }
                ContextMenuContent {
                    ContextMenuItem { value: "edit".to_string(), index: 0usize, on_select: |_| {}, "Edit" }
                }
            }
        }
    });
    assert!(
        html.contains("role=\"button\" aria-haspopup=\"menu\""),
        "{html}"
    );
    assert!(html.contains("aria-expanded=false"), "{html}");
    assert!(html.contains("data-state=\"closed\""), "{html}");
    assert!(html.contains("right click here"), "{html}");
    assert!(!html.contains("role=\"menuitem\""), "{html}");
}

#[test]
fn date_picker_renders_group_with_spinbutton_segments() {
    let html = render(|| rsx! { DatePicker {} });
    assert!(html.contains("role=\"group\""), "{html}");
    assert_eq!(html.matches("role=\"spinbutton\"").count(), 3, "{html}");
    assert!(html.contains("aria-label=\"Show Calendar\""), "{html}");
    assert!(html.contains("data-state=\"closed\""), "{html}");
}

#[test]
fn dialog_renders_nothing_while_closed() {
    // The dialog body is only emitted after mount; a closed dialog must not leak content.
    let html = render(|| {
        rsx! {
            Dialog { open: false,
                DialogTitle { "Are you sure?" }
                DialogDescription { "This cannot be undone." }
            }
        }
    });
    assert!(!html.contains("Are you sure?"), "{html}");
    assert!(!html.contains("role=\"dialog\""), "{html}");
}

#[test]
fn drag_and_drop_list_exposes_sortable_roles() {
    let html = render(|| {
        rsx! {
            DragAndDropList { items: vec![rsx! { "One" }, rsx! { "Two" }], aria_label: "Tasks".to_string() }
        }
    });
    assert!(html.contains("aria-label=\"Tasks\""), "{html}");
    assert!(
        html.contains("aria-roledescription=\"sortable list\""),
        "{html}"
    );
    assert_eq!(
        html.matches("aria-roledescription=\"sortable item\"")
            .count(),
        2,
        "{html}"
    );
    assert!(html.contains("role=\"status\""), "{html}");
    assert!(html.contains("One") && html.contains("Two"), "{html}");
}

#[test]
fn dropdown_menu_trigger_is_collapsed_when_closed() {
    // Content is not rendered until the menu opens; check the trigger state.
    let html = render(|| {
        rsx! {
            DropdownMenu {
                DropdownMenuTrigger { "Actions" }
                DropdownMenuContent {
                    DropdownMenuItem::<&'static str> { value: "edit", index: 0usize, "Edit" }
                }
            }
        }
    });
    assert!(html.contains("aria-expanded=false"), "{html}");
    assert!(html.contains("aria-haspopup=\"listbox\""), "{html}");
    assert!(html.contains("data-state=\"closed\""), "{html}");
    assert!(html.contains("Actions"), "{html}");
}

#[test]
fn field_marks_invalid_and_announces_error() {
    let html = render(|| {
        rsx! {
            Field { invalid: true,
                FieldLabel { html_for: "email", "Email" }
                FieldDescription { id: "email-desc", "We never share it." }
                FieldError { id: "email-err", message: Some("Required".to_string()) }
            }
        }
    });
    assert!(html.contains("data-invalid=\"true\""), "{html}");
    assert!(html.contains("for=\"email\""), "{html}");
    assert!(
        html.contains("id=\"email-err\" role=\"alert\">Required"),
        "{html}"
    );
}

#[test]
fn hover_card_trigger_is_focusable_button() {
    // Content is only shown on hover/focus after mount; check the closed trigger.
    let html = render(|| {
        rsx! {
            HoverCard {
                HoverCardTrigger { "Dioxus" }
                HoverCardContent { "Rust framework" }
            }
        }
    });
    assert!(html.contains("role=\"button\""), "{html}");
    assert!(html.contains("tabindex=\"0\""), "{html}");
    assert!(html.contains("data-state=\"closed\""), "{html}");
    assert!(html.contains("Dioxus"), "{html}");
}

#[test]
fn item_group_has_list_role_and_slots() {
    let html = render(|| {
        rsx! {
            ItemGroup {
                Item { variant: ItemVariant::Outline,
                    ItemContent { ItemTitle { "Title" } ItemDescription { "Description" } }
                }
            }
        }
    });
    assert!(html.contains("role=\"list\""), "{html}");
    assert!(html.contains("data-variant=\"outline\""), "{html}");
    assert!(html.contains("data-slot=\"item-title\">Title"), "{html}");
    assert!(html.contains("Description"), "{html}");
}

#[test]
fn kbd_renders_kbd_elements() {
    let html = render(|| rsx! { KbdGroup { Kbd { "Ctrl" } "+" Kbd { "K" } } });
    assert!(html.contains(">Ctrl</kbd>"), "{html}");
    assert!(html.contains(">K</kbd>"), "{html}");
    assert_eq!(html.matches("<kbd").count(), 2, "{html}");
}

#[test]
fn label_renders_label_with_for() {
    let html = render(|| rsx! { Label { html_for: "name", "Name" } });
    assert!(html.contains("<label"), "{html}");
    assert!(html.contains("for=\"name\""), "{html}");
    assert!(html.contains("Name</label>"), "{html}");
}

#[test]
fn menubar_exposes_menubar_and_menuitem_roles() {
    let html = render(|| {
        rsx! {
            Menubar {
                MenubarMenu { index: 0usize,
                    MenubarTrigger { "File" }
                    MenubarContent {
                        MenubarItem { index: 0usize, value: "new".to_string(), "New" }
                    }
                }
            }
        }
    });
    assert!(html.contains("role=\"menubar\""), "{html}");
    assert!(
        html.contains("role=\"menu\" data-state=\"closed\""),
        "{html}"
    );
    assert!(html.contains("role=\"menuitem\""), "{html}");
    assert!(html.contains("File"), "{html}");
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    NavbarHome {},
}

#[component]
fn NavbarHome() -> Element {
    rsx! {
        Navbar { aria_label: "Main",
            NavbarItem { index: 0usize, value: "home".to_string(), to: "/", "Home" }
            NavbarNav { index: 1usize,
                NavbarTrigger { "More" }
                NavbarContent {
                    NavbarItem { index: 0usize, value: "docs".to_string(), to: "/", "Docs" }
                }
            }
        }
    }
}

#[test]
fn navbar_exposes_navigation_and_current_link() {
    // NavbarItem renders a router Link, so the navbar is mounted inside a minimal Router.
    let html = render(|| rsx! { Router::<Route> {} });
    assert!(
        html.contains("role=\"navigation\" aria-label=\"Main\""),
        "{html}"
    );
    assert!(html.contains("role=\"menubar\""), "{html}");
    assert!(html.contains("href=\"/\""), "{html}");
    assert!(html.contains("aria-current=\"page\""), "{html}");
    assert!(html.contains("Home") && html.contains("More"), "{html}");
}

#[test]
fn popover_trigger_renders_closed() {
    // Content is only rendered once open (after mount); check the closed root/trigger.
    let html = render(|| {
        rsx! {
            PopoverRoot {
                PopoverTrigger { "Open popover" }
                PopoverContent { "Dimensions" }
            }
        }
    });
    assert!(html.contains("data-state=\"closed\""), "{html}");
    assert!(html.contains("<button"), "{html}");
    assert!(html.contains("Open popover"), "{html}");
    assert!(!html.contains("Dimensions"), "{html}");
}

#[test]
fn progress_exposes_progressbar_values() {
    let html = render(|| rsx! { Progress { aria_label: "Upload", value: 40.0 } });
    assert!(html.contains("role=\"progressbar\""), "{html}");
    assert!(html.contains("aria-valuenow=40"), "{html}");
    assert!(html.contains("aria-valuemax=100"), "{html}");
    assert!(html.contains("aria-label=\"Upload\""), "{html}");
}

#[test]
fn radio_group_exposes_radiogroup_and_radio_roles() {
    let html = render(|| {
        rsx! {
            RadioGroup { aria_label: "Colour",
                RadioItem { value: "blue".to_string(), index: 0usize, "Blue" }
                RadioItem { value: "red".to_string(), index: 1usize, disabled: true, "Red" }
            }
        }
    });
    assert!(html.contains("role=\"radiogroup\""), "{html}");
    assert_eq!(html.matches("role=\"radio\"").count(), 2, "{html}");
    assert!(html.contains("aria-checked=false"), "{html}");
    assert!(html.contains("data-disabled=true"), "{html}");
    assert!(html.contains("Blue"), "{html}");
}

#[test]
fn scroll_area_sets_scroll_direction() {
    let html = render(|| rsx! { ScrollArea { p { "Scrollable content" } } });
    assert!(html.contains("data-scroll-direction=\"both\""), "{html}");
    assert!(html.contains("overflow-y:auto"), "{html}");
    assert!(html.contains("Scrollable content"), "{html}");
}

#[test]
fn select_renders_closed_listbox_trigger() {
    let html = render(|| {
        rsx! {
            Select::<String> {
                SelectGroup {
                    SelectGroupLabel { "Fruits" }
                    SelectOption::<String> { index: 0usize, value: "apple".to_string(), text_value: "Apple", "Apple" }
                }
            }
        }
    });
    // Options live in the popup, which only mounts when open.
    assert!(html.contains("data-state=\"closed\""), "{html}");
    assert!(html.contains("aria-haspopup=\"listbox\""), "{html}");
    assert!(html.contains("aria-expanded=false"), "{html}");
    assert!(html.contains("Select an option"), "{html}");
}

#[test]
fn separator_exposes_separator_role_and_orientation() {
    let html = render(|| rsx! { Separator { horizontal: false, decorative: false } });
    assert!(html.contains("role=\"separator\""), "{html}");
    assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
}

#[test]
fn sheet_renders_nothing_during_ssr() {
    let html = render(|| {
        rsx! {
            Sheet { open: true,
                SheetHeader { SheetTitle { "Edit profile" } }
            }
        }
    });
    // The dialog mounts client-side only, so even an open sheet leaves no markup.
    assert!(!html.contains("Edit profile"), "{html}");
    assert!(!html.contains("role=\"dialog\""), "{html}");
}

#[test]
fn sidebar_renders_expanded_state_and_trigger() {
    let html = render(|| {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarContent {
                        SidebarGroup {
                            SidebarGroupLabel { "Platform" }
                            SidebarMenu { SidebarMenuItem { SidebarMenuButton { "Home" } } }
                        }
                    }
                }
                SidebarInset { SidebarTrigger {} }
            }
        }
    });
    assert!(html.contains("data-state=\"expanded\""), "{html}");
    assert!(html.contains("data-side=\"left\""), "{html}");
    assert!(html.contains("<main"), "{html}");
    assert!(html.contains("Platform") && html.contains("Home"), "{html}");
    assert!(html.contains("Toggle Sidebar"), "{html}");
}

#[test]
fn skeleton_renders_styled_placeholder() {
    let html = render(|| rsx! { Skeleton { style: "width: 3rem;" } });
    assert!(html.contains("dx-skeleton"), "{html}");
    assert!(html.contains("style=\"width: 3rem;\""), "{html}");
}

#[test]
fn slider_exposes_slider_role_and_value() {
    let html = render(|| {
        rsx! {
            Slider { label: "Volume", horizontal: true, min: 0.0, max: 100.0, step: 1.0, default_value: 40.0 }
        }
    });
    assert!(html.contains("role=\"slider\""), "{html}");
    assert!(html.contains("aria-valuenow=40"), "{html}");
    assert!(html.contains("aria-valuemax=100"), "{html}");
    assert!(html.contains("aria-label=\"Volume\""), "{html}");
}

#[test]
fn spinner_exposes_status_role_and_label() {
    let html = render(|| rsx! { Spinner { label: "Loading data" } });
    assert!(html.contains("role=\"status\""), "{html}");
    assert!(html.contains("aria-label=\"Loading data\""), "{html}");
}

#[test]
fn table_renders_semantic_markup_and_aria_sort() {
    let html = render(|| {
        rsx! {
            Table {
                TableCaption { "Invoices" }
                TableHeader {
                    TableRow {
                        TableHead { sort: SortDirection::Ascending, on_sort: move |_| {}, "Invoice" }
                    }
                }
                TableBody { TableRow { TableCell { "INV-001" } } }
            }
        }
    });
    assert!(html.contains("<caption"), "{html}");
    assert!(html.contains("scope=\"col\""), "{html}");
    assert!(html.contains("aria-sort=\"ascending\""), "{html}");
    assert!(html.contains("INV-001"), "{html}");
}

#[test]
fn tag_group_exposes_grid_row_and_gridcell_roles() {
    let html = render(|| {
        rsx! {
            tg::TagGroup {
                tg::TagGroupLabel { "Labels" }
                tg::TagList {
                    tg::Tag { index: 0usize, value: "bug", "bug" tg::RemoveButton {} }
                }
            }
        }
    });
    assert!(html.contains("role=\"grid\""), "{html}");
    assert!(html.contains("role=\"row\""), "{html}");
    assert!(html.contains("role=\"gridcell\""), "{html}");
    assert!(html.contains("aria-selected=false"), "{html}");
    assert!(html.contains("Labels") && html.contains("bug"), "{html}");
}

#[test]
fn toast_provider_renders_empty_region() {
    // Toasts only appear after a client-side `use_toast()` call; assert the provider.
    let html = render(|| rsx! { ToastProvider { "App body" } });
    assert!(html.contains("App body"), "{html}");
    assert!(html.contains("role=\"region\""), "{html}");
    assert!(html.contains("aria-label=\"0 notifications\""), "{html}");
}

#[test]
fn toggle_exposes_pressed_state() {
    let html = render(|| rsx! { Toggle { aria_label: "Bold", default_pressed: true, "B" } });
    assert!(html.contains("aria-pressed=true"), "{html}");
    assert!(html.contains("data-state=\"on\""), "{html}");
    assert!(html.contains("aria-label=\"Bold\""), "{html}");
}

#[test]
fn toggle_group_renders_unpressed_items() {
    let html = render(|| {
        rsx! {
            ToggleGroup { horizontal: true,
                ToggleItem { index: 0usize, "B" }
                ToggleItem { index: 1usize, "I" }
            }
        }
    });
    assert!(html.contains("data-orientation=\"horizontal\""), "{html}");
    assert_eq!(html.matches("aria-pressed=false").count(), 2, "{html}");
    assert!(html.contains(">B<") && html.contains(">I<"), "{html}");
}

#[test]
fn toolbar_exposes_toolbar_role_and_separator() {
    let html = render(|| {
        rsx! {
            Toolbar { aria_label: "Text formatting",
                ToolbarGroup { ToolbarButton { index: 0usize, on_click: move |_| {}, "Bold" } }
                ToolbarSeparator {}
            }
        }
    });
    assert!(html.contains("role=\"toolbar\""), "{html}");
    assert!(html.contains("aria-label=\"Text formatting\""), "{html}");
    assert!(html.contains("role=\"separator\""), "{html}");
    assert!(html.contains("Bold"), "{html}");
}

#[test]
fn virtual_list_exposes_list_role_and_total_height() {
    let html = render(|| {
        rsx! {
            VirtualList {
                count: 100usize,
                buffer: 2usize,
                estimate_size: |_| 20u32,
                render_item: move |i: usize| rsx! { p { "Row {i}" } },
            }
        }
    });
    // Rows are measured after mount, so SSR only emits the sized scroll container.
    assert!(html.contains("role=\"list\""), "{html}");
    assert!(html.contains("height:2000px"), "{html}");
}

#[test]
fn icons_render_as_svg() {
    use ui::icons::Check;
    let html = render(|| rsx! { Check { size: 16, "aria-hidden": "true" } });
    assert!(
        html.starts_with("<svg") && html.contains("aria-hidden=\"true\""),
        "{html}"
    );
}

#[test]
fn chips_are_a_named_group_of_checkboxes() {
    let html = render(|| {
        rsx! {
            Chips { label: "Status",
                Chip { default_checked: true, "Open" }
                Chip { "Closed" }
            }
        }
    });
    assert!(
        html.contains("role=\"group\"") && html.contains("aria-label=\"Status\""),
        "{html}"
    );
    assert_eq!(html.matches("type=\"checkbox\"").count(), 2, "{html}");
    assert_eq!(html.matches("data-state=\"on\"").count(), 1, "{html}");
}

#[test]
fn controlled_chip_follows_checked() {
    let html = render(|| rsx! { Chip { checked: true, default_checked: false, "On" } });
    assert!(html.contains("data-state=\"on\""), "{html}");
}

#[test]
fn link_is_a_plain_anchor() {
    let html = render(|| rsx! { TextLink { href: "/docs", "Docs" } });
    assert!(
        html.starts_with("<a") && html.contains("href=\"/docs\""),
        "{html}"
    );
    assert!(!html.contains("target="), "{html}");
}

#[test]
fn link_external_opens_safely_and_says_so() {
    let html =
        render(|| rsx! { TextLink { href: "https://x.test", external: true, current: true, "X" } });
    assert!(html.contains("target=\"_blank\""), "{html}");
    assert!(html.contains("rel=\"noopener noreferrer\""), "{html}");
    assert!(html.contains("opens in a new tab"), "{html}");
    assert!(html.contains("aria-current=\"page\""), "{html}");
}

#[test]
fn hooks_media_query_is_false_off_the_web() {
    fn app() -> Element {
        let wide = use_media_query("(min-width: 1px)");
        let (_, copied) = use_copy_clipboard();
        rsx! { p { "{wide} {copied}" } }
    }
    assert_eq!(render(app), "<p>false false</p>");
}

#[test]
fn modal_portal_renders_its_children_in_place_off_native() {
    let html = render(|| rsx! { div { id: "host", ModalPortal { "hello" } } });
    assert!(html.contains(r#"<div id="host">hello</div>"#), "{html}");
}
