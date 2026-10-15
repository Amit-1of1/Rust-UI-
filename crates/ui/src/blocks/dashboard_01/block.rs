use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::card::{
    Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle,
};
use crate::components::chart::{BarChart, ChartDatum, LineChart};
use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::empty::{Empty, EmptyDescription, EmptyHeader, EmptyTitle};
use crate::components::icons::{LayoutDashboard, Receipt, Settings, Users};
use crate::components::separator::Separator;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, SidebarGroupLabel, SidebarHeader,
    SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail,
    SidebarTrigger,
};
use crate::components::switch::Switch;
use crate::components::table::{
    Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow,
};
use crate::components::theme_customizer::{use_theme, ThemeMode};
use dioxus::prelude::*;

#[css_module("/src/blocks/dashboard_01/style.css")]
struct Styles;

/// One link in the [`DashboardShell`] sidebar.
#[derive(Clone, PartialEq)]
pub struct NavItem {
    pub id: String,
    pub label: String,
    /// Shown before the label and alone when the sidebar is collapsed. Hidden from screen readers.
    pub icon: Element,
}

impl NavItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, icon: Element) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon,
        }
    }
}

/// The frame of an app screen: a collapsible sidebar (to icons), a header with the sidebar
/// toggle, a dark-mode switch and an account menu, and a `<main>` with the page title as its
/// `h1`. Put the page in `children`.
#[component]
pub fn DashboardShell(
    #[props(into)] brand: String,
    nav: Vec<NavItem>,
    /// The `id` of the current page; its link gets `aria-current` and its label is the `h1`.
    #[props(into)]
    active: String,
    #[props(default)] on_navigate: Callback<String>,
    /// Two letters for the account button.
    #[props(into, default = "ME".to_string())]
    initials: String,
    #[props(default)] on_sign_out: Callback<()>,
    children: Element,
) -> Element {
    let theme = use_theme();
    let title = nav
        .iter()
        .find(|n| n.id == active)
        .map(|n| n.label.clone())
        .unwrap_or_default();
    let logo: String = brand.chars().take(1).collect();

    rsx! {
        SidebarProvider { class: Styles::dx_block_dashboard, "data-block": "dashboard_01",
            Sidebar { collapsible: SidebarCollapsible::Icon, aria_label: "Main",
                SidebarHeader {
                    div { class: Styles::dx_block_dashboard_brand,
                        span { class: Styles::dx_block_dashboard_logo, "aria-hidden": "true", "{logo}" }
                        span { class: Styles::dx_block_dashboard_brand_name, "{brand}" }
                    }
                }
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "Workspace" }
                        SidebarMenu {
                            for item in nav {
                                SidebarMenuItem { key: "{item.id}",
                                    SidebarMenuButton {
                                        is_active: item.id == active,
                                        tooltip: rsx! { {item.label.clone()} },
                                        aria_current: (item.id == active).then_some("page"),
                                        onclick: {
                                            let id = item.id.clone();
                                            move |_| on_navigate.call(id.clone())
                                        },
                                        span { class: Styles::dx_block_dashboard_icon, "aria-hidden": "true", {item.icon} }
                                        span { "{item.label}" }
                                    }
                                }
                            }
                        }
                    }
                }
                SidebarRail {}
            }
            SidebarInset {
                header { class: Styles::dx_block_dashboard_header,
                    SidebarTrigger {}
                    Separator { height: "1rem", horizontal: false }
                    span { class: Styles::dx_block_dashboard_crumb, "{brand}" }
                    div { class: Styles::dx_block_dashboard_tools,
                        Switch {
                            checked: theme.is_dark(),
                            aria_label: "Dark mode",
                            on_checked_change: move |dark: bool| {
                                theme.set_mode(if dark { ThemeMode::Dark } else { ThemeMode::Light })
                            },
                        }
                        DropdownMenu {
                            DropdownMenuTrigger { aria_label: "Account menu", "{initials}" }
                            DropdownMenuContent {
                                DropdownMenuItem::<&'static str> {
                                    index: 0usize,
                                    value: "sign-out",
                                    on_select: move |_| on_sign_out.call(()),
                                    "Sign out"
                                }
                            }
                        }
                    }
                }
                div { class: Styles::dx_block_dashboard_main,
                    h1 { class: Styles::dx_block_dashboard_title, "{title}" }
                    {children}
                }
            }
        }
    }
}

const REVENUE: [(&str, f32); 6] = [
    ("Jan", 42.0),
    ("Feb", 47.5),
    ("Mar", 51.0),
    ("Apr", 49.2),
    ("May", 56.8),
    ("Jun", 61.3),
];

