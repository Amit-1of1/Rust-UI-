use super::component::*;
use crate::components::button::Button;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Open drawer" }
        Drawer { open: open(), on_open_change: move |v| open.set(v),
            DrawerTitle { "Move goal" }
            DrawerDescription { "Set your daily activity goal. Drag the handle down, or press Escape, to close." }
            p { style: "margin: 0.5rem 0;", "Pick a number that feels realistic for this week." }
            DrawerClose { "Close" }
        }
    }
}
