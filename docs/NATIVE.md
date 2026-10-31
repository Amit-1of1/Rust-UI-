# Native (Blitz) status

Tested 2026-10-14 on `dioxus-native` 0.7.10 (Blitz 0.2) **with our patches** to `blitz-dom`, `blitz-paint` and `dioxus-native-dom` (in `vendor/`, see [vendor/README.md](../vendor/README.md)). Static components, the modals (dialog, alert dialog, sheet, drawer, command palette), click-driven popups (popover, dropdown, select, combobox, date picker, multi select), the click controls (checkbox, switch, radio, toggle, accordion, collapsible, tabs), hover UI (tooltip, hover card), the menubar, clicking a slider track, keyboard focus (Tab, Shift+Tab, arrow keys between tabs, Enter and Space on buttons) work. What is still missing is anything that needs `document::eval` (see Limits). Native is still a preview, not a target.

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

`apps/native/tests/engine.rs` checks the renderer patches one by one with a few lines of markup each (all 12 fail on the unpatched crates). `apps/native/tests/interactions.rs` is the real check. It runs the demos on `dioxus-native-dom` (the same Blitz document the window uses, without a window or GPU), injects the kit CSS exactly as `#[css_module]` hashes it, clicks things, and asserts on layout and on CPU-painted pixels. It runs in `cargo test --workspace`, so `scripts/check.sh` covers it. `NATIVE_SHOTS=1 cargo test -p native --test interactions` also saves PNGs to `target/native-headless/`. `THEME=dark` forces the dark theme in `apps/native`.

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
| Modals cover only their parent, run off the right edge | `position: fixed` is laid out as `absolute`, and Taffy places it relative to the *parent*; `translate(-50%, -50%)` is dropped (Blitz skips any transform with a percentage) | Fixed in the renderer fork (`position: fixed` against the window, percentage transforms); the kit's earlier `ModalPortal`/`OverlayLayer` workaround was removed. Dialogs are still centred by the backdrop with flexbox (same look on the web) |
| Modal invisible if CSS animations don't run | backdrop is `opacity: 0` and only the animation's `forwards` fill made it visible | `[data-state="open"]` sets `opacity: 1` itself |
| `checkbox` does not toggle | the primitive's hidden form input is moved away with a percentage `translate` and `pointer-events: none`, neither supported, so it sat on the button and took the click | the input is sized 0x0 in `checkbox/style.css` |
| `multi_select` list empty | Blitz sizes a scrolling child's grid row to 0 | the panel is a flex column |
| `progress` demo frozen | its timer was a JavaScript `setInterval` | the demo uses buttons |
| `LineChart` has no line | Blitz draws inline SVG with usvg and ignores page CSS, so a `fill: none; stroke: …` class draws nothing | presentation attributes (`stroke="currentColor"`) as a fallback; CSS still wins where it applies |

### Fixed in the renderer (our forks, 2026-10-14)

These were limits of Blitz 0.2 / dioxus-native 0.7.10. They are now fixed on our forks of the crates (`Amit-1of1/blitz`, `Amit-1of1/dioxus`); the table in [FORKS.md](FORKS.md) has the details and the test for each.

- **Pointer and hover events** (`pointerdown/move/up`, `mouseenter/leave`, `mouseover/out`, pointer enter/leave/over/out) are delivered, with real client, page and element coordinates. Tooltip and hover card open on hover; the menubar opens; clicking a slider track moves the thumb.
- **`onmounted`** fires, and `MountedData` answers `get_client_rect`, `set_focus`, `scroll_to`, `scroll` and the scroll queries. With **focus events** (`focus`, `blur`, `focusin`, `focusout`) this makes the primitives' roving focus work: arrow keys move focus between tabs (tested); radio and toggle groups, toolbars and menus use the same collection code.
- **Keyboard:** a click focuses the nearest focusable element; Tab and Shift+Tab move through focusable elements; Enter and Space activate buttons.
- **`position: fixed`** is laid out against the window.
- **Percentage transforms** (`translate(-50%, -50%)`) work, transforms nest, and text moves with its box.
- **`pointer-events: none`** and `visibility: hidden` let clicks through.
- **`z-index`** works across ancestors (stacking contexts), for painting and for clicks.
- **Click offset** beside text in a padded block is gone; boxes sticking out of their parent are clickable.
- `disabled: false` and `hidden: false` no longer disable or hide (false boolean attributes are removed, as on the web).
- **Keyed lists** keep every item when reordered (upstream dropped a node moved within its own parent).
- **Right click** fires `contextmenu`; **images** fire `load` and, when they fail, `error`; checkboxes and radios fire `change`.
- **Forms:** a submit button click and Enter in a field fire `submit`.
- `MountedData::scroll` goes to a position, as on the web.

