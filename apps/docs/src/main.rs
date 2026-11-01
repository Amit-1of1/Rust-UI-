//! Documentation site for the Atoll kit: one page per component (live demo plus its
//! `docs.md`) and the install and theming guides. The page list and the demos come from
//! `generated.rs`, which `atoll registry` writes, so a component can't be left out.

#[rustfmt::skip] // generated, one entry per line
mod generated;
mod examples;
mod landing;
mod pricing;
mod theme_builder;

use dioxus::prelude::*;
use generated::{demo, Entry, ENTRIES};
use ui::prelude::*;

const DOCS_CSS: Asset = asset!("/assets/docs.css");
const LANDING_CSS: Asset = asset!("/assets/landing.css");
const TOOLS_CSS: Asset = asset!("/assets/tools.css");

const GUIDES: &[(&str, &str, &str)] = &[
    (
        "start",
        "Getting started",
        include_str!("../../../docs/GETTING_STARTED.md"),
    ),
    (
        "install",
        "Installing components",
        include_str!("../../../docs/INSTALL.md"),
    ),
    (
        "theming",
        "Theming",
        include_str!("../../../docs/THEMING.md"),
    ),
    (
        "brand",
        "Brand and looks",
        include_str!("../../../docs/BRAND.md"),
    ),
    (
        "foundations",
        "Foundations",
        include_str!("../../../docs/FOUNDATIONS.md"),
    ),
    (
        "hard-questions",
        "Hard questions",
        include_str!("../../../docs/HARD_QUESTIONS.md"),
    ),
];

fn main() {
    dioxus::launch(App);
}

/// Markdown to HTML. The input is this repository's own docs, never user text.
fn markdown(source: &str) -> String {
    // The page supplies its own <h1>.
    let body = match source.strip_prefix("# ") {
        Some(rest) => rest.split_once('\n').map_or("", |(_, tail)| tail),
        None => source,
    };
    let mut html = String::new();
    // Tables: the keyboard table generated from each component's contract.toml.
    let parser = pulldown_cmark::Parser::new_ext(body, pulldown_cmark::Options::ENABLE_TABLES);
    pulldown_cmark::html::push_html(&mut html, parser);
    // Code blocks can scroll sideways, so keyboard users must be able to focus them.
    html.replace("<pre>", "<pre tabindex=\"0\">")
}

#[component]
fn App() -> Element {
    let mut page = use_signal(String::new);
    // Keep `page` in step with the URL hash, so links, back and forward all work.
    use_effect(move || {
        spawn(async move {
            let mut hash = document::eval(
                "const send = () => dioxus.send(location.hash.slice(1));
                 window.addEventListener('hashchange', send);
                 send();",
            );
            while let Ok(value) = hash.recv::<String>().await {
                page.set(value);
                // Land on the new page's heading so screen readers announce it.
                let _ = document::eval(
                    "window.scrollTo(0, 0);
                     setTimeout(() => document.getElementById('page-title')?.focus({ preventScroll: true }), 60)",
                );
            }
        });
    });
    let mut filter = use_signal(String::new);
    rsx! {
        ThemeStyles {}
        PersistTheme {}
        document::Stylesheet { href: DOCS_CSS }
        document::Stylesheet { href: LANDING_CSS }
        document::Stylesheet { href: TOOLS_CSS }
        a { class: "docs-skip", href: "#page-title", "Skip to content" }
        Fragment {
            header { class: "docs-header",
                a { class: "docs-brand", href: "#",
                    AtollMark { size: 28 }
                    "Atoll"
                }
                span { class: "docs-tagline", "Accessible, animated components for Dioxus" }
                LookSelect {}
            }
            div { class: "docs-layout",
                nav { class: "docs-nav", "aria-label": "Documentation",
                    Label { html_for: "docs-filter", "Filter components" }
                    Input {
                        id: "docs-filter",
                        r#type: "search",
                        placeholder: "Filter components",
                        value: filter,
                        oninput: move |e: FormEvent| filter.set(e.value()),
                    }
                    h2 { class: "docs-nav-heading", "Guides" }
                    ul {
                        for (slug , title , _) in GUIDES {
                            li { key: "{slug}", NavLink { slug, title: *title, current: page() } }
                        }
                        li { NavLink { slug: "theme-builder", title: "Theme builder", current: page() } }
                        li { NavLink { slug: "examples", title: "Example apps", current: page() } }
                        li { NavLink { slug: "pricing", title: "Pricing", current: page() } }
                    }
                    h2 { class: "docs-nav-heading", "Components" }
                    ul {
                        for entry in ENTRIES.iter().filter(|e| matches(e, &filter())) {
                            li { key: "{entry.name}",
                                NavLink { slug: entry.name, title: entry.title, current: page() }
                            }
                        }
                    }
                }
                main { class: "docs-main", Page { slug: page() } }
            }
        }
    }
}

