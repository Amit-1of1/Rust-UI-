# NativeSelect

The browser's own `<select>`, styled like `Input`. Choose it over `Select` on phones (the OS picker), in long forms, for long option lists, and in server-rendered pages, where it works before the app has loaded. Use `Select` when the open list needs custom styling.

```rust
NativeSelect { id: "plan", value: plan(), onchange: move |e: FormEvent| plan.set(e.value()),
    NativeSelectOption { value: "starter", "Starter" }
    NativeSelectOption { value: "team", "Team" }
    NativeSelectGroup { label: "Legacy",
        NativeSelectOption { value: "pro-2024", disabled: true, "Pro (2024)" }
    }
}
```

Every `select` attribute (`name`, `value`, `required`, `disabled`, `multiple`, `aria-*`) goes to the `<select>`, and `class` is added to the kit's. Label it with a `Label { html_for }` or `aria_label`. `aria-invalid="true"` draws the error border.

## Accessibility

- Role and name: a native `select` (combobox or listbox), named by its label; options and `optgroup` labels are read by screen readers as usual. The chevron is `aria-hidden`.
- Keyboard: the browser's own (arrow keys, type to jump, Enter or Space to open).
- Reduced motion: no animation.

## Source

Original to this repo; shadcn/ui's `native-select` was the reference (not copied).

## Stability

Experimental. New on 2026-10-31.