### Fixed in the primitives (our fork, 2026-10-14)

The primitives used `document::eval` for document-level listeners. Where it can't run, our fork of DioxusLabs/components does the same with element events ([FORKS.md](FORKS.md) has the list and tests):

- **Escape** closes dialogs, alert dialogs, popovers and the context menu, and focus goes back to the button that opened the overlay (docs/notes/focus-return.md).
- **Clicking outside** closes dialogs, popovers and the context menu.
- **Focus traps:** dialogs and popovers take focus when they open, and Tab doesn't leave a modal one.
- **Dragging** works for the slider, the color area and the drag-and-drop list.
- **Context menu** opens at the right-click point. **Avatar** shows its image once it loads, and its fallback when the image fails.
- **Toast:** F6 moves focus to the notifications.
- **Theme customizer** works inside a `ThemeScope` wrapper.

### Fixed in the kit (2026-10-14)

The kit's own `eval` calls now go through the element handle (`MountedData`) where that works on every renderer, and keep the script only where it adds something on the web:

- **Multi select** returns focus to its button on Escape; **form** focuses the first invalid field on a failed submit (`TextField` does it; your own controls pass `onmounted` to `field.mounted`); **input OTP** moves focus between boxes, and typing over a filled box replaces its character.
- **Carousel** Previous/Next scroll the track (instantly on native; smoothly on the web unless reduced motion is on).
- **Sidebar:** Ctrl+B (Cmd+B) toggles it while focus is inside the layout.

The `ModalPortal`/`OverlayLayer` workaround (and the `overlay` component) is gone: with the patched renderer, `position: fixed` covers the window by itself. The other workarounds from the first pass (flexbox-centred dialogs, the 0x0 checkbox input, the tabs `[hidden]` rule) stay; they look the same on the web. The kit's `native` feature now does nothing and is kept so apps that turn it on keep building.

### Limits (still open)

- **`document::eval` answers every call with an error**, and there is no JavaScript engine. Neither the primitives nor the kit need it for their behaviour any more. What still uses it only on the web: selecting a box's text in input OTP, smooth carousel scrolling, the sidebar's cookie and its mobile breakpoint (native always gets the desktop layout), and setting the theme on `<html>`.
- `SwipeDismiss` drags (drawer) need pointer capture, which native does not have; the Close button and Escape are the way out.
- **CSS on inline SVG is ignored:** chart bars and dots draw in the default black; grid lines are missing.
- **Theme:** dark follows the window's colour scheme (`prefers-color-scheme`; X11 reports none, so Xvfb is light), and `data-theme="dark"` on any element switches its subtree. `ThemeCustomizer` changes it inside a `ThemeScope` wrapper (it can't reach `<html>` without `eval`); or set `data-theme` on a wrapper from Rust. A wrapper with `data-theme` resets an accent set on an outer element; put `data-accent` on the same wrapper.
- **Window defaults to 800x600**; fonts fall back to the system serif (`Inter` is not bundled).
- Limits of the renderer patches themselves are listed in [FORKS.md](FORKS.md#known-limits-of-the-patches).

### What to do next

1. On the next dioxus-native release, move the fork branches to it (FORKS.md, "Moving to a new upstream release") and run `cargo test -p native` and the smoke test first.
