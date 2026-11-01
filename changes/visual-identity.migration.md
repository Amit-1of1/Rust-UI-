### The default look is Lagoon, not black and white (PR #83)

**Who:** apps that render `ThemeStyles {}` (or copied `theme.css`) and liked the old neutral look, or that relied on the default accent being black.

**Before:** buttons and checked controls were near-black (white in dark mode), the page was pure white or black, text used the app's font or Inter, and corners were 8 px.

**After:** the accent is lagoon teal, the neutrals have a slight sea-water tint (same contrast), the kit's fonts load (Figtree, Bricolage Grotesque for titles) and corners are 12 px.

**What to do:**
- To keep the old look, pick the `classic` preset: `ThemeStyles { preset: "classic" }`, or `data-preset="classic"` on `<html>` in your `index.html`.
- To keep your own font, set `--font-sans` and `--font-heading` on `:root` in your stylesheet, or `ThemeStyles { fonts: false }` to skip the kit's font files.
- To keep 8 px corners, set `--radius: 0.5rem` on `:root` (or `use_theme().set_radius(8)`).
- Copy users: `atoll add --force` the components you copied to pick up the `--radius`-based corners and the heading font, and re-copy `theme.css` with `atoll init`.
