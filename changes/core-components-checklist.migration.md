### `Progress` has its own props (PR #N)

**Who:** code that names `ui::progress::ProgressProps` or builds it by hand (the `rsx!` form `Progress { value, max, aria_label }` is unchanged).

**Before:** `Progress(props: dioxus_primitives::progress::ProgressProps)`.

**After:** `Progress(props: ui::progress::ProgressProps)`, the same `value`, `max` and attributes plus `label`, `show_value` and `size`. Children are still ignored.

**What to do:** import `ui::progress::ProgressProps` instead of the primitive's, if you name it.
