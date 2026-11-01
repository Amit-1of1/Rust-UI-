### `DataColumn` and `UiStrings` have new fields

**Who:** code that builds a `DataColumn` or a `UiStrings` with a struct literal and no `..Default::default()`/`..DataColumn::new(..)`. Code using `DataColumn::new(..)` and its builder methods, or `UiStrings { .., ..UiStrings::default() }`, is not affected.

**Before:** `DataColumn { header, sortable, numeric }`.

**After:** `DataColumn` also has `hidden`, `hideable`, `width` and `min_width`; `UiStrings` also has `columns`, `export_csv` and `resize_column`.

**What to do:** build columns with `DataColumn::new("Name").sortable()`, or add `..DataColumn::new("")` to the literal; give a full `UiStrings` literal the three new strings.
