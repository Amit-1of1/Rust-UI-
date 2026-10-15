# Screen readers: what is automated and what needs a person

No screen reader runs in CI or in the cloud sessions that build this kit. What a machine can check
is the accessibility tree the browser hands to a screen reader; what it cannot check is how a
screen reader turns that tree into speech and how it behaves in browse mode. This page splits the
two.

## Automated (scripts/e2e.sh, every engine in .github/workflows/e2e.yml)

| Check | Script |
| --- | --- |
| axe-core, WCAG 2 A/AA and best practices, light, dark, RTL, accent presets | `tests/e2e/axe.mjs` |
| Every control in the engine's own accessibility tree has a role and a name; checked, selected, pressed, expanded, value and heading level are exposed | `tests/e2e/a11y_tree.mjs` |
| Role, name and state of the main components, closed and opened from the keyboard, against snapshots in `tests/e2e/snapshots/aria/` | `tests/e2e/a11y_tree.mjs` |
| Combobox `aria-controls` points at its list; tooltip describes its trigger only while open; toasts land in a live region | `tests/e2e/a11y_tree.mjs` |
| Keys per WAI-ARIA APG pattern (tabs, accordion, checkbox, switch, radio, slider, dialog, menu, select, toggle) and focus return | `tests/e2e/keyboard.mjs` |
| Tab through the whole gallery: every stop visible and changing when focused, no positive tabindex, no trap, Shift+Tab retraces | `tests/e2e/walkthrough.mjs` |
| Forced colors (Windows High Contrast) keep control edges and focus outlines | `tests/e2e/forced_colors.mjs` (WebKit cannot emulate it and skips) |

Update the snapshots with `UPDATE_SNAPSHOTS=1 node a11y_tree.mjs` (in `tests/e2e`, against a
served gallery) and read the YAML diff before committing it: a changed role or name is an
announced change.

## Found and fixed by the automated pass (2026-10-14)

- The sidebar's show-on-hover "More" buttons had no focus ring and below 768px were never
  shown at all. They now get a ring, show on their own focus, and stay visible on narrow
  screens.
- The open Select list had no name; it is now labelled by the Select's label ("Fruit, list box").
- Pagination's "More pages" text sat inside `aria-hidden`, so the ellipsis item was an empty
  list item; it is read now.
- Demo names a reader would stumble on: "Demo Slider", "Progressbar Demo", toggle buttons named
  "B", "I", "U" and emoji read before each fruit name.

## Engine issue found and fixed: WebKit 26.0 wasm traps (2026-10-15)

- Playwright 1.56.1's WebKit (26.0, build 2215) trapped inside the apps' wasm at random:
  "Out of bounds memory access", "Unreachable code" (a Rust panic) or "access to a null
  reference", in a different place each time (the allocator, a component's render, the diff),
  always under a wasm-bindgen closure call. Split and unsplit builds alike, the gallery, the
  booking app and debug builds, never in Chromium or Firefox. It failed most WebKit CI jobs.
- `tests/e2e/start_stress.mjs` measured it: 18 traps in 1,250 gallery loads on WebKit 26.0, none
  in 450 loads on Playwright 1.63.0's WebKit 26.6 (build 2359), on the same build. Three full
  WebKit runs of `scripts/e2e.sh` on 26.6 had no trap either. A trap reading a null native
  address from Rust code, which never emits a null check, points at the engine's compiled code,
  not at the app.
- Fix: Playwright is pinned to 1.63.0. To check a future WebKit, run
  `BROWSER=webkit LOADS=150 GALLERY_URL=... node start_stress.mjs` in `tests/e2e`; every trap
  prints with its stack.

## Needs a person with a screen reader

Run each item with NVDA + Firefox and NVDA + Chrome on Windows, and VoiceOver + Safari on macOS
(VoiceOver on iOS for the touch items). Open the built gallery (`dx serve --platform web` in
`apps/gallery`). Tick an item only when every pairing passes; note the pairing and what was said
when one does not.

### Announcements

- [ ] **Select**: the trigger is a `<button aria-haspopup="listbox">`, not `role="combobox"` as
      in Radix. On focus, the label, the current value and "pop-up"/"menu pop-up" are read; after
      picking an option the new value is read.
- [ ] **Dropdown menu**: items are `role="option"`, not `menuitem` (docs/GUARDRAILS.md). Check it
      is announced as a list of choices users understand, with position ("2 of 5"); note whether
      NVDA switches to focus mode.
- [ ] **Combobox**: typing announces the number of results or the active option
      (`aria-activedescendant`), and an empty result says "No results".
- [ ] **Command palette** (Ctrl+K): opening announces the dialog and its search field; moving
      through results reads each item. Group headings ("File", "View") are plain text inside
      the listbox, not `group` roles, so a reader does not say the group with the item; decide
      whether that is good enough.
- [ ] **Tabs**: manual activation (arrows move, Enter/Space selects). The tab's selected state
      and "n of m" are read; the panel is announced with its tab's name.
- [ ] **Tooltip**: the description is read after the trigger's name on focus, and not read when
      the tooltip is closed.
- [ ] **Slider**: value changes are read as the arrows move it, with any units.
- [ ] **Date picker / calendar**: the grid reads the full date and the selected and today
      states; the date segment's dangling `aria-labelledby` (upstream bug, `KNOWN` in
      `crates/ui/tests/aria.rs`) does not cause a missing or wrong name.
- [ ] **Input OTP**: each box reads its position ("digit 2 of 6") and paste fills all boxes
      with one announcement, not six.
- [ ] **Field and validation**: an error appears and is read once (live region or on focus via
      `aria-describedby`), and `aria-invalid` is announced as "invalid entry".
- [ ] **Toast**: a toast is announced once, politely, without moving focus; an action toast's
      button can be reached (F6 / landmark navigation).
- [ ] **Progress, spinner, status**: progress values are read on request, not on every frame;
      a spinner is not announced repeatedly.
- [ ] **Charts**: read as one image with the `<title>` and the `<desc>` summary.
- [ ] **Data table**: column headers are read with each cell; the sort state ("sorted
      ascending") is read on the sorted header; row selection checkboxes read their row.

### Focus and navigation

- [ ] **Dialog, alert dialog, sheet, drawer**: on open, the screen reader reads the title (and
      the description for alert dialog), browse mode cannot leave the dialog (`aria-modal`), and
      focus returns to the trigger on close.
- [ ] **Popover, hover card**: content is reachable from the keyboard in browse mode and not
      read twice. The popover dialog takes its name from its trigger ("Open popover"), not its
      heading; check that reads sensibly.
- [ ] **Context menu**: reachable from the keyboard (Shift+F10 / Menu key) and announced as a
      menu.
- [ ] **Menubar**: the primitive nests each top-level item as `menubar > menu > menuitem`
      (APG has `menubar > menuitem[aria-haspopup]`). Check it is announced as a menu bar with
      "File, menu item, 1 of 2" and that arrow keys move between menus in focus mode.
- [ ] **Drag and drop list, swipe to dismiss, carousel**: the keyboard alternative is
      discoverable from what is read (instructions or button names).
- [ ] **Landmarks and headings**: the gallery and docs site can be navigated by landmark (D / R)
      and heading (H) with sensible names and no duplicates.

### Motion and settings

- [ ] With "reduce motion" on in the OS, nothing moves that a screen reader user would lose
      their place over (springs are instant).
- [ ] With Windows High Contrast on (real, not emulated), focus and control edges stay visible.

## Results

Record runs here: date, pairing (reader + browser + OS), who ran it, items that failed.

| Date | Pairing | By | Failed items |
| --- | --- | --- | --- |
| | | | |
