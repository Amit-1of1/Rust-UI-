//! Live gallery of every component in the kit. A component that isn't shown here
//! isn't done (docs/GUARDRAILS.md).

use dioxus::prelude::*;
use ui::prelude::*;

const GALLERY_CSS: Asset = asset!("/assets/gallery.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ThemeStyles {}
        document::Stylesheet { href: GALLERY_CSS }
        ToastProvider {
            header { class: "gallery-header",
                h1 { "Rust UI kit" }
                Badge { variant: BadgeVariant::Secondary, "week 1" }
            }
            main { class: "gallery-main",
                Section { title: "Button", blurb: "Variants and sizes.", ButtonDemo {} }
                Section { title: "Badge", blurb: "Status labels.", BadgeDemo {} }
                Section { title: "Input and label", blurb: "Text entry with an accessible label.", InputDemo {} }
                Section { title: "Checkbox and switch", blurb: "Binary choices; Space toggles.", ToggleDemo {} }
                Section { title: "Card", blurb: "A grouped surface with header, content and footer.", CardDemo {} }
                Section { title: "Tabs", blurb: "Arrow keys move between tabs.", TabsDemo {} }
                Section { title: "Accordion", blurb: "Enter or Space expands; one open at a time.", AccordionDemo {} }
                Section { title: "Dialog", blurb: "Modal with focus trap; Escape closes.", DialogDemo {} }
                Section { title: "Sheet", blurb: "Side panel built on the dialog primitive.", SheetDemo {} }
                Section { title: "Popover", blurb: "Anchored floating content.", PopoverDemo {} }
                Section { title: "Dropdown menu", blurb: "Arrow keys navigate, Enter selects.", DropdownDemo {} }
                Section { title: "Tooltip", blurb: "Shown on hover and keyboard focus.", TooltipDemo {} }
                Section { title: "Toast", blurb: "Transient notifications.", ToastDemo {} }
                Section { title: "Separator", blurb: "Visual divider.", SeparatorDemo {} }
            }
        }
    }
}

#[component]
fn Section(title: &'static str, blurb: &'static str, children: Element) -> Element {
    rsx! {
        section { class: "gallery-section",
            h2 { {title} }
            p { {blurb} }
            {children}
        }
    }
}

#[component]
fn ButtonDemo() -> Element {
    rsx! {
        div { class: "gallery-row",
            Button { "Primary" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
            Button { variant: ButtonVariant::Destructive, "Delete" }
            Button { variant: ButtonVariant::Link, "Link" }
        }
        div { class: "gallery-row", style: "margin-top: 0.75rem;",
            Button { size: ButtonSize::Xs, "Extra small" }
            Button { size: ButtonSize::Sm, "Small" }
            Button { "Default" }
            Button { size: ButtonSize::Lg, "Large" }
            Button { disabled: true, "Disabled" }
        }
    }
}

#[component]
fn BadgeDemo() -> Element {
    rsx! {
        div { class: "gallery-row",
            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge { variant: BadgeVariant::Secondary, VerifiedIcon {} "Verified" }
        }
    }
}

#[component]
fn InputDemo() -> Element {
    let mut name = use_signal(String::new);
    rsx! {
        div { class: "gallery-stack",
            Label { html_for: "gallery-name", "Name" }
            Input {
                id: "gallery-name",
                placeholder: "Enter your name",
                value: name,
                oninput: move |e: FormEvent| name.set(e.value()),
            }
            if !name.read().is_empty() {
                p { style: "margin: 0; font-size: 0.875rem;", "Hello, {name}!" }
            }
        }
    }
}

#[component]
fn ToggleDemo() -> Element {
    let mut on = use_signal(|| false);
    rsx! {
        div { class: "gallery-row",
            Checkbox { id: "terms", name: "terms" }
            Label { html_for: "terms", "Accept terms" }
            Switch {
                checked: on(),
                aria_label: "Airplane mode",
                on_checked_change: move |v| on.set(v),
            }
            span { style: "font-size: 0.875rem;", if on() { "On" } else { "Off" } }
        }
    }
}

#[component]
fn CardDemo() -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Create project" }
                CardDescription { "Deploy your new project in one click." }
            }
            CardContent {
                div { class: "gallery-stack",
                    Label { html_for: "gallery-project", "Project name" }
                    Input { id: "gallery-project", placeholder: "my-app" }
                }
            }
            CardFooter { style: "justify-content: space-between;",
                Button { variant: ButtonVariant::Outline, "Cancel" }
                Button { "Create" }
            }
        }
    }
}

