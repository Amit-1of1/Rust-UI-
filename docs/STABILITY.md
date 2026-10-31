# API stability

Which parts of `atoll-ui` (`ui`), `atoll-motion` (`motion`) and `atoll-cli` (`atoll`) you can build on, as of 2026-10-14. The rules for changing each level are in [VERSIONING.md](VERSIONING.md).

| Level | Promise |
|---|---|
| **Stable** | Breaking changes only in a minor release while `0.x` (major after `1.0`), each with a CHANGELOG entry and upgrade steps in [MIGRATION.md](MIGRATION.md), and a deprecation period when practical. |
| **Experimental** | Usable, tested and documented, but the shape may still change in any release. Changes are listed in the CHANGELOG; there is no deprecation period. Pin a `rev`/tag if you rely on it. |
| **Internal** | Public only so this repo's apps, tools and tests can reach it. No promise at all; don't use it from an app. |

Everything is young (the kit started on 2026-10-10), so "stable" means "we now treat changes to it as breaking", not "it will never change". The kit has no `1.0` yet.

## How to tell

- Items listed as experimental or internal below say so in their rustdoc (first paragraph or a `# Stability` section), and experimental components say so in a "Stability" section of their `docs.md` (shown on the docs site).
- Anything not listed below as experimental or internal, and reachable from `ui::prelude`, `ui::hooks`, `ui::icons`, `ui::motion` or the crate roots, is stable.

## `atoll-ui` (`ui`)

### Stable

- **`ui::prelude`** and the components it re-exports, with their props, enums and child components. That covers the components copied from DioxusLabs/components (accordion, alert dialog, aspect ratio, avatar, button, calendar, card, checkbox, collapsible, color picker, combobox, context menu, date picker, dialog, drag-and-drop list, dropdown menu, hover card, input, label, menubar, navbar, pagination, popover, progress, radio group, scroll area, select, separator, sheet, sidebar, skeleton, slider, switch, tabs, textarea, toast, toggle, toggle group, toolbar, tooltip, virtual list), the ones written here (alert, breadcrumb, command, field, kbd, spinner, table) and the ones adapted from Rust/UI (bottom nav, button group, callout, chips, empty, input group, link, marquee, shimmer, status, stepper), plus carousel, drawer, dropzone, input OTP, item, multi select and tag group, except where listed under experimental.
- **`ThemeStyles`**, **`THEME_CSS`**.
- **`ui::hooks`**: `use_hotkey`, `use_media_query`, `use_copy_clipboard`.
- **`ui::icons`**: the icon components (`ui::icons::Check`), `IconProps` with `size` (pixels or a CSS length) and SVG attributes, and `IconSize` built from a number or string. Icon names follow Lucide; when Lucide renames an icon the old name stays as an alias, and removing one is a breaking change here.
- **`UiStrings`**, `UiStringsProvider`, `use_ui_strings`, `fill`. New fields can be added in any release; build `UiStrings` with `..UiStrings::default()`.
- **Common behaviour**: `class` on a styled component adds to the kit's class; every other attribute you pass lands on the component's root element, except where a component's docs say it moves to the focusable control (`Select`, `Combobox`).
- **Theme contract** (`crates/ui/assets/theme.css`, [THEMING.md](THEMING.md)): the colour, radius, density, overlay and spring tokens (`--primary-color`, `--accent-color`, `--accent-contrast`, `--radius`, `--density`, `--overlay-color`, `--error-text-color`, `--spring-gentle|snappy|bouncy` and their `-duration`s, ...), the `data-theme` (`light`, `dark`), `data-accent` (the names in `ACCENTS`) and `data-density` (the names in `DENSITIES`) attributes, and the `.sr-only` class.
- **Registry item names** (the component folder names) and the `component.json` format, since `dx components add` and `atoll add` depend on them.
- **Features**: `web`, `desktop`.

### Experimental

