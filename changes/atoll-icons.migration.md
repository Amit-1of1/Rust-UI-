### `IconNode` has a new `Fill` variant (PR #85)

**Who:** code that matches on `ui::icons::IconNode` exhaustively (a custom icon renderer or exporter). Code that only draws icons is not affected.

**Before:** `IconNode` had seven variants, `Path` to `Polygon`.

**After:** an eighth, `IconNode::Fill { d }`, a path filled with `currentColor` and no stroke, used by Atoll icons for solid shapes.

**What to do:** add an arm for `IconNode::Fill { d }` (draw `<path d fill="currentColor" stroke="none">`), or a `_` arm.
