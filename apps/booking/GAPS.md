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

## Fixed in the follow-up (PR #40)

| # | Severity | Gap | Fix |
|---|---|---|---|
| 10 | Medium | Kit links were plain `<a href>`: under the router a full page load. | Kit `router` feature: inside a `Router`, `TextLink`, `BreadcrumbLink` and `PaginationLink` send a plain left click on a path the route enum parses through the router, add the router's base path to the `href`, and `TextLink` marks the current route. Modified clicks, external links and unknown paths stay ordinary links. `SidebarMenuButton` takes the router's `Link` through `as`. The breadcrumb's `onclick` workaround is gone; `booking.mjs` checks the breadcrumb doesn't reload the page. |
| 11 | Medium | Nothing moved focus after an in-app navigation. | `MainContent` (`main id="main-content" tabindex="-1"`) takes focus after each route change with the `router` feature, not on the first load. The layout's effect is gone. |
| 12 | Medium | No number input. | `NumberInput` (`role="spinbutton"`, `aria-valuenow/min/max`, Up/Down, Page Up/Down, Home/End, -/+ buttons out of the Tab order) and `NumberField` for `use_form`. Guests use it; `booking.mjs` drives keys and buttons. |
| 13 | Medium | `DateRangePicker` couldn't be labelled. | Primitives fork `5b21f56`: an `aria-labelledby`/`aria-label` passed to the picker names its group, and segments drop the dangling `aria-labelledby`. The app's `role="group"` wrappers are gone. A `DateRangeField` for `use_form` is still to do. |
| 14 | Low | A fallback-only `Avatar` had no way to get a name. | `label` prop; without a name the avatar is decorative (`aria-hidden`). The app's `aria_hidden` is gone. |
| 15 | Low | A button inside `HoverCardTrigger` is nested-interactive. | Documented in `hover_card/docs.md`. |
| 16 | Low | `CarouselItem` centred a full-bleed photo to zero width. | `CarouselItem { media: true }`; the app CSS is gone. |
| 17 | Low | `Card` had no media slot; its radius ignored `--radius`. | `CardMedia`; the card radius is `2 × --radius`. The descendant selector and negative margins are gone. |
| 18 | Low | Date segments didn't move on after "01". | Primitives fork `5b21f56`: a segment that holds all its digits moves on. `booking.mjs` types "01". |
| 19 | Low | `Pagination` was link-only. | `Pager { page, pages, on_page_change }`: buttons, ends `aria-disabled`, long ranges collapse. The app's own pager is gone. |
| 20 | Low | The controlled `Select` wanted a memo. | `Select { selected: filters.read().sort }`. |
| 21 | Low | No skip link in the kit. | `SkipLink`; the app CSS is gone. |
| 22 | Low | Toasts were `role="alertdialog"`. | Primitives fork `5b21f56`: a named group with a `status` or `alert` message. `booking.mjs` checks the booking toast is a status. |

## Open

- A `DateRangeField` (date range bound to `use_form`) would let checkout validate dates like its other fields. The kit has one since 2026-10-30 (with `days_between`); checkout still keeps its own `trip_error` until it is moved over.

## Things that worked well

- Every kit component sat inside router layouts and routes without changes, and the router's `Link` already marks the current page.
- `use_form` + `TextField`/`SelectField`: per-step forms with focus-first-invalid and `aria-describedby` errors needed no extra code, and `field.value()` fed the booking straight in.
- `DateRangePicker`'s `min_date` and `disabled_ranges` covered "no past dates" and "already booked" directly.
- `RangeSlider`, `ToggleGroup` (multi-select) and `Checkbox` made the filter column in about 60 lines.
- Tabs, accordion and carousel passed axe in both themes with no app code beyond content.
- `CommandDialog` with 24 stays grouped by region was a dozen lines, and `use_hotkey` gave Ctrl+K.
- Every colour comes from theme tokens; dark mode needed no app CSS.
