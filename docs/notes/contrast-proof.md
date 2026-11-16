# Contrast proof

`crates/cli/src/contrast.rs` proves WCAG AA for what the CSS draws; `crates/cli/tests/contrast.rs` runs it on the kit, `atoll contrast --theme x.css` on a user's theme.

- It reads CSS, not pixels: each rule that sets `color`, a background, or a focus style gives pairs. State rules inherit from their base rule (`.item` is the base of `.item:hover`); a part's host background comes from the selector's ancestors, else from where `crates/ui/tests/snapshots/*.html` render it, else (never rendered, e.g. closed overlays) from the page plus the component's `-content`/`-popover`/`-panel`/... parts. So **after adding a demo, refresh the snapshots**; the proof uses them.
- Text needs 4.5:1, large text (24px, 18.66px bold) 3:1. An element that never renders text (an SVG icon) needs 3:1; one that sets only a background and renders no text (dots, bars, thumbs) is skipped, as are `::before`/`::after` with empty content.
- Focus: only the parts that differ from the unfocused rule count (outline, border-color, box-shadow layers). An outer ring is checked against what is around the element, an inset one against the element. One part at 3:1 is enough. A 50%-alpha ring alone fails; that's why switch, colour picker and the invalid button now use a solid 2px `--focused-border-color` / `--error-text-color` ring.
- Hover fills: `color-mix(fill 90%, transparent)` fades a coloured fill toward a white page and drops white labels under 4.5:1 (green, orange, error). Mix toward `--secondary-color-5` instead (similar luminance to the AA fills), or use space toggles per mode.
- Disabled states (and `data-unavailable`) and `forced-colors` blocks are exempt. The proof resolves `var()`, `light-dark()`, the `--light`/`--dark` toggles, `color-mix()` in sRGB/OKLab and alpha; anything else (gradients) is reported as skipped, and the test fails on skips.

## Mutation testing of the proof itself

`cargo mutants -p atoll-cli -f crates/cli/src/contrast.rs` (about 75 minutes on 4 cores with
`-j 3`; run `--lib` first, then `--iterate` with the full suite) catches every mutant except five
that can't change behaviour:

- `split_top` and `Nesting::add_html`: deleting the `(Some(_), _) => {}` arm inside a quote; the
  catch-all `_ => {}` does the same.
- `Ctx::substitute`: `b < a` to `b <= a`; `var(` and `light-dark(` can't start at the same index.
- `color_mix`: the `words.len() > 1` guard (to `true` or `>=`); a one-word part has no percentage
  to split off either way.

The integration tests share one run of the proof (`kit_report()`), so a mutant costs about
60 s instead of five full runs.
