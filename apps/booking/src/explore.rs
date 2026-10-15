//! Explore: search box, dates and guests, a filter column (price range, type, amenities),
//! sort, a card grid with paging, and an empty state.

use crate::data::{self, money, Kind, Stay, AMENITIES};
use crate::{use_store, PageTitle, Route};
use dioxus::prelude::*;
use std::collections::HashSet;
use ui::icons::Star;
use ui::prelude::*;

pub const PRICE_MIN: f64 = 50.0;
pub const PRICE_MAX: f64 = 400.0;
const PAGE_SIZE: usize = 9;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Sort {
    #[default]
    Recommended,
    PriceLow,
    PriceHigh,
    Rating,
}

impl Sort {
    const ALL: [Sort; 4] = [
        Sort::Recommended,
        Sort::PriceLow,
        Sort::PriceHigh,
        Sort::Rating,
    ];

    fn label(self) -> &'static str {
        match self {
            Sort::Recommended => "Recommended",
            Sort::PriceLow => "Price: low to high",
            Sort::PriceHigh => "Price: high to low",
            Sort::Rating => "Top rated",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Filters {
    pub query: String,
    pub range: Option<DateRange>,
    pub guests: u32,
    pub price: (f64, f64),
    /// Indices into `Kind::ALL`; empty means any type.
    pub kinds: HashSet<usize>,
    pub amenities: HashSet<&'static str>,
    pub sort: Sort,
    pub page: usize,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            query: String::new(),
            range: None,
            guests: 2,
            price: (PRICE_MIN, PRICE_MAX),
            kinds: HashSet::new(),
            amenities: HashSet::new(),
            sort: Sort::Recommended,
            page: 0,
        }
    }
}

impl Filters {
    fn matches(&self, stay: &Stay) -> bool {
        let q = self.query.trim().to_lowercase();
        let text_ok = q.is_empty()
            || stay.name.to_lowercase().contains(&q)
            || stay.region.to_lowercase().contains(&q);
        let kind_ok = self.kinds.is_empty()
            || Kind::ALL
                .iter()
                .position(|k| *k == stay.kind)
                .is_some_and(|i| self.kinds.contains(&i));
        let price = stay.price as f64;
        let dates_ok = self
            .range
            .is_none_or(|r| !data::booked(stay.id).iter().any(|b| data::overlaps(b, &r)));
        text_ok
            && kind_ok
            && dates_ok
            && stay.max_guests >= self.guests
            && price >= self.price.0
            && price <= self.price.1
            && self.amenities.iter().all(|a| stay.amenities.contains(a))
    }

    /// Filters a user can clear (not the sort or page).
    fn active(&self) -> bool {
        let d = Filters::default();
        self.query != d.query
            || self.range.is_some()
            || self.price != d.price
            || !self.kinds.is_empty()
            || !self.amenities.is_empty()
    }
}

