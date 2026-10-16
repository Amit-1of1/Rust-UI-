use dioxus::prelude::*;
use ui::prelude::*;

const FAQ: [(&str, &str); 3] = [
    (
        "Is there a webview on native?",
        "No. The native build draws with Blitz: Stylo for CSS, Taffy for layout, Vello for paint.",
    ),
    (
        "Do the overlays need JavaScript?",
        "No. Escape, outside clicks and focus traps run in Rust on every renderer.",
    ),
    (
        "Which code is per platform?",
        "Only the renderer label and the launch call. Pages, state and CSS are shared.",
    ),
];

#[component]
pub fn Overlays() -> Element {
    let toasts = use_toast();
    let mut dialog = use_signal(|| false);
    let mut sheet = use_signal(|| false);
    let mut popover = use_signal(|| false);
    let mut picked = use_signal(|| None::<&'static str>);

    rsx! {
        div { class: "sc-grid",
            Card {
                CardHeader {
                    CardTitle { "Modals" }
                    CardDescription { "Centred dialogs and edge sheets over the whole window." }
                }
                CardContent {
                    div { class: "sc-row",
                        Button { id: "open-dialog", variant: ButtonVariant::Outline, onclick: move |_| dialog.set(true), "Open dialog" }
                        Button { id: "open-sheet", variant: ButtonVariant::Outline, onclick: move |_| sheet.set(true), "Open sheet" }
                    }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Popups" }
                    CardDescription { "Anchored to their trigger." }
                }
                CardContent {
                    div { class: "sc-row",
                        PopoverRoot { open: popover(), on_open_change: move |v| popover.set(v),
                            PopoverTrigger { id: "open-popover", "Popover" }
                            PopoverContent { gap: "0.5rem",
                                strong { "Share link" }
                                span { class: "sc-muted", "Anyone with the link can view." }
                                Button { variant: ButtonVariant::Outline, onclick: move |_| popover.set(false), "Done" }
                            }
                        }
                        DropdownMenu { default_open: false,
                            DropdownMenuTrigger { id: "open-menu", "Actions" }
                            DropdownMenuContent {
                                for (i , a) in ["Edit", "Duplicate", "Archive"].into_iter().enumerate() {
                                    DropdownMenuItem::<&'static str> {
                                        key: "{a}",
                                        value: a,
                                        index: i,
                                        on_select: move |v| picked.set(Some(v)),
                                        {a}
                                    }
                                }
                            }
                        }
                        Tooltip {
                            TooltipTrigger { "Hover me" }
                            TooltipContent { side: ContentSide::Top, "Tooltips open on hover and focus" }
                        }
                    }
                    if let Some(a) = picked() {
                        p { id: "picked", class: "sc-muted", "Picked: {a}" }
                    }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Toasts" }
                    CardDescription { "Stacked in the corner, dismissed on their own." }
                }
                CardContent {
                    div { class: "sc-row",
                        Button {
                            id: "toast-success",
                            variant: ButtonVariant::Outline,
                            onclick: move |_| toasts.notify_success(("Published", "Your page is live.")),
                            "Success"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| toasts.notify_error(("Failed", "Try again in a minute.")),
                            "Error"
                        }
                    }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Questions" }
                }
                CardContent {
                    Accordion { allow_multiple_open: false, horizontal: false,
                        for (i , (q , a)) in FAQ.into_iter().enumerate() {
                            AccordionItem { key: "{i}", index: i,
                                AccordionTrigger { {q} }
                                AccordionContent {
                                    p { class: "sc-accordion-text", {a} }
                                }
                            }
                        }
                    }
                }
            }
        }
        Dialog { open: dialog(), on_open_change: move |v| dialog.set(v),
            title: "Delete project?",
            description: "This removes the project and its deployments. It cannot be undone.",
            div { class: "sc-dialog-actions",
                Button { variant: ButtonVariant::Outline, onclick: move |_| dialog.set(false), "Cancel" }
                Button {
                    id: "confirm-delete",
                    variant: ButtonVariant::Destructive,
                    onclick: move |_| {
                        dialog.set(false);
                        toasts.notify_info("Project deleted (demo)");
                    },
                    "Delete"
                }
            }
        }
        Sheet {
            open: sheet(),
            on_open_change: move |v| sheet.set(v),
            title: "Notifications",
            description: "Choose what reaches you.",
            div { class: "sc-sheet-body",
                for (i , label) in ["Mentions", "Deploys", "Billing"].into_iter().enumerate() {
                    div { key: "{label}", class: "sc-row",
                        Switch { default_checked: i < 2, aria_label: label }
                        span { {label} }
                    }
                }
            }
            SheetFooter {
                Button { onclick: move |_| sheet.set(false), "Save" }
            }
            SheetContentClose {}
        }
    }
}
