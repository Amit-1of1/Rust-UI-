# `data-theme` on a subtree does not switch light-dark() tokens in Chromium (found 2026-10-31)

`theme.css` says `data-theme="dark"` on any element switches its own subtree. On the web that
is not what happens in current Chromium: the colour tokens (`--primary-color: light-dark(#fff,
#000)` and the rest) are declared on `:root`, and the browser resolves `light-dark()` in a
custom property where it is declared, so `getComputedStyle(el).getPropertyValue('--primary-color')`
on a `[data-theme="dark"]` box inside a light page is still `#fff`. `color-scheme` switches
(native form controls go dark); the kit's colours don't.

Found while building the docs Theme builder, whose preview first used its own `data-theme`;
it now switches the whole page with `use_theme().set_mode(...)` instead.

What still works: `data-theme` on `<html>` (what `ThemeCustomizer` and `use_theme` write), and
the native renderer, which takes the generated fallback block, where root tokens are already
re-declared on `[data-theme]`. What doesn't: `ThemeScope` or a hand-written `data-theme` used
to show one dark panel inside a light web page.

Possible fix (not done here, it touches every app's theme): re-declare the root tokens on
`[data-theme]` in the `light-dark()` path too, as the fallback does, and decide whether a
nested `data-theme` should keep an ancestor's `data-accent` (the fallback currently resets it).
