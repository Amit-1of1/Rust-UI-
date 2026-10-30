### Added

- **ui:** `TimePicker` and `TimeField` take `min` and `max`: the arrow keys, Home and End skip hours and minutes that can't make an allowed time, a typed time outside the range is kept but marked invalid, and a `max` earlier than `min` runs past midnight. `TimeField` adds the range rule itself; `time_between` is that rule for other fields (experimental).
- **ui:** `DateRangeField` binds a `DateRangePicker` to a `use_form` field as `2026-10-15/2026-10-20`, with `days_between` to limit the length; `format_iso_date(_range)` and `parse_iso_date(_range)` read and write the values (experimental).
- **ui:** `RadarChart`: one axis per category and one shape per series, with legend, data table and the usual text summary (experimental).
- **ui:** `keyboard: true` on bar, line, area, pie and radar charts: the chart becomes one tab stop, the arrow keys, Home and End move between points, and the point under the cursor is outlined and announced in a status line. Pure moves in `ChartCursor`/`ChartKey`. New `UiStrings` fields `chart_keys_hint`, `chart_point` and `time_out_of_range` (experimental).
- **ui:** Keyboard contracts (`contract.toml`) for the chart and the time picker.
