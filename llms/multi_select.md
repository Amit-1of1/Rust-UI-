# MultiSelect

- Copy into your app: `rust-ui add multi_select` or `dx components add multi_select --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::multi_select::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

Pick several options from a list.

```rust
MultiSelect { label: "Teams", options: vec![MultiOption::new("design", "Design"), MultiOption::new("eng", "Engineering")],
              on_change: move |values| save(values) }
```

A button shows the selection ("Design, Engineering +2"). It opens a panel with a filter box and a list of native checkboxes. `summary()` and `filter_options()` are public, pure helpers.

## Accessibility

- Role and name: the button has `aria-expanded` and `aria-controls` and is named "label: current selection"; the panel is a `role="group"` of real checkboxes, so no custom listbox keyboard model is needed; the count or "No options match." is in a `role="status"` region.
- Keyboard: Enter/Space opens the button, Tab moves through the filter, checkboxes and the Clear/Done buttons, Space toggles a checkbox, Escape closes and returns focus to the button.
- Pointer: a click outside the panel closes it.
- Reduced motion: no animation.

## Source

Original to this repo (a checkbox-group disclosure, not an ARIA listbox). Rust/UI's `multi_select.rs` was only used to see the feature set.

## API

```rust
pub struct MultiOption {
    pub value: String,
    pub label: String,
}

/// Button text for the current selection: the placeholder, the labels (up to two), or
/// "A, B +3".
pub fn summary(labels: &[&str], placeholder: &str) -> String

/// Options whose label contains every word of the query, ignoring case.
pub fn filter_options<'a>(options: &'a [MultiOption], query: &str) -> Vec<&'a MultiOption>

/// Pick several options from a list. A button shows the selection and opens a panel of native
/// checkboxes with a filter box. Escape or "Done" closes it and returns focus to the button.
#[component]
pub fn MultiSelect(
    options: Vec<MultiOption>,
    /// Accessible name, for example "Teams".
    #[props(into)]
    label: String,
    /// Shown when nothing is selected. Defaults to "Select…" ([`UiStrings`](crate::components::ui_strings::UiStrings)).
    #[props(into)]
    placeholder: Option<String>,
    /// Values selected at first.
    #[props(default)]
    default_selected: Vec<String>,
    #[props(default)] disabled: bool,
    /// Called with the selected values (in option order) after every change.
    #[props(default)]
    on_change: Callback<Vec<String>>,
) -> Element
```

## Example

```rust
use crate::components::multi_select::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut picked = use_signal(|| vec!["design".to_string()]);
    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            MultiSelect {
                label: "Teams",
                placeholder: "Choose teams",
                default_selected: vec!["design".to_string()],
                options: vec![
                    MultiOption::new("design", "Design"),
                    MultiOption::new("eng", "Engineering"),
                    MultiOption::new("ops", "Operations"),
                    MultiOption::new("sales", "Sales"),
                    MultiOption::new("support", "Support"),
                ],
                on_change: move |v| picked.set(v),
            }
            p { style: "margin: 0; font-size: 0.875rem;", "Selected: {picked().join(\", \")}" }
        }
    }
}
```
