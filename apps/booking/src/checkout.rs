//! Checkout in three steps: the trip (dates, guests, rate), the guest's details, and payment
//! with a final review. Each step validates before the next one opens.

use crate::data::{self, money, Rate, Trip, TripStatus};
use crate::explore::{GuestCounter, Photo};
use crate::{use_store, Draft, PageTitle, Route};
use dioxus::prelude::*;
use ui::prelude::*;

const COUNTRIES: [&str; 8] = [
    "Australia",
    "Canada",
    "Germany",
    "India",
    "Japan",
    "Norway",
    "United Kingdom",
    "United States",
];

const STEPS: [(&str, &str); 3] = [
    ("Trip", "Dates, guests, rate"),
    ("Your details", "Who is staying"),
    ("Payment", "Review and pay"),
];

fn digits(v: &str) -> String {
    v.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn phone_rule() -> Validator {
    custom(|v| {
        if digits(v).len() >= 7 {
            Ok(())
        } else {
            Err("Enter a phone number with at least 7 digits.".into())
        }
    })
}

fn card_rule() -> Validator {
    custom(|v| {
        let d = digits(v);
        if d.len() == 16 && v.chars().all(|c| c.is_ascii_digit() || c == ' ') {
            Ok(())
        } else {
            Err("Enter the 16 digits on the front of the card.".into())
        }
    })
}

fn expiry_rule() -> Validator {
    custom(|v| {
        let ok = v.len() == 5
            && v.as_bytes()[2] == b'/'
            && v[..2].parse::<u8>().is_ok_and(|m| (1..=12).contains(&m))
            && v[3..].parse::<u8>().is_ok();
        if ok {
            Ok(())
        } else {
            Err("Use MM/YY, for example 04/29.".into())
        }
    })
}

fn cvc_rule() -> Validator {
    custom(|v| {
        if (3..=4).contains(&v.len()) && v.chars().all(|c| c.is_ascii_digit()) {
            Ok(())
        } else {
            Err("Enter the 3 or 4 digit code on the back.".into())
        }
    })
}

#[component]
pub fn Checkout(id: usize) -> Element {
    let mut store = use_store();
    let toasts = use_toast();
    let Some(stay) = store.stays.read().get(id).cloned() else {
        return rsx! {
            Callout { variant: CalloutVariant::Warning, title: "Stay not found",
                Link { to: Route::Explore {}, "Back to all stays" }
            }
        };
    };
    let draft = store
        .draft
        .peek()
        .filter(|d| d.stay == id)
        .unwrap_or(Draft {
            stay: id,
            range: None,
            guests: 2,
        });

    let step = use_signal(|| 0usize);
    // The furthest step reached; later steps can't be jumped to from the stepper.
    let mut reached = use_signal(|| 0usize);
    let mut range = use_signal(|| draft.range);
    let mut guests = use_signal(|| draft.guests.min(stay.max_guests));
    let mut rate = use_signal(|| Rate::Flexible);
    let mut trip_error = use_signal(|| None::<String>);

    let details = use_form();
    let name = details.field(vec![required(), max_len(80)]);
    let email_f = details.field(vec![required(), email()]);
    let phone = details.field(vec![required(), phone_rule()]);
    let country = details.field(vec![required()]);
    let requests = details.field(vec![max_len(300)]);
    let terms = details.field(vec![custom(|v| {
        if v == "true" {
            Ok(())
        } else {
            Err("Accept the house rules to continue.".into())
        }
    })]);

    let payment = use_form();
    let holder = payment.field(vec![required()]);
    let card = payment.field(vec![required(), card_rule()]);
    let expiry = payment.field(vec![required(), expiry_rule()]);
    let cvc = payment.field(vec![required(), cvc_rule()]);

    let booked = use_memo(move || data::booked(id));
    let price = stay.price;
    let quote = use_memo(move || {
        range()
            .map(|r| data::nights(&r))
            .filter(|n| *n > 0)
            .map(|n| data::quote(price, n, rate()))
    });

    let mut go = move |to: usize| {
        let mut step = step;
        step.set(to);
        if to > reached() {
            reached.set(to);
        }
    };
    let trip_next = move |_| {
        let clash = range().is_some_and(|r| booked.read().iter().any(|b| data::overlaps(b, &r)));
        let problem = if quote().is_none() {
            Some("Choose a check-in and a check-out date.")
        } else if clash {
            Some("Part of that range is already booked.")
        } else {
            None
        };
        trip_error.set(problem.map(String::from));
        if problem.is_none() {
            go(1);
        }
    };
    let stay_name = stay.name.clone();
    let confirm = move |_| {
        let (Some(r), Some(q)) = (range(), quote()) else {
            return;
        };
        let mut trips = store.trips;
        let id_next = trips.read().iter().map(|t| t.id).max().unwrap_or(1000) + 1;
        trips.write().push(Trip {
            id: id_next,
            stay: id,
            range: r,
            guests: guests(),
            rate: rate(),
            guest_name: name.value(),
            total: q.total,
            status: TripStatus::Confirmed,
        });
        store.draft.set(None);
        toasts.notify_success(format!("Booked {stay_name}"));
        navigator().push(Route::Trips {});
    };

    let flexible = data::quote(price, 1, Rate::Flexible).lodging;
    let strict = data::quote(price, 1, Rate::NonRefundable).lodging;

    rsx! {
        PageTitle { title: "Checkout: {stay.name}" }
        h1 { "Confirm and pay" }
        Stepper {
            total: STEPS.len(),
            current: step,
            aria_label: "Checkout steps",
            for (i , (title , description)) in STEPS.into_iter().enumerate() {
                StepperItem { key: "{i}", step: i, title, description, disabled: i > reached() }
            }
        }
        div { class: "tr-checkout",
            div { class: "tr-checkout-step",
                match step() {
                    0 => rsx! {
                        Card {
                            CardHeader { CardTitle { h2 { class: "tr-h3", "Your trip" } } }
                            CardContent { class: "tr-stack",
                                div { class: "tr-search-field",
                                    span { id: "co-dates-label", class: "tr-label", "Dates" }
                                    DateRangePicker {
                                        aria_labelledby: "co-dates-label",
                                        selected_range: range(),
                                        min_date: data::today(),
                                        disabled_ranges: booked(),
                                        on_range_change: move |r| {
                                            range.set(r);
                                            trip_error.set(None);
                                        },
                                    }
                                }
                                div { class: "tr-search-field",
                                    span { id: "co-guests-label", class: "tr-label", "Guests" }
                                    GuestCounter {
                                        labelledby: "co-guests-label",
                                        value: guests(),
                                        max: stay.max_guests,
                                        on_change: move |g| guests.set(g),
                                    }
                                }
                                fieldset { class: "tr-fieldset",
                                    legend { class: "tr-label", "Rate" }
                                    RadioGroup {
                                        value: Some(rate().id().to_string()),
                                        on_value_change: move |v: String| rate.set(Rate::from_id(&v)),
                                        aria_label: "Rate",
                                        RadioItem { value: Rate::Flexible.id().to_string(), index: 0usize,
                                            "Flexible, {money(flexible)} a night: free cancellation until 48 hours before"
                                        }
                                        RadioItem { value: Rate::NonRefundable.id().to_string(), index: 1usize,
                                            "Non-refundable, {money(strict)} a night"
                                        }
                                    }
                                }
                                if let Some(message) = trip_error() {
                                    p { class: "tr-error", role: "alert", "{message}" }
                                }
                            }
                            CardFooter { class: "tr-actions",
                                Button { onclick: trip_next, "Continue" }
                            }
                        }
                    },
                    1 => rsx! {
                        Card {
                            CardHeader { CardTitle { h2 { class: "tr-h3", "Who is staying" } } }
                            CardContent {
                                Form {
                                    id: "details-form",
                                    state: details,
                                    on_valid: move |_| go(2),
                                    TextField { field: name, label: "Full name", autocomplete: "name" }
                                    TextField { field: email_f, label: "Email", r#type: "email", autocomplete: "email",
                                        description: "We send the confirmation and the lockbox code here."
                                    }
                                    TextField { field: phone, label: "Phone", r#type: "tel", autocomplete: "tel" }
                                    SelectField {
                                        field: country,
                                        label: "Country",
                                        placeholder: "Choose a country",
                                        options: COUNTRIES.iter().map(|c| FieldOption::from(*c)).collect::<Vec<_>>(),
                                    }
                                    TextareaField {
                                        field: requests,
                                        label: "Requests for the host (optional)",
                                        maxlength: "300",
                                    }
                                    CheckboxField {
                                        field: terms,
                                        label: "I accept the house rules and the cancellation policy",
                                    }
                                    div { class: "tr-actions",
                                        Button { variant: ButtonVariant::Outline, r#type: "button", onclick: move |_| go(0), "Back" }
                                        Button { r#type: "submit", "Continue" }
                                    }
                                }
                            }
                        }
                    },
                    _ => rsx! {
                        Card {
                            CardHeader {
                                CardTitle { h2 { class: "tr-h3", "Payment" } }
                                CardDescription { "This is a demo: no card is charged. Try 4242 4242 4242 4242." }
                            }
                            CardContent {
                                Form {
                                    id: "payment-form",
                                    state: payment,
                                    on_valid: confirm,
                                    TextField { field: holder, label: "Name on card", autocomplete: "cc-name" }
                                    TextField {
                                        field: card,
                                        label: "Card number",
                                        autocomplete: "cc-number",
                                        inputmode: "numeric",
                                        maxlength: "19",
                                        placeholder: "1234 1234 1234 1234",
                                    }
                                    div { class: "tr-row",
                                        TextField {
                                            field: expiry,
                                            label: "Expiry",
                                            autocomplete: "cc-exp",
                                            placeholder: "MM/YY",
                                            maxlength: "5",
                                        }
                                        TextField {
                                            field: cvc,
                                            label: "Security code",
                                            autocomplete: "cc-csc",
                                            inputmode: "numeric",
                                            maxlength: "4",
                                        }
                                    }
                                    div { class: "tr-actions",
                                        Button { variant: ButtonVariant::Outline, r#type: "button", onclick: move |_| go(1), "Back" }
                                        Button { r#type: "submit",
                                            if let Some(q) = quote() {
                                                "Confirm and pay {money(q.total)}"
                                            } else {
                                                "Confirm and pay"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                }
            }
            aside { class: "tr-summary", aria_labelledby: "summary-title",
                Card {
                    CardMedia { Photo { hue: stay.hue, label: "Photo of {stay.name}" } }
                    CardHeader {
                        CardTitle { h2 { id: "summary-title", class: "tr-h3", "{stay.name}" } }
                        CardDescription { "{stay.region}" }
                    }
                    CardContent {
                        dl { class: "tr-quote",
                            div {
                                dt { "Dates" }
                                dd {
                                    if let Some(r) = range() {
                                        "{data::date_range_text(&r)}"
                                    } else {
                                        "Not chosen"
                                    }
                                }
                            }
                            div {
                                dt { "Guests" }
                                dd { "{guests()}" }
                            }
                            if let Some(q) = quote() {
                                div {
                                    dt { "{q.nights} nights" }
                                    dd { "{money(q.lodging)}" }
                                }
                                div {
                                    dt { "Fees" }
                                    dd { "{money(q.cleaning + q.service)}" }
                                }
                                    div { class: "tr-quote-total",
                                    dt { "Total" }
                                    dd { "{money(q.total)}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
