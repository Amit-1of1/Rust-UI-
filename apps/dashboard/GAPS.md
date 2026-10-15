# Gaps found building Acme Admin

`apps/dashboard` is the second real app on the kit (the first is `apps/trial`). It is a small admin dashboard: a collapsible sidebar, KPI cards, two charts, a customers table with badges and a row menu, an edit sheet, add and delete dialogs, an orders `DataTable` with date-range and status filters, a settings form with a combobox and switches, the theme customizer, toasts, and a Ctrl+K command palette. `tests/e2e/dashboard.mjs` runs axe on every page and overlay in light and dark (0 violations) and drives the add, edit, delete, filter and palette flows.

Severity: **High** means a real app hits it early and the result is broken or inaccessible. **Medium** means a workaround is needed and is easy to get wrong. **Low** means friction or polish.

## Fixed in the kit (PR #22)

| # | Severity | Gap | Fix |
|---|---|---|---|
| 1 | High | `FormField::set` read `touched` reactively. Loading a record into a form from an effect subscribed that effect to the field, so blurring the field (clicking Save) re-ran the effect and silently threw away the user's edit. | `set` reads with `peek()`. The edit-sheet flow in `dashboard.mjs` covers it. |
| 2 | High | Passing `class` to `Card` (and its parts) replaced the card's own class, so the card lost its border, padding and background. | `Card*` add the caller's class to their own. Render test `card_keeps_its_own_class_next_to_the_callers`. The same bug is in most other components; see open gap 10. |
| 3 | High | `SheetContentClose` (the X button, also used by the mobile sidebar) had no accessible name: axe `button-name`, critical. | It is named from a new `UiStrings::close` ("Close"); the icon is `aria-hidden`. |
| 4 | High | `DataTable` cells were text only. Any status badge, link or row menu meant dropping `DataTable` and rewriting its header sort wiring, paging and status line (about 120 lines in `customers.rs`). | New `render_cell: |(row, col)| rsx!{..}` prop. Text in `rows` still drives filter and sort. Orders uses it for status badges. |
| 5 | Medium | `DataTable` keeps page and selection as row indices. When the parent filtered `rows`, the selection silently pointed at other rows. | Both reset when `rows` changes. |
| 6 | Medium | `SidebarMenuButton` and `BreadcrumbLink` took no `onclick`, so in-app navigation without a router needed the `as:` render-prop escape hatch. | Both take `onclick`. |
| 7 | Medium | The active `SidebarMenuButton` was marked only with `data-active`; screen readers were not told which page is current. | `is_active` also sets `aria-current="page"`. |
| 8 | Medium | Value types were not exported: naming a date range or a checkbox state needed direct `dioxus-primitives` and `time` dependencies. | `DateRange`, `Date`, `Month` and `CheckboxState` are in the prelude. |
| 9 | Medium | `AlertDialogAction` closes the dialog *before* calling `on_click`. Clearing "which item to delete" in `on_open_change(false)` means `on_click` sees nothing and the delete silently does nothing. | Documented in `alert_dialog/docs.md`; the app keeps the target and the open flag in separate signals. The order itself is upstream behaviour (see open gap 13). |

## Closed since (follow-up PR)

The app now uses each fix instead of its workaround, and `tests/e2e/dashboard.mjs` still passes axe on every page and overlay in light and dark.

