use crate::components::date_picker::*;
use dioxus::prelude::*;

use time::Date;

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut selected_range = use_signal(|| None::<DateRange>);

    rsx! {
        div { style: "display: grid; gap: 1rem; justify-items: start;",
            DatePicker {
                selected_date: selected_date(),
                on_value_change: move |v| {
                    tracing::info!("Selected date changed: {:?}", v);
                    selected_date.set(v);
                },
            }
            DateRangePicker {
                selected_range: selected_range(),
                on_range_change: move |r| selected_range.set(r),
            }
        }
    }
}
