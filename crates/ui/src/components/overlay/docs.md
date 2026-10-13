# Overlay

`ModalPortal` and `OverlayLayer`: the plumbing that makes full-window overlays (dialog, alert dialog, sheet, drawer, command palette) cover the window on the native renderer. On the web and the desktop webview both do nothing and the markup stays where it is.

Blitz, the native renderer, treats `position: fixed` like `absolute` relative to the *parent* element, so a backdrop with `inset: 0` covers only the box it sits in. With the crate's `native` feature, `ModalPortal` moves its children into `OverlayLayer`, a window-sized box rendered once at the root (`ThemeStyles` does it for you), where `fixed; inset: 0` means the window again.

Use `ModalPortal` around your own fixed-position overlays. Copied into an app, the components check the app crate's own `native` feature, so a native app declares `native = []` in its `[features]` and turns it on.

On native, portalled content sees the contexts provided above the layer and inside the portal, not the ones in between. Provide what a modal needs inside it, or above `ThemeStyles`.

## Accessibility

- Role and name: none of its own; the portalled content keeps its roles.
- Keyboard: unchanged. On native the content moves to the end of the document, after the rest of the page in tab order; the kit's modals are the only thing interactive while open.
- Reduced motion: no animation of its own.

## Source

Original to this repo.
