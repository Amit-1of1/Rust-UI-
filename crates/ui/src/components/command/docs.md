# Command

A command palette: a search box over a filtered list, inside a dialog. `filter_items` and `step_active` are pure helpers.

Open it with `use_hotkey("k", move || open.set(true))` for Ctrl+K (Cmd+K on macOS). That works on the web only, so keep a visible button too.

## Accessibility

- Role and name: a `combobox` input controlling a labelled `listbox`, with `aria-activedescendant` on the highlighted option
- Keyboard: Ctrl+K opens it (web, with `use_hotkey`); type to filter, Arrow keys move the highlight, Enter runs it, Escape closes
- Reduced motion: none
