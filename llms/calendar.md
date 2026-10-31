# Calendar

- Copy into your app: `atoll add calendar` or `dx components add calendar --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::calendar::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Calendar component is used to display a calendar interface, allowing users to select dates. It provides a grid layout of days for a specific month and year.

## Component Structure

```rust
Calendar {
    // The currently selected date in the calendar (if any).
    selected_date,
    on_date_change: |date: Option<CalendarDate>| {
        // This callback is triggered when a date is selected in the calendar.
        // The date parameter contains the selected date.
    },
    // The current view date of the calendar, which determines the month and year displayed.
    view_date,
    on_view_change: |date: CalendarDate| {
        // This callback is triggered when the view date changes.
        // The date parameter contains the new view date.
    },
    // Optional number of pre-composed month views to render.
    month_count: 1,
}
```

The styled `Calendar` and `RangeCalendar` render a complete month view by default, including the header, navigation, month/year selectors, and date grid.

## API

```rust
pub struct CalendarProps {
    /// The selected date
    #[props(default)]
    pub selected_date: ReadSignal<Option<Date>>,

    /// Callback when selected date changes
    #[props(default)]
    pub on_date_change: Callback<Option<Date>>,

    /// Callback when display weekday
    #[props(default = Callback::new(|weekday: Weekday| weekday_abbreviation(weekday).to_string()))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// Callback when display month
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    pub on_format_month: Callback<Month, String>,

    /// The month being viewed
    #[props(default = ReadSignal::new(Signal::new(UtcDateTime::now().date())))]
    pub view_date: ReadSignal<Date>,

    /// The current date (used for highlighting today)
    #[props(default = UtcDateTime::now().date())]
    pub today: Date,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<Date>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// First day of the week
    #[props(default = Weekday::Sunday)]
    pub first_day_of_week: Weekday,

    /// Lower limit of the range of available dates
    #[props(default = fixed_date(1925, Month::January, 1))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = fixed_date(2050, Month::December, 31))]
    pub max_date: Date,

    /// Specify how many months are visible at once
    #[props(default = 1)]
    pub month_count: u8,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<calendar::DateRange>>,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

pub struct RangeCalendarProps {
    /// The selected range
    #[props(default)]
    pub selected_range: ReadSignal<Option<calendar::DateRange>>,

    /// Callback when selected date range changes
    #[props(default)]
    pub on_range_change: Callback<Option<calendar::DateRange>>,

    /// Callback when display weekday
    #[props(default = Callback::new(|weekday: Weekday| weekday_abbreviation(weekday).to_string()))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// Callback when display month
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    pub on_format_month: Callback<Month, String>,

    /// The month being viewed
    #[props(default = ReadSignal::new(Signal::new(UtcDateTime::now().date())))]
    pub view_date: ReadSignal<Date>,

    /// The current date (used for highlighting today)
    #[props(default = UtcDateTime::now().date())]
    pub today: Date,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<Date>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// First day of the week
    #[props(default = Weekday::Sunday)]
    pub first_day_of_week: Weekday,

    /// Lower limit of the range of available dates
    #[props(default = fixed_date(1925, Month::January, 1))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = fixed_date(2050, Month::December, 31))]
    pub max_date: Date,

    /// Specify how many months are visible at once
    #[props(default = 1)]
    pub month_count: u8,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<calendar::DateRange>>,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Calendar(props: CalendarProps) -> Element

#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element
```

## Example

```rust
use crate::components::calendar::*;
use dioxus::prelude::*;
use time::{macros::date, Date};

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| date!(2026 - 05 - 15));
    rsx! {
        div { style: "padding: 20px;",
            Calendar {
                selected_date: selected_date(),
                on_date_change: move |date| {
                    tracing::info!("Selected date: {:?}", date);
                    selected_date.set(date);
                },
                view_date: view_date(),
                on_view_change: move |new_view: Date| {
                    tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
                    view_date.set(new_view);
                },
                min_date: date!(1995 - 07 - 21),
                max_date: date!(2035 - 09 - 11),
            }
        }
    }
}
```
