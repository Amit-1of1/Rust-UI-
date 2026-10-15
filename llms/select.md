# Select

- Copy into your app: `rust-ui add select` or `dx components add select --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::select::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Select component is used to create a dropdown menu that allows users to select one or more options from the select groups.

## Component Structure

```rust
Select::<String> {
    value: "option1",
    on_value_change: |value: String| {
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

## Labelling

Give every Select a name. `aria_label` or `aria_labelledby` on `Select` (or `SelectMulti`) is moved to the trigger button, and the trigger adds itself to `aria-labelledby`, so screen readers announce the label and the current value ("Fruit, Apple").

```rust
// A visible label.
Label { id: "fruit-label", "Fruit" }
Select::<String> { aria_labelledby: "fruit-label", /* options */ }

// No visible label.
Select::<String> { aria_label: "Fruit", /* options */ }
```

## API

```rust
pub use dioxus_primitives::select::SelectGroup;

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
