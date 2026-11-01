use crate::components::dropdown_menu::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut picked = use_signal(|| None::<&'static str>);
    let pick = move |v| picked.set(Some(v));
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            DropdownMenu { default_open: false,
                DropdownMenuTrigger { "Actions" }
                DropdownMenuContent {
                    DropdownMenuItem::<&'static str> { value: "Edit", index: 0usize, on_select: pick, "Edit" }
                    DropdownMenuItem::<&'static str> { value: "Duplicate", index: 1usize, on_select: pick, "Duplicate" }
                    // A submenu: its trigger is item 2 of this menu, its items count from 0.
                    DropdownMenuSub { index: 2usize,
                        DropdownMenuSubTrigger { "Move to" }
                        DropdownMenuSubContent {
                            DropdownMenuItem::<&'static str> { value: "Inbox", index: 0usize, on_select: pick, "Inbox" }
                            DropdownMenuItem::<&'static str> { value: "Later", index: 1usize, on_select: pick, "Later" }
                            // Submenus nest.
                            DropdownMenuSub { index: 2usize,
                                DropdownMenuSubTrigger { "Project" }
                                DropdownMenuSubContent {
                                    DropdownMenuItem::<&'static str> { value: "Atoll", index: 0usize, on_select: pick, "Atoll" }
                                    DropdownMenuItem::<&'static str> { value: "Ledgerly", index: 1usize, on_select: pick, "Ledgerly" }
                                }
                            }
                        }
                    }
                    DropdownMenuItem::<&'static str> { value: "Archive", index: 3usize, on_select: pick, "Archive" }
                    DropdownMenuItem::<&'static str> { value: "Delete", index: 4usize, on_select: pick, "Delete" }
                }
            }
            if let Some(a) = picked() {
                span { style: "font-size: 0.875rem;", "Selected: {a}" }
            }
        }
    }
}
