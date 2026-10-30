# Scoped theme

`crates/ui/assets/theme.scoped.css` and `reset.scoped.css` are generated from `theme.css` and
`reset.css` by `rust-ui registry` (`crates/cli/src/scope.rs`); never edit them, and change
`theme.css` as usual. `rust-ui registry --check` (in `scripts/check.sh`) fails when they are
stale, and `scripts/sync-main.sh` treats them as generated.

The rewrite, per selector: `:root`/`html` become `[data-rui-scope]`; a first compound made only
of attributes (`[data-theme="dark"]`, `[data-accent=...]`, `[dir="rtl"]`) matches on the scope,
inside it, or on an ancestor of it; `:where(x)` becomes `:where([data-rui-scope] x)` (still zero
specificity); anything else gets `[data-rui-scope] ` in front. `@media`/`@supports`/`@layer`/
`@container` are rewritten inside; other at-rules are copied. A unit test fails if any selector
in the kit's theme comes out unscoped.

New rules in `theme.css` therefore need no extra work, but keep them to plain selectors the
rewrite understands: a rule that must reach outside the scope (the page canvas) can't be
scoped and belongs in `reset.css`, which is opt-in.
