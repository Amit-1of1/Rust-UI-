use crate::components::badge::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge { variant: BadgeVariant::Secondary, VerifiedIcon {} "Verified" }
        }
    }
}
