use crate::components::breadcrumb::*;
use crate::components::icons::{ChevronRight, House};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1rem;",
            Breadcrumb {
                BreadcrumbItem {
                    BreadcrumbLink { href: "#",
                        House { "aria-hidden": "true", size: "0.875rem" }
                        "Home"
                    }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem { BreadcrumbLink { href: "#", "Components" } }
                BreadcrumbSeparator {}
                BreadcrumbItem { BreadcrumbPage { "Breadcrumb" } }
            }
            // A custom separator and a step you can't open yet.
            Breadcrumb { aria_label: "Billing breadcrumb",
                BreadcrumbItem { BreadcrumbLink { href: "#", "Workspace" } }
                BreadcrumbSeparator { ChevronRight { "aria-hidden": "true" } }
                BreadcrumbItem { BreadcrumbLink { href: "#", disabled: true, "Billing (owners only)" } }
                BreadcrumbSeparator { ChevronRight { "aria-hidden": "true" } }
                BreadcrumbItem { BreadcrumbPage { "Invoices" } }
            }
            // A long trail collapses its middle; "…" shows the rest.
            BreadcrumbTrail { aria_label: "Project breadcrumb", max_items: 3,
                items: vec![
                    Crumb::link("Home", "#"),
                    Crumb::link("Projects", "#"),
                    Crumb::link("Atoll", "#"),
                    Crumb::link("Settings", "#"),
                    Crumb::page("Members"),
                ],
            }
        }
    }
}
