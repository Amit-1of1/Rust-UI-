use crate::components::input_otp::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut code = use_signal(String::new);
    let mut done = use_signal(|| false);
    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            InputOtp {
                label: "Verification code",
                length: 6,
                group: 3,
                on_change: move |c: String| {
                    code.set(c);
                    done.set(false);
                },
                on_complete: move |_| done.set(true),
            }
            p { style: "margin: 0; font-size: 0.875rem;", role: "status",
                if done() { "Code {code()} entered." } else { "Enter the 6-digit code." }
            }
        }
    }
}
