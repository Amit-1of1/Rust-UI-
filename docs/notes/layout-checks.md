# WCAG layout checks on the headless renderer

`apps/native/tests/layout.rs` renders every demo on Blitz (no browser) and checks WCAG 2.2 layout rules. It runs in `cargo test -p native` (so `scripts/check.sh` and CI run it) and takes about two minutes.

| Test | Rule | Fails on |
|---|---|---|
| `targets_are_at_least_24px_or_spaced` | 2.5.8 Target Size | a control, link or widget-role element under 24x24 (its `::before`/`::after` box counts) whose 24px circle touches another target |
| `focus_is_never_hidden_behind_other_content` | 2.4.11 Focus Not Obscured | Tab through the demo at 1024px and 320px; a focused element, scrolled into view the way a browser does, that is entirely covered (hit testing at 25 points) |
| `focus_clears_the_showcase_tab_bar_on_a_phone` | 2.4.11 | the same over `apps/showcase` at phone size, with its fixed tab bar |
| `nothing_scrolls_sideways_at_320px` | 1.4.10 Reflow | a box or a line of text past the window's edge outside a container that clips or scrolls |
| `text_spacing_overrides_break_nothing` | 1.4.12 Text Spacing | new page overflow or clipped content with the W3C spacing overrides |
| `text_40_percent_longer_breaks_nothing` | translations | every text node and placeholder accented and 40% longer (words stay unbreakable, like German compounds), at 1024px and 320px |
| `right_to_left_breaks_nothing` | RTL | the demo in `dir="rtl"`, at 1024px and 320px |
| `the_checks_catch_what_they_are_for` | | each check catches a page built to fail it |

Text truncated on purpose with `text-overflow: ellipsis` doesn't count as clipped, and form controls count by their box only (they scroll their own text).

## Reading a failure

A failure lists `check: demo: element ...` lines. `LAYOUT_REPORT=1 cargo test -p native --test layout -- --nocapture` prints every finding, accepted ones included. Accepted findings sit in `KNOWN` at the top of the file with a reason; an entry is a prefix, so `"reflow: form: "` accepts all of one demo's reflow findings. A `KNOWN` entry that matches nothing fails too: delete it.

## Blitz is not a browser

Every finding was compared with Chromium, using the demo's server-rendered markup (`crates/ui/tests/snapshots/`) and the kit CSS hashed the same way (`manganis_core::transform_css(src, "css")`) in a 320px and a 1024px page. What only Blitz reports is in `KNOWN` under these reasons:

- **Text inputs and textareas are 300px wide** (browsers: 20 characters), which holds narrow columns open: most reflow and long-text entries. Fix in the blitz-dom fork (`layout/mod.rs`).
- **Inline-flex and absolute boxes inside a flex item ignore their own width and height**: pagination links shrink to their digits; the external link's screen-reader-only text gets a full-size box (and shows). Fix in the blitz-dom fork (`layout/inline.rs`, pass `SizingMode::InherentSize` to inline boxes).
- **Tab lands on `tabindex="-1"` elements** (the primitives' hidden checkbox inputs). The focus check skips them. Fix in the blitz-dom fork (`focus_step`).
- **No `::after` box on the sidebar's menu action, and positioned elements without a `z-index` are painted and hit in DOM order**, so the menu button covers the action.
- **Calendar**: Blitz's table layout makes it wider than Chromium does.
- **Fonts**: Inter isn't bundled, so Blitz falls back to a wider serif, and breaks long strings differently.
- **No `position: sticky`** (laid out as `relative`) and **no `scroll-padding`** (the showcase test reads its value from `showcase.css`), and Taffy has **no right-to-left flex or grid order**: the RTL run checks logical properties and overflow, not mirrored order.

## What the checks found in the kit (fixed)

- **Data table** overflowed the page at 320px in a grid parent: its scroll container couldn't shrink below the table. `.dx-data-table { min-width: 0 }`.
- **Pagination** ran past a 320px window. Its list wraps now.
- **Time picker** segments were 20x18px side by side (2.5.8). They are 24x24; the picker keeps its height.
- **Showcase** at phone size: focus scrolled into view ended up under the fixed tab bar (browsers do this too). `html { scroll-padding-bottom: 5rem }` in the phone layout.

Still open: the **date range field** needs about 315px and can't wrap between its two dates without changing the primitive's markup (`KNOWN`, `reflow: date_picker`).
