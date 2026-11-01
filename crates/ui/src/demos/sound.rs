use crate::components::sound::*;
use crate::components::switch::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut alerts = use_signal(|| false);
    rsx! {
        SoundProvider { taps: true,
            div { style: "display: flex; flex-direction: column; gap: 1rem; max-width: 28rem;",
                p { style: "margin: 0; font-size: 0.875rem;",
                    "Off until you turn it on. Then pick Ripple, Glass, Driftwood or Off for each sound, and try the switch below."
                }
                SoundSettings {}
                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                    Switch {
                        checked: alerts(),
                        aria_label: "Email alerts",
                        on_checked_change: move |v| alerts.set(v),
                    }
                    span { style: "font-size: 0.875rem;", "Email alerts" }
                }
            }
        }
    }
}
