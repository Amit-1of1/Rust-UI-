### Fixed

- **ui:** `Pagination` (and `Pager`) no longer render two `aria-label`s when you name the navigation: the default "pagination" is used only when you give no `aria-label` or `aria-labelledby`.
- **ui:** `ContextMenuTrigger` is styled by a class instead of a second `style` attribute, which replaced the primitive's own style, so a long press on iOS no longer opens the system callout or selects text over the menu.
- **ui:** The drag-and-drop list's grip icon no longer has `aria-hidden` twice.

### Added

- **ui:** The ARIA check (`crates/ui/tests/aria.rs`) fails on any element with the same attribute twice.
