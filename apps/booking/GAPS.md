# Gaps found building Trailhead Stays

`apps/booking` is the third real app on the kit, after `apps/trial` (Taskboard) and `apps/dashboard` (Acme Admin). It is a different kind of app: a routed, content-heavy booking site with long forms. Explore has a search box, a date range, a guest counter, filters (price range slider, type toggles, amenity checkboxes), sort and paging. A stay's page has a breadcrumb, a photo carousel, tabs, an accordion, a host hover card and a booking box with a live price breakdown. Checkout is three steps (stepper, validated forms, rate radio group, required terms checkbox, card fields). My trips has tabs and a cancel confirmation. Ctrl+K searches every stay. It is the first app on the Dioxus router.

`tests/e2e/booking.mjs` runs axe on every route and overlay in light and dark (0 violations) and drives search, filters, sort, paging, the palette, booking, all three checkout steps with their errors, and cancelling.

Severity: **High** means a real app hits it early and the result is broken or inaccessible. **Medium** means a workaround is needed and is easy to get wrong. **Low** means friction or polish.

## Fixed in the kit (this PR)

| # | Severity | Gap | Fix |
|---|---|---|---|
| 1 | High | Typing a complete start date into a `DateRangePicker` froze the page. The range input and its two date inputs copied values into each other's signals, and a signal write notifies even when the value is unchanged, so the effects looped forever. No demo or test typed into a range picker, so nothing caught it. | Fixed on the primitives fork (`set_if_changed`, commit `71fed69`, see docs/FORKS.md) and the pin bumped. The date picker demo now also shows a range picker; `booking.mjs` types both dates. |
| 2 | High | `ItemGroup` is `role="list"` but its `Item`s had no `listitem` role: axe `aria-required-children` (critical) on every grouped list. | An `Item` inside an `ItemGroup` is `role="listitem"`. Render test `item_in_a_group_is_a_list_item`. |
| 3 | Medium | `Stepper` could not be driven from outside (only `default_step`), and step content could only live in its `footer`. A checkout that moves on only after a form validates had no clean way to do it. | New `current: Signal<usize>` prop; the panels can sit outside the stepper. Render test `stepper_follows_a_controlled_step`. |
| 4 | Medium | A checkbox could not join `use_form`. "Accept the terms" needed its own signal and message, its error only appeared once every other field was valid, and a failed submit didn't focus it. | `CheckboxField` (value `"true"` or empty, so a rule makes it required). |
| 5 | Medium | No textarea bound to a form field. | `TextareaField`. |
| 6 | Medium | `TextField` dropped every attribute it did not name, so a card number could not get `inputmode="numeric"`, `maxlength` or `pattern` (phones showed the full keyboard). | `TextField` and `TextareaField` pass other attributes to their control; the field's own id, value and ARIA wiring win. Render test `form_text_field_passes_extra_attributes_to_its_input`. |
| 7 | Medium | A router `Link` that should look like a button ("Back to all stays", "Explore stays") could not borrow the kit's button style: the class is a private, hashed CSS-module name. | `button_look(variant, size)` returns the class and data attributes to spread onto any element (`Link { to, attributes: button_look(..) }`). |
| 8 | Low | `Accordion` set an inline `width: 15rem`, so it never filled its column. | Removed; it fills its container (the demos already cap their width). Snapshot updated. |
| 9 | Medium | Test bug, not kit: `dashboard.mjs` clicked the "Dark mode" switch for its dark pass, but under a dark system scheme the switch is already on, so the click turned dark mode **off** and the "dark" axe scans ran on the light theme. | Both scripts now check that the switch follows the system and don't click it. |

## Open (reported, not fixed here)

