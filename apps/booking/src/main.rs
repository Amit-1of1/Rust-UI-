//! Trailhead Stays: a small cabin booking app built only from the kit. Its job is to find what
//! the kit is missing for a content-heavy, routed app with long forms; everything found is
//! written down in `GAPS.md`.
//!
//! Routes: Explore (search, filters, sort, paging), a stay's page (carousel, tabs, accordion,
//! hover card, date range booking box), a three-step checkout (stepper, forms, radio group,
//! checkbox) and My trips (tabs, cancel confirmation). Ctrl+K searches stays.

mod checkout;
mod data;
mod explore;
mod stay;
mod trips;

use checkout::Checkout;
use data::{Stay, Trip};
use dioxus::prelude::*;
use explore::{Explore, Filters};
use stay::StayPage;
use trips::Trips;
use ui::icons::{Search, Tent};
use ui::prelude::*;

const BOOKING_CSS: Asset = asset!("/assets/booking.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Shell)]
        #[route("/")]
        Explore {},
        #[route("/stays/:id")]
        StayPage { id: usize },
        #[route("/checkout/:id")]
        Checkout { id: usize },
        #[route("/trips")]
        Trips {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

/// What the booking box hands to checkout.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Draft {
    pub stay: usize,
    pub range: Option<DateRange>,
    pub guests: u32,
}

/// App-wide state, shared through context.
#[derive(Clone, Copy)]
pub struct Store {
    pub stays: Signal<Vec<Stay>>,
    pub trips: Signal<Vec<Trip>>,
    /// Kept here, not in Explore, so filters survive a visit to a stay and back.
    pub filters: Signal<Filters>,
    pub draft: Signal<Option<Draft>>,
}

pub fn use_store() -> Store {
    use_context::<Store>()
}

/// `document.body.dataset.ready = "1"` once mounted: the e2e script waits for it.
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
fn App() -> Element {
    let stays = use_signal(data::stays);
    let trips = use_signal(|| data::seed_trips(&stays.peek()));
    use_context_provider(|| Store {
        stays,
        trips,
        filters: Signal::new(Filters::default()),
        draft: Signal::new(None),
    });
    use_effect(mark_ready);
    rsx! {
        ThemeStyles {}
        document::Stylesheet { href: BOOKING_CSS }
        ToastProvider { Router::<Route> {} }
    }
}

#[component]
fn Shell() -> Element {
    let store = use_store();
    let mut palette = use_signal(|| false);
    use_hotkey("k", move || palette.set(true));
    let theme = use_theme();

    let upcoming = store
        .trips
        .read()
        .iter()
        .filter(|t| trips::is_upcoming(t))
        .count();
    let mut commands = vec![
        CommandItem::new("page:explore", "Explore stays").group("Pages"),
        CommandItem::new("page:trips", "My trips").group("Pages"),
        CommandItem::new("theme", "Toggle dark mode").group("Actions"),
    ];
    commands.extend(store.stays.read().iter().map(|s| {
        CommandItem::new(format!("stay:{}", s.id), s.name.clone())
            .group(format!("Stays in {}", s.region))
    }));

    rsx! {
        SkipLink {}
        header { class: "tr-header",
            Link { to: Route::Explore {}, class: "tr-brand",
                Tent { "aria-hidden": "true", size: "1.25rem" }
                span { "Trailhead" }
            }
            nav { class: "tr-nav", aria_label: "Main",
                // The router's Link marks the current page with aria-current itself.
                Link { to: Route::Explore {}, class: "tr-nav-link", "Explore" }
                Link { to: Route::Trips {}, class: "tr-nav-link",
                    "My trips"
                    if upcoming > 0 {
                        Badge { variant: BadgeVariant::Secondary,
                            span { class: "sr-only", "(" }
                            "{upcoming}"
                            span { class: "sr-only", " upcoming)" }
                        }
                    }
                }
            }
            div { class: "tr-header-tools",
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| palette.set(true),
                    Search { "aria-hidden": "true", size: "1rem" }
                    "Search stays"
                    span { class: "tr-wide-only",
                        KbdGroup {
                            Kbd { "Ctrl" }
                            Kbd { "K" }
                        }
                    }
                }
                Switch {
                    checked: theme.is_dark(),
                    aria_label: "Dark mode",
                    on_checked_change: move |dark: bool| {
                        theme.set_mode(if dark { ThemeMode::Dark } else { ThemeMode::Light })
                    },
                }
            }
        }
        // `MainContent` (kit `router` feature) moves focus here after an in-app navigation,
        // so keyboard and screen reader users start at the new page.
        MainContent { class: "tr-main", Outlet::<Route> {} }
        footer { class: "tr-footer",
            p { "Trailhead Stays is a demo app for the Rust UI kit. Nothing here is real." }
        }
        CommandDialog {
            open: palette(),
            on_open_change: move |v| palette.set(v),
            items: commands,
            on_select: move |id: String| {
                let nav = navigator();
                match id.as_str() {
                    "page:explore" => _ = nav.push(Route::Explore {}),
                    "page:trips" => _ = nav.push(Route::Trips {}),
                    "theme" => {
                        theme.set_mode(if theme.is_dark() { ThemeMode::Light } else { ThemeMode::Dark })
                    }
                    other => {
                        if let Some(id) = other.strip_prefix("stay:").and_then(|i| i.parse().ok()) {
                            nav.push(Route::StayPage { id });
                        }
                    }
                }
            },
        }
    }
}

/// Sets the document title for the page (a real app needs this for every route).
#[component]
pub fn PageTitle(title: String) -> Element {
    rsx! {
        document::Title { "{title} · Trailhead" }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        PageTitle { title: "Not found" }
        Empty {
            EmptyHeader {
                EmptyTitle { level: 1, "Page not found" }
                EmptyDescription { "There is nothing at /{segments.join(\"/\")}." }
            }
            EmptyContent {
                Link { to: Route::Explore {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "Back to all stays" }
            }
        }
    }
}
