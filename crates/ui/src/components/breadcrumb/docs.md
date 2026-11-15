# Breadcrumb

A trail of links to the current page.

```rust
Breadcrumb {
    BreadcrumbItem { BreadcrumbLink { href: "/", House { "aria-hidden": "true" } "Home" } }
    BreadcrumbSeparator {}
    BreadcrumbItem { BreadcrumbPage { "Settings" } }
}
```

- Icons go inside `BreadcrumbLink`, decorative (`aria-hidden`); use them for every step or for none.
- `BreadcrumbSeparator` draws "/" by default; pass your own: `BreadcrumbSeparator { ChevronRight { "aria-hidden": "true" } }`.
- `BreadcrumbLink { disabled: true }` keeps a step in the trail but can't be followed (no `href`, `aria-disabled`, skipped by Tab).

## Collapsing long trails

`BreadcrumbTrail` builds the trail from a list and, when there are more than `max_items` steps, shows the first and the last `max_items - 1` with a "…" button between. Pressing it shows every step and moves focus to the first one it revealed. `visible_crumbs` is the rule on its own.

```rust
BreadcrumbTrail { max_items: 3, items: vec![
    Crumb::link("Home", "/"),
    Crumb::link("Projects", "/projects"),
    Crumb::link("Atoll", "/projects/atoll"),
    Crumb::page("Members"),
] }
```

For hand-built trails, `BreadcrumbEllipsis { label, on_click }` is the same button.

## Accessibility

- Role and name: a `nav` named "Breadcrumb" (translatable); the current page is not a link and has `aria-current="page"`; separators are `aria-hidden`; the "…" button is named "Show N more"
- Keyboard: links are tabbable in order; Enter or Space on "…" expands the trail and focuses the first revealed step
- Reduced motion: none