fn matches(entry: &Entry, query: &str) -> bool {
    let q = query.trim().to_lowercase();
    q.is_empty() || entry.title.to_lowercase().contains(&q) || entry.name.contains(&q)
}

#[component]
fn NavLink(slug: &'static str, title: &'static str, current: String) -> Element {
    let active = current == slug;
    rsx! {
        a {
            href: "#{slug}",
            "aria-current": if active { "page" },
            {title}
        }
    }
}

#[component]
fn Page(slug: String) -> Element {
    match slug.as_str() {
        "pricing" => return rsx! { pricing::PricingPage {} },
        "theme-builder" => return rsx! { theme_builder::ThemeBuilderPage {} },
        "examples" => return rsx! { examples::ExamplesPage {} },
        _ => {}
    }
    if let Some((_, title, source)) = GUIDES.iter().find(|(s, _, _)| *s == slug) {
        return rsx! {
            document::Title { "{title} - Atoll" }
            h1 { id: "page-title", tabindex: "-1", "{title}" }
            div { class: "docs-prose", dangerous_inner_html: markdown(source) }
        };
    }
    let Some(entry) = ENTRIES.iter().find(|e| e.name == slug) else {
        return rsx! { landing::LandingPage {} };
    };
    rsx! {
        document::Title { "{entry.title} - Atoll" }
        h1 { id: "page-title", tabindex: "-1", "{entry.title}" }
        if entry.has_demo {
            section { "aria-labelledby": "preview-heading",
                h2 { id: "preview-heading", "Preview" }
                div { class: "docs-preview", {demo(entry.name)} }
            }
        }
        div { class: "docs-prose", dangerous_inner_html: markdown(entry.docs) }
        if !entry.contract.is_empty() {
            div { class: "docs-prose", dangerous_inner_html: markdown(entry.contract) }
        }
    }
}

/// Atoll's mark: a ring of reef broken by channels, the lagoon inside it and one island
/// (docs/brand/atoll-mark.svg). Drawn in the theme's accent, so every look recolours it.
#[component]
pub fn AtollMark(size: u32) -> Element {
    rsx! {
        svg {
            class: "atoll-mark",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 32 32",
            "aria-hidden": "true",
            circle { cx: "16", cy: "16", r: "8", style: "fill: var(--accent-color); opacity: 0.25;" }
            path {
                d: "M26.40 14.54A10.5 10.5 0 0 1 15.63 26.49 M9.25 24.04A10.5 10.5 0 0 1 6.56 11.40 M11.40 6.56A10.5 10.5 0 0 1 20.60 6.56",
                fill: "none",
                stroke_width: "4",
                stroke_linecap: "round",
                style: "stroke: var(--accent-color);",
            }
            circle { cx: "18.5", cy: "17.5", r: "2.75", fill: "#f97362" }
        }
    }
}

/// The header's look switcher: the theme presets, remembered between visits by
/// `PersistTheme` (and shared with the gallery when both are served from one place).
#[component]
fn LookSelect() -> Element {
    let theme = use_theme();
    rsx! {
        div { class: "docs-look",
            Label { html_for: "docs-look", "Look" }
            NativeSelect {
                id: "docs-look",
                value: theme.preset(),
                onchange: move |e: FormEvent| theme.set_preset(&e.value()),
                for info in PRESET_INFO {
                    NativeSelectOption { key: "{info.name}", value: info.name, "{info.name}" }
                }
            }
        }
    }
}
