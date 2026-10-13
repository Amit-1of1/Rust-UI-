use crate::components::button::{Button, ButtonVariant};
use crate::components::hooks::{use_copy_clipboard, use_media_query, MOBILE_QUERY};
use crate::components::kbd::{Kbd, KbdGroup};
use dioxus::prelude::*;

const INSTALL: &str = "rust-ui add hooks";

#[component]
pub fn Demo() -> Element {
    let mobile = use_media_query(MOBILE_QUERY);
    let dark = use_media_query("(prefers-color-scheme: dark)");
    let (copy, copied) = use_copy_clipboard();
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            p { style: "margin: 0;",
                "use_media_query(MOBILE_QUERY): "
                strong { if mobile() { "phone-sized" } else { "wider than a phone" } }
                ". use_media_query(prefers-color-scheme: dark): "
                strong { if dark() { "dark" } else { "light (or not on the web)" } }
                "."
            }
            div { style: "display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap;",
                code { "{INSTALL}" }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| copy.call(INSTALL.to_string()),
                    if copied() { "Copied" } else { "Copy" }
                }
            }
            p { style: "margin: 0;",
                "use_hotkey: "
                KbdGroup {
                    Kbd { "Ctrl" }
                    Kbd { "K" }
                }
                " opens the command palette above."
            }
        }
    }
}
