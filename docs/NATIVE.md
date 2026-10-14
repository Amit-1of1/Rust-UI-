# Native (Blitz) status

Tested 2026-10-13 on `dioxus-native` 0.7.10 (Blitz 0.2). Static components, the modals (dialog, alert dialog, sheet, drawer, command palette), click-driven popups (popover, dropdown, select, combobox, date picker, multi select) and the click controls (checkbox, switch, radio, toggle, accordion, collapsible, tabs) work. Anything driven by pointer events, hover or `document::eval` does not (see Limits). Native is still a preview, not a target.

## How to run it

```sh
# once: software Vulkan + the xkb x11 library (Ubuntu names)
apt-get install -y mesa-vulkan-drivers libxkbcommon-x11-0 xvfb xdotool imagemagick

scripts/native-smoke.sh --build        # every demo, one window each, screenshots in target/native-smoke
scripts/native-smoke.sh dialog tabs    # just these
DEMO=table dx serve --renderer native --platform linux   # look at one (in apps/native)
```

`apps/native` renders one demo (`DEMO=<name>`) with the theme. Use `dx build --renderer native`, not plain `cargo build`: without `dx` the stylesheet paths are never rewritten and nothing is styled. `EXTRA_CSS=/path.css` injects a stylesheet at start-up so CSS overrides can be tried without a rebuild (class names from `css_module` are hashed, so match with attribute selectors or a wrapper class).

The smoke test fails a demo that exits, panics or draws a blank window. It cannot tell "looks right" from "looks wrong"; read the screenshots.

`apps/native/tests/interactions.rs` is the real check. It runs the demos on `dioxus-native-dom` (the same Blitz document the window uses, without a window or GPU), injects the kit CSS exactly as `#[css_module]` hashes it, clicks things, and asserts on layout and on CPU-painted pixels. It runs in `cargo test --workspace`, so `scripts/check.sh` covers it. `NATIVE_SHOTS=1 cargo test -p native --test interactions` also saves PNGs to `target/native-headless/`. `THEME=dark` forces the dark theme in `apps/native`.

## Results

All 68 demos start and draw (smoke test, before and after this pass). The headless interaction tests went from 2 of 10 passing on `main` to 10 of 10.

### Fixed

| Symptom | Cause | Fix |
| --- | --- | --- |
| Full gallery completely unstyled, never repaints | `SidebarProvider` ran `loop { if eval.recv().await.is_ok() { toggle } }`. Without JS `recv` fails at once, the loop never yields, the event loop starves and stylesheet-load events are never delivered | `while eval.recv().await.is_ok()` (`sidebar/component.rs`) |
| `data_table` shows no rows | Blitz collapses a `<table>` that is a child of `display: grid` | `.dx-data-table` is a flex column |
| `carousel` shows no slides | same pattern (`.dx-carousel` grid with an overflow track) | flex column |
| `calendar` shows no days | `display: flex` on `tbody`/`tr` inside a table renders nothing in Blitz | table layout (`table-layout: fixed`, `border-spacing`) |
| `tabs` panel invisible | the primitive sets `hidden: false`; Blitz keeps the attribute and the UA `[hidden]` rule wins | `.dx-tabs-content[data-state="active"] { display: block }` |
| Overlays "do not open" beside their trigger | Not an overlay bug: Blitz hit-tests the inline children of a block with padding (a button next to text) off by that padding, so the click lands on the parent. Wrapping in a `div` "fixed" it because the parent stopped being an inline box | `apps/native` puts its padding on an outer box. Apps: don't put padding or a border on a block whose children are all inline (buttons, text); wrap them in a `div` |
| Modals cover only their parent, run off the right edge | `position: fixed` is laid out as `absolute`, and Taffy places it relative to the *parent*; `translate(-50%, -50%)` is dropped (Blitz skips any transform with a percentage) | With the `native` feature, `ModalPortal` (component `overlay`) moves dialog, alert dialog, sheet and drawer into a window-sized `OverlayLayer` that `ThemeStyles` renders. Dialogs are centred by the backdrop with flexbox instead of a percentage transform (same look on the web) |
| Modal invisible if CSS animations don't run | backdrop is `opacity: 0` and only the animation's `forwards` fill made it visible | `[data-state="open"]` sets `opacity: 1` itself |
| `checkbox` does not toggle | the primitive's hidden form input is moved away with a percentage `translate` and `pointer-events: none`, neither supported, so it sat on the button and took the click | the input is sized 0x0 in `checkbox/style.css` |
| `multi_select` list empty | Blitz sizes a scrolling child's grid row to 0 | the panel is a flex column |
| `progress` demo frozen | its timer was a JavaScript `setInterval` | the demo uses buttons |
| `LineChart` has no line | Blitz draws inline SVG with usvg and ignores page CSS, so a `fill: none; stroke: …` class draws nothing | presentation attributes (`stroke="currentColor"`) as a fallback; CSS still wins where it applies |

### Limits (Blitz 0.2 / dioxus-native 0.7.10; not worked around)

- **No pointer events.** `dioxus-native` does not deliver `pointerdown/up/move` (`convert_pointer_data` is `unimplemented!`). The menubar trigger, slider dragging, drag-and-drop list, `SwipeDismiss` (drawer drag; its Close button and Escape work) and the context menu do nothing. Hover-only UI (tooltip, hover card) is untested and likely inert.
- **No `onmounted`, so no focus or measuring from Rust** (`convert_mounted_data` is `unimplemented!`). Together with `document::eval` answering every call with an error, focus management (dialogs' focus trap, `multi_select`/`form`/`input_otp` focus moves), the carousel scroll, `theme_customizer` (it sets attributes on `<html>`), the sidebar keyboard shortcut and `use_hotkey` are inert. There is no Rust-side replacement to switch to until Blitz implements these.
- **No `pointer-events`:** every element takes clicks inside its box, even an empty or invisible one. That is why the overlay layer is 0x0 and its children exist only while a modal is open.
- **Percentage transforms are dropped entirely** (`translate(-50%)` and anything combined with it). Popover, tooltip and hover card content centred on the trigger is shifted right by half its width; sheet slide-in animations don't slide. Visible and usable, not pretty.
- **`z-index` only orders siblings.** A dropdown can be covered by a later sibling of one of its ancestors. Modals avoid this via the overlay layer.
- **Overlay layer caveats:** it assumes the app is not scrolled (the layer scrolls with the page) and sets `body { margin: 0 }`. Portalled content sees contexts from above `ThemeStyles` and from inside the modal only.
- **CSS on inline SVG is ignored:** chart bars and dots draw in the default black; grid lines are missing.
- **Theme:** dark works. It follows the window's colour scheme (`prefers-color-scheme`; X11 reports none, so Xvfb is light), and `data-theme="dark"` on any element switches its subtree. `ThemeCustomizer` can't change it (it needs `eval`); set `data-theme` on a wrapper from Rust instead. A wrapper with `data-theme` resets an accent set on an outer element; put `data-accent` on the same wrapper.
- **Window defaults to 800x600**; fonts fall back to the system serif (`Inter` is not bundled).

### What to do next if native ever matters

1. Report upstream (Blitz / dioxus-native): the inline-root hit-test offset, `position: fixed` against the parent, percentage transforms, pointer and mounted events. Each reproduces in a few lines with `apps/native/tests/common`.
2. On the next dioxus-native release, run `cargo test -p native --test interactions` and the smoke test first; when pointer and mounted events land, the eval-based focus code can move to `MountedData::set_focus`.