#[component]
fn TabsDemo() -> Element {
    rsx! {
        Tabs { default_value: "account".to_string(), horizontal: true, max_width: "24rem",
            TabList {
                TabTrigger { value: "account".to_string(), index: 0usize, "Account" }
                TabTrigger { value: "password".to_string(), index: 1usize, "Password" }
            }
            TabContent { index: 0usize, value: "account".to_string(),
                p { "Make changes to your account here." }
            }
            TabContent { index: 1usize, value: "password".to_string(),
                p { "Change your password here." }
            }
        }
    }
}

#[component]
fn AccordionDemo() -> Element {
    let items = [
        (
            "Is it accessible?",
            "Yes. Behaviour comes from dioxus-primitives, which follows WAI-ARIA.",
        ),
        (
            "Is it styled?",
            "Yes. It ships with a neutral, shadcn-like theme you own.",
        ),
        (
            "Is it animated?",
            "Open and close are animated in CSS; springs arrive in week 2.",
        ),
    ];
    rsx! {
        div { style: "max-width: 32rem;",
            Accordion { allow_multiple_open: false, horizontal: false,
                for (i, (q, a)) in items.into_iter().enumerate() {
                    AccordionItem { index: i,
                        AccordionTrigger { {q} }
                        AccordionContent {
                            p { style: "margin: 0; padding-bottom: 1rem;", {a} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogDemo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Open dialog" }
        Dialog { open: open(), on_open_change: move |v| open.set(v),
            DialogTitle { "Are you sure?" }
            DialogDescription { "This action cannot be undone." }
            div { class: "gallery-row", style: "justify-content: flex-end; margin-top: 1rem;",
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Cancel" }
                Button { variant: ButtonVariant::Destructive, onclick: move |_| open.set(false), "Delete" }
            }
        }
    }
}

#[component]
fn SheetDemo() -> Element {
    let mut open = use_signal(|| false);
    let mut side = use_signal(|| SheetSide::Right);
    let mut show = move |s: SheetSide| {
        side.set(s);
        open.set(true);
    };
    rsx! {
        div { class: "gallery-row",
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Left), "Left" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Right), "Right" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Top), "Top" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Bottom), "Bottom" }
        }
        Sheet { open: open(), on_open_change: move |v| open.set(v), "data-side": side().as_str(),
            SheetHeader {
                SheetTitle { "Edit profile" }
                SheetDescription { "Make changes to your profile here." }
            }
            div { class: "gallery-stack", style: "padding: 0 1rem;",
                Label { html_for: "sheet-name", "Name" }
                Input { id: "sheet-name", initial_value: "Ferris" }
            }
            SheetFooter {
                Button { onclick: move |_| open.set(false), "Save changes" }
            }
            SheetContentClose {}
        }
    }
}

#[component]
fn PopoverDemo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
            PopoverTrigger { "Open popover" }
            PopoverContent { gap: "0.5rem",
                strong { "Dimensions" }
                span { style: "font-size: 0.875rem;", "Set the dimensions for the layer." }
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Done" }
            }
        }
    }
}

#[component]
fn DropdownDemo() -> Element {
    let mut picked = use_signal(|| None::<&'static str>);
    let actions = ["Edit", "Duplicate", "Archive", "Delete"];
    rsx! {
        div { class: "gallery-row",
            DropdownMenu { default_open: false,
                DropdownMenuTrigger { "Actions" }
                DropdownMenuContent {
                    for (i, a) in actions.into_iter().enumerate() {
                        DropdownMenuItem::<&'static str> {
                            value: a,
                            index: i,
                            on_select: move |v| picked.set(Some(v)),
                            {a}
                        }
                    }
                }
            }
            if let Some(a) = picked() {
                span { style: "font-size: 0.875rem;", "Selected: {a}" }
            }
        }
    }
}

#[component]
fn TooltipDemo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { "Hover or focus me" }
            TooltipContent { side: ContentSide::Top, "Add to library" }
        }
    }
}

#[component]
fn ToastDemo() -> Element {
    let toasts = use_toast();
    rsx! {
        div { class: "gallery-row",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.success("Saved".to_string(), ToastOptions::new().description("Your changes are stored.")),
                "Success toast"
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.error("Failed".to_string(), ToastOptions::new().description("Something went wrong.")),
                "Error toast"
            }
        }
    }
}

#[component]
fn SeparatorDemo() -> Element {
    rsx! {
        div { style: "max-width: 20rem;",
            strong { "Rust UI kit" }
            p { style: "margin: 0.25rem 0; font-size: 0.875rem;", "A personal component library." }
            Separator { horizontal: true, decorative: true, style: "margin: 1rem 0;" }
            div { class: "gallery-row", style: "font-size: 0.875rem;", "Docs" Separator { horizontal: false, decorative: true, style: "height: 1rem;" } "Source" }
        }
    }
}
