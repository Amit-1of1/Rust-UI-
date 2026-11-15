### Added

- **docs:** `docs/CHECKLIST.md`, a scorecard of every kit component against the designsystemchecklist.com "Core components" list (166 items), also on the docs site; `crates/ui/tests/checklist.rs` keeps its numbers honest. (#86)
- **ui:** `Button` and `IconButton` `loading`: a spinner over the label without changing the size, `aria-busy`, and presses (including repeat form submits) ignored while it is on (experimental). (#86)
- **ui:** Badge `size` (`BadgeSize`), `Success` and `Warning` variants, `on_dismiss` with `dismiss_label`, `dot`, and `BadgeAnchor` to pin a badge to a corner of another element (experimental). (#86)
- **ui:** `AvatarGroup` (overlapping avatars with a "+N" avatar) (experimental). (#86)
- **ui:** Alert `Success`, `Warning` and `Info` variants, an `icon` slot, `AlertActions`, and `live` (`AlertLive::Assertive`, `Polite`, `Off`) (experimental). (#86)
- **ui:** `Progress` `label`, `show_value` and `size` (`ProgressSize`); `Spinner` `size` (`SpinnerSize`); `Skeleton` `shape` (`SkeletonShape::Rect`, `Circle`, `Text`) and `lines` (experimental). (#86)
- **ui:** `TextLink` `variant` (`LinkVariant::Default`, `Muted`, `Inherit`, `Destructive`) and `disabled` (experimental). (#86)
- **ui:** `CheckboxGroup` (a fieldset with a legend); indeterminate checkboxes now show a dash (experimental). (#86)
- **ui:** New components `List`/`ListItem` (bulleted, numbered, plain) and `Image` (aspect ratio, `srcset`, fallback, required `alt`) (experimental). (#86)
- **ui:** `UiStrings::remove_named` and `UiStrings::more_count`, translated in every built-in locale. (#86)
- **ui:** `BreadcrumbTrail` with `Crumb` items and `max_items` (collapses the middle into a "show more" `BreadcrumbEllipsis` that expands in place), custom `BreadcrumbSeparator` children and a disabled `BreadcrumbLink` (experimental). (#86)
- **ui:** `Pager` page sizes (`page_sizes`, `page_size`, `on_page_size_change`) and unknown page counts (`has_next`); page buttons are named "Page N" (experimental). (#86)
- **ui:** `Tabs` `stretch` (equal-width tabs) and `Carousel` `item_size` (how much of each slide shows) (experimental). (#86)
- **ui:** `Tooltip` `delay_ms` (400 ms by default) before opening on hover; focus opens it at once (experimental). (#86)
- **ui:** Toasts show an icon per kind, pause while hovered or focused, and take one action button through `ToastsExt::notify_action` (`ToastAction`) (experimental). (#86)
- **ui:** `Dialog` `size` (`DialogSize`) and `show_close`, `DialogFooter`, `DialogClose` and `DialogContentClose` (experimental). (#86)
- **ui:** `FieldSize` for `Input` and `Textarea` (`field_size`), and a `start` slot on `Select` (experimental). (#86)
- **ui:** `Card` `horizontal`, `CardLink` (whole-card link) and `CardGroup` (responsive grid of cards) (experimental). (#86)
- **ui:** `UiStrings::show_more_crumbs`, `pagination`, `more_pages`, `page_number` and `rows_per_page`, translated in every built-in locale. (#86)

### Changed

- **ui:** `Spinner` draws its ring in the parent's text colour (`currentColor`), so it follows buttons, alerts and coloured text. (#86)
- **ui:** `Skeleton` is hidden from screen readers (`aria-hidden`); mark the loading region `aria-busy` instead. (#86)
- **ui:** Buttons show a pressed state (`:active`, `aria-pressed="true"`). (#86)
- **ui:** `Progress` takes the kit's own `ProgressProps` instead of the primitive's. (#86)
- **ui:** `Tooltip` takes the kit's own `TooltipProps` and waits `delay_ms` before opening on hover. (#86)
- **ui:** The accordion panel is a `region` named by its trigger. (#86)
- **ui:** `Pager` `pages` is optional (0 with `has_next` set means the count is unknown). (#86)
- **forks:** atoll-primitives pinned at `cf9e790`: toasts pause their countdown while hovered or focused (docs/FORKS.md). (#86)

### Fixed

- **ui:** Error states show on `Input`, `Checkbox`, `RadioGroup` and `Switch` with `aria-invalid="true"` or inside an invalid `Field`; the input's edge is a box-shadow, so the old border colour never showed. (#86)
- **ui:** The pagination landmark and its "more pages" text are translated instead of fixed English. (#86)
- **ui:** Select shows its error state when the trigger is `aria-invalid="true"`. (#86)
- **ui:** Avatar images of any ratio are cropped to the shape (`object-fit: cover`) instead of stretched. (#86)
