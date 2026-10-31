# Sidebar

- Copy into your app: `rust-ui add sidebar` or `dx components add sidebar --git https://github.com/Amit-1of1/Rust-UI-` (also copies `button`, `icons`, `separator`, `sheet`, `skeleton`, `tooltip`, `ui_strings`).
- Import: `use crate::components::sidebar::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Sidebar component is a vertical interface panel fixed to the screen edge, displaying navigation menus or filters that enable quick access to different sections of an application.

## Component Structure

```rust
// Provider: supplies open/side/collapsible signals and ⌘/Ctrl+B toggle
SidebarProvider {
    Sidebar {
        side: SidebarSide::Left,                     // left/right placement
        variant: SidebarVariant::Sidebar,            // chrome: Sidebar | Floating | Inset
        collapsible: SidebarCollapsible::Offcanvas,  // behavior: Offcanvas | Icon | None

        // Layout - Header
        SidebarHeader {
            SidebarTrigger {}                        // toggle button (as)
        }

        // Layout - Scrollable content area
        SidebarContent {
            SidebarGroup {
                SidebarGroupLabel { "..." }          // optional label (as)
                SidebarGroupAction { "..." }         // optional action (as)
                SidebarGroupContent {                // wraps menus
                    SidebarMenu {
                        SidebarMenuItem {
                            SidebarMenuButton {      // primary item (as)
                                is_active: true,     // highlight state
                                tooltip: rsx!("..."),// Option<Element>; wraps tooltip only when Some
                                Icon {}              // icon node
                                span { "..." }       // text node
                            }
                            SidebarMenuAction { show_on_hover: true, Icon {} } // trailing action (as)
                            SidebarMenuBadge { "+..." }                        // optional badge
                        }
                        SidebarMenuItem {            // nested submenu
                            SidebarMenuSub {
                                SidebarMenuSubItem {
                                    SidebarMenuSubButton { "..." } // submenu button/link (as)
                                }
                            }
                        }
                    }
                }
            }
        }

        // Layout -  Footer
        SidebarFooter {
            SidebarMenu { /* ... */ }
        }
    }

    // Optional desktop rail controller placed between rail and content
    SidebarRail {}                               // draggable resize handle

    // Layout - Main content area beside the rail
    SidebarInset { /* ... */ }
}
```

## Behaviors
- Layout: `variant` adjusts chrome (`Floating/Inset` adds padding/radius); `side` selects the start (`Left`) or end (`Right`) edge, which swap on right-to-left pages (docs/RTL.md).
- Collapse: `Offcanvas` hides the rail; `Icon` keeps a thin icon strip and hides labels/actions/badges; `None` is static.
- Keyboard: ⌘/Ctrl+B toggles via provider. Focus rings are defined in `sidebar/style.css`; keep or replace with `:focus-visible` styles.
- Tooltips: `tooltip: Option<Element>` on `SidebarMenuButton`; `None` skips wrapping in Tooltip.
- Landmarks: `Sidebar` is a navigation landmark named "Sidebar" (`UiStrings::sidebar`; pass `aria_label` to rename it, or `role` to change it). `SidebarInset` renders `<main>`, so it is the page's `main` landmark: don't add another `main` inside it.

## Custom Rendering with `as`
Supported components: `SidebarTrigger`, `SidebarGroupLabel`, `SidebarGroupAction`, `SidebarMenuButton`, `SidebarMenuAction`, `SidebarMenuSubButton`. Use `as: |attrs| rsx! { ... }` and spread `..attrs` to retain merged attributes, state data, and handlers.

## API

```rust
pub enum SidebarState {
    #[default]
    Expanded,
    Collapsed,
}

pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

pub struct SidebarCtx {
    pub state: Memo<SidebarState>,
    pub side: Signal<SidebarSide>,
    pub is_mobile: Signal<bool>,
    // From use_controlled:
    open: Memo<bool>,
    set_open: Callback<bool>,
    // Mobile state:
    open_mobile: Signal<bool>,
}

