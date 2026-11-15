### Added

- **ui:** `DataTable` column menu (`column_menu`, `DataColumn::hidden()`/`always_visible()`, `on_columns_change`): readers show and hide columns; hidden columns aren't searched or exported (experimental). (#96)
- **ui:** `DataTable` column resizing (`resizable_columns`, `DataColumn::width()`/`min_width()`, `on_column_resize`): drag a header's edge or use the arrow keys on its `role="separator"` handle (experimental). (#96)
- **ui:** `DataTable` CSV export (`exportable`, `on_export`, `export_name`) and the pure `table_csv`: RFC 4180, guarded against spreadsheet formula injection; downloads in the browser, hands the text to `on_export` on desktop and native (experimental). (#96)
- **ui:** `DataTable` virtualised rows (`virtual_height`, `row_height`) and the pure `virtual_window`: only the rows in view are rendered, with a sticky header, `aria-rowcount` and `aria-rowindex`; a 20,000-row table renders ~25 rows (experimental). (#96)
- **ui:** `TableHead` `trailing` slot, placed after the header content and outside its sort button. (#96)
- **ui:** `UiStrings` `columns`, `export_csv` and `resize_column`, translated in every bundled locale. (#96)
- **cli:** component manifests list `js-sys` for components that use it. (#96)

### Changed

- **ui:** `table_view`'s filter no longer matches text in columns marked `hidden`. (#96)
