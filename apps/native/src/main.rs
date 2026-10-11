//! Renders one demo (`DEMO=<name>`, default `button`) in a Blitz window, with the
//! theme. `scripts/native-smoke.sh` runs it under Xvfb for every demo and checks
//! that each one starts, draws and picks up its styles.

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
    rsx! {
        ThemeStyles {}
        style { "{extra}" }
        main { style: "padding: 24px; max-width: 760px;", {generated::demo(&name)} }
    }
}