const PLANS: [(&str, f32); 3] = [("Free", 128.0), ("Pro", 64.0), ("Team", 21.0)];

/// (order, customer, status, amount in dollars)
const ORDERS: [(u32, &str, &str, u32); 5] = [
    (1042, "Ada Lovelace", "Paid", 240),
    (1041, "Grace Hopper", "Pending", 1_180),
    (1040, "Alan Turing", "Paid", 64),
    (1039, "Katherine Johnson", "Refunded", 320),
    (1038, "Edsger Dijkstra", "Paid", 99),
];

/// A complete dashboard screen: [`DashboardShell`] with an overview page (key figures, two
/// charts, recent orders) and placeholders for the other pages. Replace the sample data with
/// your own.
#[component]
pub fn Dashboard01() -> Element {
    let mut page = use_signal(|| "overview".to_string());
    let nav = vec![
        NavItem::new("overview", "Overview", rsx! { LayoutDashboard {} }),
        NavItem::new("customers", "Customers", rsx! { Users {} }),
        NavItem::new("orders", "Orders", rsx! { Receipt {} }),
        NavItem::new("settings", "Settings", rsx! { Settings {} }),
    ];

    rsx! {
        DashboardShell {
            brand: "Acme",
            nav,
            active: page(),
            on_navigate: move |id| page.set(id),
            initials: "AL",
            if page() == "overview" {
                Overview { on_view_orders: move |_| page.set("orders".to_string()) }
            } else {
                Empty {
                    EmptyHeader {
                        EmptyTitle { level: 2, "Nothing here yet" }
                        EmptyDescription { "Build this page from kit components." }
                    }
                }
            }
        }
    }
}

#[component]
fn Overview(on_view_orders: Callback<()>) -> Element {
    let revenue: Vec<ChartDatum> = REVENUE
        .iter()
        .map(|(m, v)| ChartDatum::new(*m, *v))
        .collect();
    let plans: Vec<ChartDatum> = PLANS.iter().map(|(p, v)| ChartDatum::new(*p, *v)).collect();
    rsx! {
        section { class: Styles::dx_block_dashboard_kpis, aria_label: "Key figures",
            Kpi { label: "Monthly revenue", value: "$61.3k", note: "+7.9% on last month" }
            Kpi { label: "Active customers", value: "213", note: "18 on trial" }
            Kpi { label: "Churn", value: "2.1%", note: "4 customers" }
            Kpi { label: "Open orders", value: "12", note: "3 need review" }
        }
        div { class: Styles::dx_block_dashboard_grid,
            Card {
                CardHeader {
                    CardTitle {
                        h2 { class: Styles::dx_block_dashboard_card_title, "Revenue" }
                    }
                    CardDescription { "Thousands of dollars per month" }
                }
                CardContent {
                    LineChart { data: revenue, label: "Revenue per month, thousands of dollars" }
                }
            }
            Card {
                CardHeader {
                    CardTitle {
                        h2 { class: Styles::dx_block_dashboard_card_title, "Customers by plan" }
                    }
                    CardDescription { "All active customers" }
                }
                CardContent {
                    BarChart { data: plans, label: "Customers by plan" }
                }
            }
        }
        Card {
            CardHeader {
                CardTitle {
                    h2 { class: Styles::dx_block_dashboard_card_title, "Recent orders" }
                }
                CardAction {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        onclick: move |_| on_view_orders.call(()),
                        "View all"
                    }
                }
            }
            CardContent {
                Table {
                    TableCaption { "The five most recent orders" }
                    TableHeader {
                        TableRow {
                            TableHead { "Order" }
                            TableHead { "Customer" }
                            TableHead { "Status" }
                            TableHead { numeric: true, "Amount" }
                        }
                    }
                    TableBody {
                        for (id , customer , status , amount) in ORDERS {
                            TableRow { key: "{id}",
                                TableCell { "#{id}" }
                                TableCell { "{customer}" }
                                TableCell {
                                    Badge { variant: status_badge(status), "{status}" }
                                }
                                TableCell { numeric: true, "${amount}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn status_badge(status: &str) -> BadgeVariant {
    match status {
        "Paid" => BadgeVariant::Primary,
        "Refunded" => BadgeVariant::Destructive,
        _ => BadgeVariant::Secondary,
    }
}

#[component]
fn Kpi(label: &'static str, value: &'static str, note: &'static str) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardDescription { "{label}" }
                CardTitle { class: Styles::dx_block_dashboard_kpi_value, "{value}" }
            }
            CardContent {
                p { class: Styles::dx_block_dashboard_muted, "{note}" }
            }
        }
    }
}
