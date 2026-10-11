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

/// Something the user did to one box of an [`InputOtp`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OtpEvent {
    /// The box at `slot` received `text`: one character when typing, several for a paste
    /// or SMS autofill, empty when its character was deleted.
    Input {
        slot: usize,
        text: String,
    },
    /// Backspace in the box at `slot`.
    Backspace(usize),
    Left(usize),
    Right(usize),
    Home,
    End,
}

/// What [`OtpState::apply`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpOutcome {
    /// Not ours; let the browser handle the key.
    Ignored,
    /// Only focus moved.
    Moved,
    /// The code changed (and focus may have moved).
    Changed,
}

/// The code inside an [`InputOtp`] and which box has focus. Pure, so the keyboard
/// behaviour is tested without a browser (`crates/ui/tests/keyboard_state.rs`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OtpState {
    pub chars: Vec<Option<char>>,
    pub focus: usize,
    /// Accept only digits (otherwise letters and digits).
    pub numeric: bool,
}

impl OtpState {
    pub fn new(length: usize, numeric: bool) -> Self {
        Self {
            chars: vec![None; length.max(1)],
            focus: 0,
            numeric,
        }
    }

    pub fn accepts(&self, c: char) -> bool {
        if self.numeric {
            c.is_ascii_digit()
        } else {
            c.is_ascii_alphanumeric()
        }
    }

    /// The characters entered so far, in order, skipping empty boxes.
    pub fn code(&self) -> String {
        self.chars.iter().flatten().collect()
    }

    pub fn is_complete(&self) -> bool {
        self.chars.iter().all(Option::is_some)
    }

    pub fn apply(&mut self, event: OtpEvent) -> OtpOutcome {
        let last = self.chars.len() - 1;
        match event {
            OtpEvent::Input { slot, text } if slot <= last => {
                let typed: Vec<char> = text.chars().filter(|&c| self.accepts(c)).collect();
                if typed.is_empty() {
                    self.chars[slot] = None;
                    self.focus = slot;
                    return OtpOutcome::Changed;
                }
                // One character is typing; more than one is a paste or autofill.
                let mut at = slot;
                for c in typed.into_iter().take(last + 1 - slot) {
                    self.chars[at] = Some(c);
                    at += 1;
                }
                self.focus = at.min(last);
                OtpOutcome::Changed
            }
            OtpEvent::Backspace(slot) if slot > 0 && slot <= last && self.chars[slot].is_none() => {
                self.chars[slot - 1] = None;
                self.focus = slot - 1;
                OtpOutcome::Changed
            }
            OtpEvent::Left(slot) if slot > 0 && slot <= last => self.move_to(slot - 1),
            OtpEvent::Right(slot) if slot < last => self.move_to(slot + 1),
            OtpEvent::Home => self.move_to(0),
            OtpEvent::End => self.move_to(last),
            _ => OtpOutcome::Ignored,
        }
    }

    fn move_to(&mut self, slot: usize) -> OtpOutcome {
        self.focus = slot;
        OtpOutcome::Moved
    }
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
    let mut state = use_signal(move || OtpState::new(length, numeric));
    let apply = {
        let base = base.clone();
        move |event: OtpEvent| -> OtpOutcome {
            let outcome = state.write().apply(event);
            let now = state.read().clone();
            if outcome == OtpOutcome::Changed {
                on_change.call(now.code());
                if now.is_complete() {
                    on_complete.call(now.code());
                }
            }
            if outcome != OtpOutcome::Ignored {
                focus_slot(&base, now.focus);
            }
            outcome
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
                    value: state.read().chars[i].map(String::from).unwrap_or_default(),
                    onfocus: {
                        let base = base.clone();
                        move |_| {
                            let _ = document::eval(&format!("document.getElementById('{base}-{i}')?.select()"));
                        }
                    },
                    oninput: {
                        let mut apply = apply.clone();
                        move |e: FormEvent| {
                            apply(OtpEvent::Input { slot: i, text: e.value() });
                        }
                    },
                    onkeydown: {
                        let mut apply = apply.clone();
                        move |e: KeyboardEvent| {
                            let event = match e.key() {
                                Key::Backspace => OtpEvent::Backspace(i),
                                Key::ArrowLeft => OtpEvent::Left(i),
                                Key::ArrowRight => OtpEvent::Right(i),
                                Key::Home => OtpEvent::Home,
                                Key::End => OtpEvent::End,
                                _ => return,
                            };
                            if apply(event) != OtpOutcome::Ignored {
                                e.prevent_default();
                            }
                        }
                    },
                }
                }
            }
        }
    }
}
