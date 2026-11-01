//! The home page (`#`): what the kit is, why it's different, honest trade-offs, and the two
//! launch offers (Founding licence, design partners). Built from kit components only, like
//! every page here, so the landing page is itself a demo of the kit.
//!
//! Every claim on this page must be true of `main` and has a row in `docs/CLAIMS.md`. The
//! numbers come from `bench/README.md` and the generated component list;
//! `crates/cli/tests/claims.rs` checks both.

use crate::generated::{demo, ENTRIES};
use crate::pricing::{FOUNDING_ENDS, PRODUCT};
use dioxus::prelude::*;
use ui::prelude::*;

/// The public repository. GitHub redirects the old URL if the repository is renamed.
pub const REPO_URL: &str = "https://github.com/Amit-1of1/Rust-UI-";

/// Where a design partner applies: a prefilled issue, so no account set-up is needed.
const PARTNER_URL: &str = "https://github.com/Amit-1of1/Rust-UI-/issues/new?title=Design%20partner%3A%20%3Cyour%20app%3E&body=App%20and%20link%3A%0AWhat%20you%20want%20to%20build%20first%3A%0ADioxus%20version%20and%20platforms%3A%0A";

/// Registry items that are support code (hooks, the icon set, translated strings), not
/// components. `crates/cli/tests/claims.rs` keeps this list in step with the registry.
const UTILITIES: &[&str] = &["hooks", "icons", "ui_strings"];

/// The reasons to pick this kit, each one checkable in the repository.
const FEATURES: &[(&str, &str)] = &[
    (
        "Keyboard-tested, not just keyboard-friendly",
        "Every demo is scanned with axe and walked with Tab alone in Chromium, Firefox and WebKit on every change. The menus, dialogs, pickers and other widgets ship a keyboard contract (what Tab, Escape and the arrows do) whose every row runs as a test. A dialog without a title or an icon button without a label doesn't compile.",
    ),
    (
        "Motion that respects people",
        "Springs, enter and exit, layout (FLIP) and drag gestures. All of it switches off under prefers-reduced-motion, in the same change that adds it.",
    ),
    (
        "A look of its own, and four more",
        "Lagoon teal, sea-glass neutrals, Bricolage Grotesque and Figtree by default. Reef, Abyss, Driftwood and Classic change colours, fonts, corners and density with one attribute, at build time or live. Plain CSS, fonts shipped with the kit, no Tailwind build step.",
    ),
    (
        "You own the code",
        "Copy a component into your app with one command and change anything, the way shadcn/ui works. Or depend on the crate and update with a diff.",
    ),
    (
        "Web and desktop from one codebase",
        "The same components run on Dioxus web and desktop. The native renderer (Blitz) path is tested headless too, in preview.",
    ),
    (
        "Made for coding agents",
        "An MCP server, llms.txt and an agent skill, so Claude Code, Cursor and others use the real API instead of guessing it.",
    ),
];

/// What is not great yet. Saying it up front saves everyone time.
const TRADE_OFFS: &[(&str, &str)] = &[
    (
        "Download size",
        "Wasm is bigger than JavaScript: the benchmark app's first screen is 306 KB (brotli, split wasm) against 101 KB for the same app in React, so about 1.3 s more on slow 4G. Startup after download is a tie.",
    ),
    (
        "Native renderer",
        "Blitz support is preview quality; some keyboard rows in the contracts are marked \"not yet\" on native.",
    ),
    (
        "One framework",
        "Dioxus 0.7 only. No Leptos or Yew port is planned.",
    ),
];

