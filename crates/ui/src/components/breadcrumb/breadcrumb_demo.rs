use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbItem { BreadcrumbLink { href: "#", "Home" } }
            BreadcrumbSeparator {}
            BreadcrumbItem { BreadcrumbLink { href: "#", "Components" } }
            BreadcrumbSeparator {}
            BreadcrumbItem { BreadcrumbPage { "Breadcrumb" } }
        }
    }
}
