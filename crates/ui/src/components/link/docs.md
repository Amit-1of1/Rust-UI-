# Link

`TextLink`: an inline text link, a plain `<a href>` with an underline, the kit's text colour and a visible focus ring. `external: true` opens it in a new tab safely and says so; `current: true` marks the page you are on.

```rust
p { "Read the " TextLink { href: "/docs/theming", "theming guide" } "." }
TextLink { href: "https://lucide.dev", external: true, "Lucide" }
```

It is named `TextLink` so it never clashes with the router's `Link`.

## With the Dioxus router

Turn on the kit's `router` feature (`ui = { ..., features = ["router"] }`). Inside a `Router`, `TextLink`, `BreadcrumbLink` and `PaginationLink` then send a plain left click on a path `href` (`"/stays/3"`) through the router instead of loading the page again, add the router's base path to the rendered `href`, and `TextLink` marks itself `aria-current="page"` while its path is the current route. Ctrl/Cmd-click, middle-click, external links and `href`s the route enum doesn't parse stay ordinary links. `SidebarMenuButton` renders a button; for a route, pass the router's `Link` through its `as` prop. The helpers (`follow_in_app`, `in_app_href`, `is_current_route`) are in `ui::hooks` for your own links. For buttons that look like links, use `Button { variant: ButtonVariant::Link }`; for actions that do not navigate, use a button.

## Accessibility

- Role and name: a native link named by its text; external links add "(opens in a new tab)" (from `UiStrings`) and a hidden icon
- Keyboard: Tab reaches it, Enter follows it
- State: `aria-current="page"` when `current` (or, with the `router` feature, when its path is the current route), shown with a thicker underline, not colour alone
- Reduced motion: the hover underline changes without transition

## Source

Idea from Rust/UI `link.rs` (rust-ui/ui@43ddc65, MIT). Not copied: its `force_reload` mode renders a `<button>` that sets `window.location` through web-sys, which breaks middle-click, "open in new tab" and the link role, so it is left out; active-route matching (`PathMatchType`) is replaced by the explicit `current` prop.
