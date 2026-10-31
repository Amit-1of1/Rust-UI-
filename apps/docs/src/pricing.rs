//! The Pricing page (`#pricing`): the free kit and the Pro pack, as planned in
//! `docs/pro/README.md`. Built from kit components only, like every page here.
//!
//! Nothing is sold until `CHECKOUT_URL` is set: until then the Pro buttons are disabled and
//! the page says so.

use dioxus::prelude::*;
use ui::prelude::*;

/// The product's name on this page. A working name: change it here when the final one is
/// picked (docs/pro/README.md, "Before launch").
pub const PRODUCT: &str = "Atoll";

/// The checkout page (Polar) for each Pro tier. Empty until the owner creates the products;
/// an empty URL keeps that tier's button disabled.
const CHECKOUT_URL_PERSONAL: &str = "";
const CHECKOUT_URL_TEAM: &str = "";

const LICENSE_URL: &str = "https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/pro/LICENSE-PRO.md";

struct Plan {
    id: &'static str,
    name: &'static str,
    price: &'static str,
    per: &'static str,
    blurb: &'static str,
    features: &'static [&'static str],
    checkout: Option<&'static str>,
    featured: bool,
}

const PLANS: &[Plan] = &[
    Plan {
        id: "free",
        name: "Free",
        price: "$0",
        per: "forever",
        blurb: "The whole kit, open source.",
        features: &[
            "Every component and block in these docs",
            "Light and dark themes, accent presets, motion",
            "The atoll CLI and MCP server",
            "MIT OR Apache-2.0",
        ],
        checkout: None,
        featured: false,
    },
    Plan {
        id: "personal",
        name: "Pro Personal",
        price: "$129",
        per: "one time, for one developer",
        blurb: "Everything in Pro, with lifetime updates.",
        features: &[
            "Marketing and app blocks",
            "A SaaS starter app (web and desktop)",
            "Pro motion components",
            "Unlimited personal and client projects",
        ],
        checkout: Some(CHECKOUT_URL_PERSONAL),
        featured: true,
    },
    Plan {
        id: "team",
        name: "Pro Team",
        price: "$399",
        per: "one time, up to 10 developers",
        blurb: "Pro for a whole team or agency.",
        features: &[
            "Everything in Pro Personal",
            "Up to 10 developers on one licence",
            "Access managed through GitHub",
            "Priority answers in Pro discussions",
        ],
        checkout: Some(CHECKOUT_URL_TEAM),
        featured: false,
    },
];

const COMPARE: &[(&str, &str, &str)] = &[
    ("Components", "All", "All"),
    (
        "Blocks (whole screens)",
        "Login, sign-up, settings, dashboard",
        "Plus marketing and app blocks",
    ),
    (
        "App templates",
        "Example apps in the repository",
        "SaaS starter, kept up to date",
    ),
    (
        "Motion",
        "Motion layer and presets",
        "Plus Pro motion components",
    ),
    (
        "Install",
        "atoll add, dx components add",
        "The same, from the Pro repository",
    ),
    ("Licence", "MIT OR Apache-2.0", "Commercial, per developer"),
];

const FAQ: &[(&str, &str)] = &[
    (
        "Does the free kit lose anything?",
        "No. Everything free today stays free and open source. Pro only adds new blocks, templates and motion components.",
    ),
    (
        "How do I get Pro after paying?",
        "Your GitHub account is invited to the private Pro repository. Install from it with the same commands: atoll add --source <Pro repository URL> <name>.",
    ),
    (
        "Can I use Pro in client work?",
        "Yes, in as many end products as you like. You can't resell or share the Pro source itself as a kit, template or library.",
    ),
    (
        "What happens when I ask for a refund?",
        "Within 30 days, as long as you haven't cloned the Pro repository, you get your money back and the invitation is removed.",
    ),
];

#[component]
pub fn PricingPage() -> Element {
    let selling = PLANS
        .iter()
        .any(|p| p.checkout.is_some_and(|u| !u.is_empty()));
    rsx! {
        document::Title { "Pricing - {PRODUCT}" }
        h1 { id: "page-title", tabindex: "-1", "Pricing" }
        p { class: "docs-lede",
            "The kit is free and open source. Pro adds premium blocks, a starter app and motion components, for a one-time price."
        }
        if !selling {
            Callout { variant: CalloutVariant::Info, title: "Pro is not on sale yet",
                "The prices below are planned. The buttons turn on when checkout opens."
            }
        }
        div { class: "pricing-grid",
            for plan in PLANS {
                PlanCard { key: "{plan.id}", plan_id: plan.id }
            }
        }
        section { "aria-labelledby": "compare-heading",
            h2 { id: "compare-heading", "Free and Pro side by side" }
            Table {
                TableCaption { "What each tier includes" }
                TableHeader {
                    TableRow {
                        TableHead { "Feature" }
                        TableHead { "Free" }
                        TableHead { "Pro" }
                    }
                }
                TableBody {
                    for (feature , free , pro) in COMPARE {
                        TableRow { key: "{feature}",
                            TableCell { {*feature} }
                            TableCell { {*free} }
                            TableCell { {*pro} }
                        }
                    }
                }
            }
        }
        section { class: "pricing-faq", "aria-labelledby": "faq-heading",
            h2 { id: "faq-heading", "Questions" }
            for (q , a) in FAQ {
                h3 { key: "{q}", {*q} }
                p { {*a} }
            }
            p {
                "Read the full "
                a { href: LICENSE_URL, "Pro licence" }
                "."
            }
        }
    }
}

#[component]
fn PlanCard(plan_id: &'static str) -> Element {
    let Some(plan) = PLANS.iter().find(|p| p.id == plan_id) else {
        return rsx! {};
    };
    let variant = if plan.featured {
        ButtonVariant::Primary
    } else {
        ButtonVariant::Outline
    };
    rsx! {
        Card { "data-plan": plan.id,
            CardHeader {
                div { class: "pricing-card-head",
                    CardTitle { {plan.name} }
                    if plan.featured { Badge { "Most popular" } }
                }
                CardDescription { {plan.blurb} }
            }
            CardContent {
                p { class: "pricing-price",
                    span { class: "pricing-amount", {plan.price} }
                    " "
                    span { class: "pricing-per", {plan.per} }
                }
                ul { class: "pricing-list",
                    for f in plan.features {
                        li { key: "{f}", {*f} }
                    }
                }
            }
            CardFooter {
                match plan.checkout {
                    None => rsx! {
                        a { href: "#install", ..button_look(ButtonVariant::Outline, ButtonSize::Default), "Install the free kit" }
                    },
                    Some(url) if !url.is_empty() => rsx! {
                        a { href: url, ..button_look(variant, ButtonSize::Default), "Buy {plan.name}" }
                    },
                    Some(_) => rsx! {
                        Button { variant, disabled: true, "Available at launch" }
                    },
                }
            }
        }
    }
}
