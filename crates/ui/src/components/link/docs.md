# Link

`TextLink`: an inline text link, a plain `<a href>` with an underline, the kit's text colour and a visible focus ring. `external: true` opens it in a new tab safely and says so; `current: true` marks the page you are on.

```rust
p { "Read the " TextLink { href: "/docs/theming", "theming guide" } "." }
TextLink { href: "https://lucide.dev", external: true, "Lucide" }
```

It is named `TextLink` so it never clashes with the router's `Link`. For buttons that look like links, use `Button { variant: ButtonVariant::Link }`; for actions that do not navigate, use a button.

## Accessibility

- Role and name: a native link named by its text; external links add "(opens in a new tab)" (from `UiStrings`) and a hidden icon
- Keyboard: Tab reaches it, Enter follows it
- State: `aria-current="page"` when `current`, shown with a thicker underline, not colour alone
- Reduced motion: the hover underline changes without transition

## Source

Idea from Rust/UI `link.rs` (rust-ui/ui@43ddc65, MIT). Not copied: its `force_reload` mode renders a `<button>` that sets `window.location` through web-sys, which breaks middle-click, "open in new tab" and the link role, so it is left out; active-route matching (`PathMatchType`) is replaced by the explicit `current` prop.
