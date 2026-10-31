//! An app outside this workspace that uses the kit the way someone else's app would: from the
//! packaged crates (what crates.io would serve), on stock Dioxus from crates.io, with no
//! `[patch]` of Dioxus or Blitz. `scripts/consumer-check.sh` builds it; see docs/PUBLISHING.md.

use dioxus::prelude::*;
use ui::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut on = use_signal(|| false);
    rsx! {
        ThemeStyles {}
        main {
            Badge { "consumer" }
            Switch { checked: on(), aria_label: "On", on_checked_change: move |v| on.set(v) }
            Button { variant: ButtonVariant::Outline, onclick: move |_| on.toggle(), "Toggle" }
        }
    }
}
