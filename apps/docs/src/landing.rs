//! The home page (`#`): what the kit is, why it's different, honest trade-offs, and the two
//! launch offers (Founding licence, design partners). Built from kit components only, like
//! every page here, so the landing page is itself a demo of the kit.
//!
//! Every claim on this page must be true of `main`; the numbers come from `bench/README.md`
//! and the generated component list.

use crate::generated::{demo, ENTRIES};
use crate::pricing::{FOUNDING_ENDS, PRODUCT};
use dioxus::prelude::*;
use ui::prelude::*;

/// The public repository. GitHub redirects the old URL if the repository is renamed.
pub const REPO_URL: &str = "https://github.com/Amit-1of1/Rust-UI-";

/// Where a design partner applies: a prefilled issue, so no account set-up is needed.
const PARTNER_URL: &str = "https://github.com/Amit-1of1/Rust-UI-/issues/new?title=Design%20partner%3A%20%3Cyour%20app%3E&body=App%20and%20link%3A%0AWhat%20you%20want%20to%20build%20first%3A%0ADioxus%20version%20and%20platforms%3A%0A";

/// The reasons to pick this kit, each one checkable in the repository.
const FEATURES: &[(&str, &str)] = &[
    (
        "Keyboard-tested, not just keyboard-friendly",
        "Every component ships a keyboard contract (what Tab, Escape and the arrows do) and CI runs it, with axe checks in Chromium, Firefox and WebKit. A dialog without a title or an icon button without a label doesn't compile.",
    ),
    (
        "Motion that respects people",
        "Springs, enter and exit, layout (FLIP) and drag gestures. All of it switches off under prefers-reduced-motion, in the same change that adds it.",
    ),
    (
        "Plain CSS themes",
        "Light and dark tokens, accent presets and a theme from your logo. No Tailwind build step in your CI.",
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
        "Wasm is bigger than JavaScript: the benchmark app's first screen is 287 KB (brotli, split wasm) against 103 KB for the same app in React. Startup after download is a tie.",
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
    let count = ENTRIES.len();
    rsx! {
        document::Title { "{PRODUCT}: accessible, animated components for Dioxus" }
        section { class: "landing-hero", "aria-labelledby": "page-title",
            Badge { "For Dioxus 0.7 · web and desktop" }
            h1 { id: "page-title", tabindex: "-1", {PRODUCT} }
            p { class: "docs-lede landing-tagline",
                "Accessible, animated components for Rust apps. Copy them into your app with one command; every one of them is keyboard-tested in three browsers."
            }
            div { class: "landing-actions",
                a { href: "#install", ..button_look(ButtonVariant::Primary, ButtonSize::Default), "Get started" }
                a { href: "#pricing", ..button_look(ButtonVariant::Outline, ButtonSize::Default), "Pricing" }
                a { href: REPO_URL, ..button_look(ButtonVariant::Ghost, ButtonSize::Default), "Source on GitHub" }
            }
            pre { class: "landing-code", tabindex: "0",
                code { "atoll init\natoll add dialog command data_table" }
            }
        }

        ul { class: "landing-stats", "aria-label": "At a glance",
            li { strong { "{count}" } span { "components, plus page blocks" } }
            li { strong { "3" } span { "browsers checked with axe in CI" } }
            li { strong { "0" } span { "Tailwind build steps" } }
            li { strong { "MIT / Apache-2.0" } span { "free kit, forever" } }
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
            a { href: PARTNER_URL, ..button_look(ButtonVariant::Outline, ButtonSize::Default), "Apply as a design partner" }
        }
    }
}
