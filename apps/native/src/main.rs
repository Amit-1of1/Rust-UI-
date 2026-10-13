//! Renders one demo (`DEMO=<name>`, default `button`) in a Blitz window, with the
//! theme. `scripts/native-smoke.sh` runs it under Xvfb for every demo and checks
//! that each one starts, draws and picks up its styles. `tests/interactions.rs` drives the
//! same demos headless (clicks, layout, pixels) without a window.

use dioxus::prelude::*;
use ui::prelude::*;

// The demo list is generated for the docs site; reuse it rather than keep a second one.
#[path = "../../docs/src/generated.rs"]
#[rustfmt::skip]
#[allow(dead_code)]
mod generated;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let name = std::env::var("DEMO").unwrap_or_else(|_| "button".into());
    // Optional CSS file to try overrides without rebuilding: EXTRA_CSS=/path/to.css
    let extra = std::env::var("EXTRA_CSS")
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .unwrap_or_default();
    // THEME=dark|light forces a theme; otherwise the window's colour scheme decides.
    let theme = std::env::var("THEME").unwrap_or_default();
    rsx! {
        ThemeStyles {}
        style { "{extra}" }
        // The padding sits on an outer box on purpose: Blitz 0.2 hit-tests the inline children
        // of a padded block (a button beside text) off by the padding, so clicks miss them.
        main { style: "padding: 24px; max-width: 760px;",
            div { "data-theme": (!theme.is_empty()).then_some(theme), {generated::demo(&name)} }
        }
    }
}
