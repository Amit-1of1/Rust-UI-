use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/input_otp/style.css")]
struct Styles;

fn focus_slot(base: &str, index: usize) {
    // `base` is generated here (`dx-otp-<n>`), never user text.
    let _ = document::eval(&format!(
        "var e=document.getElementById('{base}-{index}');if(e){{e.focus();e.select();}}"
    ));
}

/// One-time-code input: one box per character with auto-advance, Backspace to go back, arrow
/// keys, and paste or SMS autofill spread across the boxes. Each box is a real input, so it
/// works with screen readers and password managers.
#[component]
pub fn InputOtp(
    /// Number of characters.
    #[props(default = 6)]
    length: usize,
    /// Accessible name of the whole code, for example "Verification code".
    #[props(into, default = "One-time code".to_string())]
    label: String,
    /// Only accept digits (the default). Off accepts letters and digits.
    #[props(default = true)]
    numeric: bool,
    /// Draw a dash after every `group` boxes (0 for none), for example 3 for `123-456`.
    #[props(default = 0)]
    group: usize,
    #[props(default)] invalid: bool,
    #[props(default)] disabled: bool,
    /// Called on every change with the code so far.
    #[props(default)]
    on_change: Callback<String>,
    /// Called once all boxes are filled.
    #[props(default)]
    on_complete: Callback<String>,
) -> Element {
    let length = length.clamp(1, 12);
    let base = use_hook(|| {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        format!("dx-otp-{}", NEXT.fetch_add(1, Ordering::Relaxed))
    });
    let mut chars = use_signal(move || vec![None::<char>; length]);

    let accept = move |c: char| {
        if numeric {
            c.is_ascii_digit()
        } else {
            c.is_ascii_alphanumeric()
        }
    };
    let publish = move |chars: &[Option<char>]| {
        let code: String = chars.iter().flatten().collect();
        on_change.call(code.clone());
        if chars.iter().all(Option::is_some) {
            on_complete.call(code);
        }
    };

    rsx! {
        div {
            class: Styles::dx_input_otp,
            role: "group",
            "aria-label": "{label}",
            "data-invalid": invalid.then_some("true"),
            for i in 0..length {
                Fragment { key: "{i}",
                if group > 0 && i > 0 && i % group == 0 {
                    span { class: Styles::dx_input_otp_dash, "aria-hidden": "true", "–" }
                }
                input {
                    id: "{base}-{i}",
                    class: Styles::dx_input_otp_slot,
                    r#type: "text",
                    inputmode: if numeric { "numeric" } else { "text" },
                    autocomplete: if i == 0 { "one-time-code" } else { "off" },
                    "aria-label": "{label}, character {i + 1} of {length}",
                    "aria-invalid": invalid.then_some("true"),
                    disabled,
                    value: chars.read()[i].map(String::from).unwrap_or_default(),
                    onfocus: {
                        let base = base.clone();
                        move |_| {
                            let _ = document::eval(&format!("document.getElementById('{base}-{i}')?.select()"));
                        }
                    },
                    oninput: {
                        let base = base.clone();
                        move |e: FormEvent| {
                            let typed: Vec<char> = e.value().chars().filter(|c| accept(*c)).collect();
                            let mut next = chars.read().clone();
                            if typed.is_empty() {
                                next[i] = None;
                                chars.set(next.clone());
                                publish(&next);
                                return;
                            }
                            // One character is typing; more than one is a paste or autofill.
                            let mut at = i;
                            for c in typed.into_iter().take(length - i) {
                                next[at] = Some(c);
                                at += 1;
                            }
                            chars.set(next.clone());
                            publish(&next);
                            focus_slot(&base, at.min(length - 1));
                        }
                    },
                    onkeydown: {
                        let base = base.clone();
                        move |e: KeyboardEvent| match e.key() {
                            Key::Backspace if chars.read()[i].is_none() && i > 0 => {
                                e.prevent_default();
                                let mut next = chars.read().clone();
                                next[i - 1] = None;
                                chars.set(next.clone());
                                publish(&next);
                                focus_slot(&base, i - 1);
                            }
                            Key::ArrowLeft if i > 0 => {
                                e.prevent_default();
                                focus_slot(&base, i - 1);
                            }
                            Key::ArrowRight if i + 1 < length => {
                                e.prevent_default();
                                focus_slot(&base, i + 1);
                            }
                            Key::Home => {
                                e.prevent_default();
                                focus_slot(&base, 0);
                            }
                            Key::End => {
                                e.prevent_default();
                                focus_slot(&base, length - 1);
                            }
                            _ => {}
                        }
                    },
                }
                }
            }
        }
    }
}
