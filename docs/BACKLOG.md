# Component and essentials backlog

**Status (2026-10-13):** `chips` and `link` (`TextLink`) are built, the icon set is decided (`docs/ICONS.md`) and the shared hooks are a registry item (`hooks`). Batch 1 is complete.

**Earlier (2026-10-10):** Batch 1, `form`, `data_table`, `input_otp`, `input_group`, `carousel`, `multi_select`, `dropzone` and `drawer` (with a vertical, one-way `SwipeDismiss`) are built (PR #12). `date range` already existed (`DateRangePicker` in `date_picker`) and `navigation_menu` is covered by our `navbar`, so both are dropped. Component batches are done; next is the essentials list below and the research ideas (`rust-ui doctor`, then `rust-ui theme --brand`).

Written 2026-10-10 after checking two upstream repos by cloning them (not by reading the web UI):

- `DioxusLabs/components` at `main` (9a75825, the commit we pin): **nothing newer than what we copied**, except a `form` component we skipped.
- `rust-ui/ui` (rust-ui.com), commit 43ddc65, MIT, Copyright (c) 2026 Max Wells. 87 files under `app_crates/registry/src/ui`, plus `crates/autoform`, `crates/icons`, and ~35 hooks under `registry/src/hooks`.

**How borrowing works here.** Rust/UI styles with Tailwind classes; we use plain CSS and theme tokens. So we copy the *structure and behaviour* (props, state, markup, aria) and rewrite the styling as `style.css` with our tokens. Every borrowed file gets a `THIRD_PARTY.md` row (source, commit, MIT). Interactive pieces must sit on `dioxus-primitives` or native elements and pass the axe gate in light, dark and every accent. Create each with `rust-ui new <name>`.

We have 52 components. Missing ones, in build order:

## Batch 1: presentational (fast lane, no JS)

| Component | Source | Notes |
|---|---|---|
| callout | Rust/UI `callout.rs` (41 lines) | Like `alert`, for inline notes |
| empty (empty state) | Rust/UI `empty.rs` (62) | |
| status (dot + label) | Rust/UI `status.rs` (51) | Never colour alone: keep text |
| shimmer | Rust/UI `shimmer.rs` (46) | Reduced motion: static |
| marquee | Rust/UI `marquee.rs` (60) | Must pause on hover/focus and under reduced motion |
| button_group | Rust/UI `button_group.rs` (65) | `role="group"` plus label |
| ~~chips~~ done | Rust/UI `chips.rs` (36) | Focusable checkbox, check mark as well as colour |
| ~~link~~ done | Rust/UI `link.rs` (49) | `TextLink`; upstream's web_sys use was a `force_reload` button, left out |
| stepper | Rust/UI `stepper.rs` + `use_stepper` (161) | Use `aria-current="step"` |
| bottom_nav | Rust/UI `bottom_nav.rs` | |

## Batch 2: interactive (careful lane)

| Component | Source | Notes |
|---|---|---|
| ~~form (validation, field errors)~~ done | `DioxusLabs/components` `preview/.../form` + Rust/UI `use_form.rs` | Biggest gap for real apps; link errors with `aria-describedby` |
| input_otp | Rust/UI `input_otp.rs` + hook (92) | Native inputs, paste support |
| input_group | Rust/UI `input_group.rs` (188) | |
| drawer | Rust/UI `drawer.rs` (208) | Build on our dialog plus `SwipeDismiss` |
| carousel | Rust/UI `carousel.rs` (210) | Pause control, arrow keys, `aria-roledescription` |
| multi_select | Rust/UI `multi_select.rs` (345) | Build on our `combobox` |
| navigation_menu | Rust/UI `navigation_menu.rs` (368) | Compare with primitives `navbar` first |
| dropzone | Rust/UI `dropzone.rs` (498) | Needs a keyboard path (button) as well as drag |
| ~~data_table (sort, filter, paging, selection)~~ done | our `table` + Rust/UI `data_table`/`data_grid` ideas | Highest value for dashboards |
| date range picker | extend our `date_picker` | |

Skip for now: `input_phone` (546 lines, large data tables), `workflow`, `bubble`/`message`/`attachment` (chat UI), `mask`, `header`/`footer` blocks.

## Essentials that are not components

1. ~~**Icons.**~~ Done: every Lucide icon generated into the `icons` item (`docs/ICONS.md`).
2. **Hooks**: the `hooks` item ships `use_hotkey`, `Selection`, `use_media_query`, `use_copy_clipboard`. Still to borrow if a component needs them: `use_click_outside`, `use_lock_body_scroll`, a debounce helper.
3. **Theming guide** (accent, radius, dark mode, brand colour). Tokens exist; the guide does not.
4. **Docs site**: a page per component from each `docs.md`, with the live demo.
5. **RTL/direction**: Rust/UI has `direction_provider`; add logical CSS properties and an RTL axe pass.
6. **Publish path**: `rust-ui add` from a git URL works; crates.io is blocked by the git-pinned primitives.
7. **CI**: workflows exist but GitHub never starts a runner (check Settings > Actions and billing).
8. **Real-app trial**: use the kit in one real project to find gaps before adding more.
9. **Optional**: schema-driven forms like Rust/UI `autoform` (it targets Leptos, so it would be a rewrite, not a copy).

## Order

1. Batch 1 (one PR, about 10 components).
2. `form`, then `data_table` (the two most reusable), then the rest of Batch 2.
3. Icons decision, hooks (docs site: apps/docs; theming guide: docs/THEMING.md).