| # | Severity | Gap | Fix |
|---|---|---|---|
| 10 | High | Passing `class` to most components rendered two `class` attributes, so either the kit's styling or the caller's class was lost. | Every styled component merges its class with the caller's through `merge_attributes` (about 130 call sites in 50 components). `tests/class.rs` renders 77 components with a class and fails on a duplicate `class` attribute or a dropped kit class. The app's `Form { class }` and `TableCell { class }` now keep their kit styling. |
| 11 | Medium | The `Select` trigger couldn't be labelled. | Already fixed by PR #23 (`aria_label` / `aria_labelledby` move to the trigger). The app's `role="group"` wrappers are gone. |
| 12 | Medium | `Sidebar` was a plain `div`; `SidebarInset` being the page's `main` was undocumented. | `Sidebar` is a navigation landmark named "Sidebar" (`UiStrings::sidebar`; `aria_label` renames it, `role` overrides it), on desktop, static and mobile. `SidebarInset` is documented as the page's `main`. Render test `sidebar_is_a_navigation_landmark_that_can_be_renamed`. |
| 14 | Medium | Light/dark had two sources of truth. | `use_theme()`: one shared mode / accent / density / radius state with `is_dark()`, read and written by `ThemeCustomizer` and app code. The header switch uses it; `dashboard.mjs` checks the switch and the customizer agree both ways. |
| 15 | Medium | Only text fields joined `use_form`. | `SelectField` and `ComboboxField` bind a `Select` or `Combobox` to a `FormField`: label, `aria-invalid`, error in `aria-describedby`, focus on a failed submit. `Select` and `Combobox` now move `id` and those attributes to their focusable control, and `Select` takes a placeholder. Settings uses `ComboboxField` (no hand-written alert); the edit sheet uses `SelectField`. `tests/e2e/form_fields.mjs` covers it in the gallery. |
| 16 | Low | No initial values; `reset()` emptied. | `form.field_with(initial, rules)`; `field.load(value)` for loading a record (sets the value and what `reset()` returns to, clears the error). |
| 18 | Low | Outside filters couldn't sit next to `DataTable`'s filter box. | `toolbar: rsx! { .. }` slot beside the filter box (shown even with `filterable: false`). Orders puts its date range and status toggles there; the gallery demo has a "Research only" toggle. |
| 19 | Low | Chart marks were solid `currentColor`, no hover values. | Marks use `--accent-color`; each bar and point has an SVG `<title>` with its value, shown on hover. Multi-series and a legend are still open (below). |
| 20 | Low | `toasts.success(...)` needed `String` and `ToastOptions`. | `ToastsExt` (prelude): `toasts.notify_success("Saved")`, or a `(title, description)` pair. |
| 21 | Low | No `sr-only` class. | `.sr-only` in `theme.css`; the icons demo uses it, the app's own copy is gone. |
| 22 | Low | Icons needed a direct `dioxus-icons` dependency. | `ui::icons` already re-exported them; the app's direct dependency is removed. |
| 23 | Low | The `demos` module was compiled into every app. | `ui::demos` is behind the kit's `demos` feature, turned on by the gallery, docs, `apps/native` and the kit's tests. The dashboard bundle no longer carries demo code or `sidebar/demo.css`. |
| 24 | Low | Sortable header text didn't line up with the cells below it. | Measured in the gallery: start-aligned columns already lined up (the button's negative start margin); right-aligned numeric columns ended 22 px short (the button's end padding and arrow). `TableHead` and `TableCell` take `numeric: true` (end-aligned, tabular figures; the sort arrow goes before the text and the end padding is pulled back). `DataTable` uses it for numeric columns; `tests/e2e/table.mjs` checks both edges to 1 px. |
| 13 | Medium | Toasts were `role="alertdialog"`. | Fixed on the primitives fork (`5b21f56`, docs/FORKS.md): a toast is a named, focusable group, and its message a `status` (info, success) or `alert` (error, warning). `dashboard.mjs` checks no alertdialog appears with the "Customer deleted" toast. (PR #PRNUM) The AlertDialogAction close-then-call order (gap 9) is unchanged. |
| 17 | Low | A controlled raw `Select` needed a `use_memo` adapter. | `Select` takes `selected: T`, a plain value the parent passes on each render; `value` still takes a signal. (PR #PRNUM) |

## Open (reported, not fixed here)

| # | Severity | Gap | Workaround in the app | Suggested fix |
|---|---|---|---|---|
| 19 | Low | Charts are single-series with no legend. | None. | Multi-series with a legend, and a focusable data-table fallback. |

## Things that worked well

- `SidebarProvider` with `collapsible: Icon` gave collapse, Ctrl+B, tooltips in icon mode and a mobile sheet for free.
- `CommandDialog` + `use_hotkey` made the palette a dozen lines, groups included.
- `use_form` + `TextField`: errors, `aria-invalid`, `aria-describedby` and focus-first-invalid on submit worked with no extra code.
- `table_view` being a pure function made the composed customers table straightforward once the markup was copied.
- Every colour comes from theme tokens, so dark mode and the accent presets needed no app CSS.
