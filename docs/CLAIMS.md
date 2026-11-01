# Claims ledger

Every claim Atoll makes in public (the README, the docs site's home and pricing pages, the
launch posts and outreach drafts, the Pro pages and the competitor comparison), with the
evidence for it and the command that reproduces it. A claim that isn't in this file shouldn't
be made; a claim that is here and stops being true is a bug.

Audited on 2026-10-31 against `main` at `e303195`, on a 4-core shared cloud VM with headless
Chromium 141 (Playwright 1.63.0). Counted claims are checked on every push by
`cargo test -p atoll-cli --test claims` (`crates/cli/tests/claims.rs`); the browser claims run
in `.github/workflows/e2e.yml`; size and speed are measured by the scripts named in each row.
For the questions a skeptical reviewer asks, see [HARD_QUESTIONS.md](HARD_QUESTIONS.md) (its size table was updated to these measurements); build and hot-reload times are in bench/README.md, "Build times".

**Verdicts:** **CI** proven on every push. **Measured** re-measured for this audit, not
guarded by CI. **Fixed** was false or unproven; the code or the wording changed in this audit.
**Open** not proven; the claim is worded around it.

## Counts

| Claim | Verdict | Evidence | Reproduce |
|---|---|---|---|
| 81 components and 4 blocks | CI | `registry.json` has 88 items: 4 blocks, 3 support items (`hooks`, `icons`, `ui_strings`) and 81 components. The home page counted 83 before this audit (it included the support items). | `cargo test -p atoll-cli --test claims` |
| 28 keyboard contracts, every key tested | CI | One `contract.toml` per component: accordion, alert dialog, chart, chat, chat composer, checkbox, collapsible, combobox, command, dialog, drawer, dropdown menu, menubar, multi select, number field, popover, radio group, select, sheet, slider, switch, tabs, time picker, toggle, toggle group, tool call, toolbar, tooltip. Sheet, drawer, tooltip and toggle group were added in this audit (24 before). | `cargo run -p atoll-cli -- contract`; `node tests/e2e/contracts.mjs` |
| "Every component ships a keyboard contract" (old home page, launch posts) | Fixed | False: 28 of 80. Components with keyboard behaviour but no contract: context menu, calendar, date picker, date-time picker, colour picker, carousel, OTP input, resizable, tag group, data table, sidebar, drag-and-drop list. Several are driven by their own e2e scripts (otp, carousel, resizable, table, pickers, time pickers); the context menu can't be opened from the keyboard at all (the primitive only opens on right click). Wording now says which components have contracts. | `cargo run -p atoll-cli -- doctor` lists some; this list was made by reading each demo |
| Every Lucide icon | Measured | 1,890 icon components generated from Lucide 1.57.0 (`crates/ui/src/components/icons/lucide.rs`). docs/ICONS.md's "1,705 icons" is the 2026-10-13 option table for the old `dioxus-icons` crate, kept as history. | `grep -c "pub fn" crates/ui/src/components/icons/lucide.rs` |

## Accessibility

