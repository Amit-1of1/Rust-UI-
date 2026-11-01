### Added

- **docs:** `docs/CHECKLIST.md`, a scorecard of every kit component against the designsystemchecklist.com "Core components" list (166 items), also on the docs site; `crates/ui/tests/checklist.rs` keeps its numbers honest.
- **ui:** `Button` and `IconButton` `loading`: a spinner over the label without changing the size, `aria-busy`, and presses (including repeat form submits) ignored while it is on (experimental).
- **ui:** Badge `size` (`BadgeSize`), `Success` and `Warning` variants, `on_dismiss` with `dismiss_label`, `dot`, and `BadgeAnchor` to pin a badge to a corner of another element (experimental).
- **ui:** `AvatarGroup` (overlapping avatars with a "+N" avatar) (experimental).
- **ui:** Alert `Success`, `Warning` and `Info` variants, an `icon` slot, `AlertActions`, and `live` (`AlertLive::Assertive`, `Polite`, `Off`) (experimental).
- **ui:** `Progress` `label`, `show_value` and `size` (`ProgressSize`); `Spinner` `size` (`SpinnerSize`); `Skeleton` `shape` (`SkeletonShape::Rect`, `Circle`, `Text`) and `lines` (experimental).
- **ui:** `TextLink` `variant` (`LinkVariant::Default`, `Muted`, `Inherit`, `Destructive`) and `disabled` (experimental).
- **ui:** `CheckboxGroup` (a fieldset with a legend); indeterminate checkboxes now show a dash (experimental).
- **ui:** New components `List`/`ListItem` (bulleted, numbered, plain) and `Image` (aspect ratio, `srcset`, fallback, required `alt`) (experimental).
- **ui:** `UiStrings::remove_named` and `UiStrings::more_count`, translated in every built-in locale.

### Changed

- **ui:** `Spinner` draws its ring in the parent's text colour (`currentColor`), so it follows buttons, alerts and coloured text.
- **ui:** `Skeleton` is hidden from screen readers (`aria-hidden`); mark the loading region `aria-busy` instead.
- **ui:** Buttons show a pressed state (`:active`, `aria-pressed="true"`).
- **ui:** `Progress` takes the kit's own `ProgressProps` instead of the primitive's.

### Fixed

- **ui:** Error states show on `Input`, `Checkbox`, `RadioGroup` and `Switch` with `aria-invalid="true"` or inside an invalid `Field`; the input's edge is a box-shadow, so the old border colour never showed.
- **ui:** Avatar images of any ratio are cropped to the shape (`object-fit: cover`) instead of stretched.
