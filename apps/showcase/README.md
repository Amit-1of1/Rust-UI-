# Rust UI Showcase

One crate, one set of pages, every Dioxus renderer. The pages (Home, Forms, Overlays,
Settings), their state and their CSS are shared; the only per-target code is the launch call
in `src/main.rs` and the label that says which renderer is running. Wide windows get a side
rail; at 640px and below the same pages get a bottom tab bar (`BottomNav`), chosen by a CSS
media query so it works on every renderer.

The native build draws with Blitz (Stylo, Taffy, Vello) through `dioxus-native`: no webview
and no JavaScript.

## Run it

From `apps/showcase`:

| Target | Command |
| --- | --- |
| Web | `dx serve --platform web` |
| Desktop, native (Blitz) | `dx serve --platform linux --renderer native` (or `macos`, `windows`) |
| Desktop, webview | `dx serve --platform desktop` |
| Android, native | not wired up yet, see below |

Use `dx`, not plain `cargo run`, for the native build: without `dx` the stylesheet paths are
not rewritten and nothing is styled (docs/NATIVE.md).

## Checks

- `cargo test -p native --test showcase`: the app on the headless Blitz document (the
  renderer the native window uses, without a GPU) at 1024x768 and at a 390x844 phone size.
  Layout (rail vs tab bar, stacked cards, no overflow), navigation, form validation and
  save with a toast, dialog/sheet/popover/menu inside the window, dark mode switch.
  `NATIVE_SHOTS=1` saves PNGs to `target/native-headless/showcase-*.png`. `scripts/check.sh`
  and CI run it.
- `tests/e2e/showcase.mjs` (in `scripts/e2e.sh`): the web build at 1100x780 and 390x844,
  light and dark. Every page and the open dialog and sheet pass axe; navigation, form and
  toast flows; no sideways scrolling.

## Status per target (2026-10-15)

Only what was actually run is marked as working.

| Target | Status | How it was checked |
| --- | --- | --- |
| Web (Chromium) | Works | `showcase.mjs` passes at desktop and phone size, light and dark |
| Web (Firefox, WebKit) | Works | `showcase.mjs` passes on both in CI (`scripts/e2e.sh`) |
| Desktop, native, Linux | Works, with the gaps below | Real `dx build --renderer native` window under Xvfb (software Vulkan): navigated with real clicks, dark mode, window resized to 390x844 to get the phone layout, opened a dialog; no panics. Plus the headless tests |
| Desktop, native, macOS / Windows | Not run | Same crate and renderer; no machine here |
| Phone layout on native | Works (headless and resized window) | Headless tests at 390x844, and the real window resized to 390x844 |
| Android, native | Not working yet | `cargo check --target aarch64-linux-android` stops in `openssl-sys` (needs the Android NDK, which this container can't download). The app also needs an `android_main` entry point that hands the activity to `blitz_shell::set_android_app`; dx 0.7.10's Android template is built for the webview. Not attempted beyond the check |
| iOS, native | Not run | Needs macOS and Xcode |
| Desktop, webview (wry) | Not run | Builds with `--features desktop` are not checked here (no WebKitGTK in this container) |

## Known gaps on native

Found while building this app; none needs code in the app.

1. **`display: contents` wrappers (fixed in the blitz-dom fork).** Blitz 0.2.4 dropped the
   box of an element inside a `display: contents` parent (`ThemeScope`, `ToastProvider`),
   so the whole app was as wide as its longest line. The pinned blitz-dom fork now treats a
   contents child's children as direct children (docs/FORKS.md); the engine test
   `children_of_display_contents_keep_their_box` covers it, and the app needs no CSS for it.
2. **Hairline outlines.** `box-shadow` with zero blur and a spread (the kit's 1px input,
   select, popover and menu outlines) is not drawn by the CPU painter used in headless
   captures, so those controls look borderless in `target/native-headless`. The windowed
   (GPU) renderer fills them.
3. **Charts** draw in black (CSS on inline SVG is ignored, docs/NATIVE.md), which is
   invisible in dark mode.
4. **Dark mode switch** in the header is hard to see in dark mode on native.
5. Headless captures place the slider thumb below the track; the window draws it on the track.

Screenshots: `rust-ui/showcase/` in the project files.
