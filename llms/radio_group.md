# Radio group

- Copy into your app: `rust-ui add radio_group` or `dx components add radio_group --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::radio_group::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The RadioGroup component is used to create a group of radio buttons that allows the user to select one option from a set.

## Component Structure

```rust
// The RadioGroup component wraps all radio items in the group.
RadioGroup {
    // The value property represents the currently selected radio button in the group.
    value: "option1",
    on_value_change: |value: String| {
        // This callback is triggered when the selected radio button changes.
        // The value parameter contains the value of the newly selected radio button.
    },
    // The RadioItem component represents each individual radio button in the group.
    RadioItem {
        // The index of the radio item, used to determine the order in which items are displayed.
        index: 0,
        // The value of the radio button, which is used to identify the selected option and will be passed to the on_value_change callback when selected.
        value: "option1",
        // The contents of the radio item button
        {children}
    }
}
```

## API

```rust
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element

#[component]
pub fn RadioItem(props: RadioItemProps) -> Element
```

## Example

```rust
use crate::components::radio_group::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        RadioGroup { aria_label: "Favourite colour",
            RadioItem { value: "option1".to_string(), index: 0usize, "Blue" }
            RadioItem { value: "option2".to_string(), index: 1usize, "Red" }
            RadioItem { value: "option3".to_string(), index: 2usize, disabled: true, "Green" }
        }
    }
}
```
