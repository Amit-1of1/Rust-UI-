The Badge is a component designed to display small, distinct labels that help highlight important content or status indicators. Perfect for use cases like notifications, status labels, or categorization.

## Component Structure

```rust
Badge {
    {children}
}
```

## Colours, sizes and icons

`variant` picks the role: `Primary`, `Secondary` (a faded fill that draws less attention), `Outline`, `Destructive`, `Success` and `Warning`. The status colours come from the theme's success, warning and error tokens, and their text passes WCAG AA on the fill (`atoll contrast`). `size` is `Sm`, `Default` or `Lg`. Put a decorative icon before the text (`aria-hidden`); it follows the badge's text size and colour, so add words as well: colour and icon alone don't say what a badge means.

## Dismissible badges

`on_dismiss` adds a remove button after the text, named "Remove {dismiss_label}":

```rust
Badge { variant: BadgeVariant::Outline, dismiss_label: "Rust", on_dismiss: move |_| remove("Rust"), "Rust" }
```

Removing the badge removes the button that had focus, so move focus somewhere that stays, such as the next badge or the group around them (the demo focuses a `tabindex: "-1"` group). Otherwise keyboard users land back at the top of the page.

For a whole set of removable, selectable values with arrow-key navigation, use `TagGroup`.

## Dots and positioning

`dot: true` draws a round marker with no text. Give it an `aria-label` when it means something on its own ("Online"); without one it is decorative. `BadgeAnchor` pins a badge to a corner of another element, such as a count on a bell button. Put the count in the button's own name too, and hide the badge itself (`aria_hidden: "true"`) so it isn't read twice:

```rust
BadgeAnchor { badge: rsx! { Badge { size: BadgeSize::Sm, aria_hidden: "true", "3" } },
    IconButton { label: "Notifications, 3 unread", Bell { "aria-hidden": "true" } }
}
```
