# Native (Blitz) status

Tested 2026-10-11 on `dioxus-native` 0.7.10 (Blitz 0.2), software Vulkan (lavapipe), Xvfb. The kit is **partly usable on native**: static and stateless components work, anything that opens an overlay does not. Treat native as a preview, not a target.

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

## Results

All 67 demos start and draw. After the fixes below every demo draws its static state; what does not work is interaction, positioning and CSS on inline SVG, listed under Limits.

### Fixed

| Symptom | Cause | Fix |
| --- | --- | --- |
| Full gallery completely unstyled, never repaints | `SidebarProvider` ran `loop { if eval.recv().await.is_ok() { toggle } }`. Without JS `recv` fails at once, the loop never yields, the event loop starves and stylesheet-load events are never delivered | `while eval.recv().await.is_ok()` (`sidebar/component.rs`) |
| `data_table` shows no rows | Blitz collapses a `<table>` that is a child of `display: grid` | `.dx-data-table` is a flex column |
| `carousel` shows no slides | same pattern (`.dx-carousel` grid with an overflow track) | flex column |
| `calendar` shows no days | `display: flex` on `tbody`/`tr` inside a table renders nothing in Blitz | table layout (`table-layout: fixed`, `border-spacing`) |
| `tabs` panel invisible | the primitive sets `hidden: false`; Blitz keeps the attribute and the UA `[hidden]` rule wins | `.dx-tabs-content[data-state="active"] { display: block }` |
| `LineChart` has no line | Blitz draws inline SVG with usvg and ignores page CSS, so a `fill: none; stroke: …` class draws nothing | presentation attributes (`stroke="currentColor"`) as a fallback; CSS still wins where it applies |

### Limits (not fixed, mostly Blitz or the pinned primitives)

- **Overlays do not open reliably.** `Dialog`, `Popover`, `DropdownMenu` etc. render nothing when their trigger sits directly beside the (initially empty) overlay with no element in between: the node is inserted but never shown. Repro: `Button` then `Dialog` as siblings does nothing; wrap the `Dialog` in a `div` and it opens. Not fixed here because it is in how Blitz applies the placeholder replacement, and every primitive overlay has the same shape.
- **`position: fixed` is relative to the parent, not the window,** and `transform: translate(-50%, -50%)` is not applied. An open dialog is mispositioned (starts at the window's centre line and runs off the right edge) and the backdrop covers only its parent.
- **Everything built on `document::eval` is inert:** checkbox toggling, focus management in `multi_select`/`form`/`input_otp`, the theme switch in `theme_customizer`, the progress demo's timer, `use_hotkey`. Dioxus' native document answers every `eval` with an error.
- **CSS on inline SVG is ignored:** chart bars and dots draw in the default black (no theme, no dark mode); the grid lines are missing.
- **Light theme only.** Blitz does not match `prefers-color-scheme` here and there is no `html` element to put `data-theme` on. (It did render fine with the theme as shipped: the token pairs resolve to the light values.)
- **Window defaults to 800x600**; fonts fall back to the system serif (`Inter` is not bundled).
- Starting from `dx serve` hot reload was not tried.

### What to do next if native ever matters

1. Ask upstream (Blitz) about the placeholder insertion and `position: fixed` bugs; both are reproducible with the probe in this document.
2. Replace the primitives' `use_animated_open` / `use_outside_dismiss` with Rust-only versions in a kit-owned overlay layer (needs GUARDRAILS to allow it, since it stops using those primitives).
3. Keep running `scripts/native-smoke.sh` after the next dioxus-native release; most "Limits" above are renderer bugs and may just go away.
