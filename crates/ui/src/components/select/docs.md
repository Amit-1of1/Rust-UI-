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

## Start slot and errors

`Select { start: rsx! { ShoppingBasket {} } }` puts an icon or short text before the value; it is hidden from screen readers. When the trigger has `aria_invalid: "true"` (or sits in a `Field` with an error) it gets the error outline and an alert icon.