| Claim | Verdict | Evidence | Reproduce |
|---|---|---|---|
| axe passes on every demo, light and dark, on Chromium, Firefox and WebKit | CI | `tests/e2e/axe.mjs` scans the whole gallery (every demo is in it; `atoll doctor` fails a demo missing from the gallery) in light, dark and RTL, on each engine of the e2e matrix. Green on all three engines for `main` at `e303195` (run 41203281135, 2026-10-31). Covers the default (closed) state; open overlays are scanned by blocks.mjs, dashboard.mjs, booking.mjs and showcase.mjs. | `scripts/e2e.sh --browsers chromium,firefox,webkit` |
| axe under every accent preset | CI (Chromium) | The accent loop in `scripts/e2e.sh` runs all 5 presets in light and dark, on Chromium only. The claims test fails if a preset in theme.css is missing from the loop. | `cargo test -p atoll-cli --test claims` |
| Keyboard-only walkthrough of the whole gallery | CI | `tests/e2e/walkthrough.mjs`: every Tab stop visible with a focus indicator, no positive tabindex, Shift+Tab retraces, the walk reaches the end. | `GALLERY_URL=... node tests/e2e/walkthrough.mjs` |
| Keyboard state machines exhaustively model-checked: 1,182,416 states, 33.5M transitions, 263 configurations | CI | `crates/ui/tests/model_check.rs` (PR #73, merged 2026-10-31); it found and fixed two time picker range bugs. | `cargo test -p atoll-ui --test model_check` |
| Text contrast is proved for every accent preset | CI | `atoll_cli::contrast` evaluates every text/background pair in every component stylesheet, light and dark, every preset, including hover and selected states; fails on any unevaluated value. | `cargo test -p atoll-cli --test contrast` |
| A dialog without a title, or an icon button without a label, doesn't compile | CI | `compile_fail` doc tests on `Dialog`, `AlertDialog`, `Sheet`, `Drawer`, `IconButton`, `Avatar` and the chat components. | `cargo test -p atoll-ui --doc` |
| "Accessible" | Open | No screen reader has run (docs/SCREEN_READERS.md). axe finds only part of WCAG failures (Deque puts automated coverage at about 57% of issues). The copy says "accessible by test" and points here. | read docs/SCREEN_READERS.md |
| Accessibility Conformance Report from the tests | Fixed | `atoll acr` writes a draft VPAT from `docs/acr/wcag22.toml` (which checks prove each criterion) and the contracts, not from test results. Launch copy said "from the test results"; now "from the kit's checks". | `cargo run -p atoll-cli -- acr --product Demo` |

## Motion

| Claim | Verdict | Evidence | Reproduce |
|---|---|---|---|
| Springs, enter/exit, gestures and layout animation | CI | `crates/motion` unit tests; `tests/e2e/motion.mjs` samples frames mid-animation. | `cargo test -p atoll-motion`; `node tests/e2e/motion.mjs` |
| No unexplained layout animation: 173 composite, 74 paint, 24 explained layout | CI | `crates/cli/tests/motion.rs::no_unexplained_layout_animation_in_the_kit` (PR #75) classifies every animated property in the kit's CSS. | `cargo test -p atoll-cli --test motion` |
| 70 of 70 composite-only overlay animations run on the compositor; under reduced motion, nothing runs over 1 ms | CI | `tests/e2e/frames.mjs` (PR #75) presses every contract row: on every engine it checks each animated property is composite, paint or an explained layout property, and that nothing runs over 1 ms under reduced motion; the compositor verdict comes from a Chromium trace, so that part is Chromium only. Numbers: `bench/results/frames-2026-10-31-chromium.json`. | `node tests/e2e/frames.mjs` |
| All of it off under reduced motion | CI | theme.css zeroes every CSS animation and transition under `prefers-reduced-motion: reduce` (`atoll doctor` fails without it); springs read `use_reduced_motion`. motion.mjs checks springs, drag and layout are instant under reduced motion (Chromium only, timing-sensitive); frames.mjs checks no CSS animation runs over 1 ms under reduced motion on all three engines. | `node tests/e2e/motion.mjs` |

## Platforms

| Claim | Verdict | Evidence | Reproduce |
|---|---|---|---|
| Web | CI | Six apps built in release and tested in three engines (e2e.yml). | `scripts/e2e.sh --release` |
| Desktop | CI (builds) | `scripts/getting-started-check.sh` checks the desktop build of a fresh app; the gallery builds with `--features desktop`. Desktop is the system webview, so behaviour equals the web; no interaction tests run in a desktop window. | `scripts/getting-started-check.sh` |
| Android | CI (launch only) | `.github/workflows/android.yml`: `cargo check` for two ABIs on every PR; an emulator job builds the gallery APK, installs it and fails on a crash or panic (weekly, and on PRs touching the Android setup; last green 2026-10-31). Nothing is clicked on Android. iOS: not built. | Actions, "Android" |
| Native (Blitz), "preview" | CI | `cargo test -p native`: interaction tests, every contract row marked native (and rows marked not native must still fail), a keyboard fuzzer over every demo. docs/NATIVE.md lists what is missing. README said "It has not been run"; fixed. | `cargo test -p native` |
| No Tailwind build step | CI | No manifest, config or workflow mentions Tailwind. | `cargo test -p atoll-cli --test claims` |

## Size and speed

| Claim | Verdict | Evidence | Reproduce |
|---|---|---|---|
| Benchmark app's first screen: 287 KB vs 103 KB for React (home page, Show HN draft) | Fixed | Re-measured: **306 KB** (5 requests, 2.29 s on slow 4G) vs React **101 KB** (0.97 s) and Svelte 71 KB, brotli, split wasm. 3.0x React. The kit grew since 2026-10-13. Home page and drafts updated; `claims.rs` fails if the home page quotes a KB figure bench/README.md doesn't have. | `scripts/release-web.sh trial`, then `node bench/size.mjs` (bench/README.md, "Re-measured") |
| Split wasm cuts the gallery's first screen by about 70% (817 → 238 KB) | Fixed | 2026-10-14 numbers. Now 295 KB first screen of 1,105 KB in all (73% left for later); the unsplit build wasn't re-measured, so drafts now quote the current first screen and total instead of the cut. | `scripts/release-web.sh gallery`; `node bench/size.mjs gallery=...` |
| Startup after download is a tie | Measured | Ready to use 140 / 147 / 132 ms (Rust / React / Svelte), 488 / 493 / 463 ms at 4x CPU slowdown. | `node bench/run.mjs` |
| Fastest on 6 of 9 DOM operations (bench/README) | Fixed | Now 4 of 9 (replace 1,000, select, swap, create 10,000). README updated. | `node bench/run.mjs` |
| 532 lines vs about 1,400 | Fixed | 594 now (the split added `lazy` wrappers); React 1,400, Svelte 1,348. | `wc -l apps/trial/src/*.rs apps/trial/assets/*.css` |
| Release rebuild after a one-line change: 45 s | Measured | 46 s, twice (`dx build --release`, Taskboard). | change one string in apps/trial, `time dx build --release` |
| Docs site's first screen: 326 KB (was 1,089 KB); home page ready in 2.4 s on slow 4G (was 6.4 s) | Measured (budget in CI) | Split release build (`DocPage`, docs/notes/split-docs-and-render-cost.md), measured before/after in one session on 2026-11-01. A component page opened from a link got slower: 7.0 s vs 6.1 s on slow 4G. Total download grew 13% (1,276 KB). The new sizes are the docs budgets in bench/budgets.json. | `scripts/release-web.sh docs`, then `node tests/e2e/budgets.mjs` |
| Size and startup budgets in CI | CI | `tests/e2e/budgets.mjs` with `bench/budgets.json` (PR #77): `scripts/e2e.sh` fails on more than 10% growth of the first screen, the main wasm or the total download, or 2x startup. Baseline matches the numbers above. | `node tests/e2e/budgets.mjs` |

## Tooling

| Claim | Verdict | Evidence | Reproduce |
|---|---|---|---|
| `atoll create`, then `dx serve` | Fixed | `scripts/getting-started-check.sh` (release-readiness.yml) creates an app, builds web and desktop and compiles the guide's sample, using `--path` to the checkout. **The git route was broken for outside users:** GitHub's default branch was still an old work branch (`claude/project-thread-ogx0eo`), so the Quick start's `cargo install --git ... atoll-cli` failed with "could not find `atoll-cli`", and `atoll create --git` wrote a dependency cargo resolved to that branch ("no matching package named `atoll-ui`"). Fixed: the docs say `--branch main`, `atoll create --git` writes `branch = "main"` for this repository, and `atoll --source <this repo>` clones `main`. With those, a fresh `cargo install --git ... --branch main atoll-cli` and `atoll create my-app --git` built for wasm32 on 2026-10-31. The owner still needs to set the default branch to `main` (docs/RELEASE.md). | `scripts/getting-started-check.sh` |
| Copy components with `dx components add --git <repo>` (README, INSTALL.md) | Open (owner) | Broken for outside users on 2026-10-31: `dx` resolves component dependencies such as `icons` from GitHub's default branch (an old work branch), so `dx components add badge --git ... --rev <commit>` stops with "Component 'icons' not found", and without a rev it copies months-old code. Fixed by setting the default branch to `main` (docs/RELEASE.md); INSTALL.md says so and points to `atoll add` until then. | `dx components add badge --git https://github.com/Amit-1of1/Rust-UI-` in a fresh app |
| `atoll add` / `update` with a three-way merge | CI | `crates/cli/tests/cli.rs`, `crates/cli/tests/update.rs`. | `cargo test -p atoll-cli` |
| `atoll explain` | Fixed (copy) | Explains compiler errors from kit components (`crates/cli/tests/explain.rs`). The r/rust draft said it "prints the a11y contract"; that is `atoll contract <name>`. | `cargo test -p atoll-cli --test explain` |
| `atoll theme --from-image`, `atoll mcp`, agent skill, llms.txt | CI | `crates/cli/tests/{logo,mcp,info}.rs`; `skills/atoll/SKILL.md`; llms.txt regenerated and checked by `atoll registry --check`. | `cargo test -p atoll-cli` |
| Charts: bar, line, area, pie, radar, radial; keyboard cursor | CI | Chart contract (keyboard rows); hover text is an SVG `<title>`, not a styled tooltip. Outreach said "tooltips"; now "hover titles". | `node tests/e2e/contracts.mjs chart` |
| Chat set: log, streaming text, tool-call cards, composer | CI | chat, chat_composer and tool_call contracts and `tests/e2e/chat.mjs`. The composer has a slot for attach buttons; it does not handle attachments itself. Outreach said "composer with attachments"; fixed. | `node tests/e2e/chat.mjs` |

## Pro and pricing

| Claim | Verdict | Evidence | Reproduce |
|---|---|---|---|
| Pro v1: 13 marketing blocks, 2 motion components, a SaaS starter | Measured | Counted in the private Pro source folder (`docs/pro/CATALOG.md` lists them with screenshots). | `ls <Pro source>/crates/ui/src/blocks` |
| Pro is axe-clean in light and dark | Open (Chromium only) | Run on Chromium by the Pro thread; Firefox and WebKit run once the private `atoll-pro` repository exists and its `pro.yml` runs. Launch copy now says so. | `scripts/pro-e2e.sh` in the Pro repo |
| SaaS starter on web and desktop (pricing page) | Measured (desktop builds) | Web: the Pro thread drove the full flow in Chromium. Desktop: `cargo check -p saas-starter --no-default-features --features desktop` passes in a Pro repo built with `scripts/pro-repo.sh --overlay` (2026-10-31); nobody has clicked through it in a desktop window, and its demo session lives in `localStorage`, so on desktop sign-in is not kept across restarts. | `scripts/pro-repo.sh /tmp/atoll-pro --overlay <Pro source>`, then the `cargo check` |
| Nothing is sold yet | Measured | `CHECKOUT_URL_*` in `apps/docs/src/pricing.rs` are empty, so the buy buttons are disabled and the page says Pro is not on sale. | read pricing.rs |

## Competitors

Checked on 2026-10-31 (web search; numbers are what each project states).

| Claim | Verdict | Evidence |
|---|---|---|
| Other Dioxus kits are "larger in count" | Fixed (copy) | dioxus-shadcn: 83 components (2026-10-05). dioxus-element-plug: "107+" components. So two kits have more, and Atoll has 80. The crib sheet now says so instead of implying all are bigger. |
| No other Dioxus kit has an agent skill | False, removed | dioxus-element-plug ships a `SKILL.md` and `AGENTS.md`. What we found no other Dioxus kit doing: an MCP server, tested keyboard contracts, a contrast proof, motion. |
| Rust/UI has a Dioxus edition | Measured | dioxus.rust-ui.com changelog (components added 2026-08-08). |

## Not claimed, on purpose

- Nothing is on crates.io yet; installs are from git (README, docs/PUBLISHING.md).
- No screen-reader testing (above).
- Firefox and WebKit don't run the motion timing checks or the accent-preset axe scans.
- The primitives are a fork of DioxusLabs/components (`atoll-primitives`), and split wasm builds
  need our fork of `dx` (docs/FORKS.md). Apps that use the kit need no fork themselves
  (`scripts/consumer-check.sh`).
