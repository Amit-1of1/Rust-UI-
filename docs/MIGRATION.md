# Migration notes

Upgrade steps for each breaking change in the [CHANGELOG](../CHANGELOG.md), newest first. Nothing has been released yet, so these are grouped by the pull request that made the change. If you copied components (`atoll add`), run `atoll diff` to see which of your files the change touches, then re-copy with `atoll add --force <name>` or apply the change by hand.

What counts as breaking, and how deprecations work, is in [VERSIONING.md](VERSIONING.md).

Notes for changes merged since the last fold are in `changes/*.migration.md` ([changes/README.md](../changes/README.md)).

## Unreleased

### Toasts are not dialogs; unnamed avatars are decorative (PR #40)

**Who:** tests or CSS that find toasts by `role="alertdialog"`; apps with avatars that have no name; code that names `dioxus_primitives::select::SelectProps` for the kit's `Select`.

**Before:** every toast was `role="alertdialog"` with `aria-modal="false"`, and its content a `role="alert"`. `Avatar` was `role="img"` with no name unless you passed `aria-label`, which axe reports (`role-img-alt`). The kit's `Select` took dioxus-primitives' `SelectProps<T>`.

**After:**
- A toast is `role="group"`, named by its title and still focusable (F6). Its content is `role="status"` for info and success toasts and `role="alert"` for errors and warnings (primitives fork `5b21f56`).
- `Avatar` takes a `label`. Without `label`, `aria-label` or `aria-labelledby` it renders `aria-hidden="true"`. `ImageAvatar` uses its `alt` as the name when there is no `label`.
- `Select` takes the kit's own `SelectProps<T>`: the same props plus `selected`.

**What to do:**
- Tests: find a toast by its text or `getByRole("status")` / `getByRole("alert")`; `getByRole("alertdialog")` now only matches real alert dialogs.
- Avatars: pass `label: "Ada Lovelace"` where the avatar is the only place the name appears; leave it out where the name is shown beside it.
- `Select`: nothing, unless you built its props by hand; use `ui::prelude::SelectProps`.
- Copy users: `atoll add --force toast avatar select`, and bump `dioxus-primitives` to the fork commit in `Cargo.toml`.

### Icons are generated in the kit (PR #35)

**Who:** apps that use `ui::icons`, name `dioxus_icons` directly, or match icon markup in tests or CSS.

**Before:** `ui::icons` re-exported `dioxus-icons` 0.1.0 (Lucide 1.14.0). Icons rendered exactly the attributes you passed.

**After:** the kit generates every Lucide 1.57.0 icon itself. Every name `dioxus-icons` exported still exists (renamed icons keep their old name as an alias), and `Check { size: "1rem" }` still compiles. What changed:
- Icons without a `label` (or an `aria-label`) get `aria-hidden="true"`. With a `label` they are `role="img"`.
- Without a `size`, an icon gets `data-size="lg"` and follows `--icon-size-lg` (1.5rem, 24px by default).
- The `<svg>` carries `data-icon="<lucide name>"`, `data-stroke="theme"` (when you pass no `stroke_width`) and, on arrows, chevrons and the like, `data-mirror-rtl`, which mirrors it inside `dir="rtl"`.
- `IconSize` is an enum (`Xs`, `Sm`, `Md`, `Lg`, `Xl`, `Custom`); numbers and strings still convert into it.
- Some icons were redrawn upstream between Lucide 1.14 and 1.57 (for example `BadgeCheck`).
- `dioxus-icons` is no longer a dependency of the kit.

**What to do:**
- If an icon carries meaning on its own, give it `label: "..."` instead of relying on surrounding text. Remove `"aria-hidden": "true"` where you like; it is the default now (passing it is still fine).
- If your app imported `dioxus_icons` directly, import from `ui::icons` instead, or keep your own `dioxus-icons` dependency.
- If an icon should not mirror in right-to-left text, pass `mirror_rtl: false`.
- Copy users: `atoll add --force icons`, then drop `dioxus-icons` from your `Cargo.toml` if nothing else uses it.

### `class` adds to the kit's class (PR #30)

**Who:** apps that pass `class` to a styled component.

**Before:** most components spread your attributes after their own `class`, which rendered two `class` attributes. Only one survives, so either the kit's styling or your class was lost.

**After:** both apply on every renderer. `Button { class: "w-full", ... }` renders `class="dx-button … w-full"`.

**What to do:**
- If you passed `class` to *replace* the kit's look, that no longer works. Override the look with CSS that targets your class (it now sits next to the kit's), or set the theme tokens on a wrapper.
- If you worked around the lost class with a wrapper `div`, you can drop the wrapper.
- `RadioItem`, `NavbarItem` and `PopoverContent` take `class` as a named prop of the primitive; it is joined the same way.

### `Select` and `Combobox` put form attributes on the focusable control (PR #30)

**Who:** apps that pass `id`, `aria-describedby`, `aria-invalid`, `aria-required` (and for `Combobox`, `aria-labelledby`) to `Select` or `Combobox`, or that select those elements by id in CSS or tests.

**Before:** the attributes landed on the component's wrapper `div`, so `<label for>` pointed at a non-focusable element and screen readers didn't announce the error.

