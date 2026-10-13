# Icons

The kit's one icon set: Lucide (1,700+ icons) through the `dioxus-icons` crate, re-exported here so apps and components import icons from one place.

```rust
use ui::icons::{Search, Trash};

rsx! {
    // Decorative: the button text names the action, so hide the icon.
    button { Trash { size: "1rem", "aria-hidden": "true" } "Delete" }
    // Icon-only: name the control, not the SVG.
    button { "aria-label": "Search", Search { size: "1rem", "aria-hidden": "true" } }
}
```

Icons draw with `currentColor`, so they follow the text colour in both themes. `size` sets width and height (a number of pixels or any CSS length). Any SVG attribute passes through: `stroke_width`, `class`, `style`.

Import icons by path (`ui::icons::Check`), not with a glob: Lucide has icons named `Link`, `Badge`, `Calendar` and so on that would clash with the components.

## Accessibility

- Role and name: icons carry no name of their own. Mark decorative icons `"aria-hidden": "true"`; put the name on the control that holds an icon-only button (`aria-label`) or next to it as text.
- Keyboard: not focusable.
- Reduced motion: icons do not animate.

## Source

`dioxus-icons` 0.1.0 (DioxusLabs, MIT; Lucide icons ISC). The choice and the alternatives are in `docs/ICONS.md`.
