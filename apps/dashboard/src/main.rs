//! Acme Admin: a small admin dashboard built only from the kit. Its job is to find what the
//! kit is missing for a real app; everything found is written down in `GAPS.md`.
//!
//! Pages: Overview (cards, charts, recent orders), Customers (custom-cell table, row actions,
//! edit sheet, delete confirmation, add dialog), Orders (DataTable with date range and status
//! filters), Settings (form, combobox, switches, theme). Ctrl+K opens the command palette.

mod customers;
mod data;
mod orders;
mod overview;
mod settings;

use data::Customer;
use dioxus::prelude::*;
use ui::icons::{LayoutDashboard, Receipt, Settings as SettingsIcon, Users};
use ui::prelude::*;

const DASHBOARD_CSS: Asset = asset!("/assets/dashboard.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Page {
    Overview,
    Customers,
    Orders,
    Settings,
}

impl Page {
    pub const ALL: [Page; 4] = [
        Page::Overview,
        Page::Customers,
        Page::Orders,
        Page::Settings,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Page::Overview => "Overview",
            Page::Customers => "Customers",
            Page::Orders => "Orders",
            Page::Settings => "Settings",
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Page::Overview => "overview",
            Page::Customers => "customers",
            Page::Orders => "orders",
            Page::Settings => "settings",
        }
    }

    pub fn from_id(id: &str) -> Option<Page> {
        Page::ALL.into_iter().find(|p| p.id() == id)
    }
}

/// App-wide state, shared through context.
#[derive(Clone, Copy)]
pub struct Store {
    pub customers: Signal<Vec<Customer>>,
    pub page: Signal<Page>,
    /// Set from the command palette: open the "Add customer" dialog.
    pub adding: Signal<bool>,
}

pub fn use_store() -> Store {
    use_context::<Store>()
}

/// `document.body.dataset.ready = "1"` once mounted: the e2e scripts wait for it.
fn mark_ready() {
    #[cfg(target_arch = "wasm32")]
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        let _ = body.dataset().set("ready", "1");
    }
}

#[component]
fn App() -> Element {
    let customers = use_signal(|| data::customers(48));
    use_context_provider(|| Store {
        customers,
        page: Signal::new(Page::Overview),
        adding: Signal::new(false),
    });
    use_effect(mark_ready);
    rsx! {
        ThemeStyles {}
        document::Stylesheet { href: DASHBOARD_CSS }
        ToastProvider { Shell {} }
    }
}

#[component]
fn Shell() -> Element {
    let mut store = use_store();
    let mut palette = use_signal(|| false);
    use_hotkey("k", move || palette.set(true));
    // The same state ThemeCustomizer (Settings) uses, so the switch and the customizer agree.
    let theme = use_theme();
    let set_dark = move |dark: bool| {
        theme.set_mode(if dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        })
    };

    let page = (store.page)();
    let mut commands: Vec<CommandItem> = Page::ALL
        .iter()
        .map(|p| CommandItem::new(p.id(), format!("Go to {}", p.title())).group("Navigate"))
        .collect();
    commands.extend([
        CommandItem::new("add-customer", "Add customer").group("Actions"),
        CommandItem::new("theme", "Toggle dark mode").group("Actions"),
    ]);

    rsx! {
        SidebarProvider {
            Sidebar {
                collapsible: SidebarCollapsible::Icon,
                aria_label: "Main",
                SidebarHeader {
                    div { class: "db-brand",
                        span { class: "db-logo", "aria-hidden": "true", "A" }
                        span { class: "db-brand-name", "Acme Admin" }
                    }
                }
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "Workspace" }
                        SidebarMenu {
                            for p in Page::ALL {
                                SidebarMenuItem { key: "{p.id()}",
                                    SidebarMenuButton {
                                        is_active: p == page,
                                        tooltip: rsx! { {p.title()} },
                                        onclick: move |_| store.page.set(p),
                                        PageIcon { page: p }
                                        span { {p.title()} }
                                    }
                                    if p == Page::Customers {
                                        SidebarMenuBadge { "{store.customers.read().len()}" }
                                    }
                                }
                            }
                        }
                    }
                }
                SidebarRail {}
            }
            SidebarInset {
                header { class: "db-header",
                    SidebarTrigger {}
                    Separator { height: "1rem", horizontal: false }
                    div { class: "db-crumbs",
                        Breadcrumb {
                            BreadcrumbItem {
                                BreadcrumbLink {
                                    href: "#",
                                    onclick: move |e: MouseEvent| {
                                        e.prevent_default();
                                        store.page.set(Page::Overview);
                                    },
                                    "Acme"
                                }
                            }
                            BreadcrumbSeparator {}
                            BreadcrumbItem {
                                BreadcrumbPage { {page.title()} }
                            }
                        }
                    }
                    div { class: "db-header-tools",
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            onclick: move |_| palette.set(true),
                            "Search"
                            span { class: "db-wide-only",
                                KbdGroup {
                                    Kbd { "Ctrl" }
                                    Kbd { "K" }
                                }
                            }
                        }
                        Switch {
                            checked: theme.is_dark(),
                            aria_label: "Dark mode",
                            on_checked_change: set_dark,
                        }
                        UserMenu {}
                    }
                }
                // SidebarInset is already the page's <main>.
                div { class: "db-main", id: "main-content",
                    h1 { class: "db-title", {page.title()} }
                    match page {
                        Page::Overview => rsx! { overview::Overview {} },
                        Page::Customers => rsx! { customers::Customers {} },
                        Page::Orders => rsx! { orders::Orders {} },
                        Page::Settings => rsx! { settings::Settings {} },
                    }
                }
            }
        }
        CommandDialog {
            open: palette(),
            on_open_change: move |v| palette.set(v),
            items: commands,
            on_select: move |id: String| match id.as_str() {
                "add-customer" => {
                    store.page.set(Page::Customers);
                    store.adding.set(true);
                }
                "theme" => set_dark(!theme.is_dark()),
                other => {
                    if let Some(p) = Page::from_id(other) {
                        store.page.set(p);
                    }
                }
            },
        }
    }
}

#[component]
fn PageIcon(page: Page) -> Element {
    match page {
        Page::Overview => rsx! { LayoutDashboard {} },
        Page::Customers => rsx! { Users {} },
        Page::Orders => rsx! { Receipt {} },
        Page::Settings => rsx! { SettingsIcon {} },
    }
}

#[component]
fn UserMenu() -> Element {
    let mut store = use_store();
    let toasts = use_toast();
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { aria_label: "Account menu", "AL" }
            DropdownMenuContent {
                DropdownMenuItem::<&'static str> {
                    index: 0usize,
                    value: "settings",
                    on_select: move |_| store.page.set(Page::Settings),
                    "Settings"
                }
                DropdownMenuItem::<&'static str> {
                    index: 1usize,
                    value: "logout",
                    on_select: move |_| toasts.notify_info("Signed out (demo)"),
                    "Sign out"
                }
            }
        }
    }
}
