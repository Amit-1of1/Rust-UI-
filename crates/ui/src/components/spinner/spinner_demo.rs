use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; gap: 1rem; align-items: center;",
            Spinner {}
            Spinner { style: "width: 1.5rem; height: 1.5rem;" }
            Spinner { style: "width: 2rem; height: 2rem; border-width: 3px;" }
        }
    }
}
