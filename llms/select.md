# Select

- Copy into your app: `atoll add select` or `dx components add select --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `icons`).
- Import: `use crate::components::select::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Select component is used to create a dropdown menu that allows users to select one or more options from the select groups.

## Component Structure

```rust
Select::<String> {
    selected: "option1".to_string(),
    on_value_change: |value: Option<String>| {
        // Handle the change in selected value.
    },
    SelectGroup {
        SelectGroupLabel { "Group A" }
        SelectOption::<String> {
            index: 0,
            value: "option1",
            "Option 1"
        }
    }
}
```

## Controlled values

`selected` takes a plain value (`selected: filters.read().sort`): the select shows whatever the parent passes on each render, with no signal or `use_memo` adapter. `value` takes a `ReadSignal<Option<T>>` when the state already lives in one (`value: Some(choice.into())`), and `default_value` leaves the select uncontrolled. In forms, `SelectField` binds a `use_form` field instead.

## Labelling

Give every Select a name. `aria_label` or `aria_labelledby` on `Select` (or `SelectMulti`) is moved to the trigger button, and the trigger adds itself to `aria-labelledby`, so screen readers announce the label and the current value ("Fruit, Apple").

```rust
// A visible label.
Label { id: "fruit-label", "Fruit" }
Select::<String> { aria_labelledby: "fruit-label", /* options */ }

// No visible label.
Select::<String> { aria_label: "Fruit", /* options */ }
```

## Keyboard and accessibility

Follows the WAI-ARIA [Select-Only Combobox](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/) pattern. Roles: `listbox`, `option`. Works on native (Blitz): partly.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | On the trigger | Opens the list. | yes |
| Space | On the trigger | Opens the list. | yes |
| ArrowDown | In the open list | Moves focus to the first option, then to the next one, skipping disabled ones. | yes |
| Enter | On an option | Picks the option, closes the list and shows the value on the trigger. | yes |
| Enter | After picking an option | Focus goes back to the trigger. | not yet |
| Escape | In the open list | Closes the list without changing the value. | yes |
| Escape | After the list closes | Focus goes back to the trigger. | not yet |

States:

- `open`: The list of options is shown.
- `closed`: Only the trigger, showing the current value, is shown.

Known gaps:

- The trigger is a button with `aria-haspopup=listbox`, not `role=combobox` as in the APG example.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
pub use dioxus_primitives::select::SelectGroup;

/// Props for [`Select`]: dioxus-primitives' `SelectProps` plus `selected`, a plain value.
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// The selected option as a plain value: the select is controlled and shows whatever the
    /// parent passes on each render (`selected: filters.read().sort`), with no signal or memo
    /// needed. Wins over `value`.
    #[props(default, into)]
    pub selected: Option<T>,

    /// The controlled value as a signal; its `None` means no option is selected.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// The initial value when uncontrolled. `None` shows the placeholder.
    #[props(default)]
    pub default_value: Option<T>,

    /// Called when the selected value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the list.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Name for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Whether arrow keys wrap around at the ends of the list.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// How long typed letters are remembered for typeahead.
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element
```

## Example

```rust
use crate::components::select::*;
use dioxus::prelude::*;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Orangeade,
    Strawberry,
    Watermelon,
}

impl Fruit {
    const fn emoji(&self) -> &'static str {
        match self {
            Fruit::Apple => "🍎",
            Fruit::Banana => "🍌",
            Fruit::Orange => "🍊",
            Fruit::Orangeade => "🧃",
            Fruit::Strawberry => "🍓",
            Fruit::Watermelon => "🍉",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let fruits = Fruit::iter().enumerate().map(|(i, f)| {
        rsx! {
            SelectOption::<Option<Fruit>> { index: i, value: f, text_value: "{f}",
                disabled: matches!(f, Fruit::Orange),
                // The emoji is decoration: read "Apple", not "red apple Apple".
                span { "aria-hidden": "true", "{f.emoji()} " }
                "{f}"
            }
        }
    });

    rsx! {
        Select::<Option<Fruit>> {
            width: "12rem",
            aria_label: "Fruit",
            SelectGroup {
                SelectGroupLabel { "Fruits" }
                {fruits}
            }
            SelectGroup {
                SelectGroupLabel { "Other" }
                SelectOption::<Option<Fruit>> {
                    index: Fruit::COUNT,
                    value: None,
                    text_value: "Other",
                    "Other"
                }
            }
        }
    }
}
```