| Item | Why |
|---|---|
| `form`: `use_form`, `Form`, `FormState`, `FormField`, `TextField`, `SelectField`, `ComboboxField`, `TimeField`, `DateTimeField`, `DateRangeField`, `FieldOption`, `Validator` and the rules, and `#[derive(Form)]` (crate `atoll-form-derive`) | Reshaped twice this week (`mounted`, then `field_with`/`load`/`reset` and the select and combobox fields). Raw `Select` wiring (dashboard gap 17) is still open. The derive is new on 2026-10-15 and its attributes may change with use. |
| `time_picker` and `date_time_picker` (`TimePicker`, `TimeState`, `HourCycle`, `DateTimePicker` and the ISO helpers) | New on 2026-10-15; min/max times and a time list popover may change the props. |
| `data_table` | Kit-written; the `toolbar` slot and `render_cell` are new and the column API is still settling. |
| `chart`: `BarChart`, `LineChart`, `AreaChart`, `PieChart`, `RadarChart`, `RadialChart`, `ChartLegend`, `ChartSeries`, `LegendMarker`, `ChartCursor`, `ChartKey` | Kit-written SVG; multi-series, the legend, pie and radial charts are new (charts v2), radar charts and `keyboard` on 2026-10-30, and their props may still change with use. The `--chart-*` tokens are part of the theme contract once the props settle. |
| `theme_customizer`: `use_theme`, `Theme`, `ThemeMode`, `ThemeScope` (and `ThemeCustomizer`'s markup) | New on 2026-10-14; `ThemeScope` exists for the native renderer, which is itself a preview. `ACCENTS` and `DENSITIES` values are stable (theme contract above). |
| Scoped theme and base styles: `ThemeStyles`'s `scoped` and `reset` props, `THEME_SCOPED_CSS`, `RESET_CSS`, `RESET_SCOPED_CSS`, `set_writes_html`, the `data-rui-scope` attribute `ThemeScope` renders, `reset.css`, and `atoll init --scoped/--reset`, `atoll postbuild --scoped` | New on 2026-10-30 for adding the kit to existing apps ([EXISTING_APP.md](EXISTING_APP.md)); the scope selector and what the reset covers may change with use. |
| `hooks::use_focus_opener`, `use_return_focus`, `ReturnFocus`, `FocusInside` | New on 2026-10-30: overlays return focus to their opener without a script (native). Built into the kit's overlays; the hook shape may change before your own overlays rely on it. |
| `atoll add` editing `mod.rs`, the crate root and `Cargo.toml` (`--no-wire` to opt out) | New on 2026-10-30; which files it edits and how it reports a differing pin may change. |
| `ToastsExt` and `ToastMessage` | New on 2026-10-14. `ToastProvider` and `use_toast` are stable. |
| `icons`: `Icon`, `IconData`, `IconNode`, the `icons::lucide` constants (`lucide::CHECK`, `ALL`), the `label` and `mirror_rtl` props and the named `IconSize` steps | New on 2026-10-14 with the generated Lucide set; the data shape may change when Lucide adds new SVG elements. |
| `hooks::Selection`, `use_selection`, `use_selected` | Built for the benchmark app; the API may change with its next use. |
| `number_field`: `NumberInput`, `NumberField` | New on 2026-10-15 (booking gap 12); `f64` values and the `empty` prop may change once forms need decimals or units. |
| The `router` feature, `hooks::follow_in_app`, `in_app_href`, `is_current_route`, and `MainContent`'s focus move | New on 2026-10-15 (booking gaps 10 and 11); rides on the Dioxus router's untyped context. |
| `chat`, `chat_composer`, `tool_call`: `ChatLog`, `ChatMessage`, `ChatRole`, `StreamingText`, `TypingIndicator`, `ChatAnnouncer`, `use_chat_announcer`, `ChatComposer`, `composer_key`, `ToolCall`, `ToolStatus`, `AgentSteps`, `AgentStep` | New on 2026-10-30 (agent UI set); props may change once a real agent app uses them (attachments, message grouping, retry). |
| `Pager`, `page_items`, `CardMedia`, `CarouselItem`'s `media`, `Select`'s `selected` | New on 2026-10-15 (booking and dashboard gaps). |
| Feature `native` | Does nothing today and is kept so apps that enable it keep building. Native (Blitz) support is a preview ([NATIVE.md](NATIVE.md)). |
| Re-exports from `dioxus-primitives` in the prelude (`use_toast`, `ToastOptions`, `ContentSide`) | Their shape follows the pinned primitives rev (our fork). A rev bump that changes them is announced as breaking. |

### Internal

- `ui::demos` (feature `demos`): the gallery and docs demos. The feature exists for this repo's apps and tests.
- Feature `contract-check`: makes the kit's build script check every `contract.toml`. It only builds inside this repository, because it reads the CLI's source.
- `ui::blocks` (feature `blocks`, on with `demos`): whole screens meant to be copied with `atoll add` and then changed. Their props may change in any release; a copy you own doesn't change under you.
- `ui::css_bundle`: the bundle hook used by `ThemeStyles` and `atoll postbuild`. Use `ThemeStyles` and the `postbuild` command instead of calling it.
- `ui::components::<name>` module paths. Import from `ui::prelude` (or `ui::hooks`, `ui::icons`); the module layout follows the registry and may move. `drag_and_drop_list` and `tag_group` are the exception: both export `RemoveButton`, so import them by module path.

## `atoll-motion` (`motion`, also `ui::motion`)

### Stable

- `SpringPreset`, `Transition`, `use_reduced_motion`, `Pop`.
- `AnimatePresence`, `use_is_present`, `PresenceMode` as used with `Pop` (keyed children, the default mode).

### Experimental

| Item | Why |
|---|---|
| `use_motion`, `MotionHandle`, `MotionStyle`, `AnimationConfig`, `AnimationError` | Re-exported straight from `dioxus-motion` `=0.4.0`, a one-maintainer crate whose API changes between minors. Moving off 0.4 changes these types. Prefer `use_spring` and `Pop`. |
| `use_spring`, `Spring` | Thin wrapper that may change when the engine does. |
| `FlipList`, `FlipItem` | Own FLIP implementation; web and desktop webview only. |
| `SwipeDismiss`, `SwipeDirection`, `SwipeOutcome`, `SwipeTracker` | Gesture API grew `vertical`/`one_way` recently and depends on pointer capture (none on native). |
| `motion::physics` (`SpringConfig`, `SpringState`, `Glide`, `GlideConfig`, `Animation`, `VelocityTracker`, `rubber_band`, `rubber_band_clamp`, `zoom_about`, `pan_bounds`) | Our own spring and momentum maths, new; names and defaults may still move. |
| `use_motion_value`, `MotionValue` | New frame loop over `motion::physics`; may change with the engine's timer. |
| `Draggable`, `DragAxis`, `DragBounds`, `DragEnd`, `DragModel` | New gesture API; pointer capture on web only. |
| `PinchZoom`, `PinchTracker`, `PinchFrame`, `pinch_view`, `zoom_view`, `clamp_view` | New gesture API; two-finger pinch needs touch, the rest works with mouse and keyboard. |
| `LayoutGroup`, `LayoutItem`, `LayoutRect`, `LayoutDelta` | New layout and shared-element animation; web and desktop webview only (needs element measurement). |

### Internal

- `motion::css` (`CssSpring`, `stylesheet`, `BEGIN_MARKER`, `END_MARKER`): generates the spring tokens in `theme.css`. The tokens are stable; the generator is not.

## `atoll-cli` (`atoll`)

### Stable

- The commands `list`, `init`, `add` and `diff` with their flags (`--dest`, `--force`), and the global `--source` / `ATOLL_SOURCE`.
- The `registry.json` format these commands read. Items with `"kind": "block"` (added 2026-10-15) keep their files under `crates/ui/src/blocks`; a CLI from before then can't install them, but reads everything else as before.
- Items with `"tier": "pro"` (added 2026-10-30) are Pro items from a private registry ([pro/README.md](pro/README.md)); older CLIs ignore the field and install them like any other item. The public registry has none.

### Experimental

- `atoll add` / `diff --blocks-dest` and the block items themselves: new on 2026-10-15.
- `atoll theme --brand <hex>`: its output changed to `light-dark()` pairs on 2026-10-13. `--from-image <logo>` is new on 2026-10-30; how it picks the colour may be tuned.
- `atoll postbuild <public>`: new on 2026-10-13; the bundle layout may change.
- `atoll doctor`: its checks grow with the guardrails, so a passing component can start warning after an update.
- `atoll acr` and `docs/acr/wcag22.toml`: new on 2026-10-30; rows change as checks are added.
- `atoll update [names] [--dry-run]`, the `atoll-lock.json` format, the `.atoll/base/` folder and the global `--lock`: new on 2026-10-15. `add` writes the lockfile since then.
- `atoll docs [name]` and the generated `llms.txt`, `llms-full.txt` and `llms/<name>.md`: new on 2026-10-15; the page layout may change.
- `atoll mcp` (MCP server over stdio) and its tool names, arguments and replies: new on 2026-10-15.
- `atoll info [--json]` (its JSON fields), `atoll skill` and `skills/atoll/SKILL.md`: new on 2026-10-30; fields may be added.
- `atoll contract` and the `contract.toml` format (docs/CONTRACTS.md): new on 2026-10-16; steps may be added.
- `atoll explain` and its `--json` shape: new on 2026-10-30; which errors it explains will grow.

### Internal

- `atoll new` and `atoll registry`: tools for working inside this repository.
- The library crate `atoll_cli` (every module and function). It is the binary's implementation, public for the repo's tests. Use the commands instead.

## Marking experimental items

When adding or reviewing a public item:

- In rustdoc, add a section to the item (or its module):

  ```rust
  /// # Stability
  ///
  /// Experimental: may change in any release (docs/STABILITY.md).
  ```

  Use "Internal: not part of the public API (docs/STABILITY.md)." for internal items.
- For a component, end its `docs.md` with a `## Stability` section saying it is experimental and why.
- Add it to the right table above, and move it to "Stable" (with a CHANGELOG "Changed" line) once its shape has held for a release.
