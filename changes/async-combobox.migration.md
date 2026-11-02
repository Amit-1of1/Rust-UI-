### `ComboboxProps` and `UiStrings` have new fields (PR #92)

**Who:** code that builds `ComboboxProps` or `UiStrings` with a struct literal that lists every field and has no `..`. Components written in `rsx!` (`Combobox { ... }`) and strings built with `..UiStrings::default()` (what docs/STABILITY.md asks for) are not affected.

**Before:** `ComboboxProps` ended at `list_aria_label`; `UiStrings` ended at `increase`.

**After:** `ComboboxProps` has `list_busy` and `announcement`; `UiStrings` has `loading_results`, `loading_more`, `load_more`, `one_result`, `result_count`, `results_failed` and `try_again`.

**What to do:** add `..UiStrings::default()` (or `..Default::default()`) to the literal, or fill in the new fields.
