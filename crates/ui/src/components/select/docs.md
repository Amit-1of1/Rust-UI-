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