#[component]
pub fn LandingPage() -> Element {
    let count = ENTRIES
        .iter()
        .filter(|e| !UTILITIES.contains(&e.name))
        .count();
    let contracts = ENTRIES.iter().filter(|e| !e.contract.is_empty()).count();
    rsx! {
        document::Title { "{PRODUCT}: accessible, animated components for Dioxus" }
        section { class: "landing-hero", "aria-labelledby": "page-title",
            Badge { "For Dioxus 0.7 · web and desktop" }
            div { class: "landing-title",
                crate::AtollMark { size: 64 }
                h1 { id: "page-title", tabindex: "-1", {PRODUCT} }
            }
            p { class: "docs-lede landing-tagline",
                "Accessible, animated components for Rust apps. Copy them into your app with one command; every one of them is checked with axe and the keyboard in three browsers."
            }
            div { class: "landing-actions",
                a { href: "#start", ..button_look(ButtonVariant::Primary, ButtonSize::Default), "Get started" }
                a { href: "#pricing", ..button_look(ButtonVariant::Outline, ButtonSize::Default), "Pricing" }
                a { href: REPO_URL, ..button_look(ButtonVariant::Ghost, ButtonSize::Default), "Source on GitHub" }
            }
            pre { class: "landing-code", tabindex: "0",
                code { "atoll init\natoll add dialog command data_table" }
            }
        }

        ul { class: "landing-stats", "aria-label": "At a glance",
            li { strong { "{count}" } span { "components, plus page blocks" } }
            li { strong { "{contracts}" } span { "keyboard contracts, every key tested" } }
            li { strong { "3" } span { "browsers checked with axe in CI" } }
            li { strong { "0" } span { "Tailwind build steps" } }
            li { strong { "Free" } span { "kit, MIT or Apache-2.0" } }
        }

        section { class: "landing-looks", "aria-labelledby": "looks-heading",
            h2 { id: "looks-heading", "Five looks, one attribute" }
            p {
                "Pick a look and this whole site changes: colours, fonts, corners, density and icon stroke. Each one passes the same contrast proof and axe scan as the default. "
                a { href: "#brand", "How the looks are made" }
                "."
            }
            div { class: "landing-looks-grid",
                PresetPicker { label: "Look for this site" }
                Card { class: "landing-looks-preview",
                    CardHeader {
                        CardTitle { "Reef check" }
                        CardDescription { "Survey notes for the north channel" }
                    }
                    CardContent {
                        div { class: "landing-looks-row",
                            Badge { "Live" }
                            Badge { variant: BadgeVariant::Secondary, "12 divers" }
                            Badge { variant: BadgeVariant::Outline, "Low tide 06:40" }
                        }
                        div { class: "landing-looks-row",
                            Switch { aria_label: "Share with the team", default_checked: true }
                            span { "Share with the team" }
                        }
                        Progress { value: 64.0, aria_label: "Survey progress" }
                    }
                    CardFooter { class: "landing-looks-row",
                        Button { "Save survey" }
                        Button { variant: ButtonVariant::Outline, "Export" }
                    }
                }
            }
            pre { class: "landing-code", tabindex: "0",
                code { "// at build time\nThemeStyles {{ preset: \"reef\" }}\n\n// or live\nuse_theme().set_preset(\"abyss\");" }
            }
        }

        section { "aria-labelledby": "try-heading",
            h2 { id: "try-heading", "Try it here" }
            p { "This page is built with the kit. Open the dialog, then press Escape: focus goes back to the button." }
            div { class: "docs-preview", {demo("dialog")} }
        }

        section { "aria-labelledby": "why-heading",
            h2 { id: "why-heading", "Why {PRODUCT}" }
            div { class: "landing-grid",
                for (title , body) in FEATURES {
                    Card { key: "{title}",
                        CardHeader {
                            CardTitle { {*title} }
                        }
                        CardContent {
                            p { class: "landing-card-text", {*body} }
                        }
                    }
                }
            }
        }

        section { "aria-labelledby": "tradeoffs-heading",
            h2 { id: "tradeoffs-heading", "What isn't great yet" }
            dl { class: "landing-tradeoffs",
                for (title , body) in TRADE_OFFS {
                    div { key: "{title}",
                        dt { {*title} }
                        dd { {*body} }
                    }
                }
            }
        }

        section { "aria-labelledby": "founding-heading",
            h2 { id: "founding-heading", "Founding licence" }
            Callout { variant: CalloutVariant::Info, title: "Pro pack, founding price",
                "Pro adds marketing and app blocks, a SaaS starter app and extra motion components. Founding licences cost $79 for one developer or $249 for a team, once, with every Pro item that ever ships. The offer ends on {FOUNDING_ENDS} or after 100 licences."
            }
            p {
                a { href: "#pricing", "See what Pro includes and the refund promise" }
                "."
            }
        }

        section { "aria-labelledby": "partner-heading",
            h2 { id: "partner-heading", "Design partners" }
            p {
                "Shipping a real Dioxus app? Five teams get Pro free for life in return for honest feedback after trying {PRODUCT} on one screen, and a short quote if they like it. Their top missing components get built first."
            }
            div { class: "landing-actions",
                a { href: PARTNER_URL, ..button_look(ButtonVariant::Outline, ButtonSize::Default), "Apply as a design partner" }
            }
        }
    }
}
