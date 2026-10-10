# Command

A command palette: a search box over a filtered list, inside a dialog. `filter_items` and `step_active` are pure helpers.

## Accessibility

- Role and name: a `combobox` input controlling a labelled `listbox`, with `aria-activedescendant` on the highlighted option
- Keyboard: type to filter, Arrow keys move the highlight, Enter runs it, Escape closes
- Reduced motion: none
