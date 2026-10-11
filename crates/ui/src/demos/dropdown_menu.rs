use crate::components::dropdown_menu::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut picked = use_signal(|| None::<&'static str>);
    let actions = ["Edit", "Duplicate", "Archive", "Delete"];
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            DropdownMenu { default_open: false,
                DropdownMenuTrigger { "Actions" }
                DropdownMenuContent {
                    for (i, a) in actions.into_iter().enumerate() {
                        DropdownMenuItem::<&'static str> {
                            value: a,
                            index: i,
                            on_select: move |v| picked.set(Some(v)),
                            {a}
                        }
                    }
                }
            }
            if let Some(a) = picked() {
                span { style: "font-size: 0.875rem;", "Selected: {a}" }
            }
        }
    }
}
