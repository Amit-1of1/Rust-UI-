use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut wide = use_signal(|| false);
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 0.75rem;",
            Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Open dialog" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| wide.set(true), "Open a large dialog" }
        }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Are you sure?",
            description: "This action cannot be undone.",
            size: DialogSize::Sm,
            DialogFooter {
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Cancel" }
                Button { variant: ButtonVariant::Destructive, onclick: move |_| open.set(false), "Delete" }
            }
        }
        Dialog {
            open: wide(),
            on_open_change: move |v| wide.set(v),
            title: "Invite people",
            description: "They get an email with a link to join the workspace.",
            size: DialogSize::Lg,
            show_close: true,
            p { style: "margin: 0;", "Paste one email address per line, or import a CSV file." }
            DialogFooter {
                Button { onclick: move |_| wide.set(false), "Send invites" }
            }
        }
    }
}