pub fn use_sidebar() -> SidebarCtx

pub fn use_is_mobile() -> Signal<bool>

#[component]
pub fn SidebarProvider(
    #[props(default = true)] default_open: bool,
    #[props(default)] open: ReadSignal<Option<bool>>,
    #[props(default)] on_open_change: Callback<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The sidebar panel. It is a navigation landmark named [`UiStrings::sidebar`]
/// (`role` and `aria_label` override it); on small screens it becomes a sheet.
///
/// [`UiStrings::sidebar`]: crate::components::ui_strings::UiStrings
#[component]
pub fn Sidebar(
    #[props(default)] side: SidebarSide,
    #[props(default)] variant: SidebarVariant,
    #[props(default)] collapsible: SidebarCollapsible,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarTrigger(
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
) -> Element

#[component]
pub fn SidebarRail(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let ctx = use_sidebar();

/// The page's main content beside the sidebar. It renders `<main>`, so it is the page's
/// `main` landmark: don't put another `main` inside it.
#[component]
pub fn SidebarInset(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarSeparator(
    #[props(default = true)] horizontal: bool,
    #[props(default = true)] decorative: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element

#[component]
pub fn SidebarGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarGroupLabel(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarGroupAction(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarGroupContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarMenu(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarMenuItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

pub enum SidebarMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

#[component]
pub fn SidebarMenuButton(
    #[props(default = false)] is_active: bool,
    #[props(default)] variant: SidebarMenuButtonVariant,
    #[props(default)] size: SidebarMenuButtonSize,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] tooltip: Option<Element>,
    /// Click handler for the default `<button>` (ignored when `as` renders something else).
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element

#[component]
pub fn SidebarMenuAction(
    #[props(default = false)] show_on_hover: bool,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarMenuBadge(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarMenuSkeleton(
    #[props(default = false)] show_icon: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element

#[component]
pub fn SidebarMenuSub(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SidebarMenuSubItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

pub enum SidebarMenuSubButtonSize {
    Sm,
    #[default]
    Md,
}

#[component]
pub fn SidebarMenuSubButton(
    #[props(default = false)] is_active: bool,
    #[props(default)] size: SidebarMenuSubButtonSize,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::avatar::{AvatarImageSize, ImageAvatar};
use crate::components::button::{Button, ButtonVariant};
use crate::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::icons::{ChevronRight, Circle};
use crate::components::separator::Separator;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuAction, SidebarMenuBadge,
    SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSide,
    SidebarTrigger, SidebarVariant,
};
use crate::components::skeleton::Skeleton;
use dioxus::prelude::*;

#[css_module("/src/components/sidebar/demo.css")]
struct DemoStyles;

#[css_module("/src/components/sidebar/style.css")]
struct Styles;

#[derive(Clone, PartialEq)]
struct Team {
    name: &'static str,
    plan: &'static str,
}

#[derive(Clone, PartialEq)]
struct NavMainItem {
    title: &'static str,
    url: &'static str,
    is_active: bool,
    items: &'static [SubItem],
}

#[derive(Clone, PartialEq)]
struct SubItem {
    title: &'static str,
    url: &'static str,
}

#[derive(Clone, PartialEq)]
struct Project {
    name: &'static str,
    url: &'static str,
}

const TEAMS: &[Team] = &[
    Team {
        name: "Acme Inc",
        plan: "Enterprise",
    },
    Team {
        name: "Acme Corp.",
        plan: "Startup",
    },
    Team {
        name: "Evil Corp.",
        plan: "Free",
    },
];

const NAV_MAIN: &[NavMainItem] = &[
    NavMainItem {
        title: "Playground",
        url: "#",
        is_active: true,
        items: &[
            SubItem {
                title: "History",
                url: "#",
            },
            SubItem {
                title: "Starred",
                url: "#",
            },
            SubItem {
                title: "Settings",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Models",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "Genesis",
                url: "#",
            },
            SubItem {
                title: "Explorer",
                url: "#",
            },
            SubItem {
                title: "Quantum",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Documentation",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "Introduction",
                url: "#",
            },
            SubItem {
                title: "Get Started",
                url: "#",
            },
            SubItem {
                title: "Tutorials",
                url: "#",
            },
            SubItem {
                title: "Changelog",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Settings",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "General",
                url: "#",
            },
            SubItem {
                title: "Team",
                url: "#",
            },
            SubItem {
                title: "Billing",
                url: "#",
            },
            SubItem {
                title: "Limits",
                url: "#",
            },
        ],
    },
];

const PROJECTS: &[Project] = &[
    Project {
        name: "Design Engineering",
        url: "#",
    },
    Project {
        name: "Sales & Marketing",
        url: "#",
    },
    Project {
        name: "Travel",
        url: "#",
    },
];

#[component]
pub fn Demo() -> Element {
    let side = use_signal(|| SidebarSide::Left);
    let collapsible = use_signal(|| SidebarCollapsible::Offcanvas);

    rsx! {
        SidebarProvider {
            Sidebar {
                variant: SidebarVariant::Sidebar,
                collapsible: collapsible(),
                side: side(),
                SidebarHeader {
                    TeamSwitcher { teams: TEAMS }
                }
                SidebarContent {
                    NavMain { items: NAV_MAIN }
                    NavProjects { projects: PROJECTS }
                }
                SidebarFooter { NavUser {} }
                SidebarRail {}
            }
            SidebarInset {
                header { style: "display:flex; align-items:center; justify-content:space-between; height:3.5rem; flex-shrink:0; padding:0 1rem; border-bottom:1px solid var(--dx-sidebar-border); background:var(--primary-color-1);",
                    div { style: "display: flex; align-items: center; gap: 0.75rem;",
                        SidebarTrigger {}
                        Separator { height: "1rem", horizontal: false }
                        span { "Sidebar Setting" }
                    }
                }
                div { style: "display:flex; flex:1; flex-direction:column; gap:1.5rem; padding:1.5rem; min-height:0; overflow-y:auto; overflow-x:hidden;",
                    DemoSettingControls { side, collapsible }
                    Skeleton { style: "height: 10rem; width: 100%; flex-shrink:0;" }
                    Skeleton { style: "height: 20rem; width: 100%; flex-shrink:0;" }
                }
            }
        }
    }
}

#[component]
fn TeamSwitcher(teams: &'static [Team]) -> Element {
    let mut active_team = use_signal(|| 0usize);

    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu { class: Styles::dx_sidebar_dropdown_menu,
                    DropdownMenuTrigger { class: Styles::dx_sidebar_dropdown_menu_trigger,
                        as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton { class: DemoStyles::dx_sidebar_menu_disclosure_button, size: SidebarMenuButtonSize::Lg, attributes,
                                div { style: "display:flex; flex-shrink:0; align-items:center; justify-content:center; width:2rem; height:2rem; aspect-ratio:1; border-radius:0.5rem; background:var(--dx-sidebar-accent); color:var(--dx-sidebar-accent-foreground);",
                                    DemoIcon {}
                                }
                                div { class: DemoStyles::dx_sidebar_info_block,
                                    span { class: DemoStyles::dx_sidebar_info_title, {teams[active_team()].name} }
                                    span { class: DemoStyles::dx_sidebar_info_subtitle, {teams[active_team()].plan} }
                                }
                                ChevronIcon {}
                            }
                        },
                    }
                    DropdownMenuContent { class: Styles::dx_sidebar_dropdown_menu_content,
                        div { style: "padding:0.5rem; font-size:0.75rem; opacity:0.7;",
                            "Teams"
                        }
                        for (idx , team) in teams.iter().enumerate() {
                            DropdownMenuItem {
                                index: idx,
                                value: idx,
                                on_select: move |v: usize| active_team.set(v),
                                DemoIcon {}
                                {team.name}
                                span { style: "margin-left:auto; font-size:0.75rem; opacity:0.7;",
                                    "⌘{idx + 1}"
                                }
                            }
                        }
                        Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                        DropdownMenuItem {
                            index: teams.len(),
                            value: 999usize,
                            on_select: move |_: usize| {},
                            DemoIcon {}
                            div { style: "opacity:0.7; font-weight:500;", "Add team" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavMain(items: &'static [NavMainItem]) -> Element {
    rsx! {
        SidebarGroup {
            SidebarGroupLabel { "Platform" }
            SidebarMenu {
                for item in items.iter() {
                    Collapsible {
                        default_open: item.is_active,
                        as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuItem { key: "{item.title}", attributes,
                                CollapsibleTrigger { class: Styles::dx_sidebar_collapsible_trigger,
                                    as: move |attributes: Vec<Attribute>| rsx! {
                                        SidebarMenuButton {
                                            class: DemoStyles::dx_sidebar_menu_disclosure_button,
                                            tooltip: rsx! {
                                                {item.title}
                                            },
                                            attributes,
                                            DemoIcon {}
                                            span { {item.title} }
                                            ChevronIcon {}
                                        }
                                    },
                                }
                                CollapsibleContent {
                                    SidebarMenuSub {
                                        for sub_item in item.items {
                                            SidebarMenuSubItem { key: "{sub_item.title}",
                                                SidebarMenuSubButton {
                                                    as: move |attributes: Vec<Attribute>| rsx! {
                                                        a { href: sub_item.url, ..attributes,
                                                            span { {sub_item.title} }
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn NavProjects(projects: &'static [Project]) -> Element {
    rsx! {
        SidebarGroup { class: DemoStyles::dx_sidebar_hide_on_collapse,
            SidebarGroupLabel { "Projects" }
            SidebarMenu {
                for project in projects.iter() {
                    SidebarMenuItem { key: "{project.name}",
                        SidebarMenuButton {
                            as: move |attributes: Vec<Attribute>| rsx! {
                                a { href: project.url, ..attributes,
                                    DemoIcon {}
                                    span { {project.name} }
                                }
                            },
                        }
                        DropdownMenu { class: Styles::dx_sidebar_dropdown_menu,
                            DropdownMenuTrigger { class: Styles::dx_sidebar_dropdown_menu_trigger,
                                as: move |attributes: Vec<Attribute>| rsx! {
                                    SidebarMenuAction { show_on_hover: true, attributes,
                                        DemoIcon {}
                                        span { class: Styles::dx_sr_only, "More" }
                                    }
                                },
                            }
                            DropdownMenuContent { class: Styles::dx_sidebar_dropdown_menu_content,
                                DropdownMenuItem {
                                    index: 0usize,
                                    value: "view".to_string(),
                                    on_select: move |_: String| {},
                                    DemoIcon {}
                                    span { "View Project" }
                                }
                                DropdownMenuItem {
                                    index: 1usize,
                                    value: "share".to_string(),
                                    on_select: move |_: String| {},
                                    DemoIcon {}
                                    span { "Share Project" }
                                }
                                Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                                DropdownMenuItem {
                                    index: 2usize,
                                    value: "delete".to_string(),
                                    on_select: move |_: String| {},
                                    DemoIcon {}
                                    span { "Delete Project" }
                                }
                            }
                        }
                    }
                }
                SidebarMenuItem {
                    SidebarMenuButton { style: "opacity:0.7; font-weight:500;",
                        DemoIcon {}
                        span { "More" }
                    }
                    SidebarMenuBadge { "+99" }
                }
            }
        }
    }
}

#[component]
fn NavUser() -> Element {
    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu { class: Styles::dx_sidebar_dropdown_menu,
                    DropdownMenuTrigger { class: Styles::dx_sidebar_dropdown_menu_trigger,
                        as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton { class: DemoStyles::dx_sidebar_menu_disclosure_button, size: SidebarMenuButtonSize::Lg, attributes,
                                ImageAvatar {
                                    size: AvatarImageSize::Small,
                                    style: "border-radius:0.5rem;",
                                    src: crate::demos::avatar::DEMO_AVATAR_SRC.to_string(),
                                    alt: "dioxus avatar",
                                    aria_label: "dioxus avatar",
                                    "DX"
                                }
                                div { class: DemoStyles::dx_sidebar_info_block,
                                    span { class: DemoStyles::dx_sidebar_info_title, "Dioxus" }
                                    span { class: DemoStyles::dx_sidebar_info_subtitle, "m@example.com" }
                                }
                                ChevronIcon {}
                            }
                        },
                    }
                    DropdownMenuContent { class: Styles::dx_sidebar_dropdown_menu_content,
                        div { style: "display:flex; align-items:center; gap:0.5rem; padding:0.375rem 0.25rem; text-align:left; font-size:0.875rem;",
                            ImageAvatar {
                                size: AvatarImageSize::Small,
                                style: "border-radius:0.5rem;",
                                src: crate::demos::avatar::DEMO_AVATAR_SRC.to_string(),
                                alt: "dioxus avatar",
                                aria_label: "dioxus avatar",
                                "DX"
                            }
                            div { class: DemoStyles::dx_sidebar_info_block,
                                span { class: DemoStyles::dx_sidebar_info_title, "Dioxus" }
                                span { class: DemoStyles::dx_sidebar_info_subtitle, "m@example.com" }
                            }
                        }
                        Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                        DropdownMenuItem {
                            index: 0usize,
                            value: "upgrade".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Upgrade to Pro"
                        }
                        Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                        DropdownMenuItem {
                            index: 1usize,
                            value: "account".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Account"
                        }
                        DropdownMenuItem {
                            index: 2usize,
                            value: "billing".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Billing"
                        }
                        DropdownMenuItem {
                            index: 3usize,
                            value: "notifications".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Notifications"
                        }
                        Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                        DropdownMenuItem {
                            index: 4usize,
                            value: "logout".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Log out"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DemoSettingControls(
    side: Signal<SidebarSide>,
    collapsible: Signal<SidebarCollapsible>,
) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem; padding: 0.75rem; border: 1px solid var(--dx-sidebar-border); border-radius: 0.75rem; background: var(--primary-color-2);",
            div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; flex-wrap: wrap;",
                span { style: "font-size: 0.75rem; font-weight: 600; color: var(--secondary-color-4);",
                    "Side"
                }
                div { style: "display: inline-flex; gap: 0.5rem;",
                    Button {
                        variant: if side() == SidebarSide::Left { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| side.set(SidebarSide::Left),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Left"
                    }
                    Button {
                        variant: if side() == SidebarSide::Right { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| side.set(SidebarSide::Right),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Right"
                    }
                }
            }
            div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; flex-wrap: wrap;",
                span { style: "font-size: 0.75rem; font-weight: 600; color: var(--secondary-color-4);",
                    "Collapse"
                }
                div { style: "display: inline-flex; gap: 0.5rem; flex-wrap: wrap;",
                    Button {
                        variant: if collapsible() == SidebarCollapsible::Offcanvas { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::Offcanvas),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Offcanvas"
                    }
                    Button {
                        variant: if collapsible() == SidebarCollapsible::Icon { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::Icon),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Icon"
                    }
                    Button {
                        variant: if collapsible() == SidebarCollapsible::None { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::None),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "None"
                    }
                }
            }
        }
    }
}

#[component]
fn DemoIcon() -> Element {
    rsx! {
        Circle {
            class: DemoStyles::dx_sidebar_icon,
            size: "24px",
        }
    }
}

#[component]
fn ChevronIcon() -> Element {
    rsx! {
        ChevronRight {
            class: format!("{} {}", DemoStyles::dx_sidebar_icon, DemoStyles::dx_sidebar_chevron),
            size: "24px",
        }
    }
}
```
