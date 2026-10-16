use crate::{platform, renderer};
use dioxus::prelude::*;
use ui::prelude::*;

fn visitors() -> Vec<ChartDatum> {
    [
        ("Jan", 186.0),
        ("Feb", 305.0),
        ("Mar", 237.0),
        ("Apr", 173.0),
        ("May", 209.0),
        ("Jun", 264.0),
    ]
    .into_iter()
    .map(|(label, value)| ChartDatum::new(label, value))
    .collect()
}

#[component]
pub fn Home() -> Element {
    let mut progress = use_signal(|| 40_u32);
    rsx! {
        Card { class: "sc-hero",
            CardHeader {
                CardTitle { "One codebase, every screen" }
                CardDescription {
                    "These pages are the same Rust code on the web, on the desktop and on phones. "
                    "Native builds draw them with Blitz, without a webview."
                }
            }
            CardContent {
                div { class: "sc-chips",
                    Badge { id: "renderer", {renderer()} }
                    Badge { variant: BadgeVariant::Outline, id: "platform", {platform()} }
                }
            }
        }
        div { class: "sc-stats",
            for (label , value , note) in [
                ("Components", "70+", "from one kit"),
                ("Targets", "3", "web, desktop, mobile"),
                ("JavaScript", "0 lines", "in this app"),
            ]
            {
                Card { key: "{label}", class: "sc-stat",
                    CardHeader {
                        CardDescription { {label} }
                        CardTitle { class: "sc-stat-value", {value} }
                    }
                    CardContent {
                        span { class: "sc-muted", {note} }
                    }
                }
            }
        }
        div { class: "sc-grid",
            Card {
                CardHeader {
                    CardTitle { "Visitors" }
                    CardDescription { "January to June" }
                }
                CardContent {
                    BarChart { data: visitors(), label: "Visitors per month, January to June", height: 180.0 }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Setup" }
                    CardDescription { "Driven by buttons, so it works without a JavaScript timer." }
                }
                CardContent {
                    div { class: "sc-stack",
                        Progress { aria_label: "Setup progress", value: progress() as f64 }
                        span { class: "sc-muted", id: "progress-value", "{progress}% done" }
                        div { class: "sc-row",
                            Button {
                                id: "progress-add",
                                variant: ButtonVariant::Outline,
                                onclick: move |_| progress.set((progress() + 20).min(100)),
                                "Add 20%"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| progress.set(0),
                                "Reset"
                            }
                        }
                    }
                }
            }
        }
    }
}
