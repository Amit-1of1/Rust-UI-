# Gaps found building Acme Admin

`apps/dashboard` is the second real app on the kit (the first is `apps/trial`). It is a small admin dashboard: a collapsible sidebar, KPI cards, two charts, a customers table with badges and a row menu, an edit sheet, add and delete dialogs, an orders `DataTable` with date-range and status filters, a settings form with a combobox and switches, the theme customizer, toasts, and a Ctrl+K command palette. `tests/e2e/dashboard.mjs` runs axe on every page and overlay in light and dark (0 violations) and drives the add, edit, delete, filter and palette flows.

Severity: **High** means a real app hits it early and the result is broken or inaccessible. **Medium** means a workaround is needed and is easy to get wrong. **Low** means friction or polish.

## Fixed in the kit (this PR)

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

## Open (reported, not fixed here)

| # | Severity | Gap | Workaround in the app | Suggested fix |
|---|---|---|---|---|
| 10 | High | Most components spread `..attributes` after their own `class`, which renders two `class` attributes. In the browser the caller's wins and the kit's styling is gone; in SSR HTML the first wins. Checked: `Input`, `Textarea`, `Kbd`, `Breadcrumb`, `Table`, `TableCell`, `Badge`, `Empty`, `Spinner`, `SheetHeader`, `Pagination`, `Label` (Label the other way round: the caller's class is dropped). `Button` and `Separator` merge correctly. | Wrap the component in a `div` with the class instead. | One sweep with the `merge_class` helper from `card` (or `merge_attributes`), plus a test that renders each component with `class: "x"`. It touches many files that parallel work is changing, so it should be its own PR. |
| 11 | Medium | The `Select` trigger can't be labelled: attributes land on the wrapper, not the `<button>`. | `div { role: "group", aria-labelledby }` around it (also found by `apps/trial`). | A styled `Select` props struct with `aria_label` / `aria_labelledby` / `trigger_id` forwarded to `SelectTrigger`. |
| 12 | Medium | `SidebarInset` renders `<main>`, and `Sidebar` is a plain `div`. An app that adds its own `main` gets duplicate-landmark errors, and the sidebar's content is outside any landmark (axe `region`). | No inner `main`; `Sidebar { role: "navigation", aria_label: "Main" }`. | Document that `SidebarInset` is the page's `main`, and give `Sidebar` a navigation landmark by default (check the gallery demo with axe). |
| 13 | Medium | Toasts are `role="alertdialog"` (dioxus-primitives). Screen readers announce each as a dialog, and `getByRole("alertdialog")` in tests matches both a confirm dialog and a toast. | Tests match dialogs by name. | Report upstream; `role="status"` (or `alert` for errors) is the usual pattern. Also report the AlertDialogAction close-then-call order. |
| 14 | Medium | Light/dark has two sources of truth: the header switch writes `data-theme` with web-sys, and `ThemeCustomizer` writes it too. Changing the mode in one leaves the other showing the wrong state. | None (visible in Settings). | A `use_theme()` signal (mode, accent, radius, density) that both `ThemeCustomizer` and app code read and write. |
| 15 | Medium | Only text fields join `use_form` validation. A combobox or select can't be marked required, isn't focused on a failed submit, and its error isn't wired with `aria-describedby`. | Hand-written `role="alert"` message under the time-zone combobox. | Let `FormField` back any control (an `Option<String>` value), or add `SelectField` / `ComboboxField` wrappers like `TextField`. |
| 16 | Low | No initial values for form fields; `reset()` empties instead of restoring. | `use_hook` to `set` defaults; an effect to load the edited record. | `form.field_with(initial, rules)` and `reset()` back to the initial value. |
| 17 | Low | Controlled `Select` wants `Option<ReadSignal<Option<T>>>`, so a plain `Signal<String>` needs a `use_memo` adapter. | `use_memo(move || Some(plan()))`. | Accept `Option<T>` via `into` like `RadioGroup`. |
| 18 | Low | `DataTable`'s filter box is its own toolbar row; outside filters (date range, status) can't sit next to it. | Two toolbars. | A `toolbar: Element` slot rendered beside the filter. |
| 19 | Low | Charts are single-series with no hover values, legend or accent colour; bars are solid `currentColor`. Fine for a KPI page, thin for analytics. | None. | Use `--accent-color` for marks; multi-series and a focusable data table fallback later. |
| 20 | Low | `toasts.success(...)` takes `String`, so every call needs `.to_string()`. | `.to_string()`. | Upstream API; a kit helper taking `impl Into<String>` would do. |
| 21 | Low | No `sr-only` utility class in the theme; each app writes its own. | `.db-sr-only` in the app CSS. | Ship one in `theme.css`. |
| 22 | Low | Icons need a direct `dioxus-icons` dependency in the app. | Added it. | Re-export as `ui::icons` (it is already wrapped per GUARDRAILS' one-maintainer rule in spirit). |
| 23 | Low | The kit's `demos` module is always compiled into the library, so every app ships demo code and demo CSS (`sidebar/demo.css` is in this app's bundle). | None. | Put `demos` behind a feature that only the gallery, docs and tests enable (the size thread may already cover this). |
| 24 | Low | Sortable header text sits a few pixels right of the cell text below it (the sort button's padding). | None. | Negative inline margin on the sort button, or matching cell padding. |

## Things that worked well

- `SidebarProvider` with `collapsible: Icon` gave collapse, Ctrl+B, tooltips in icon mode and a mobile sheet for free.
- `CommandDialog` + `use_hotkey` made the palette a dozen lines, groups included.
- `use_form` + `TextField`: errors, `aria-invalid`, `aria-describedby` and focus-first-invalid on submit worked with no extra code.
- `table_view` being a pure function made the composed customers table straightforward once the markup was copied.
- Every colour comes from theme tokens, so dark mode and the accent presets needed no app CSS.
