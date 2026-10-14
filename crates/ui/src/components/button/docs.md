The button element is used to trigger actions or events in a user interface.

For a link that should look like a button (the router's `Link`, which renders its own `<a>`), spread `button_look(variant, size)` onto it: `Link { to: Route::Trips {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "My trips" }`. Don't also pass `class`. A link keeps link semantics: it navigates on Enter, not Space.

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
