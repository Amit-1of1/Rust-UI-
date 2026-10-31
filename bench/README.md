# Taskboard benchmark

One small app, Taskboard, built three ways to the same spec ([SPEC.md](SPEC.md)):

| | Code | Stack |
|---|---|---|
| Rust | [`apps/trial`](../apps/trial) | Dioxus 0.7.10, this kit (`atoll-ui`), wasm |
| React | [`bench/react`](react) | React 19.3, Radix primitives (what shadcn/ui uses), cmdk, Vite 8 |
| Svelte | [`bench/svelte`](svelte) | Svelte 5.57, Bits UI (what shadcn-svelte uses), svelte-sonner, Vite 8 |

The app uses tabs, a sortable, filterable, selectable, paged data table, a modal form with validation, a select, a radio group, a toast, a Ctrl+K command menu, a chart, a progress bar, a slider, an autosizing textarea and a dark mode switch. A `?bench` route repeats the js-framework-benchmark operations on a plain keyed table. [README-competitors.md](README-competitors.md) covers how the React and Svelte versions were built and where they deviate.

## Results

### Re-measured on 2026-10-31 (current numbers)

Rerun for the claims audit ([docs/CLAIMS.md](../docs/CLAIMS.md)) on `main` at `e303195`: the same 4-core VM type, headless Chromium 153 (Playwright 1.63.0), release builds from `scripts/release-web.sh` (split wasm, one CSS file). Raw numbers: [`results/size-2026-10-31.json`](results/size-2026-10-31.json) (`size.mjs`, 5 runs) and [`results/run-2026-10-31.json`](results/run-2026-10-31.json) (`run.mjs`, 7 runs). The kit grew from about 50 to 80 components since the tables further down, and the downloads grew with it.

| First screen, brotli (`size.mjs`) | Requests | Size | Styled on slow 4G |
|---|---|---|---|
| Taskboard, Rust (split) | 5 | **306 KB** | 2.29 s |
| Taskboard, React | 3 | **101 KB** | 0.97 s |
| Taskboard, Svelte | 3 | 71 KB | 0.83 s |
| Gallery (split; everything: 1,105 KB) | 5 | 295 KB | 2.35 s |
| Dashboard (split; everything: 752 KB) | 5 | 347 KB | 2.48 s |

The Taskboard's first screen is 3.0x React's and 4.3x Svelte's (it was 2.9x and 4.1x on 2026-10-14). The gallery's first screen grew from 238 KB to 295 KB and its lazy part from 671 KB to 806 KB.

| `run.mjs`, median of 7 | Rust | React | Svelte |
|---|---|---|---|
| Ready to use (local, fast CPU) | 140 ms | 147 ms | 132 ms |
| Ready to use (CPU slowed 4x) | 488 ms | 493 ms | 463 ms |
| Create 1,000 rows | 96 | **77** | 105 |
| Replace 1,000 rows | **84** | 85 | 95 |
| Update every 10th row | 20 | 18 | **15** |
| Select a row | **3.5** | 5.8 | 5.1 |
| Swap two rows | **13** | 63 | 15 |
| Remove a row | 19 | **18** | 20 |
| Create 10,000 rows | **734** | 839 | 992 |
| Append 1,000 to 1,000 | 128 | **109** | 144 |
| Clear 1,000 rows | 11 | 12 | **9.4** |
| Filter / sort 1,000 tasks | 16 / 14 ms | 14 / 18 ms | 16 / 24 ms |
| axe violations (4 states), keyboard checks | 0, 11/11 | 0, 11/11 | 0, 11/11 |
| Whole browser RSS with 10,000 rows | 571 MB | 545 MB | 542 MB |
| Lines written (app + CSS) | 594 | 1,400 | 1,348 |
| Release rebuild after a one-line change (`dx build --release`, 2 runs) | 46 s | | |

Startup once the bytes have arrived is still a tie (within 6% at 4x CPU). Rust is fastest on 4 of the 9 DOM operations now (6 of 9 on 2026-10-13): React is faster at creating 1,000 rows, removing a row and appending, Svelte at partial updates and clearing. The Taskboard is 594 lines now (532 before the wasm split, which added `lazy` wrappers). The tables below are the earlier rounds, kept for history.

### Build times (edit, compile, see it)

Measured 2026-10-31 on the same 4-core cloud VM, Rust 1.97, dx 0.7.10, with the gallery (it pulls in every component, so it is the slowest case for an app). Commands: [`build-times.sh`](build-times.sh) for the cargo rows, [`hot-reload.mjs`](hot-reload.mjs) for the `dx serve` rows (time from saving the file until the change shows in headless Chromium).

