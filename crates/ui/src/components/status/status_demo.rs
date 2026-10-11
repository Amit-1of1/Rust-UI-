use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 1.25rem;",
            Status { "Idle" }
            Status { kind: StatusKind::Active, pulse: true, "Online" }
            Status { kind: StatusKind::Warning, "Degraded" }
            Status { kind: StatusKind::Error, "Down" }
            Status { kind: StatusKind::Info, "Deploying" }
        }
    }
}
