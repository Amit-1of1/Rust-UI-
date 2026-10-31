// Deliberately broken app: `cargo.jsonl` is real `cargo check --message-format=json` output
// for this file (rustc 1.97, dioxus 0.7.10), trimmed to the fields `rust-ui explain` reads.
// tests/explain.rs checks what explain says about each error. Not compiled.
use dioxus::prelude::*;
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::dialog::Dialog;
use ui::components::fancy_widget::Widget;

mod components;
use components::dialog_x as _d;
fn main() {}

#[component]
fn App() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { variant: ButtonVariant::Primry, size: ButtonSize::Icon, onclick: move |_| open.set(true), "Open" }
        Button { colour: "red", "Bad prop" }
        Button { variant: "primary", "Str" }
        Dialog { open: open(), "Hello" }
        ui::components::avatar::AvatarImage { src: "a.png" }
        ui::components::button::IconButton { "x" }
    }
}
