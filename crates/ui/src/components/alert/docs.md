# Alert

A message about the whole page or one part of it. By default it interrupts: it is announced as soon as it appears. Use `Callout` for a quiet inline note.

```rust
Alert { variant: AlertVariant::Destructive, icon: rsx! { CircleAlert {} },
    AlertTitle { "Payment failed" }
    AlertDescription { "Your card was declined." }
    AlertActions { Button { size: ButtonSize::Sm, "Update card" } }
}
```

- `variant`: `Default`, `Destructive`, `Success`, `Warning` or `Info`. The status colours come from the theme's tokens and pass WCAG AA.
- `icon`: an icon in its own column before the text (hidden from screen readers; the title carries the meaning). Use one for status variants so colour isn't the only cue.
- `AlertTitle` for a short summary, `AlertDescription` for the details, `AlertActions` for buttons that answer the message.
- `live`: `Assertive` (`role="alert"`, the default) for errors, `Polite` (`role="status"`) for good news, `Off` for a message that is part of the page from the start.
- The alert fills its container's width, so on a phone it is full width.

## Accessibility

- Role and name: `role="alert"`, `role="status"` or none, from `live`
- Keyboard: none of its own; actions inside are ordinary buttons
- Reduced motion: none
