# List

A bulleted, numbered or plain list of items. It renders a real `<ul>` or `<ol>`, so screen readers announce "list, 3 items" and count as you move through it. Items hold any content, other components included, and lists nest.

```rust
List { marker: ListMarker::Number,
    ListItem { "Install the CLI" }
    ListItem { "Run " Kbd { "atoll create" } }
}
```

- `marker`: `Bullet` (default), `Number`, `Letter`, `Roman` or `None`. The ordered markers render an `<ol>`.
- `spaced: true` puts more room between items, for lists of rows or cards.
- `ListMarker::None` keeps the list role (`role="list"`), which Safari drops from lists drawn without markers.

## Component Structure

```rust
// A <ul> or <ol>, depending on the marker.
List { marker: ListMarker::Bullet, spaced: false,
    // One <li> per item; any content, nested lists included.
    ListItem { {children} }
}
```

## Accessibility

- Role and name: native `list` and `listitem`; give a plain list an `aria-label` when it is a group of its own ("Plans")
- Keyboard: none (not interactive); controls inside keep their own keys
- Reduced motion: none

## Source

Original to this repo.
