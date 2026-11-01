# Browser and OS support

Which browsers, operating systems and targets Atoll supports, and how each one is checked. "Supported" here means a test runs on it; anything else is said plainly. Each claim has a row in [CLAIMS.md](CLAIMS.md).

## Tiers

| Tier | Meaning | What we do when it breaks |
|---|---|---|
| **1: tested** | Every change runs the browser checks (axe, keyboard contracts, ARIA snapshots, flows) on it in CI. | Release blocker. Fixed before the next release. |
| **2: built** | CI builds or launches it, but no interaction tests run there. Behaviour is expected to match a tier 1 target because it uses the same engine or code path. | Fixed in a patch release once reproduced. |
| **3: best effort** | Not built or tested in CI. Fallbacks in the CSS are designed for it. | Fixed if the fix is small and doesn't hurt tier 1. Reports welcome. |
| **Not supported** | Known not to work. | Documented here; a pull request is welcome. |

## Web browsers

| Browser | Tier | Checked by |
|---|---|---|
| Chromium (Chrome, Edge, Brave, Opera), the version Playwright 1.63.0 ships | 1 | `.github/workflows/e2e.yml` on Linux, on release builds as GitHub Pages ships them (gallery and dashboard split) |
| Firefox, the version Playwright 1.63.0 ships | 1 | same |
| WebKit (Safari's engine), the version Playwright 1.63.0 ships | 1 | same (WebKit can't emulate forced colours, so that check is Chromium and Firefox only) |
| Safari on macOS and iOS | 2 | Same engine as the WebKit row, but run on Linux, not on Apple systems. Not checked on a real Mac, iPhone or iPad. |
| Chrome 123+, Firefox 120+, Safari 17.5+ (from 2023-2024) | 3 | Not tested. These versions have `light-dark()`, CSS nesting and the wasm features Rust's `wasm32-unknown-unknown` target enables by default. |
| Older browsers with WebAssembly | 3 | `theme.css` carries a generated fallback for engines without `light-dark()`; overlays keep a fixed side without CSS anchor positioning; the textarea keeps a fixed height without `field-sizing`. Expect rough edges. |
| Browsers without WebAssembly, or with JavaScript off | Not supported | Dioxus web apps need both. Server-rendered pages ([SSR.md](SSR.md)) show content before the wasm loads, but controls need it. |

Engine features the kit relies on, and what happens without them, are listed in [GUARDRAILS.md, Platform notes](GUARDRAILS.md#platform-notes-learned-keep-current) (anchor positioning, `field-sizing`, `light-dark()`).

## Desktop, mobile and native

| Target | OS | Tier | Checked by |
|---|---|---|---|
| Desktop (Dioxus desktop, system webview) | Linux (WebKitGTK) | 2 | `scripts/getting-started-check.sh` and `scripts/template-check.sh` build a fresh app's desktop target in CI; the gallery builds with `--features desktop`. No tests click in a desktop window. |
| Desktop | Windows (WebView2, Chromium-based) | 3 | Not built in CI. Same code path; WebView2 is the Chromium engine. |
| Desktop | macOS (WKWebView, WebKit) | 3 | Not built in CI. Same code path; WKWebView is the WebKit engine. |
| Android (system webview) | Android, API 34 tested | 2 | `.github/workflows/android.yml`: `cargo check` for two ABIs on every PR; an emulator job installs the gallery APK and fails on a crash (weekly). Nothing is clicked. |
| iOS (system webview) | iOS | Not supported yet | Not built. |
| Native renderer (Blitz), preview | Linux | 2 | `cargo test -p native` runs headless (no window or GPU): interaction tests, every contract row marked native, a keyboard fuzzer. A preview: some browser features are missing ([NATIVE.md](NATIVE.md)). |
| Native Android (Blitz) | Android | Not supported | Blocked on OpenSSL in Blitz's network stack and an `android_main` entry point. |

## Assistive technology

Automated checks cover roles, names, states and keyboard behaviour on every tier 1 engine. Screen readers are not run in CI. What still needs a person with NVDA, JAWS, VoiceOver or TalkBack, and what has been checked by hand, is in [SCREEN_READERS.md](SCREEN_READERS.md). Treat screen-reader output as tier 3 until that list is worked through.

## Toolchain

| Tool | Supported |
|---|---|
| Rust | stable, `rust-version` 1.88 or newer (raised only in a minor release, never to a Rust less than six months old: [VERSIONING.md](VERSIONING.md)) |
| Dioxus | 0.7.x (one Dioxus minor per kit minor; 0.8 is being prepared on its own branch) |
| `dx` | 0.7.10; split release builds need the patched `dx` from `scripts/install-dx.sh` |
| Node | only for the browser checks in `tests/e2e` (Playwright 1.63.0); apps never need Node |

## Changing this page

Moving a target up a tier means adding the CI job first. Dropping a target, or raising the minimum browser versions in tier 3, is announced in the CHANGELOG and gets a decision record ([decisions/](decisions/README.md)).
