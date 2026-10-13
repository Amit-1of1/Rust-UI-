# Taskboard benchmark

One small app, Taskboard, built three ways to the same spec ([SPEC.md](SPEC.md)):

| | Code | Stack |
|---|---|---|
| Rust | [`apps/trial`](../apps/trial) | Dioxus 0.7.10, this kit (`rust-ui-kit`), wasm |
| React | [`bench/react`](react) | React 19.3, Radix primitives (what shadcn/ui uses), cmdk, Vite 8 |
| Svelte | [`bench/svelte`](svelte) | Svelte 5.57, Bits UI (what shadcn-svelte uses), svelte-sonner, Vite 8 |

The app uses tabs, a sortable, filterable, selectable, paged data table, a modal form with validation, a select, a radio group, a toast, a Ctrl+K command menu, a chart, a progress bar, a slider, an autosizing textarea and a dark mode switch. A `?bench` route repeats the js-framework-benchmark operations on a plain keyed table. [README-competitors.md](README-competitors.md) covers how the React and Svelte versions were built and where they deviate.

## Results

These are from `results/latest.json`: release builds served as static files, headless Chromium 141, a 4-core shared cloud VM, and the median of 7 runs after one warm-up. Absolute numbers are noisy on this machine, so compare the columns with each other.

### Download and startup

| | Rust | React | Svelte |
|---|---|---|---|
| Transferred, gzip | 513 KB | 120 KB | 83 KB |
| Transferred, brotli | 378 KB | 103 KB | 72 KB |
| Requests | 15 | 3 | 3 |
| Ready to use (local, fast CPU) | 144 ms | 126 ms | 130 ms |
| Ready to use (CPU slowed 4x) | 490 ms | 460 ms | 500 ms |
| Download on slow 4G (1.6 Mbps, brotli, computed) | ~1.9 s | ~0.5 s | ~0.4 s |

The wasm file is most of the Rust download. Once the bytes have arrived, startup is a three-way tie. The size cost shows up only on the network.

### DOM operations (`?bench`, ms, lower is better)

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

Rust is fastest on 6 of the 9 operations. It is within 10% of React on creating and replacing 1,000 rows, and Svelte is fastest at updating every 10th row.

Two changes got it here (see "Tuning" below).

### The real app

| | Rust | React | Svelte |
|---|---|---|---|
| Filter 1,000 tasks | 16 ms | 17 ms | 19 ms |
| Sort 1,000 tasks | 17 ms | 19 ms | 27 ms |
| axe violations (4 states) | 0 | 0 | 0 |
| Keyboard checks passed | 11 / 11 | 11 / 11 | 11 / 11 |
| Lines written (app + CSS) | **532** | 1,400 | 1,348 |
| Release rebuild after a one-line change | 45 s | ~1 s | ~3.5 s |

### Memory

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

- About 2.6x less code to write: 532 lines against roughly 1,400. The kit ships styled, accessible components, while the React and Svelte versions style bare primitives by hand.
- Fastest on 6 of 9 DOM operations, including 10,000 rows (889 ms against 1,124 and 1,189), selection and swapping.
- Accessibility and keyboard behaviour come out of the box.
- The compiler type-checks the whole app.

Worse:

- The download is 3.7x React's and 5.3x Svelte's.
- Release builds are slow: 45 s for a one-line change. Day-to-day work uses `dx serve`, which hot-reloads.
- Every component's CSS ships as its own file, so it makes 15 requests where the others make 3.
- Svelte is still faster at updating many rows in place, and React at creating 1,000 rows.

## Ideas for the gaps that remain

These are untried. Each one needs the user's go-ahead, and the last one breaks a rule in GUARDRAILS.

1. **Split the wasm by screen.** Dioxus 0.7 has an experimental `dx build --wasm-split` that loads code marked `#[wasm_split]` on first use. The Stats and Settings tabs, the dialogs and the command menu could load later, so the first download carries only the Tasks screen.
2. **One CSS file for the components an app uses.** A `rust-ui css` command (or build script) could concatenate the styles of the components the app imports. That turns 15 requests into 2.
3. **A keyed list primitive.** Something like `use_keyed_list`, where each row's data lives in a signal owned by that row and keyed by id rather than position, so updates touch only the changed rows and moves touch none. This is the Stores idea, keyed properly.
4. **A size budget in the gate.** Fail `scripts/check.sh` when the trial's brotli wasm grows past a set limit, and use `twiggy` to show what grew. The biggest items today are Dioxus's own runtime, keyboard-name tables and JSON code used by `document::eval`.
5. **Compile templates to direct DOM calls** (how Svelte and Solid get their speed). This would bypass Dioxus's virtual DOM, which counts as building a renderer, so GUARDRAILS rules it out unless that decision changes.

## Running it

```sh
# Rust: release build (needs wasm-opt on PATH)
dx build --release --platform web -p trial
python3 -m http.server 8801 -d target/dx/trial/release/web/public &
# React and Svelte
(cd bench/react && npm ci && npm run build) && python3 -m http.server 8802 -d bench/react/dist &
(cd bench/svelte && npm ci && npm run build) && python3 -m http.server 8803 -d bench/svelte/dist &
# Measure (Playwright and axe come from tests/e2e)
cd tests/e2e && RUNS=7 node ../../bench/run.mjs > ../../bench/results/latest.json
```

`TARGETS="name=url,..."` picks the builds and `ONLY=load,ops,app,memory` runs only some parts.
