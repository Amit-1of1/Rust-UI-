use crate::components::menubar::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            Menubar {
                MenubarMenu { index: 0usize,
                    MenubarTrigger { "File" }
                    MenubarContent {
                        MenubarItem {
                            index: 0usize,
                            value: "new".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "New"
                        }
                        MenubarItem {
                            index: 1usize,
                            value: "open".to_string(),
                            disabled: true,
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Open"
                        }
                        // A submenu: its trigger is item 2 of this menu, its items count from 0.
                        MenubarSub { index: 2usize,
                            MenubarSubTrigger { "Open recent" }
                            MenubarSubContent {
                                MenubarItem {
                                    index: 0usize,
                                    value: "recent:notes.md".to_string(),
                                    on_select: move |value| {
                                        tracing::info!("Selected value: {}", value);
                                    },
                                    "notes.md"
                                }
                                MenubarItem {
                                    index: 1usize,
                                    value: "recent:budget.csv".to_string(),
                                    on_select: move |value| {
                                        tracing::info!("Selected value: {}", value);
                                    },
                                    "budget.csv"
                                }
                            }
                        }
                        MenubarItem {
                            index: 3usize,
                            value: "save".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Save"
                        }
                    }
                }
                MenubarMenu { index: 1usize,
                    MenubarTrigger { "Edit" }
                    MenubarContent {
                        MenubarItem {
                            index: 0usize,
                            value: "cut".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Cut"
                        }
                        MenubarItem {
                            index: 1usize,
                            value: "copy".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Copy"
                        }
                        MenubarItem {
                            index: 2usize,
                            value: "paste".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Paste"
                        }
                    }
                }
            }
        }
    }
}
