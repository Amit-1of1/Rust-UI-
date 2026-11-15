### Fixed

- **ui:** `nice_max` (chart axis limit) could land a hair below the data (0.05 gave 0.049999997) and returned infinity above about 2e38; now proved for all 2^32 floats.
- **ui:** `virtual_window` no longer overflows on a huge or infinite scroll offset or viewport, and returns an empty range for a NaN row height.
- **ui:** `fill` (announcement text) substituted into values it had already inserted, so an author called `{text}` or a file called `a{size}.png` was announced wrongly; it now fills in one pass.
- **ui:** `resize_pair` panicked on NaN panel limits; a NaN limit now means no limit and a non-finite drag changes nothing.
- **ui:** `parse_iso_date` accepted signs (`2026-+1-+5`); it now reads only canonical dates.
- **ui:** `format_size` showed `1024.0 KB` just below a megabyte; `format_number(-0.0)` showed `-0`; the data table treated `inf` and `NaN` cells as numbers, so a `-inf` cell in a numeric column was exported to CSV without the formula guard. A property test now reads 4,096 random exports back with an independent CSV parser and checks no formula gets through.
- **motion:** `DragBounds::clamp` panicked on bounds built by hand in the wrong order or with NaN.
