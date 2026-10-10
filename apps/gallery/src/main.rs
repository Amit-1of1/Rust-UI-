//! Live gallery of every component in the kit. A component that isn't shown here
//! isn't done (docs/GUARDRAILS.md).

use dioxus::prelude::*;
use ui::motion::{
    use_reduced_motion, use_spring, AnimatePresence, FlipItem, FlipList, Pop, SpringPreset,
    SwipeDismiss,
};
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
                Badge { variant: BadgeVariant::Secondary, "week 6" }
            }
            section { class: "gallery-main", "aria-label": "Component gallery",
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
                Section { title: "Alert dialog", blurb: "A dialog that demands an explicit answer.", ui::components::alert_dialog::alert_dialog_demo::Demo {} }
                Section { title: "Aspect ratio", blurb: "Keeps content at a fixed ratio.", ui::components::aspect_ratio::aspect_ratio_demo::Demo {} }
                Section { title: "Avatar", blurb: "Image with a fallback.", ui::components::avatar::avatar_demo::Demo {} }
                Section { title: "Calendar", blurb: "Date grid; arrow keys move between days.", ui::components::calendar::calendar_demo::Demo {} }
                Section { title: "Collapsible", blurb: "A single show/hide region.", ui::components::collapsible::collapsible_demo::Demo {} }
                Section { title: "Color picker", blurb: "Pick a color by hue, saturation and value.", ui::components::color_picker::color_picker_demo::Demo {} }
                Section { title: "Combobox", blurb: "Type to filter, arrow keys to choose.", ui::components::combobox::combobox_demo::Demo {} }
                Section { title: "Context menu", blurb: "Right-click for actions.", ui::components::context_menu::context_menu_demo::Demo {} }
                Section { title: "Date picker", blurb: "Calendar in a popover.", ui::components::date_picker::date_picker_demo::Demo {} }
                Section { title: "Drag and drop list", blurb: "Reorder with the pointer or keyboard.", ui::components::drag_and_drop_list::drag_and_drop_list_demo::Demo {} }
                Section { title: "Hover card", blurb: "Rich preview on hover or focus.", ui::components::hover_card::hover_card_demo::Demo {} }
                Section { title: "Item", blurb: "A flexible row for lists.", ui::components::item::item_demo::Demo {} }
                Section { title: "Menubar", blurb: "Application-style menus.", ui::components::menubar::menubar_demo::Demo {} }
                Section { title: "Pagination", blurb: "Page navigation.", ui::components::pagination::pagination_demo::Demo {} }
                Section { title: "Progress", blurb: "Completion indicator.", ui::components::progress::progress_demo::Demo {} }
                Section { title: "Radio group", blurb: "One choice from several; arrow keys move.", ui::components::radio_group::radio_group_demo::Demo {} }
                Section { title: "Scroll area", blurb: "Styled scrolling region.", ui::components::scroll_area::scroll_area_demo::Demo {} }
                Section { title: "Select", blurb: "Pick one option from a list.", ui::components::select::select_demo::Demo {} }
                Section { title: "Skeleton", blurb: "Loading placeholder.", ui::components::skeleton::skeleton_demo::Demo {} }
                Section { title: "Slider", blurb: "Drag or use arrow keys.", ui::components::slider::slider_demo::Demo {} }
                Section { title: "Tag group", blurb: "Selectable, removable tags.", ui::components::tag_group::tag_group_demo::Demo {} }
                Section { title: "Textarea", blurb: "Multi-line text entry.", ui::components::textarea::textarea_demo::Demo {} }
                Section { title: "Toggle", blurb: "A pressable on/off button.", ui::components::toggle::toggle_demo::Demo {} }
                Section { title: "Toggle group", blurb: "Related toggles with shared state.", ui::components::toggle_group::toggle_group_demo::Demo {} }
                Section { title: "Toolbar", blurb: "Grouped controls with roving focus.", ui::components::toolbar::toolbar_demo::Demo {} }
                Section { title: "Virtual list", blurb: "Renders only the visible rows.", ui::components::virtual_list::virtual_list_demo::Demo {} }
                Section { title: "Sidebar", blurb: "Collapsible navigation, shown in a bounded frame.", div { class: "gallery-frame", ui::components::sidebar::sidebar_demo::Demo {} } }
                Section { title: "Alert", blurb: "A callout; announced when it appears.", ui::components::alert::alert_demo::Demo {} }
                Section { title: "Breadcrumb", blurb: "Where you are in the hierarchy.", ui::components::breadcrumb::breadcrumb_demo::Demo {} }
                Section { title: "Kbd", blurb: "Keyboard keys.", ui::components::kbd::kbd_demo::Demo {} }
                Section { title: "Spinner", blurb: "Indeterminate loading.", ui::components::spinner::spinner_demo::Demo {} }
                Section { title: "Field and validation", blurb: "Label, control, hint and error wired together with aria-describedby.", ui::components::field::field_demo::Demo {} }
                Section { title: "Table", blurb: "Sortable columns announce their direction.", ui::components::table::table_demo::Demo {} }
                Section { title: "Command palette", blurb: "Type to filter, arrow keys to move, Enter to run.", ui::components::command::command_demo::Demo {} }
                Section { title: "Charts", blurb: "SVG charts with a text summary for screen readers.", ui::components::chart::chart_demo::Demo {} }
                Section { title: "Theme customizer", blurb: "Accent colour, corner radius and mode for the whole page. Everything below recolors live.", ui::components::theme_customizer::theme_customizer_demo::Demo {} }
                Section { title: "Callout", blurb: "Inline notes in default, info, success and warning styles.", ui::components::callout::callout_demo::Demo {} }
                Section { title: "Empty", blurb: "Empty state with icon, copy and actions.", ui::components::empty::empty_demo::Demo {} }
                Section { title: "Status", blurb: "A dot plus text label; never colour alone.", ui::components::status::status_demo::Demo {} }
                Section { title: "Shimmer", blurb: "Loading highlight that becomes a static tint with reduced motion.", ui::components::shimmer::shimmer_demo::Demo {} }
                Section { title: "Marquee", blurb: "Looping strip with a pause button; stops under reduced motion.", ui::components::marquee::marquee_demo::Demo {} }
                Section { title: "Button group", blurb: "Buttons joined into one segmented control.", ui::components::button_group::button_group_demo::Demo {} }
                Section { title: "Stepper", blurb: "Multi-step progress with Back and Next.", ui::components::stepper::stepper_demo::Demo {} }
                Section { title: "Bottom nav", blurb: "Mobile tab bar with aria-current.", ui::components::bottom_nav::bottom_nav_demo::Demo {} }
                Section { title: "Form", blurb: "Validation state, a Form and bound TextFields; focus jumps to the first error on submit.", ui::components::form::form_demo::Demo {} }
                Section {
                    title: "Spring motion",
                    blurb: "Three presets retargeting mid-flight keep their momentum. Click Go repeatedly.",
                    SpringDemo {}
                }
                Section {
                    title: "Presence",
                    blurb: "Items fade and scale in, finish their exit before they unmount, and the rest slide into the gap.",
                    PresenceDemo {}
                }
                Section {
                    title: "Swipe to dismiss",
                    blurb: "Drag past 40% or flick to dismiss; Delete works from the keyboard.",
                    SwipeDemo {}
                }
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

#[component]
fn SpringDemo() -> Element {
    let mut far = use_signal(|| false);
    let reduced = use_reduced_motion();
    let target: f32 = if far() { 1.0 } else { 0.0 };
    rsx! {
        div { class: "gallery-row", style: "margin-bottom: 1rem;",
            Button { onclick: move |_| far.toggle(), "Go" }
            if reduced {
                Badge { variant: BadgeVariant::Outline, "Reduced motion: springs are instant" }
            }
        }
        SpringTrack { label: "Gentle", preset: SpringPreset::Gentle, target }
        SpringTrack { label: "Snappy", preset: SpringPreset::Snappy, target }
        SpringTrack { label: "Bouncy", preset: SpringPreset::Bouncy, target }
    }
}

/// `target` is 0.0 (left) or 1.0 (right); the ball springs to it.
#[component]
fn SpringTrack(label: &'static str, preset: SpringPreset, target: ReadSignal<f32>) -> Element {
    let mut x = use_spring(0.0, preset);
    use_effect(move || x.to(target()));
    let left = x.get_value() * 100.0;
    rsx! {
        div { class: "gallery-track",
            span { class: "gallery-track-label", {label} }
            div { class: "gallery-track-rail",
                div { class: "gallery-ball", style: "left: calc({left}% - {left * 0.01 * 1.5}rem);" }
            }
        }
    }
}

#[component]
fn PresenceDemo() -> Element {
    let mut items = use_signal(|| vec![1u32, 2]);
    let mut next = use_signal(|| 3u32);
    rsx! {
        div { class: "gallery-row", style: "margin-bottom: 1rem;",
            Button {
                onclick: move |_| {
                    let id = next();
                    next.set(id + 1);
                    items.write().push(id);
                },
                "Add item"
            }
        }
        FlipList { id: "flip-list", style: "max-width: 22rem;",
            AnimatePresence {
                for id in items() {
                    FlipItem { key: "{id}",
                        Pop { class: "gallery-item", preset: SpringPreset::Snappy,
                            span { "Item {id}" }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                onclick: move |_| items.write().retain(|i| *i != id),
                                "Remove"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SwipeDemo() -> Element {
    let mut cards = use_signal(|| vec![1u32, 2, 3]);
    rsx! {
        p { id: "swipe-hint", "Drag a card sideways, or focus it and press Delete." }
        div { id: "swipe-list", style: "max-width: 22rem; display: flex; flex-direction: column; gap: 0.5rem;",
            for id in cards() {
                SwipeDismiss {
                    key: "{id}",
                    class: "gallery-item",
                    "data-card": "{id}",
                    on_dismiss: move |_| cards.write().retain(|c| *c != id),
                    "Card {id}"
                }
            }
            if cards().is_empty() {
                Button { onclick: move |_| cards.set(vec![1, 2, 3]), "Reset cards" }
            }
        }
    }
}