**After:** they move to the trigger button (`Select`) or the text input (`Combobox`). Everything else (`class`, `style`, `data-*`) stays on the wrapper. (PR #23 already moved `aria-label` and `aria-labelledby` on `Select` to the trigger.)

**What to do:**
- Update CSS or test selectors that used `#your-id` to style or find the wrapper: target the wrapper by class instead, or treat `#your-id` as the button/input.
- To bind a select or combobox to a form, prefer the new `SelectField` / `ComboboxField` (experimental), which do this wiring for you:

  ```rust
  let form = use_form();
  let role = form.field(vec![required()]);
  rsx! {
      SelectField { field: role, label: "Role", options: vec!["Admin".into(), "Member".into()], placeholder: "Pick one" }
  }
  ```

### `FormField::reset` returns to the initial value (PR #30)

**Who:** apps using `form` (experimental) that call `reset()`.

**Before:** `reset()` always emptied the field.

**After:** it returns to the field's initial value: the one given to `form.field_with(initial, rules)`, or the last `field.load(value)`. Fields declared with `form.field(rules)` start empty, so they behave as before.

**What to do:** nothing, unless you call `load()` or `field_with()` and still want an empty field on reset: call `field.set("")` after `reset()`.

### `ui::demos` needs the `demos` feature (PR #30)

**Who:** apps that render the kit's demos (`ui::demos::button::Demo {}`).

**Before:** `ui::demos` was always compiled, so every app shipped demo code and demo CSS.

**After:** it exists only with the `demos` feature.

```toml
ui = { git = "https://github.com/Amit-1of1/Rust-UI-", rev = "<commit>", package = "atoll-ui", features = ["web", "demos"] }
```

`ui::demos` is internal ([STABILITY.md](STABILITY.md)); copy the demo into your app instead if you depend on it.

### `Sidebar` is a navigation landmark (PR #30)

**Who:** apps that wrap `Sidebar` in their own `nav`, or that give it a role.

**Before:** `Sidebar` rendered a plain container, so its links sat outside every landmark (axe `region`).

**After:** it renders `role="navigation"` with the name from `UiStrings::sidebar` (on small screens, the sheet's inner panel does too). `SidebarInset` renders `<main>`.

**What to do:**
- Remove your own `nav` (or `role="navigation"`) around `Sidebar`, or you get a navigation landmark nested in another.
- To name it differently, pass `aria_label: "Workspace"`; to make it something else, pass `role`.
- Don't put another `main` inside `SidebarInset`.

### Theme state is shared: `use_theme`, and `ThemeScope` on native (PR #27, PR #30)

**Who:** apps that render `ThemeCustomizer`, write `data-theme` / `data-accent` themselves, or run on dioxus-native.

**Before:** each `ThemeCustomizer` kept its own state and wrote `<html>` through a script. A header dark-mode switch that also wrote `data-theme` could disagree with it, and on native (no JavaScript) the customizer did nothing.

**After:** one shared state (`use_theme()`, experimental) holds mode, accent, density and radius. `ThemeCustomizer` and app code read and write it, and its setters write `<html>` and every `ThemeScope`.

**What to do:**
- Replace hand-written `data-theme` writes with the shared state, so the two never disagree:

  ```rust
  let theme = use_theme();
  rsx! {
      Switch {
          checked: theme.is_dark(),
          aria_label: "Dark mode",
          on_checked_change: move |dark| theme.set_mode(if dark { ThemeMode::Dark } else { ThemeMode::Light }),
      }
  }
  ```
- On dioxus-native, wrap the app in `ThemeScope { ... }` (it is `display: contents`, so layout doesn't change). Without it, theme changes have no effect there. On the web and desktop it is harmless.
- Two `ThemeCustomizer`s on one page now show the same choices.

### `overlay` component removed (PR #29)

**Who:** apps that used `ModalPortal` or `OverlayLayer`, or relied on `ThemeStyles` rendering the overlay layer with the `native` feature.

**Before:** with the kit's `native` feature, dialog, alert dialog, sheet and drawer moved into a window-sized `OverlayLayer` that `ThemeStyles` rendered, to work around `position: fixed` in Blitz.

**After:** the patched renderer (our Blitz fork) lays out `position: fixed` against the window, so the workaround, the `overlay` registry item and its prelude exports are gone. The `native` feature still exists and does nothing.

**What to do:**
- Delete `ModalPortal { ... }` wrappers (render the children directly) and any `OverlayLayer {}`.
- If you copied components, delete `components/overlay/` and its `mod` line, and re-copy `dialog`, `alert_dialog`, `sheet` and `drawer`.
- On native, use the renderer revisions this repo pins (`[patch.crates-io]` in the root `Cargo.toml`, see [FORKS.md](FORKS.md)); unpatched Blitz shows modals inside their parent again.

### Form focus on native: `FormField::mounted` (PR #29, not breaking)

On a failed submit the form focuses the first invalid field. `TextField`, `SelectField` and `ComboboxField` do this on every renderer. If you wire your own control to a `FormField`, pass `onmounted: move |e| field.mounted(e)` to it; without it focus still moves on the web, but not on native.

### Dark mode through `light-dark()` (PR #21, not breaking)

Colour tokens in `theme.css` are now `light-dark(light, dark)` pairs, and `data-theme` only sets `color-scheme`. Overrides written the old way (`var(--dark, X) var(--light, Y)`) keep working, because `--dark`/`--light` are still defined. For a new custom accent, regenerate it with `atoll theme --brand <hex>`, which now prints `light-dark()` pairs ([THEMING.md](THEMING.md)).

### Hooks and icons are registry items (PR #21, copy users only)

`ui::hooks` and `ui::icons` keep their paths for crate users. If you copied components, components now import `components::hooks` and `components::icons`: `atoll add <name>` brings them along; with an older copy, run `atoll add hooks icons` and fix the `use` lines `atoll diff` reports.
