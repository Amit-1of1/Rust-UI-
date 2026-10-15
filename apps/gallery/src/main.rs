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

/// The blocks (`ui::blocks`): `(name, title, blurb)`. Each opens on its own page at
/// `#block/<name>`, because a block is a whole screen with its own `h1` and landmarks.
const BLOCKS: &[(&str, &str, &str)] = &[
    (
        "login_01",
        "Login",
        "Sign-in card with remember me, forgot password and a server error slot.",
    ),
    (
        "signup_01",
        "Sign-up",
        "Two steps: details with a password check, then a 6-digit email code.",
    ),
    (
        "dashboard_01",
        "Dashboard",
        "Sidebar shell, key figures, two charts and recent orders.",
    ),
    (
        "settings_01",
        "Settings",
        "Profile, notifications, appearance and a danger zone, in tabs.",
    ),
];

/// The block named in the URL (`#block/login_01`), if any.
fn block_in_url() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let hash = web_sys::window()?.location().hash().ok()?;
        let name = hash.strip_prefix("#block/")?;
        BLOCKS
            .iter()
            .any(|(n, ..)| *n == name)
            .then(|| name.to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    None
}

#[component]
fn App() -> Element {
    let mut block = use_signal(block_in_url);
    if let Some(name) = block() {
        return rsx! {
            ThemeStyles {}
            document::Stylesheet { href: GALLERY_CSS }
            nav { class: "gallery-back", aria_label: "Gallery",
                a { href: "#blocks", onclick: move |_| block.set(None), "Back to the gallery" }
            }
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "gallery-loading", Spinner { label: "Loading the block" } }
                },
                BlockPage { name }
            }
        };
    }
    rsx! {
        ThemeStyles {}
        document::Stylesheet { href: GALLERY_CSS }
        Fragment {
            header { class: "gallery-header",
                h1 { "Rust UI kit" }
                Badge { variant: BadgeVariant::Secondary, "week 6" }
            }
            section { class: "gallery-main", "aria-label": "Component gallery",
                Section { title: "Button", blurb: "Variants and sizes.", ui::demos::button::Demo {} }
                Section { title: "Badge", blurb: "Status labels.", ui::demos::badge::Demo {} }
                Section { title: "Input and label", blurb: "Text entry with an accessible label.", ui::demos::input::Demo {} }
                Section { title: "Checkbox", blurb: "Binary choice; Space toggles.", ui::demos::checkbox::Demo {} }
                Section { title: "Switch", blurb: "An on/off setting.", ui::demos::switch::Demo {} }
                Section { title: "Skip link", blurb: "A hidden link to the main content, shown on keyboard focus.", ui::demos::skip_link::Demo {} }
                Section { title: "Number field", blurb: "A number with - and + buttons and arrow keys.", ui::demos::number_field::Demo {} }
                Section { title: "Card", blurb: "A grouped surface with header, content and footer.", ui::demos::card::Demo {} }
                Section {
                    id: "blocks",
                    title: "Blocks",
                    blurb: "Whole screens built from the kit. Copy one with `rust-ui add login_01`; each opens on its own page.",
                    BlockLinks { open: move |name| block.set(Some(name)) }
                }
                // With `wasm-split` (release builds, scripts/release-web.sh) everything below
                // the first screen is one lazy wasm file, fetched as soon as the page is up.
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        div { class: "gallery-loading", Spinner { label: "Loading the rest of the gallery" } }
                    },
                    MoreSections {}
                }
            }
        }
    }
}

