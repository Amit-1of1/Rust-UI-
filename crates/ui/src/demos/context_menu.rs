use crate::components::context_menu::*;
use dioxus::prelude::*;
#[component]
pub fn Demo() -> Element {
    let mut selected_item = use_signal(|| None);

    rsx! {
        ContextMenu {
            ContextMenuTrigger { "right click here" }
            ContextMenuContent {
                ContextMenuItem {
                    value: "edit".to_string(),
                    index: 0usize,
                    on_select: move |value| {
                        selected_item.set(Some(value));
                    },
                    "Edit"
                }
                ContextMenuItem {
                    value: "undo".to_string(),
                    index: 1usize,
                    disabled: true,
                    on_select: move |value| {
                        selected_item.set(Some(value));
                    },
                    "Undo"
                }
                ContextMenuItem {
                    value: "duplicate".to_string(),
                    index: 2usize,
                    on_select: move |value| {
                        selected_item.set(Some(value));
                    },
                    "Duplicate"
                }
                // A submenu: its trigger is item 3 of this menu, its items count from 0.
                ContextMenuSub { index: 3usize,
                    ContextMenuSubTrigger { "Share" }
                    ContextMenuSubContent {
                        ContextMenuItem {
                            value: "copy-link".to_string(),
                            index: 0usize,
                            on_select: move |value| selected_item.set(Some(value)),
                            "Copy link"
                        }
                        ContextMenuItem {
                            value: "email".to_string(),
                            index: 1usize,
                            on_select: move |value| selected_item.set(Some(value)),
                            "Email"
                        }
                    }
                }
                ContextMenuItem {
                    value: "delete".to_string(),
                    index: 4usize,
                    on_select: move |value| {
                        selected_item.set(Some(value));
                    },
                    "Delete"
                }
            }
        }

        if let Some(item) = selected_item() {
            span { margin_left: "10px", "Selected: {item}" }
        }
    }
}
