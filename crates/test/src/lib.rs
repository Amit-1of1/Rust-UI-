//! Test Atoll (and any Dioxus) screens inside `cargo test`: render them headless, click,
//! type, press keys, let timers run, and assert on what a user would see. No browser,
//! window, GPU or JavaScript.
//!
//! The screen is drawn by Blitz, the engine behind `dioxus-native`, with the kit's real CSS
//! (hashed exactly as `#[css_module]` hashes it), so layout, `display: none` and hit testing
//! are real: a click lands on whatever is drawn at that point, the way a user's would.
//!
//! ```no_run
//! use atoll_test::prelude::*;
//! use dioxus::prelude::*;
//! use ui::prelude::*;
//!
//! fn greeter() -> Element {
//!     let mut name = use_signal(String::new);
//!     rsx! {
//!         Label { html_for: "name", "Your name" }
//!         Input { id: "name", value: name, oninput: move |e: FormEvent| name.set(e.value()) }
//!         p { "Hello, {name}" }
//!     }
//! }
//!
//! #[test]
//! fn greets_by_name() {
//!     let mut screen = Screen::render(greeter);
//!     screen.type_into(label("Your name"), "Ada");
//!     screen.assert_visible(text("Hello, Ada"));
//! }
//! ```
//!
//! Setup, the query rules and the limits are in `docs/TESTING.md`.
//!
//! # Stability
//!
//! Experimental (docs/STABILITY.md): new on 2026-11-01; method names may still change.

mod css;
mod keys;
mod query;
mod screen;

pub use css::{css_module, css_modules_in, kit_css, kit_dir};
pub use keys::parse as parse_key;
pub use query::{css, label, placeholder, role, test_id, text, text_containing, Query};
pub use screen::{El, Rect, Screen, ScreenBuilder, HEIGHT, WIDTH};

/// `use atoll_test::prelude::*;` brings in [`Screen`] and the query functions.
pub mod prelude {
    pub use crate::query::{css, label, placeholder, role, test_id, text, text_containing, Query};
    pub use crate::screen::{El, Screen};
    pub use std::time::Duration;
}