/// Every section below the first screen. One lazy unit rather than one per demo: demos share
/// most of their code (popovers, menus, the calendar), and the splitter copies code shared by
/// several lazy parts into each of them, so 68 small parts came to 2.4x the whole app.
#[component(lazy)]
fn MoreSections() -> Element {
    rsx! {
        Section { title: "Tabs", blurb: "Arrow keys move between tabs.", ui::demos::tabs::Demo {} }
        Section { title: "Accordion", blurb: "Enter or Space expands; one open at a time.", ui::demos::accordion::Demo {} }
        Section { title: "Dialog", blurb: "Modal with focus trap; Escape closes.", ui::demos::dialog::Demo {} }
        Section { title: "Sheet", blurb: "Side panel built on the dialog primitive.", ui::demos::sheet::Demo {} }
        Section { title: "Popover", blurb: "Anchored floating content.", ui::demos::popover::Demo {} }
        Section { title: "Dropdown menu", blurb: "Arrow keys navigate, Enter selects.", ui::demos::dropdown_menu::Demo {} }
        Section { title: "Tooltip", blurb: "Shown on hover and keyboard focus.", ui::demos::tooltip::Demo {} }
        Section { title: "Toast", blurb: "Transient notifications.", ui::demos::toast::Demo {} }
        Section { title: "Separator", blurb: "Visual divider.", ui::demos::separator::Demo {} }
        Section { title: "Alert dialog", blurb: "A dialog that demands an explicit answer.", ui::demos::alert_dialog::Demo {} }
        Section { title: "Aspect ratio", blurb: "Keeps content at a fixed ratio.", ui::demos::aspect_ratio::Demo {} }
        Section { title: "Avatar", blurb: "Image with a fallback.", ui::demos::avatar::Demo {} }
        Section { title: "Calendar", blurb: "Date grid; arrow keys move between days.", ui::demos::calendar::Demo {} }
        Section { title: "Collapsible", blurb: "A single show/hide region.", ui::demos::collapsible::Demo {} }
        Section { title: "Color picker", blurb: "Pick a color by hue, saturation and value.", ui::demos::color_picker::Demo {} }
        Section { title: "Combobox", blurb: "Type to filter, arrow keys to choose.", ui::demos::combobox::Demo {} }
        Section { title: "Context menu", blurb: "Right-click for actions.", ui::demos::context_menu::Demo {} }
        Section { title: "Date picker", blurb: "Calendar in a popover.", ui::demos::date_picker::Demo {} }
        Section { title: "Time picker", blurb: "Hour and minute fields; type digits or use the arrow keys.", ui::demos::time_picker::Demo {} }
        Section { title: "Date and time picker", blurb: "A date picker and a time picker as one value.", ui::demos::date_time_picker::Demo {} }
        Section { title: "Drag and drop list", blurb: "Reorder with the pointer or keyboard.", ui::demos::drag_and_drop_list::Demo {} }
        Section { title: "Hover card", blurb: "Rich preview on hover or focus.", ui::demos::hover_card::Demo {} }
        Section { title: "Item", blurb: "A flexible row for lists.", ui::demos::item::Demo {} }
        Section { title: "Menubar", blurb: "Application-style menus.", ui::demos::menubar::Demo {} }
        Section { title: "Pagination", blurb: "Page navigation.", ui::demos::pagination::Demo {} }
        Section { title: "Progress", blurb: "Completion indicator.", ui::demos::progress::Demo {} }
        Section { title: "Radio group", blurb: "One choice from several; arrow keys move.", ui::demos::radio_group::Demo {} }
        Section { title: "Scroll area", blurb: "Styled scrolling region.", ui::demos::scroll_area::Demo {} }
        Section { title: "Select", blurb: "Pick one option from a list.", ui::demos::select::Demo {} }
        Section { title: "Skeleton", blurb: "Loading placeholder.", ui::demos::skeleton::Demo {} }
        Section { title: "Slider", blurb: "Drag or use arrow keys.", ui::demos::slider::Demo {} }
        Section { title: "Tag group", blurb: "Selectable, removable tags.", ui::demos::tag_group::Demo {} }
        Section { title: "Textarea", blurb: "Multi-line text entry.", ui::demos::textarea::Demo {} }
        Section { title: "Toggle", blurb: "A pressable on/off button.", ui::demos::toggle::Demo {} }
        Section { title: "Toggle group", blurb: "Related toggles with shared state.", ui::demos::toggle_group::Demo {} }
        Section { title: "Toolbar", blurb: "Grouped controls with roving focus.", ui::demos::toolbar::Demo {} }
        Section { title: "Virtual list", blurb: "Renders only the visible rows.", ui::demos::virtual_list::Demo {} }
        Section { title: "Sidebar", blurb: "Collapsible navigation, shown in a bounded frame.", div { class: "gallery-frame", ui::demos::sidebar::Demo {} } }
        Section { title: "Alert", blurb: "A callout; announced when it appears.", ui::demos::alert::Demo {} }
        Section { title: "Breadcrumb", blurb: "Where you are in the hierarchy.", ui::demos::breadcrumb::Demo {} }
        Section { title: "Kbd", blurb: "Keyboard keys.", ui::demos::kbd::Demo {} }
        Section { title: "Label", blurb: "Click the label to focus its input.", ui::demos::label::Demo {} }
        Section { title: "Spinner", blurb: "Indeterminate loading.", ui::demos::spinner::Demo {} }
        Section { title: "Field and validation", blurb: "Label, control, hint and error wired together with aria-describedby.", ui::demos::field::Demo {} }
        Section { title: "Table", blurb: "Sortable columns announce their direction.", ui::demos::table::Demo {} }
        Section { title: "Command palette", blurb: "Type to filter, arrow keys to move, Enter to run.", ui::demos::command::Demo {} }
        Section { title: "Charts", blurb: "SVG charts with a text summary for screen readers.", ui::demos::chart::Demo {} }
        Section { title: "Theme customizer", blurb: "Accent colour, corner radius and mode for the whole page. Everything below recolors live.", ui::demos::theme_customizer::Demo {} }
        Section { title: "Callout", blurb: "Inline notes in default, info, success and warning styles.", ui::demos::callout::Demo {} }
        Section { title: "Empty", blurb: "Empty state with icon, copy and actions.", ui::demos::empty::Demo {} }
        Section { title: "Status", blurb: "A dot plus text label; never colour alone.", ui::demos::status::Demo {} }
        Section { title: "Shimmer", blurb: "Loading highlight that becomes a static tint with reduced motion.", ui::demos::shimmer::Demo {} }
        Section { title: "Marquee", blurb: "Looping strip with a pause button; stops under reduced motion.", ui::demos::marquee::Demo {} }
        Section { title: "Button group", blurb: "Buttons joined into one segmented control.", ui::demos::button_group::Demo {} }
        Section { title: "Stepper", blurb: "Multi-step progress with Back and Next.", ui::demos::stepper::Demo {} }
        Section { title: "Bottom nav", blurb: "Mobile tab bar with aria-current.", ui::demos::bottom_nav::Demo {} }
        Section { title: "Form", blurb: "Validation state, a Form and bound TextFields; focus jumps to the first error on submit.", ui::demos::form::Demo {} }
        Section { title: "Data table", blurb: "Sort, filter, paginate and select rows; status is announced to screen readers.", ui::demos::data_table::Demo {} }
        Section { title: "Input otp", blurb: "One box per character: auto-advance, Backspace, arrow keys and paste.", ui::demos::input_otp::Demo {} }
        Section { title: "Input group", blurb: "An input with text or buttons attached inside one border.", ui::demos::input_group::Demo {} }
        Section { title: "Carousel", blurb: "Snap-scrolling slides with Previous and Next; nothing autoplays.", ui::demos::carousel::Demo {} }
        Section { title: "Dropzone", blurb: "Drop files or browse; type and size are checked, rejections are announced.", ui::demos::dropzone::Demo {} }
        Section { title: "Multi select", blurb: "Pick several options; native checkboxes with a filter box; Escape returns focus.", ui::demos::multi_select::Demo {} }
        Section { title: "Drawer", blurb: "Slides up from the bottom; drag the handle down to dismiss, or use Escape.", ui::demos::drawer::Demo {} }
        Section { title: "Translatable strings", blurb: "Words the kit shows on its own come from UiStringsProvider; switch the language.", ui::demos::ui_strings::Demo {} }
        Section { title: "Icons", blurb: "Lucide icons through ui::icons. Decorative icons are hidden from screen readers; icon-only buttons carry the name.", ui::demos::icons::Demo {} }
        Section { title: "Hooks", blurb: "Shared hooks: media queries, copy to clipboard, Ctrl+K and row selection. They install with the components that use them.", ui::demos::hooks::Demo {} }
        Section { title: "Chips", blurb: "On/off chips for filters: native checkboxes with a check mark, so state never relies on colour.", ui::demos::chips::Demo {} }
        Section { title: "Link", blurb: "Inline text links. External links open in a new tab and say so; the current page is marked with aria-current.", ui::demos::link::Demo {} }
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

/// Links to each block's own page.
#[component]
fn BlockLinks(open: Callback<String>) -> Element {
    rsx! {
        ul { class: "gallery-blocks",
            for (name , title , blurb) in BLOCKS.iter().copied() {
                li { key: "{name}",
                    a {
                        href: "#block/{name}",
                        onclick: move |_| open.call(name.to_string()),
                        "{title}"
                    }
                    p { "{blurb}" }
                }
            }
        }
    }
}

/// One block on the whole page. Lazy: with `wasm-split` a block's code downloads when opened.
#[component(lazy)]
fn BlockPage(name: String) -> Element {
    match name.as_str() {
        "login_01" => rsx! { ui::blocks::login_01::Login01 {} },
        "signup_01" => rsx! {
            ui::blocks::signup_01::Signup01 { on_verify: move |_: String| {} }
        },
        "dashboard_01" => rsx! { ui::blocks::dashboard_01::Dashboard01 {} },
        "settings_01" => rsx! { ui::blocks::settings_01::Settings01 {} },
        _ => rsx! {},
    }
}

#[component]
fn Section(
    title: &'static str,
    blurb: &'static str,
    #[props(default)] id: Option<&'static str>,
    children: Element,
) -> Element {
    rsx! {
        section { class: "gallery-section", id,
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
