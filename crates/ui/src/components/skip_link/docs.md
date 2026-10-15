# SkipLink

`SkipLink` is a "Skip to content" link that stays hidden until it has keyboard focus, then appears in the corner, so keyboard users can jump past the header and navigation. `MainContent` is the page's `<main>` landmark it points at.

```rust
rsx! {
    SkipLink {}
    header { /* logo, nav */ }
    MainContent { Outlet::<Route> {} }
}
```

`SkipLink` points at `#main-content` by default (`target` changes it) and reads `UiStrings::skip_to_content` unless it has children. It is positioned against the nearest positioned ancestor, usually the page. `MainContent` renders `main id="main-content" tabindex="-1"`, so following the skip link moves focus into it.

With the kit's `router` feature, `MainContent` inside a `Router` also takes focus after every in-app navigation (not on the first load). Keyboard and screen reader users then start at the new page, and a screen reader reads the main landmark, instead of staying on the link they used. Set the page's title (`document::Title`) and give each page a heading.

## Accessibility

- Role and name: `SkipLink` is a native link named by its text; `MainContent` is the `main` landmark
- Keyboard: the skip link is the first Tab stop; Enter moves focus to the main content
- Focus: shown with the kit's focus ring when focused; `MainContent` shows no ring, since it is not a control
- Reduced motion: nothing animates

## Source

Original to this repo (booking app gaps 11 and 21).

## Stability

`SkipLink` and `MainContent` are stable; the focus move after a route change (the `router` feature) is experimental (docs/STABILITY.md).
