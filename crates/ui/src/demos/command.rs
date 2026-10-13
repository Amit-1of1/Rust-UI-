use crate::components::button::{Button, ButtonVariant};
use crate::components::command::*;
use crate::components::kbd::{Kbd, KbdGroup};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut last = use_signal(|| None::<String>);
    let items = vec![
        CommandItem::new("new-file", "New file")
            .group("File")
            .shortcut("Ctrl N"),
        CommandItem::new("open-file", "Open file")
            .group("File")
            .shortcut("Ctrl O"),
        CommandItem::new("save-file", "Save file")
            .group("File")
            .shortcut("Ctrl S"),
        CommandItem::new("toggle-theme", "Toggle theme").group("View"),
        CommandItem::new("zoom-in", "Zoom in")
            .group("View")
            .shortcut("Ctrl +"),
        CommandItem::new("zoom-out", "Zoom out")
            .group("View")
            .shortcut("Ctrl -"),
    ];
    rsx! {
        div { style: "display: flex; align-items: center; gap: 1rem; flex-wrap: wrap;",
            Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true),
                "Open command palette"
                KbdGroup {
                    Kbd { "Ctrl" }
                    Kbd { "K" }
                }
            }
            if let Some(id) = last() {
                span { style: "font-size: 0.875rem;", "Ran: {id}" }
            }
        }
        CommandDialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            items,
            on_select: move |id| last.set(Some(id)),
            // Ctrl+K (Cmd+K) anywhere on the page; the button does the same off the web.
            hotkey: "k",
        }
    }
}