#[component]
pub fn Explore() -> Element {
    let store = use_store();
    let mut filters = store.filters;
    let results = use_memo(move || {
        let f = filters.read();
        let mut list: Vec<Stay> = store
            .stays
            .read()
            .iter()
            .filter(|s| f.matches(s))
            .cloned()
            .collect();
        match f.sort {
            Sort::Recommended => {}
            Sort::PriceLow => list.sort_by_key(|s| s.price),
            Sort::PriceHigh => list.sort_by_key(|s| std::cmp::Reverse(s.price)),
            Sort::Rating => list.sort_by_key(|s| std::cmp::Reverse(s.rating)),
        }
        list
    });
    let count = results.read().len();
    let pages = count.div_ceil(PAGE_SIZE).max(1);
    let page = filters.read().page.min(pages - 1);
    let shown: Vec<Stay> = results
        .read()
        .iter()
        .skip(page * PAGE_SIZE)
        .take(PAGE_SIZE)
        .cloned()
        .collect();
    // Every filter change goes back to the first page.
    let mut update = move |f: &dyn Fn(&mut Filters)| {
        let mut w = filters.write();
        f(&mut w);
        w.page = 0;
    };
    rsx! {
        PageTitle { title: "Explore stays" }
        section { class: "tr-hero", aria_labelledby: "hero-title",
            h1 { id: "hero-title", "Find a quiet place in the woods" }
            p { class: "tr-muted", "Cabins, yurts and treehouses near the trails." }
            div { class: "tr-search", role: "search", aria_label: "Search stays",
                div { class: "tr-search-field tr-grow",
                    Label { html_for: "q", "Where" }
                    Input {
                        id: "q",
                        r#type: "search",
                        placeholder: "Stay or region",
                        value: "{filters.read().query}",
                        oninput: move |e: FormEvent| update(&|f| f.query = e.value()),
                    }
                }
                div { class: "tr-search-field",
                    span { id: "dates-label", class: "tr-label", "Dates" }
                    DateRangePicker {
                        aria_labelledby: "dates-label",
                        selected_range: filters.read().range,
                        min_date: data::today(),
                        on_range_change: move |r| update(&|f| f.range = r),
                    }
                }
                div { class: "tr-search-field",
                    span { id: "guests-label", class: "tr-label", "Guests" }
                    GuestCounter {
                        labelledby: "guests-label",
                        value: filters.read().guests,
                        max: 6,
                        on_change: move |g| update(&|f| f.guests = g),
                    }
                }
            }
        }
        div { class: "tr-explore",
            aside { class: "tr-filters", aria_labelledby: "filters-title",
                h2 { id: "filters-title", class: "tr-h3", "Filters" }
                FilterPanel {}
            }
            section { class: "tr-results", aria_labelledby: "results-title",
                div { class: "tr-results-bar",
                    h2 { id: "results-title", class: "tr-h3", "Stays" }
                    p { class: "tr-muted", role: "status",
                        if count == 1 {
                            "1 stay"
                        } else {
                            "{count} stays"
                        }
                    }
                    div { class: "tr-sort",
                        span { id: "sort-label", class: "tr-label", "Sort by" }
                        Select::<Sort> {
                            aria_labelledby: "sort-label",
                            selected: filters.read().sort,
                            on_value_change: move |s: Option<Sort>| {
                                if let Some(s) = s {
                                    update(&|f| f.sort = s);
                                }
                            },
                            for (i , s) in Sort::ALL.into_iter().enumerate() {
                                SelectOption::<Sort> {
                                    key: "{i}",
                                    index: i,
                                    value: s,
                                    text_value: s.label(),
                                    "{s.label()}"
                                }
                            }
                        }
                    }
                }
                if count == 0 {
                    Empty {
                        EmptyHeader {
                            EmptyTitle { "No stays match" }
                            EmptyDescription { "Try other dates, fewer guests or a wider price range." }
                        }
                        EmptyContent {
                            Button {
                                variant: ButtonVariant::Outline,
                                onclick: move |_| filters.set(Filters::default()),
                                "Clear all filters"
                            }
                        }
                    }
                } else {
                    ul { class: "tr-grid", aria_label: "Stays",
                        for stay in shown {
                            li { key: "{stay.id}", StayCard { stay } }
                        }
                    }
                    if pages > 1 {
                        Pager {
                            aria_label: "Pages of stays",
                            page: page + 1,
                            pages,
                            on_page_change: move |p: usize| filters.write().page = p - 1,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FilterPanel() -> Element {
    let mut filters = use_store().filters;
    let mut update = move |f: &dyn Fn(&mut Filters)| {
        let mut w = filters.write();
        f(&mut w);
        w.page = 0;
    };
    let (lo, hi) = filters.read().price;
    let price = use_memo(move || {
        let (lo, hi) = filters.read().price;
        Some(lo..hi)
    });
    let kinds = use_memo(move || Some(filters.read().kinds.clone()));
    rsx! {
        div { class: "tr-filter-group",
            h3 { id: "price-label", class: "tr-label", "Price per night" }
            RangeSlider {
                label: "Price per night",
                min: PRICE_MIN,
                max: PRICE_MAX,
                step: 10.0,
                value: price,
                on_value_change: move |r: std::ops::Range<f64>| update(&|f| f.price = (r.start, r.end)),
            }
            p { class: "tr-muted", "{money(lo as u32)} – {money(hi as u32)}" }
        }
        div { class: "tr-filter-group",
            h3 { id: "type-label", class: "tr-label", "Type" }
            ToggleGroup {
                aria_labelledby: "type-label",
                allow_multiple_pressed: true,
                horizontal: true,
                pressed: kinds,
                on_pressed_change: move |p: HashSet<usize>| update(&|f| f.kinds = p.clone()),
                class: "tr-kinds",
                for (i , k) in Kind::ALL.iter().enumerate() {
                    ToggleItem { index: i, "{k.label()}" }
                }
            }
        }
        fieldset { class: "tr-filter-group",
            legend { class: "tr-label", "Amenities" }
            for (i , a) in AMENITIES.into_iter().enumerate() {
                div { class: "tr-check-row", key: "{a}",
                    Checkbox {
                        id: "amenity-{i}",
                        checked: Some(if filters.read().amenities.contains(a) {
                            CheckboxState::Checked
                        } else {
                            CheckboxState::Unchecked
                        }),
                        on_checked_change: move |s: CheckboxState| {
                            update(&|f| {
                                if s == CheckboxState::Checked {
                                    f.amenities.insert(a);
                                } else {
                                    f.amenities.remove(a);
                                }
                            })
                        },
                    }
                    Label { html_for: "amenity-{i}", "{a}" }
                }
            }
        }
        if filters.read().active() {
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    let mut w = filters.write();
                    let keep = (w.guests, w.sort);
                    *w = Filters { guests: keep.0, sort: keep.1, ..Filters::default() };
                },
                "Clear filters"
            }
        }
    }
}

/// The guest count: the kit's `NumberInput` (a spin button with - and +).
#[component]
pub fn GuestCounter(
    #[props(into)] labelledby: String,
    value: u32,
    max: u32,
    on_change: EventHandler<u32>,
) -> Element {
    rsx! {
        NumberInput {
            label: "guests",
            aria_labelledby: "{labelledby}",
            value: value as f64,
            min: 1.0,
            max: max as f64,
            on_value_change: move |v: f64| on_change.call(v as u32),
        }
    }
}

/// A coloured block standing in for a photo (demos never load remote images).
#[component]
pub fn Photo(hue: u32, #[props(into)] label: String, #[props(default)] shift: u32) -> Element {
    let h = (hue + shift * 40) % 360;
    rsx! {
        div {
            class: "tr-photo",
            role: "img",
            aria_label: "{label}",
            style: "--photo-hue: {h};",
        }
    }
}

#[component]
fn StayCard(stay: Stay) -> Element {
    rsx! {
        Card { class: "tr-card",
            CardMedia { Photo { hue: stay.hue, label: "Photo of {stay.name}" } }
            CardHeader {
                CardTitle {
                    h3 { class: "tr-card-title",
                        Link {
                            to: Route::StayPage { id: stay.id },
                            class: "tr-stretched",
                            "{stay.name}"
                        }
                    }
                }
                CardDescription { "{stay.region} · up to {stay.max_guests} guests" }
            }
            CardFooter { class: "tr-card-footer",
                span { class: "tr-rating",
                    Star { "aria-hidden": "true", size: "0.9rem" }
                    span { class: "sr-only", "Rated " }
                    "{stay.rating_text()}"
                    span { class: "tr-muted", " ({stay.reviews})" }
                }
                Badge { variant: BadgeVariant::Outline, "{stay.kind.label()}" }
                span { class: "tr-price",
                    strong { "{money(stay.price)}" }
                    " night"
                }
            }
        }
    }
}
