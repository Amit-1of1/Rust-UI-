//! My trips: upcoming and past trips in tabs, with a cancel confirmation.

use crate::data::{self, money, Trip, TripStatus};
use crate::{use_store, PageTitle, Route};
use dioxus::prelude::*;
use ui::prelude::*;

pub fn is_upcoming(trip: &Trip) -> bool {
    trip.status == TripStatus::Confirmed && trip.range.end() >= data::today()
}

#[component]
pub fn Trips() -> Element {
    let store = use_store();
    let toasts = use_toast();
    let mut trips = store.trips;
    // Which trip the dialog is about, kept apart from the open flag: AlertDialogAction closes
    // the dialog before it calls on_click (dashboard GAPS.md 9).
    let mut target = use_signal(|| None::<u32>);
    let mut confirming = use_signal(|| false);

    let mut list = trips.read().clone();
    list.sort_by_key(|t| t.range.start());
    let (upcoming, rest): (Vec<Trip>, Vec<Trip>) = list.into_iter().partition(is_upcoming);
    let past: Vec<Trip> = rest.into_iter().rev().collect();
    let target_name = target()
        .and_then(|id| trips.read().iter().find(|t| t.id == id).map(|t| t.stay))
        .map(|s| store.stays.read()[s].name.clone())
        .unwrap_or_default();

    rsx! {
        PageTitle { title: "My trips" }
        h1 { "My trips" }
        Tabs { default_value: "upcoming".to_string(), horizontal: true,
            TabList { aria_label: "Trips",
                TabTrigger { value: "upcoming".to_string(), index: 0usize, "Upcoming ({upcoming.len()})" }
                TabTrigger { value: "past".to_string(), index: 1usize, "Past and cancelled ({past.len()})" }
            }
            TabContent { index: 0usize, value: "upcoming".to_string(),
                if upcoming.is_empty() {
                    Empty {
                        EmptyHeader {
                            EmptyTitle { level: 2, "No upcoming trips" }
                            EmptyDescription { "Find somewhere quiet for your next weekend." }
                        }
                        EmptyContent {
                            Link { to: Route::Explore {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "Explore stays" }
                        }
                    }
                } else {
                    TripList {
                        trips: upcoming,
                        on_cancel: move |id| {
                            target.set(Some(id));
                            confirming.set(true);
                        },
                    }
                }
            }
            TabContent { index: 1usize, value: "past".to_string(),
                TripList { trips: past, on_cancel: move |_| {} }
            }
        }
        AlertDialog {
            open: confirming(),
            on_open_change: move |v| confirming.set(v),
            AlertDialogTitle { "Cancel this trip?" }
            AlertDialogDescription { "{target_name}: the host is told right away. This can't be undone." }
            AlertDialogActions {
                AlertDialogCancel { "Keep trip" }
                AlertDialogAction {
                    on_click: move |_| {
                        if let Some(id) = target() {
                            if let Some(t) = trips.write().iter_mut().find(|t| t.id == id) {
                                t.status = TripStatus::Cancelled;
                            }
                            toasts.notify_info("Trip cancelled");
                        }
                        target.set(None);
                    },
                    "Cancel trip"
                }
            }
        }
    }
}

#[component]
fn TripList(trips: Vec<Trip>, on_cancel: EventHandler<u32>) -> Element {
    let store = use_store();
    rsx! {
        ItemGroup { class: "tr-trips",
            for trip in trips {
                {
                    let stay = store.stays.read()[trip.stay].clone();
                    let upcoming = is_upcoming(&trip);
                    let (badge, label) = match (trip.status, upcoming) {
                        (TripStatus::Cancelled, _) => (BadgeVariant::Destructive, "Cancelled"),
                        (_, true) => (BadgeVariant::Primary, "Confirmed"),
                        (_, false) => (BadgeVariant::Secondary, "Completed"),
                    };
                    rsx! {
                        Item { key: "{trip.id}", variant: ItemVariant::Outline,
                            ItemContent {
                                ItemTitle {
                                    Link { to: Route::StayPage { id: stay.id }, "{stay.name}" }
                                    Badge { variant: badge, "{label}" }
                                }
                                ItemDescription {
                                    "{data::date_range_text(&trip.range)} · {trip.guests} guests · {money(trip.total)} · booking #{trip.id}"
                                }
                            }
                            if upcoming {
                                ItemActions {
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        size: ButtonSize::Sm,
                                        aria_label: "Cancel trip to {stay.name}",
                                        onclick: move |_| on_cancel.call(trip.id),
                                        "Cancel"
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
