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
                Section { title: "Button", blurb: "Variants and sizes.", ui::components::button::button_demo::Demo {} }
                Section { title: "Badge", blurb: "Status labels.", ui::components::badge::badge_demo::Demo {} }
                Section { title: "Input and label", blurb: "Text entry with an accessible label.", ui::components::input::input_demo::Demo {} }
                Section { title: "Checkbox", blurb: "Binary choice; Space toggles.", ui::components::checkbox::checkbox_demo::Demo {} }
                Section { title: "Switch", blurb: "An on/off setting.", ui::components::switch::switch_demo::Demo {} }
                Section { title: "Card", blurb: "A grouped surface with header, content and footer.", ui::components::card::card_demo::Demo {} }
                Section { title: "Tabs", blurb: "Arrow keys move between tabs.", ui::components::tabs::tabs_demo::Demo {} }
                Section { title: "Accordion", blurb: "Enter or Space expands; one open at a time.", ui::components::accordion::accordion_demo::Demo {} }
                Section { title: "Dialog", blurb: "Modal with focus trap; Escape closes.", ui::components::dialog::dialog_demo::Demo {} }
                Section { title: "Sheet", blurb: "Side panel built on the dialog primitive.", ui::components::sheet::sheet_demo::Demo {} }
                Section { title: "Popover", blurb: "Anchored floating content.", ui::components::popover::popover_demo::Demo {} }
                Section { title: "Dropdown menu", blurb: "Arrow keys navigate, Enter selects.", ui::components::dropdown_menu::dropdown_menu_demo::Demo {} }
                Section { title: "Tooltip", blurb: "Shown on hover and keyboard focus.", ui::components::tooltip::tooltip_demo::Demo {} }
                Section { title: "Toast", blurb: "Transient notifications.", ui::components::toast::toast_demo::Demo {} }
                Section { title: "Separator", blurb: "Visual divider.", ui::components::separator::separator_demo::Demo {} }
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
                Section { title: "Data table", blurb: "Sort, filter, paginate and select rows; status is announced to screen readers.", ui::components::data_table::data_table_demo::Demo {} }
                Section { title: "Input otp", blurb: "One box per character: auto-advance, Backspace, arrow keys and paste.", ui::components::input_otp::input_otp_demo::Demo {} }
                Section { title: "Input group", blurb: "An input with text or buttons attached inside one border.", ui::components::input_group::input_group_demo::Demo {} }
                Section { title: "Carousel", blurb: "Snap-scrolling slides with Previous and Next; nothing autoplays.", ui::components::carousel::carousel_demo::Demo {} }
                Section { title: "Dropzone", blurb: "Drop files or browse; type and size are checked, rejections are announced.", ui::components::dropzone::dropzone_demo::Demo {} }
                Section { title: "Multi select", blurb: "Pick several options; native checkboxes with a filter box; Escape returns focus.", ui::components::multi_select::multi_select_demo::Demo {} }
                Section { title: "Drawer", blurb: "Slides up from the bottom; drag the handle down to dismiss, or use Escape.", ui::components::drawer::drawer_demo::Demo {} }
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
