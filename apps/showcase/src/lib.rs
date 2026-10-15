//! Rust UI Showcase: one codebase for the web, the desktop and phones.
//!
//! The same components, state and CSS run on every Dioxus renderer: the browser
//! (WebAssembly), native (Blitz draws the page itself with wgpu, no webview and no
//! JavaScript; desktop and mobile) and the desktop webview. Nothing here is behind a
//! platform `cfg` except the label that says which renderer is running and the ready marker
//! for the browser checks. Wide windows get a side rail; at phone width (640px and below) the
//! same pages get a bottom tab bar, chosen by a CSS media query so it works everywhere.
//!
//! `apps/native/tests/showcase.rs` drives this app headless on Blitz at desktop and phone
//! sizes; `tests/e2e/showcase.mjs` does the same in browsers. README.md has the per-target
//! status.

mod forms;
mod home;
mod overlays;
mod settings;

use dioxus::prelude::*;
use ui::icons::{House, Layers, Settings as SettingsIcon, SlidersHorizontal};
use ui::prelude::*;

pub const SHOWCASE_CSS: Asset = asset!("/assets/showcase.css");

/// The renderer this build draws with.
pub fn renderer() -> &'static str {
    if cfg!(feature = "native") {
        "Native (Blitz)"
    } else if cfg!(feature = "desktop") {
        "Desktop webview"
    } else if cfg!(target_arch = "wasm32") {
        "Web (WebAssembly)"
    } else {
        "Server render"
    }
}

/// The operating system, or "browser" on the web.
pub fn platform() -> &'static str {
    match std::env::consts::OS {
        "" | "unknown" => "browser",
        "macos" => "macOS",
        "ios" => "iOS",
        "linux" => "Linux",
        "windows" => "Windows",
        "android" => "Android",
        other => other,
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Page {
    Home,
    Forms,
    Overlays,
    Settings,
}

impl Page {
    pub const ALL: [Page; 4] = [Page::Home, Page::Forms, Page::Overlays, Page::Settings];

    pub fn title(self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Forms => "Forms",
            Page::Overlays => "Overlays",
            Page::Settings => "Settings",
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Page::Home => "home",
            Page::Forms => "forms",
            Page::Overlays => "overlays",
            Page::Settings => "settings",
        }
    }
}

/// `document.body.dataset.ready = "1"` once mounted: the browser checks wait for it.
fn mark_ready() {
    #[cfg(target_arch = "wasm32")]
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        let _ = body.dataset().set("ready", "1");
    }
}

#[component]
pub fn App() -> Element {
    use_effect(mark_ready);
    rsx! {
        ThemeStyles {}
        document::Stylesheet { href: SHOWCASE_CSS }
        // Native has no JavaScript to set the theme on <html>; ThemeScope carries it there.
        // `.sc-root` turns it and ToastProvider from `display: contents` into plain blocks
        // (showcase.css explains why).
        div { class: "sc-root",
            ThemeScope {
                ToastProvider { Shell {} }
            }
        }
    }
}

#[component]
fn Shell() -> Element {
    let mut page = use_signal(|| Page::Home);
    let theme = use_theme();
    let current = page();

    rsx! {
        div { class: "sc-app",
            header { class: "sc-header",
                div { class: "sc-brand",
                    span { class: "sc-logo", "aria-hidden": "true", "R" }
                    span { class: "sc-brand-name", "Rust UI" }
                }
                div { class: "sc-header-tools",
                    Badge { variant: BadgeVariant::Secondary, class: "sc-renderer", {renderer()} }
                    div { class: "sc-theme",
                        Switch {
                            checked: theme.is_dark(),
                            aria_label: "Dark mode",
                            on_checked_change: move |dark: bool| {
                                theme.set_mode(if dark { ThemeMode::Dark } else { ThemeMode::Light })
                            },
                        }
                    }
                }
            }
            div { class: "sc-body",
                nav { class: "sc-rail", aria_label: "Pages",
                    for p in Page::ALL {
                        Button {
                            key: "{p.id()}",
                            class: "sc-rail-link",
                            id: "rail-{p.id()}",
                            variant: if p == current { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                            "aria-current": (p == current).then_some("page"),
                            onclick: move |_| page.set(p),
                            PageIcon { page: p }
                            span { {p.title()} }
                        }
                    }
                }
                main { class: "sc-main", id: "main-content",
                    h1 { class: "sc-title", {current.title()} }
                    match current {
                        Page::Home => rsx! { home::Home {} },
                        Page::Forms => rsx! { forms::Forms {} },
                        Page::Overlays => rsx! { overlays::Overlays {} },
                        Page::Settings => rsx! { settings::Settings {} },
                    }
                }
            }
            div { class: "sc-bottom",
                BottomNav { label: "Pages",
                    for p in Page::ALL {
                        BottomNavItem {
                            key: "{p.id()}",
                            label: p.title(),
                            current: p == current,
                            onclick: move |_| page.set(p),
                            icon: rsx! { PageIcon { page: p } },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PageIcon(page: Page) -> Element {
    match page {
        Page::Home => rsx! { House {} },
        Page::Forms => rsx! { SlidersHorizontal {} },
        Page::Overlays => rsx! { Layers {} },
        Page::Settings => rsx! { SettingsIcon {} },
    }
}
