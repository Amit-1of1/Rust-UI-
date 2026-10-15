# Changelog

Changes to `rust-ui-kit` (`ui`), `rust-ui-motion` (`motion`) and `rust-ui-cli` (`rust-ui`). The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); version numbers follow [docs/VERSIONING.md](docs/VERSIONING.md) (semver, `0.x`). Upgrade steps for every breaking change are in [docs/MIGRATION.md](docs/MIGRATION.md), and which items are stable or experimental is in [docs/STABILITY.md](docs/STABILITY.md).

Nothing has been released, tagged or published yet. All three crates are at `0.1.0`, and everything since the first commit is under "Unreleased"; the first tag, `v0.1.0`, will close it. Entries name the pull request that made the change.

Pull requests don't edit this file: each adds its entries as a fragment in [changes/](changes/README.md), and `rust-ui changelog` folds them in here before a release. Until then, the newest changes are in `changes/`.

## Unreleased

### Breaking

- **ui:** Toasts are no longer `role="alertdialog"`: a toast is a named, focusable group, and its message is a `status` region (info, success) or an `alert` (error, warning). Fixed on the primitives fork. `Avatar` without a `label`, `aria-label` or `aria-labelledby` is decorative (`aria-hidden="true"`); `ImageAvatar` takes its name from `alt`. `Select` has its own props struct (`ui`'s `SelectProps`, with the new `selected`) instead of dioxus-primitives'. (#40) [Migration](docs/MIGRATION.md#toasts-are-not-dialogs-unnamed-avatars-are-decorative-pr-40)
- **ui:** Icons are generated from Lucide 1.57.0 inside the kit instead of coming from `dioxus-icons` (Lucide 1.14.0). Every old icon name still works, but some icons are redrawn upstream, icons are `aria-hidden` unless labelled, `IconSize` is now an enum, and `dioxus-icons` is no longer a dependency. (#35) [Migration](docs/MIGRATION.md#icons-are-generated-in-the-kit-pr-35)
- **ui:** `class` on a styled component now adds to the kit's class instead of rendering a second `class` attribute, where one of the two was lost. Applies to every component. (#30) [Migration](docs/MIGRATION.md#class-adds-to-the-kits-class-pr-30)
- **ui:** `Select` and `Combobox` move `id`, `aria-describedby`, `aria-invalid` and `aria-required` (and on `Combobox`, `aria-labelledby`) from the wrapper to the focusable control. (#30) [Migration](docs/MIGRATION.md#select-and-combobox-put-form-attributes-on-the-focusable-control-pr-30)
- **ui (experimental `form`):** `FormField::reset` returns to the field's initial value (`field_with`, `load`) instead of always emptying it. (#30) [Migration](docs/MIGRATION.md#formfieldreset-returns-to-the-initial-value-pr-30)
- **ui:** `ui::demos` is only compiled with the new `demos` feature. (#30) [Migration](docs/MIGRATION.md#uidemos-needs-the-demos-feature-pr-30)
- **ui:** `Sidebar` renders a navigation landmark (`role="navigation"`, named by `UiStrings::sidebar`) by default. (#30) [Migration](docs/MIGRATION.md#sidebar-is-a-navigation-landmark-pr-30)
- **ui:** `ThemeCustomizer` no longer keeps its own state; it reads and writes the shared `use_theme()` state, and on dioxus-native theme changes apply only inside a `ThemeScope`. (#27, #30) [Migration](docs/MIGRATION.md#theme-state-is-shared-use_theme-and-themescope-on-native-pr-27-pr-30)
- **ui:** Removed the `overlay` component (`ModalPortal`, `OverlayLayer`) and its prelude exports; `ThemeStyles` no longer renders an overlay layer. The `native` feature is kept but does nothing. (#29) [Migration](docs/MIGRATION.md#overlay-component-removed-pr-29)
- **ui:** `Select` moves `aria-label` / `aria-labelledby` to its trigger button. (#23)
- **ui (copy users):** demos moved out of the component folders into `crates/ui/src/demos`, so `rust-ui add` and `dx components add` no longer copy them. (#14)

### Added

- **ui:** `router` feature: inside a Dioxus `Router`, `TextLink`, `BreadcrumbLink` and `PaginationLink` navigate in the app on a plain click, add the router's base path to their `href`, and `TextLink` marks the current route; helpers `hooks::follow_in_app`, `in_app_href`, `is_current_route`. `SkipLink` and `MainContent` (which takes focus after an in-app navigation with the `router` feature). `NumberInput` and `NumberField` (spin button with -/+, `use_form` binding). `Pager` for client-side paging with buttons. `CardMedia`, `CarouselItem { media: true }`, `Avatar`'s `label`, `Select`'s plain `selected` value. `UiStrings::skip_to_content`, `decrease`, `increase`. Experimental except `SkipLink`, `MainContent` and `label`. (#40)
- **ui:** All 1,890 Lucide icons (was 1,705), plus `Icon { icon: lucide::CHECK }` for icons chosen at runtime, `label` (named `role="img"`), theme sizes `IconSize::Xs` … `Xl` (`--icon-size-*`), `--icon-stroke-width`, and mirroring of directional icons in right-to-left text (experimental). `scripts/gen-lucide-icons.mjs` regenerates the set. (#35)
- **ui:** `TextareaField` and `CheckboxField` for `use_form`; `TextField` passes other attributes (`inputmode`, `maxlength`, ...) to its input (experimental `form`). `Stepper` takes `current: Signal<usize>` to be driven from outside. `button_look(variant, size)` styles a router `Link` as a button. (#34)
- **ui:** `use_theme()`, `Theme` and `ThemeMode`: one shared mode, accent, density and radius state with `is_dark()` (experimental). (#30)
- **ui:** `SelectField`, `ComboboxField`, `FieldOption`, `FormState::field_with` and `FormField::load` (experimental). `Select` takes a `placeholder`. (#30)
- **ui:** `DataTable` `toolbar` slot beside the filter box (experimental). (#30)
- **ui:** `ToastsExt` (`toasts.notify_success("Saved")`, or a `(title, description)` pair) and `ToastMessage` (experimental). (#30)
- **ui:** `TableHead` and `TableCell` take `numeric: true` (end-aligned, tabular figures). (#30)
- **ui:** Global `.sr-only` class in `theme.css`. (#30)
- **ui:** `FormField::mounted`, so a failed submit focuses the first invalid field on every renderer. (#29)
- **ui:** `ThemeScope`, which applies the theme choices where `<html>` can't be reached (native). (#27)
- **ui:** `chips` and `link` (`TextLink`) components; `ui::icons` (Lucide) and `ui::hooks` as registry items, with `use_media_query` and `use_copy_clipboard`. (#21)
- **ui:** `--overlay-color` token for modal scrims. (#20)
- **ui, cli:** `rust-ui postbuild` bundles an app's stylesheets into one file and `ThemeStyles` uses it (`ui::css_bundle`). (#25)
- **ui:** `hooks::Selection` (`use_selection`, `use_selected`) (experimental). (#18)
- **ui:** `UiStrings` / `UiStringsProvider` for translatable kit text, `--density` presets, keyboard logic as pure state. (#15)
- **ui:** Accent presets (`data-accent`), `--radius`, `ThemeCustomizer`. (#11)
- **ui:** Batch 1 and 2 components: callout, empty, status, shimmer, marquee, button group, stepper, bottom nav, input group, form, data table, input OTP, carousel, multi select, dropzone, drawer. (#12)
- **ui:** Spring easing tokens (`--spring-gentle|snappy|bouncy`), CSS anchor positioning for floating content, Ctrl+K for the command palette (`use_hotkey`). (#12, #13)
- **motion:** `SwipeDismiss` pointer capture and release momentum (#8); `SwipeDismiss` vertical and one-way swipes (#12); `FlipList` / `FlipItem` (#4).
- **cli:** `rust-ui theme --brand`, `rust-ui doctor`, `rust-ui new`, copying from a git URL (`--source`). (#7, #10, #12, #13)
- **ui, motion, cli:** Initial kit: Dioxus 0.7 workspace, styled components on dioxus-primitives, light/dark theme, motion presets with reduced-motion support, registry and `rust-ui` CLI. (#1, #2, #3)

### Changed

- **ui:** `Card`'s corner radius is twice the theme `--radius` (1rem by default, as before), so the radius setting reaches cards. (#40)
- **ui:** `Accordion` no longer sets an inline `width: 15rem`; it fills its container. Wrap it in a sized box if you relied on the old width. (#34)
- **ui:** Theme colour tokens are `light-dark()` pairs and `data-theme` sets `color-scheme`; the old `var(--dark, X) var(--light, Y)` form still works. `rust-ui theme --brand` prints `light-dark()` pairs. (#21)
- **ui:** Chart marks use `--accent-color` and show their value on hover. (#30)
- **ui:** dioxus-primitives comes from our fork (`Amit-1of1/dioxus-components`), pinned by commit; native renderer crates are patched from forks ([FORKS.md](docs/FORKS.md)). No kit API changed. (#26, #27)
- **ui:** Right-to-left support through logical CSS properties. (#12)
- **ui:** `rust-ui-kit` is `publish = false` (git install only); `rust-ui-motion` and `rust-ui-cli` package cleanly but are unpublished. (#19)

### Fixed

- **ui:** `DatePicker` and `DateRangePicker` take their name from an `aria-label` or `aria-labelledby` you pass (the English "Date" / "Date Range" is only a fallback), date segments no longer point `aria-labelledby` at a missing id, and a segment moves on once it holds all its digits ("01" for a month). Primitives fork `5b21f56`. Found by the booking app (`apps/booking/GAPS.md`). (#40)
- **ui:** Many native (Blitz) fixes: Escape, outside click, focus traps, drags, context menu, toast F6, field focus, carousel, sidebar shortcut. (#24, #26, #27, #29)
- **ui:** A `DateRangePicker` no longer freezes the page after a complete start date is typed (primitives fork `71fed69`). An `Item` inside an `ItemGroup` is `role="listitem"`. Found by the booking app (`apps/booking/GAPS.md`). (#34)
- **ui:** Kit gaps found by the dashboard app (`apps/dashboard/GAPS.md`): `FormField::set` no longer subscribes effects, `Card` merges a caller's class, and more. (#22, #30)
