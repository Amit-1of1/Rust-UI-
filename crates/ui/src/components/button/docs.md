The button element is used to trigger actions or events in a user interface.

For a link that should look like a button (the router's `Link`, which renders its own `<a>`), spread `button_look(variant, size)` onto it: `Link { to: Route::Trips {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "My trips" }`. Don't also pass `class`. A link keeps link semantics: it navigates on Enter, not Space.

## Loading

`loading: true` keeps the button the same size and puts a spinner over its label. The label is
still read, `aria-busy="true"` says the action is under way, and the button stays focusable
(`aria-disabled`, not `disabled`) but ignores presses, so a submit button can't send its form
twice. Set it back to `false` when the work finishes.

```rust
Button { loading: saving(), onclick: move |_| save(), "Save" }
```

## Icon-only buttons

A button that shows only an icon is an `IconButton`. Its `label` is required (it becomes the `aria-label`), so an icon button with no name doesn't compile; `ButtonSize` has no icon sizes, so a plain `Button` can't be squared into one either. Mark the icon decorative.

```rust
IconButton { label: "Search", variant: ButtonVariant::Ghost, size: IconButtonSize::Sm,
    Search { "aria-hidden": "true" }
}
```

## Component Structure

```rust
button {
    // Global html attributes
    class: "dx-button",
    "data-style": "outline",
    "data-size": "default",
    // Children
    {children}
}
```
