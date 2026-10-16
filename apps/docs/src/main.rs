//! Documentation site for the Rust UI kit: one page per component (live demo plus its
//! `docs.md`) and the install and theming guides. The page list and the demos come from
//! `generated.rs`, which `rust-ui registry` writes, so a component can't be left out.

#[rustfmt::skip] // generated, one entry per line
mod generated;

use dioxus::prelude::*;
use generated::{demo, Entry, ENTRIES};
use ui::prelude::*;

const DOCS_CSS: Asset = asset!("/assets/docs.css");

const GUIDES: &[(&str, &str, &str)] = &[
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
        document::Stylesheet { href: DOCS_CSS }
        a { class: "docs-skip", href: "#page-title", "Skip to content" }
        Fragment {
            header { class: "docs-header",
                a { class: "docs-brand", href: "#", "Rust UI" }
                span { class: "docs-tagline", "Accessible, animated components for Dioxus" }
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
    if let Some((_, title, source)) = GUIDES.iter().find(|(s, _, _)| *s == slug) {
        return rsx! {
            document::Title { "{title} - Rust UI" }
            h1 { id: "page-title", tabindex: "-1", "{title}" }
            div { class: "docs-prose", dangerous_inner_html: markdown(source) }
        };
    }
    let Some(entry) = ENTRIES.iter().find(|e| e.name == slug) else {
        return rsx! {
            document::Title { "Rust UI docs" }
            h1 { id: "page-title", tabindex: "-1", "Rust UI" }
            p { class: "docs-lede",
                "Components built on Dioxus with accessibility, a motion layer and plain-CSS theming. "
                "Pick a component on the left, or start with "
                a { href: "#install", "installing components" }
                " and "
                a { href: "#theming", "theming" }
                "."
            }
            p { "{ENTRIES.len()} components. Every page has a live demo where one exists." }
        };
    };
    rsx! {
        document::Title { "{entry.title} - Rust UI" }
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
