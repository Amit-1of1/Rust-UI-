# Focus return without a script

The dialog and popover primitives return focus on the web by remembering
`document.activeElement` in a script. Native (Blitz) has no script and Dioxus has no "which
element has focus" API, so after Escape focus stayed on the removed panel.

`hooks::focus_return` does it with element handles instead:

- Openers (`Button`, `IconButton`, `PopoverTrigger`, the colour picker trigger) spread
  `use_focus_opener()`: on focus they become the page's current opener, on blur they stop
  being it. So at any moment the opener is the focused kit control, or nobody.
- Overlays (dialog, alert dialog, sheet, drawer, popover and everything built on them) spread
  `use_return_focus()`'s attributes on the panel, which track `focusin`/`focusout`, and render
  `ReturnFocus` inside the panel. It takes the opener when the panel mounts (before the focus
  trap moves focus in) and, when it unmounts with focus still inside, focuses the opener
  from a `spawn_forever` task (its own scope is gone by then).
- A close that moved focus elsewhere first (a click on another control) doesn't steal it
  back. An overlay opened by something that isn't an opener (the app's own `button`) gets
  no return on native; on the web the primitive's script still does it.

`apps/native/tests/keyboard_fuzz.rs` (no lost focus after Escape) and the contracts' Escape
rows prove it on headless Blitz. If you add an overlay, use the same two hooks; if you add a
control that opens one, spread `use_focus_opener()` on it.
