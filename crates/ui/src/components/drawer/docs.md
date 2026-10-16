# Drawer

A panel that slides up from the bottom and can be dragged down to dismiss.

```rust
Drawer {
    open: open(),
    on_open_change: move |v| open.set(v),
    title: "Move goal", // required
    description: "Set your daily goal.",
    DrawerClose { "Close" }
}
```

It is built on the dialog primitive plus the motion layer's vertical `SwipeDismiss` (`vertical` and `one_way`): pull down far enough, or flick, and it springs away; release early and it springs back.

## Accessibility

- Role and name: a modal `dialog` named by its `title` (a required prop: leaving it out is a compile error; `hide_title` hides it visually) and described by `description` or a `DrawerDescription` child; the handle is decorative (`aria-hidden`).
- Keyboard: focus is trapped inside; Escape closes; the Close button always works, so dragging is never required.
- Reduced motion: the slide-in is removed and the drag release becomes a near-instant tween.

## Source

Original to this repo, on `dioxus-primitives` dialog and `crates/motion`. Rust/UI's `drawer.rs` was only used to see the feature set.
