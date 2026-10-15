# UiStrings

- Copy into your app: `rust-ui add ui_strings` or `dx components add ui_strings --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::ui_strings::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

Every word the kit shows or announces on its own ("Next", "Clear", "No results found.", "Go to next page", screen-reader names) lives in one `UiStrings` struct with English defaults. Wrap the app in `UiStringsProvider` to translate them, and pass a signal to switch language at runtime. Text you pass as props (labels, placeholders) always wins.

```rust
let strings = use_signal(|| UiStrings { next: "Suivant".into(), ..UiStrings::default() });
rsx! { UiStringsProvider { strings, App {} } }
```

Templates use `{name}` placeholders (`"Select {row}"`, `"Showing {first}–{last} of {total} rows"`), filled with `fill`.

Not covered: calendar weekday names (use the calendar's `day_name` prop), month names (dioxus-primitives), and the field validators' messages (pass your own validator).

## Accessibility

- Role and name: renders no element of its own; it changes the accessible names inside it.
- Keyboard: none.
- Reduced motion: no motion.

## Source

Original to this repo. The idea follows thaw's `ConfigProvider` (MIT).

## API

```rust
/// Every string the kit renders on its own. Fields are grouped by component.
pub struct UiStrings {
    // Shared
    pub previous: String,
    pub next: String,
    pub loading: String,
    pub filter: String,
    /// Name of icon-only close buttons (sheet).
    pub close: String,
    // Pagination
    pub go_to_previous_page: String,
    pub go_to_next_page: String,
    // Data table
    pub filter_rows: String,
    pub select_all_rows: String,
    /// `{row}` is the row's first cell.
    pub select_row: String,
    pub no_rows: String,
    pub table_pages: String,
    /// `{count}` is the number of rows.
    pub row_count: String,
    /// `{first}`, `{last}` and `{total}` are row numbers.
    pub showing_rows: String,
    /// `{page}` and `{pages}` are page numbers.
    pub page_of: String,
    // Multi-select
    pub select_placeholder: String,
    /// `{label}` is the multi-select's name.
    pub filter_named: String,
    /// `{label}` is the multi-select's name.
    pub options_named: String,
    pub no_options: String,
    /// `{count}` is the number of selected options.
    pub selected_count: String,
    pub clear: String,
    pub done: String,
    // Command palette
    pub command_palette: String,
    pub command_help: String,
    pub search_commands: String,
    pub search_commands_placeholder: String,
    pub commands: String,
    pub no_results: String,
    // One-time code
    pub otp_label: String,
    /// `{label}` is the code's name, `{n}` the box number and `{total}` the box count.
    pub otp_character: String,
    // Marquee
    pub play: String,
    pub pause: String,
    pub scrolling_content: String,
    /// Read after Play or Pause by screen readers.
    pub scrolling: String,
    // Dropzone
    pub drop_files: String,
    pub selected_files: String,
    pub only_one_file: String,
    /// `{types}` is the accepted file types.
    pub accepts: String,
    /// `{name}` is the file name.
    pub not_accepted: String,
    /// `{name}` is the file name, `{size}` the limit.
    pub too_large: String,
    // Navigation
    pub breadcrumb: String,
    pub main_navigation: String,
    pub toggle_sidebar: String,
    pub sidebar: String,
    pub sidebar_description: String,
    pub sortable_list: String,
    /// `{label}` is the carousel's name.
    pub slides: String,
    // Date picker
    pub show_calendar: String,
    // Time picker
    pub time: String,
    pub date_and_time: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    /// Name of the AM/PM segment.
    pub day_period: String,
    pub am: String,
    pub pm: String,
    /// Read for a time segment that has no value yet.
    pub empty_value: String,
    // Chart
    pub no_data: String,
    // Link
    /// Read after a link that opens in a new tab.
    pub opens_in_new_tab: String,
    // Skip link
    pub skip_to_content: String,
    // Number field
    /// The - button's name. `{label}` is the field's name.
    pub decrease: String,
    /// The + button's name. `{label}` is the field's name.
    pub increase: String,
}

/// Provides `strings` to every kit component below it. Pass a signal to switch language
/// at runtime.
#[component]
pub fn UiStringsProvider(strings: ReadSignal<UiStrings>, children: Element) -> Element

/// The strings from the nearest [`UiStringsProvider`], or English.
pub fn use_ui_strings() -> UiStrings

/// Replaces each `{name}` in `template` with its value.
///
/// ```
/// assert_eq!(ui::components::ui_strings::fill("Select {row}", &[("row", "Ada")]), "Select Ada");
/// ```
pub fn fill(template: &str, values: &[(&str, &str)]) -> String
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::carousel::{Carousel, CarouselItem};
use crate::components::spinner::Spinner;
use crate::components::ui_strings::*;
use dioxus::prelude::*;

fn french() -> UiStrings {
    UiStrings {
        previous: "Précédent".into(),
        next: "Suivant".into(),
        loading: "Chargement".into(),
        ..UiStrings::default()
    }
}

#[component]
pub fn Demo() -> Element {
    let mut french_on = use_signal(|| false);
    let strings = use_memo(move || {
        if french_on() {
            french()
        } else {
            UiStrings::default()
        }
    });
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; max-width: 32rem;",
            div { role: "group", "aria-label": "Language", style: "display: flex; gap: 0.5rem;",
                Button {
                    variant: if french_on() { ButtonVariant::Outline } else { ButtonVariant::Primary },
                    "aria-pressed": "{!french_on()}",
                    onclick: move |_| french_on.set(false),
                    "English"
                }
                Button {
                    variant: if french_on() { ButtonVariant::Primary } else { ButtonVariant::Outline },
                    "aria-pressed": "{french_on()}",
                    onclick: move |_| french_on.set(true),
                    "Français"
                }
            }
            UiStringsProvider { strings,
                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                    Spinner {}
                    span { "The spinner's name and the carousel buttons follow the language." }
                }
                Carousel { label: "Plans",
                    for (i , name) in ["Starter", "Team", "Business"].into_iter().enumerate() {
                        CarouselItem { label: "{i + 1} of 3", strong { "{name}" } }
                    }
                }
            }
        }
    }
}
```
