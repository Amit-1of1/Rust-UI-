use crate::{platform, renderer};
use dioxus::prelude::*;
use ui::prelude::*;

/// How each target is built. The same crate, a different Cargo feature.
const TARGETS: [(&str, &str, &str); 4] = [
    ("Web", "web", "dx serve --platform web"),
    (
        "Desktop, native",
        "native",
        "dx serve --platform linux --renderer native",
    ),
    (
        "Android, native",
        "native",
        "dx build --platform android --renderer native",
    ),
    ("Desktop, webview", "desktop", "dx serve --platform desktop"),
];

#[component]
pub fn Settings() -> Element {
    rsx! {
        Tabs { default_value: "appearance".to_string(), horizontal: true,
            TabList {
                TabTrigger { value: "appearance".to_string(), index: 0usize, "Appearance" }
                TabTrigger { value: "about".to_string(), index: 1usize, "About this build" }
            }
            TabContent { index: 0usize, value: "appearance".to_string(),
                div { class: "sc-tab-body",
                    // Writes the theme to <html> on the web and to the app's ThemeScope
                    // everywhere, so it also works on native.
                    ThemeCustomizer {}
                }
            }
            TabContent { index: 1usize, value: "about".to_string(),
                div { class: "sc-tab-body sc-stack",
                    p { id: "about-build", "This build: {renderer()} on {platform()}." }
                    div { class: "sc-table",
                        Table {
                            TableCaption { "One crate, one Cargo feature per renderer." }
                            TableHeader {
                                TableRow {
                                    TableHead { "Target" }
                                    TableHead { "Feature" }
                                    TableHead { "Command (in apps/showcase)" }
                                }
                            }
                            TableBody {
                                for (target , feature , command) in TARGETS {
                                    TableRow { key: "{target}",
                                        TableCell { {target} }
                                        TableCell {
                                            code { {feature} }
                                        }
                                        TableCell {
                                            code { {command} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
