### `Progress` has its own props (#86)

**Who:** code that names `ui::progress::ProgressProps` or builds it by hand (the `rsx!` form `Progress { value, max, aria_label }` is unchanged).

**Before:** `Progress(props: dioxus_primitives::progress::ProgressProps)`.

**After:** `Progress(props: ui::progress::ProgressProps)`, the same `value`, `max` and attributes plus `label`, `show_value` and `size`. Children are still ignored.

**What to do:** import `ui::progress::ProgressProps` instead of the primitive's, if you name it.

### `Tooltip` has its own props and a hover delay (#86)

**Who:** code that names `ui::tooltip::TooltipProps`, or tests that hover a tooltip and expect it at once.

**Before:** `Tooltip(props: dioxus_primitives::tooltip::TooltipProps)`, open as soon as the pointer arrives.

**After:** `Tooltip(props: ui::tooltip::TooltipProps)` with the same props plus `delay_ms` (400). Hover opens it after the delay; focus still opens it at once.

**What to do:** import the kit's `TooltipProps` if you name it; set `delay_ms: 0` to keep the old timing.
