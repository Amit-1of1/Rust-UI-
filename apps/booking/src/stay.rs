//! A stay's page: breadcrumb, photo carousel, tabs (overview, amenities, reviews), house rules
//! in an accordion, the host in a hover card, and a booking box with a date range and a live
//! price breakdown.

use crate::data::{self, money, Rate, REVIEWS};
use crate::explore::{GuestCounter, Photo};
use crate::{use_store, Draft, PageTitle, Route};
use dioxus::prelude::*;
use ui::icons::{Check, Star};
use ui::prelude::*;

const RULES: [(&str, &str); 3] = [
    (
        "Check-in and check-out",
        "Check in after 3 pm with the lockbox code we send the day before. Check out by 11 am.",
    ),
    (
        "Cancellation",
        "Flexible rate: free cancellation until 48 hours before check-in. Non-refundable rate: no refund, 10% cheaper.",
    ),
    (
        "Quiet hours and fires",
        "Quiet from 10 pm. Fires only in the fire ring, and never during a burn ban.",
    ),
];

#[component]
pub fn StayPage(id: usize) -> Element {
    let store = use_store();
    let Some(stay) = store.stays.read().get(id).cloned() else {
        return rsx! {
            PageTitle { title: "Stay not found" }
            Empty {
                EmptyHeader {
                    EmptyTitle { level: 1, "Stay not found" }
                    EmptyDescription { "It may have been removed." }
                }
                EmptyContent {
                    Link { to: Route::Explore {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "Back to all stays" }
                }
            }
        };
    };
    let nav = navigator();

    rsx! {
        PageTitle { title: "{stay.name}" }
        Breadcrumb { aria_label: "Breadcrumb",
            BreadcrumbItem {
                BreadcrumbLink {
                    href: "/",
                    onclick: move |e: MouseEvent| {
                        e.prevent_default();
                        nav.push(Route::Explore {});
                    },
                    "Explore"
                }
            }
            BreadcrumbSeparator {}
            BreadcrumbItem { span { class: "tr-muted", "{stay.region}" } }
            BreadcrumbSeparator {}
            BreadcrumbItem { BreadcrumbPage { "{stay.name}" } }
        }
        div { class: "tr-stay-head",
            h1 { "{stay.name}" }
            p { class: "tr-stay-meta",
                span { class: "tr-rating",
                    Star { "aria-hidden": "true", size: "0.9rem" }
                    span { class: "sr-only", "Rated " }
                    "{stay.rating_text()} · {stay.reviews} reviews"
                }
                Badge { variant: BadgeVariant::Outline, "{stay.kind.label()}" }
                span { class: "tr-muted", "{stay.region} · up to {stay.max_guests} guests" }
            }
        }
        div { class: "tr-stay",
            div { class: "tr-stay-body",
                Carousel { label: "Photos of {stay.name}",
                    for i in 0..4u32 {
                        CarouselItem { key: "{i}", label: "{i + 1} of 4",
                            Photo { hue: stay.hue, shift: i, label: "Photo {i + 1} of {stay.name}" }
                        }
                    }
                }
                Tabs { default_value: "overview".to_string(), horizontal: true,
                    TabList { aria_label: "About this stay",
                        TabTrigger { value: "overview".to_string(), index: 0usize, "Overview" }
                        TabTrigger { value: "amenities".to_string(), index: 1usize, "Amenities" }
                        TabTrigger { value: "reviews".to_string(), index: 2usize, "Reviews" }
                    }
                    TabContent { index: 0usize, value: "overview".to_string(),
                        p {
                            "A {stay.kind.label().to_lowercase()} in the {stay.region}, a short walk from the trailhead. "
                            "Sleeps {stay.max_guests}, with a covered deck and a fire ring."
                        }
                        p { class: "tr-host",
                            "Hosted by "
                            HoverCard {
                                // The trigger is itself focusable (role="button"); a real
                                // button inside it would be nested-interactive.
                                HoverCardTrigger { class: "tr-host-trigger", "{stay.host}" }
                                HoverCardContent { side: ContentSide::Bottom,
                                    div { class: "tr-host-card",
                                        // The name is right beside it; the avatar is role="img"
                                        // with no name of its own, so hide it (GAPS.md).
                                        Avatar { size: AvatarImageSize::Medium, aria_hidden: "true",
                                            AvatarFallback { {initials(stay.host)} }
                                        }
                                        div {
                                            strong { "{stay.host}" }
                                            p { class: "tr-muted", "Host since 2019 · replies within a day" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    TabContent { index: 1usize, value: "amenities".to_string(),
                        ul { class: "tr-amenities",
                            for a in stay.amenities.iter() {
                                li { key: "{a}",
                                    Check { "aria-hidden": "true", size: "1rem" }
                                    "{a}"
                                }
                            }
                        }
                    }
                    TabContent { index: 2usize, value: "reviews".to_string(),
                        ul { class: "tr-reviews",
                            for r in REVIEWS.iter() {
                                li { key: "{r.author}",
                                    Avatar { size: AvatarImageSize::Small, aria_hidden: "true",
                                        AvatarFallback { {initials(r.author)} }
                                    }
                                    div {
                                        p {
                                            strong { "{r.author}" }
                                            span { class: "tr-muted", " · {r.when}" }
                                        }
                                        p { "{r.text}" }
                                    }
                                }
                            }
                        }
                    }
                }
                section { aria_labelledby: "rules-title",
                    h2 { id: "rules-title", class: "tr-h3", "House rules" }
                    Accordion { allow_multiple_open: true, horizontal: false,
                        for (i , (q , a)) in RULES.into_iter().enumerate() {
                            AccordionItem { index: i,
                                AccordionTrigger { {q} }
                                AccordionContent {
                                    p { class: "tr-accordion-text", {a} }
                                }
                            }
                        }
                    }
                }
            }
            BookingBox { stay_id: id }
        }
    }
}

fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect()
}

#[component]
fn BookingBox(stay_id: usize) -> Element {
    let mut store = use_store();
    let stay = store.stays.read()[stay_id].clone();
    let (start_range, start_guests) = {
        let f = store.filters.peek();
        (f.range, f.guests)
    };
    // Start from what was searched on Explore.
    let mut range = use_signal(|| start_range);
    let mut guests = use_signal(|| start_guests.min(stay.max_guests));
    let booked = use_memo(move || data::booked(stay_id));
    let clash = range().is_some_and(|r| booked.read().iter().any(|b| data::overlaps(b, &r)));
    let quote = range()
        .map(|r| data::nights(&r))
        .filter(|n| *n > 0)
        .map(|n| data::quote(stay.price, n, Rate::Flexible));

    rsx! {
        aside { class: "tr-book", aria_labelledby: "book-title",
            Card {
                CardHeader {
                    CardTitle {
                        h2 { id: "book-title", class: "tr-h3",
                            "{money(stay.price)} "
                            span { class: "tr-muted", "night" }
                        }
                    }
                    CardDescription { "Flexible rate shown; non-refundable is cheaper at checkout." }
                }
                CardContent { class: "tr-book-body",
                    div { class: "tr-search-field",
                        span { id: "book-dates-label", class: "tr-label", "Dates" }
                        div { role: "group", aria_labelledby: "book-dates-label",
                            DateRangePicker {
                                selected_range: range(),
                                min_date: data::today(),
                                disabled_ranges: booked(),
                                on_range_change: move |r| range.set(r),
                            }
                        }
                    }
                    div { class: "tr-search-field",
                        span { id: "book-guests-label", class: "tr-label", "Guests" }
                        GuestCounter {
                            labelledby: "book-guests-label",
                            value: guests(),
                            max: stay.max_guests,
                            on_change: move |g| guests.set(g),
                        }
                    }
                    if clash {
                        Callout { variant: CalloutVariant::Warning, title: "Those dates are taken",
                            "Someone already booked part of that range. Pick other nights."
                        }
                    } else if let Some(q) = quote {
                        dl { class: "tr-quote",
                            div {
                                dt { "{money(q.lodging / q.nights)} × {q.nights} nights" }
                                dd { "{money(q.lodging)}" }
                            }
                            div {
                                dt { "Cleaning fee" }
                                dd { "{money(q.cleaning)}" }
                            }
                            div {
                                dt { "Service fee" }
                                dd { "{money(q.service)}" }
                            }
                            div { class: "tr-quote-total",
                                dt { "Total" }
                                dd { "{money(q.total)}" }
                            }
                        }
                    } else {
                        p { class: "tr-muted", "Pick your dates to see the total." }
                    }
                }
                CardFooter {
                    Button {
                        class: "tr-wide",
                        disabled: quote.is_none() || clash,
                        onclick: move |_| {
                            store.draft.set(Some(Draft { stay: stay_id, range: range(), guests: guests() }));
                            navigator().push(Route::Checkout { id: stay_id });
                        },
                        "Reserve"
                    }
                }
            }
        }
    }
}
