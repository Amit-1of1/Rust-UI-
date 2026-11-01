use crate::components::badge::*;
use crate::components::button::*;
use crate::components::icons::{Bell, Check, TriangleAlert};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut tags = use_signal(|| vec!["Rust", "Dioxus", "WebAssembly"]);
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge { variant: BadgeVariant::Success, Check { "aria-hidden": "true", size: "12px" } "Paid" }
            Badge { variant: BadgeVariant::Warning, TriangleAlert { "aria-hidden": "true", size: "12px" } "Overdue" }
            Badge { variant: BadgeVariant::Secondary, VerifiedIcon {} "Verified" }
        }
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; margin-top: 0.75rem;",
            Badge { size: BadgeSize::Sm, "Small" }
            Badge { "Default" }
            Badge { size: BadgeSize::Lg, "Large" }
            Badge { dot: true, variant: BadgeVariant::Success, aria_label: "Online" }
            span { "Online" }
        }
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.5rem; margin-top: 0.75rem;",
            for tag in tags() {
                Badge { key: "{tag}", variant: BadgeVariant::Outline, dismiss_label: tag,
                    on_dismiss: move |_| tags.write().retain(|t| *t != tag),
                    "{tag}"
                }
            }
        }
        div { style: "display: flex; align-items: center; gap: 1.5rem; margin-top: 1rem;",
            BadgeAnchor {
                badge: rsx! { Badge { size: BadgeSize::Sm, variant: BadgeVariant::Destructive, aria_hidden: "true", "3" } },
                IconButton { label: "Notifications, 3 unread", variant: ButtonVariant::Outline,
                    Bell { "aria-hidden": "true" }
                }
            }
            BadgeAnchor { placement: BadgePlacement::BottomEnd,
                badge: rsx! { Badge { dot: true, variant: BadgeVariant::Success } },
                IconButton { label: "Messages", variant: ButtonVariant::Ghost,
                    Bell { "aria-hidden": "true" }
                }
            }
        }
    }
}