| Step | Time |
|---|---|
| Cold build, dev profile, wasm (dependencies already downloaded) | 125 s |
| Rebuild with no change | 0.5 s |
| Rebuild after editing one kit component, dev | 19 s |
| Rebuild after editing the app, dev | 19 s |
| Cold build, release (`opt-level = "z"`, fat LTO, one codegen unit) | 160 s |
| Rebuild after editing the app, release | 53 s |
| `dx serve`: change text or markup inside `rsx!` | **0.8 s** (hot reload, no compile) |
| `dx serve`: change a component's CSS | **0.8 s** (hot reload, no compile) |
| `dx serve`: change Rust code outside `rsx!` literals | 62 s (full rebuild, then reload) |

So markup and styling iterate as fast as a JS dev server, and Rust logic changes cost about 20 s with `cargo build` (about a minute through `dx serve` on this machine, which also runs wasm-bindgen and asset processing). `dx serve --hot-patch` (Dioxus' Subsecond patching of Rust code) failed on the gallery with "Failed to emit module: failed to parse import section", so it is not counted here. A JS framework rebuilds a one-line change in about 1 to 3.5 s ("The real app" below).

### Earlier rounds (2026-10-13 and 2026-10-14)

These are from `results/latest.json`: release builds served as static files, headless Chromium 141, a 4-core shared cloud VM, and the median of 7 runs after one warm-up. Absolute numbers are noisy on this machine, so compare the columns with each other.

#### Download and startup

| | Rust | React | Svelte |
|---|---|---|---|
| Transferred, gzip | 513 KB | 120 KB | 83 KB |
| Transferred, brotli | 378 KB | 103 KB | 72 KB |
| Requests | 15 | 3 | 3 |
| Ready to use (local, fast CPU) | 144 ms | 126 ms | 130 ms |
| Ready to use (CPU slowed 4x) | 490 ms | 460 ms | 500 ms |
| Download on slow 4G (1.6 Mbps, brotli, computed) | ~1.9 s | ~0.5 s | ~0.4 s |

The wasm file is most of the Rust download. Once the bytes have arrived, startup is a three-way tie. The size cost shows up only on the network.

#### Shrinking the Rust download

The table above is the plain `dx build --release`. [`scripts/release-web.sh trial`](../scripts/release-web.sh) adds two steps:

- **One CSS file.** `atoll postbuild` puts the theme and the stylesheets of the components the app uses into one file linked from `index.html`, and `ui::ThemeStyles` stops components from linking their own. A stylesheet counts as used when its hashed class names appear in the wasm.
- **Split wasm.** `dx build --wasm-split` with the Stats and Settings tabs, the New task dialog, the command menu and the bench page as `#[component(lazy)]`. Each gets its own wasm file. The dialog downloads in the background as soon as the board is up, and the rest on first open.

| Rust only | Before | One CSS file | Both |
|---|---|---|---|
| Requests for the first screen | 15 | 6 | 6 |
| First screen, brotli | 366 KB | 368 KB | **287 KB** |
| Main wasm, brotli | 349 KB | 351 KB | **268 KB** |
| Styled first screen on slow 4G (served brotli, 1.6 Mbps, 150 ms RTT) | 2.76 s | 2.60 s | **2.16 s** |
| Ready to use (CPU slowed 4x) | 530 ms | | 450 ms |
| Everything, after opening every screen, brotli | 368 KB | 368 KB | 444 KB |

Medians of 7 runs on 2026-10-13, measured with scripts like `bench/run.mjs` against a server that sends brotli. The first screen is 22% smaller and comes up 0.6 s sooner on slow 4G. The later files hold 75 KB (the dialog, fetched right after start) and 16, 43 and 24 KB (Stats, Settings, command menu). The whole app grows by 76 KB, because split files repeat some shared code. DOM operations, keyboard checks (11/11) and axe (0 violations) didn't change.

The CSS file alone saves requests, not bytes: the component styles are 6 KB brotli in total.

#### Split by default: the gallery and the dashboard

Since 2026-10-14 `scripts/release-web.sh` splits the gallery and the dashboard too. Measured with [`size.mjs`](size.mjs): served with brotli, 5 runs, "slow 4G" is 1.6 Mbps down with 150 ms RTT, and "everything" is after every screen has been opened once. Before is the plain release build plus the one CSS file; after adds the split, built with `dx` from `scripts/install-dx.sh`. Raw numbers: `results/size-2026-10-14.json`.

| | Gallery before | Gallery after | Dashboard before | Dashboard after |
|---|---|---|---|---|
| First screen, brotli | 817 KB | **238 KB** | 510 KB | **316 KB** |
| Requests for the first screen | 9 | 5 | 6 | 5 |
| Styled first screen on slow 4G | 5.10 s | **1.87 s** | 3.33 s | **2.32 s** |
| Everything, brotli | 817 KB | 913 KB | 510 KB | 700 KB |

What is lazy was picked by measuring:

- **Gallery:** the first six sections (button to card) are in the main file; everything below them is one lazy part (`MoreSections`, 671 KB brotli) that starts downloading as soon as the first screen is up. One lazy part per demo was tried first: the first screen was 296 KB, but code shared by several demos and not by the first screen is copied into every part that uses it, so the whole gallery came to 1,957 KB, 2.4x the unsplit build.
- **Dashboard:** Overview (the first page) and the shell are in the main file. Customers (131 KB), Orders (152 KB) and Settings (81 KB) each download when first opened, and the Ctrl+K command menu (20 KB) on first open. The total grows by 190 KB because the three pages each carry their own copy of the table, select and form code; a visitor pays that only by opening every page.
- **Taskboard:** unchanged (Stats, Settings, New task dialog, command menu and bench page).

Stock `dx` 0.7.10 can't build these: its splitter left the gallery's lazy part and the dashboard's Settings page as ~450-byte files, with their code still in the main file, and crashed on the lazy command menu. Both bugs are fixed on our dioxus fork ([docs/FORKS.md](../docs/FORKS.md)).

The React and Svelte comparison, rerun the same way on 2026-10-14 (Taskboard first screen; its later files are not opened here):

| | Rust (split) | React | Svelte |
|---|---|---|---|
| First screen, brotli | 291 KB | 101 KB | 71 KB |
| Requests | 5 | 3 | 3 |
| Styled first screen on slow 4G | 2.29 s | 0.97 s | 0.81 s |

The Taskboard's split didn't change in this round, so the gap to React and Svelte is where it was (2.9x and 4.1x for the first screen).

#### DOM operations (`?bench`, ms, lower is better)

Each time runs from the start of the click to the end of the last paint it causes, read from a Chrome trace, as js-framework-benchmark does.

| | Rust | React | Svelte |
|---|---|---|---|
| Create 1,000 rows | 107 | **98** | 129 |
| Replace 1,000 rows | 99 | **96** | 124 |
| Create 10,000 rows | **889** | 1,124 | 1,189 |
| Append 1,000 to 1,000 | **135** | 156 | 153 |
| Swap two rows | **13** | 81 | 17 |
| Update every 10th row | 18 | 17 | **13** |
| Select a row | **3.1** | 7.3 | 5.6 |
| Remove a row | **18** | 21 | 26 |
| Clear 1,000 rows | **12** | 18 | 14 |

On 2026-10-13 Rust was fastest on 6 of the 9 operations (4 of 9 on 2026-10-31, above). It is within 10% of React on creating and replacing 1,000 rows, and Svelte is fastest at updating every 10th row.

Two changes got it here (see "Tuning" below).

#### The real app

| | Rust | React | Svelte |
|---|---|---|---|
| Filter 1,000 tasks | 16 ms | 17 ms | 19 ms |
| Sort 1,000 tasks | 17 ms | 19 ms | 27 ms |
| axe violations (4 states) | 0 | 0 | 0 |
| Keyboard checks passed | 11 / 11 | 11 / 11 | 11 / 11 |
| Lines written (app + CSS) | **532** | 1,400 | 1,348 |
| Release rebuild after a one-line change | 45 s | ~1 s | ~3.5 s |

#### Memory

| | Rust | React | Svelte |
|---|---|---|---|
| Whole browser RSS, idle | 378 MB | 374 MB | 372 MB |
| Whole browser RSS, 10,000 rows | 572 MB | 549 MB | 549 MB |
| JS heap, 10,000 rows | 3.7 MB | 14.8 MB | 21.1 MB |

The small JS heap for Rust is misleading, because wasm memory isn't counted in it. The whole-process numbers are the real comparison. Rust uses about 4% more with 10,000 rows, because each row is its own component.

## Tuning

The first run (PR #16) had two measurement problems. Both are fixed:

- The Rust bench page loaded no CSS, so its table had no padding, borders or highlight. That made layout and paint cheaper than in the React and Svelte pages, and a selected row never repainted. It now loads the same table styling as theirs.
- Times were read by polling the DOM and then waiting a frame, which added a variable delay to frameworks that update in a microtask. Times now come from Chrome traces.

Then two changes to the Rust side:

- **`Selection` (new in the kit, `ui::hooks`).** Each row owns a `bool` signal, and selecting a row flips only the rows that change, the idea behind SolidJS's `createSelector`. Selecting one row of 1,000 went from about 7 ms of wasm work to 0.15 ms, and from 12 ms to 3 ms end to end. `crates/ui/tests/selection.rs` checks that only the changed rows render.
- **Rows with cheap, stable props.** A row component gets its id, an `Rc<str>` title and the selection. None of these change unless the row does, so when the list re-renders Dioxus skips every unchanged row. Before, each row got fresh event-handler props and re-rendered every time. Remove dropped from 27 to 16 ms, and update from 19 to 15 ms.

What didn't help:

- Dioxus Stores (`#[derive(Store)]`) made updates faster but swap, remove and clear slower, because the stores are indexed by position, not by key.
- Inlining rows instead of using a component made no difference.
- Compiling logging out of release builds (`release_max_level_off` for `tracing` and `log`) saved only 3% of the wasm, and the app keeps it.

## What this shows

Better:

- About 2.3x less code to write: 594 lines against roughly 1,400. The kit ships styled, accessible components, while the React and Svelte versions style bare primitives by hand.
- Fastest on 4 of 9 DOM operations (2026-10-31), including 10,000 rows (734 ms against 839 and 992), selection and swapping.
- Accessibility and keyboard behaviour come out of the box.
- The compiler type-checks the whole app.

Worse:

- The download is large: with the split build the first screen is 3.0x React's and 4.3x Svelte's (2026-10-31).
- Release builds are slow: 46 s for a one-line change. Day-to-day work uses `dx serve`, which hot-reloads.
- A plain build ships every component's CSS as its own file, so it makes 15 requests where the others make 3. `scripts/release-web.sh` brings that to 6.
- Svelte is still faster at updating many rows in place, and React at creating 1,000 rows, removing a row and appending.

## Ideas for the gaps that remain

The first two are done (see [Shrinking the Rust download](#shrinking-the-rust-download)). The rest are untried. Each one needs the user's go-ahead, and the last one breaks a rule in GUARDRAILS.

1. ~~**Split the wasm by screen.**~~ Done with `#[component(lazy)]` and `dx build --wasm-split`.
2. ~~**One CSS file for the components an app uses.**~~ Done as `atoll postbuild`.
3. **A keyed list primitive.** Something like `use_keyed_list`, where each row's data lives in a signal owned by that row and keyed by id rather than position, so updates touch only the changed rows and moves touch none. This is the Stores idea, keyed properly.
4. **A size budget in the gate.** Fail `scripts/check.sh` when the trial's brotli wasm grows past a set limit, and use `twiggy` to show what grew. The biggest items today are Dioxus's own runtime, keyboard-name tables and JSON code used by `document::eval`.
5. **Compile templates to direct DOM calls** (how Svelte and Solid get their speed). This would bypass Dioxus's virtual DOM, which counts as building a renderer, so GUARDRAILS rules it out unless that decision changes.

## Running it

```sh
# Rust: release build, split, one CSS file (needs dx from scripts/install-dx.sh and wasm-opt on PATH)
scripts/release-web.sh trial
python3 -m http.server 8801 -d target/dx/trial/release/web/public &
# React and Svelte
(cd bench/react && npm ci && npm run build) && python3 -m http.server 8802 -d bench/react/dist &
(cd bench/svelte && npm ci && npm run build) && python3 -m http.server 8803 -d bench/svelte/dist &
# Measure (Playwright and axe come from tests/e2e)
cd tests/e2e && RUNS=7 node ../../bench/run.mjs > ../../bench/results/latest.json
```

`TARGETS="name=url,..."` picks the builds and `ONLY=load,ops,app,memory` runs only some parts.

Sizes and first-screen times for any set of builds (it serves each folder itself, with brotli):

```sh
scripts/install-dx.sh   # once: dx with the splitter fixes
scripts/release-web.sh gallery && scripts/release-web.sh dashboard
cd tests/e2e && node ../../bench/size.mjs gallery=../../target/dx/gallery/release/web/public \
  dashboard=../../target/dx/dashboard/release/web/public trial=../../target/dx/trial/release/web/public \
  react=../../bench/react/dist svelte=../../bench/svelte/dist
```