| # | Severity | Gap | Workaround in the app | Suggested fix |
|---|---|---|---|---|
| 10 | Medium | Kit links are plain `<a href>`: `BreadcrumbLink`, `TextLink`, `PaginationLink`, `SidebarMenuButton`. Under the router a plain `href` is a full page load, and on a static host a deep path 404s. | `onclick` + `prevent_default()` + `navigator().push(..)` on `BreadcrumbLink`; the router's own `Link` with app CSS for nav links (it sets `aria-current="page"` itself). | A `router` feature with thin wrappers that render the router's `Link` with the kit's look, or an `as` render prop on the link components. |
| 11 | Medium | Nothing moves focus or announces the page after an in-app navigation; keyboard and screen reader users stay on the link they used. | An effect in the layout focuses `<main tabindex="-1">` when the route changes (not on first load). `booking.mjs` checks it. | A small `RouteFocus` helper (or a documented pattern) next to the router feature. |
| 12 | Medium | No number input: a guest count needs a spin button with -/+, min/max and arrow keys. | Two icon `Button`s and an `<output aria-live>` inside a labelled group. | `NumberField` (`role="spinbutton"`, `aria-valuenow/min/max`, Up/Down/Home/End, -/+ buttons, `FormField` binding). |
| 13 | Medium | `DateRangePicker` can't be labelled. Its group has a hard-coded English `aria-label="Date Range"` (not from `UiStrings`), and each segment's `aria-labelledby` points at an id that doesn't exist (`span-dxc-2-label`), so it falls back to "year", "month", "day". Same for `DatePicker` ("Date"). Also seen in the dashboard. | A `role="group"` wrapper with `aria-labelledby` on the visible label. | Fork fix: take the name from props or `UiStrings`, and drop the dangling `aria-labelledby`. Then a `DateRangeField` for forms. |
| 14 | Low | `Avatar` is always `role="img"` and a fallback-only avatar (initials) has no way to get a name: axe `role-img-alt`. | `aria_hidden: "true"` where the name is shown beside it (host card, reviews). | A `label` prop for fallback avatars, or render decorative avatars `aria-hidden` by default. |
| 15 | Low | `HoverCardTrigger` is itself `role="button"` with `tabindex="0"`; a button inside it is axe `nested-interactive`. | Text directly inside the trigger. | Say so in `hover_card/docs.md` (or render a real `<button>` as the trigger). |
| 16 | Low | `CarouselItem` centres its content (flex), so a full-bleed photo has zero width until the app sets `width: 100%`. | App CSS. | A `media` variant, or `align-items: stretch` with the content filling the slide. |
| 17 | Low | `Card` has no media slot: an edge-to-edge image means undoing the card's padding, and the kit's CSS loads after the app's, so one app class doesn't win. The card radius is `1rem`, not `var(--radius)`. | A descendant selector (`.tr-grid .tr-card`) and a negative margin. | `CardMedia`, and `--radius` for the card. |
| 18 | Low | The date segments don't move on after "0" + a second digit for months or days ("09" stays on the month). | The test focuses each segment; people press Tab or the right arrow. | In the fork: advance when the segment has `max_length` digits. |
| 19 | Low | `Pagination` is link-only. Client-side paging needs `href: "#"` plus `prevent_default()`, and Previous/Next on the first/last page stay focusable and clickable. | `aria_disabled` and a guard in `onclick`. | `on_page_change` + `page`/`pages` props that render buttons and disable the ends. |
| 20 | Low | The controlled `Select` still wants `ReadSignal<Option<T>>` (dashboard gap 17). | `use_memo(move || Some(filters.read().sort))`. | As in the dashboard's GAPS.md. |
| 21 | Low | No skip link in the kit. | `a.tr-skip` with app CSS. | A `SkipLink` component using the global `.sr-only` pattern on focus. |
| 22 | Low | Toasts are still `role="alertdialog"` (dashboard gap 13). | Tests match dialogs by name. | As in the dashboard's GAPS.md. |

## Things that worked well

- Every kit component sat inside router layouts and routes without changes, and the router's `Link` already marks the current page.
- `use_form` + `TextField`/`SelectField`: per-step forms with focus-first-invalid and `aria-describedby` errors needed no extra code, and `field.value()` fed the booking straight in.
- `DateRangePicker`'s `min_date` and `disabled_ranges` covered "no past dates" and "already booked" directly.
- `RangeSlider`, `ToggleGroup` (multi-select) and `Checkbox` made the filter column in about 60 lines.
- Tabs, accordion and carousel passed axe in both themes with no app code beyond content.
- `CommandDialog` with 24 stays grouped by region was a dozen lines, and `use_hotkey` gave Ctrl+K.
- Every colour comes from theme tokens; dark mode needed no app CSS.
