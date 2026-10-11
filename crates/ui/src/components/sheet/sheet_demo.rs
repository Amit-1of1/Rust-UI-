use super::component::*;
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut side = use_signal(|| SheetSide::Right);
    let mut show = move |s: SheetSide| {
        side.set(s);
        open.set(true);
    };
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Left), "Left" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Right), "Right" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Top), "Top" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Bottom), "Bottom" }
        }
        Sheet { open: open(), on_open_change: move |v| open.set(v), "data-side": side().as_str(),
            SheetHeader {
                SheetTitle { "Edit profile" }
                SheetDescription { "Make changes to your profile here." }
            }
            div { style: "display: grid; gap: 0.5rem; max-width: 20rem; padding: 0 1rem;",
                Label { html_for: "sheet-name", "Name" }
                Input { id: "sheet-name", initial_value: "Ferris" }
            }
            SheetFooter {
                Button { onclick: move |_| open.set(false), "Save changes" }
            }
            SheetContentClose {}
        }
    }
}
