# Command

A command palette: a search box over a filtered list, inside a dialog. `filter_items` and `step_active` are pure helpers.

Pass `hotkey: "k"` to open it with Ctrl+K (Cmd+K on macOS); it uses `use_hotkey` from the `hooks` item, which `atoll add command` installs with it. That works on the web only, so keep a visible button too.

## Accessibility

- Role and name: a `combobox` input controlling a labelled `listbox`, with `aria-activedescendant` on the highlighted option
- Reduced motion: none
