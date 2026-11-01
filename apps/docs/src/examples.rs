//! Example apps (`#examples`): the kit's own test apps, offered as live demos and as
//! templates (`atoll create my-app --template <name>`). They already run in CI, in three
//! browsers with axe, so they are the most honest picture of what the kit builds.
//!
//! The template names must match the CLI's list (`crates/cli/src/template.rs`); the CLI test
//! `the_docs_list_every_template` checks this page against it.

use crate::landing::REPO_URL;
use dioxus::prelude::*;
use ui::prelude::*;

pub struct Example {
    /// `--template` name.
    pub template: &'static str,
    /// The app's name in its own header.
    pub title: &'static str,
    /// Folder under `apps/`; also its path on the published site (`examples/<app>/`).
    pub app: &'static str,
    pub what: &'static str,
    /// Kit pieces it leans on, so a reader can find a real use of each.
    pub uses: &'static [&'static str],
}

pub const EXAMPLES: &[Example] = &[
    Example {
        template: "dashboard",
        title: "Acme Admin",
        app: "dashboard",
        what: "An admin dashboard: sidebar shell, stat cards and charts, a customers table with row actions, an edit sheet and a delete confirmation, an orders table with date range and status filters, a settings form, and Ctrl+K to jump anywhere.",
        uses: &["sidebar", "chart", "data_table", "sheet", "alert_dialog", "form", "command"],
    },
    Example {
        template: "booking",
        title: "Trailhead Stays",
        app: "booking",
        what: "A routed booking app with long forms: search with filters, sort and paging; a stay page with a carousel, tabs and a date range booking box; a three-step checkout with validation; and a trips list with cancel confirmation.",
        uses: &["carousel", "tabs", "accordion", "date_picker", "stepper", "form", "radio_group"],
    },
    Example {
        template: "marketing",
        title: "Orbit",
        app: "site",
        what: "A marketing site: header, hero, features, pricing tabs and a comparison table, an FAQ accordion, a signup form with toasts, a command palette, and spring and layout motion that switches off under reduced motion.",
        uses: &["card", "tabs", "table", "accordion", "form", "toast", "kbd", "motion"],
    },
    Example {
        template: "showcase",
        title: "Atoll Showcase",
        app: "showcase",
        what: "One codebase for the web, the desktop and phones: the same pages in a browser, in a desktop webview, or drawn by the native Blitz renderer with no webview. Wide windows get a side rail, phones a bottom tab bar.",
        uses: &["bottom_nav", "dialog", "sheet", "form", "switch", "theme_customizer"],
    },
];

#[component]
pub fn ExamplesPage() -> Element {
    rsx! {
        document::Title { "Example apps - Atoll" }
        h1 { id: "page-title", tabindex: "-1", "Example apps" }
        p { class: "docs-lede",
            "Four complete apps built only from Atoll. Each one runs in CI in Chromium, Firefox and WebKit with accessibility checks, and each one is a template: start your app from it with one command and change anything."
        }
        pre { class: "landing-code", tabindex: "0",
            code { "cargo install --git {REPO_URL} --branch main atoll-cli\natoll create my-app --git --template dashboard\ncd my-app && dx serve" }
        }
        div { class: "examples-list",
            for ex in EXAMPLES {
                Card { key: "{ex.template}",
                    CardHeader {
                        CardTitle { h2 { class: "examples-title", "{ex.title}" } }
                        CardDescription { "Template: " code { "{ex.template}" } }
                    }
                    CardContent {
                        p { class: "landing-card-text", "{ex.what}" }
                        p { class: "examples-uses",
                            "Uses: "
                            for (i , name) in ex.uses.iter().enumerate() {
                                if i > 0 { ", " }
                                if *name == "motion" {
                                    span { key: "{name}", "motion" }
                                } else {
                                    a { key: "{name}", href: "#{name}", "{name}" }
                                }
                            }
                        }
                        pre { class: "landing-code", tabindex: "0",
                            code { "atoll create my-app --git --template {ex.template}" }
                        }
                        div { class: "landing-actions",
                            a {
                                href: "examples/{ex.app}/",
                                ..button_look(ButtonVariant::Primary, ButtonSize::Default),
                                "Open the live app"
                                span { class: "sr-only", ": {ex.title}" }
                            }
                            a {
                                href: "{REPO_URL}/tree/main/apps/{ex.app}",
                                ..button_look(ButtonVariant::Outline, ButtonSize::Default),
                                "Read the source"
                                span { class: "sr-only", ": {ex.title}" }
                            }
                        }
                    }
                }
            }
        }
        p { class: "docs-lede",
            "The live apps are published with this site. Running the site locally? Start one with "
            code { "dx serve" }
            " in its folder under "
            code { "apps/" }
            "."
        }
    }
}
