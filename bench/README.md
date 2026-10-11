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
| Transferred, gzip | 525 KB | 120 KB | 83 KB |
| Transferred, brotli | 385 KB | 103 KB | 72 KB |
| Requests | 15 | 3 | 3 |
| Ready to use (local, fast CPU) | 129 ms | 130 ms | 115 ms |
| Ready to use (CPU slowed 4x) | 486 ms | 481 ms | 416 ms |
| Download on slow 4G (1.6 Mbps, brotli, computed) | ~1.9 s | ~0.5 s | ~0.4 s |

Wasm is 504 KB of the Rust build's 525 KB. Once the bytes are on the machine, startup ties React. The size cost shows up only on the network.

### DOM operations (`?bench`, ms, lower is better)

| | Rust | React | Svelte |
|---|---|---|---|
| Create 1,000 rows | **86** | 108 | 109 |
| Replace 1,000 rows | **79** | 104 | 120 |
| Create 10,000 rows | **703** | 1,071 | 1,210 |
| Append 1,000 to 1,000 | **101** | 128 | 149 |
| Swap two rows | 19 | 76 | **13** |
| Update every 10th row | 32 | 19 | **14** |
| Select a row | 10 | 6 | **6** |
| Remove a row | 24 | **15** | 18 |
| Clear 1,000 rows | 15 | 10 | **9** |

Rust wins every bulk create, by 1.3x to 1.7x. It loses the small updates, where each event pays a fixed cost to cross between wasm and JS.

### The real app

| | Rust | React | Svelte |
|---|---|---|---|
| Filter 1,000 tasks | 14 ms | 16 ms | 14 ms |
| Sort 1,000 tasks | 13 ms | 17 ms | 20 ms |
| axe violations (4 states) | 0 | 0 | 0 |
| Keyboard checks passed | 11 / 11 | 11 / 11 | 11 / 11 |
| Lines written (app + CSS) | **513** | 1,400 | 1,348 |
| Release rebuild after a one-line change | 45 s | ~1 s | ~3.5 s |

### Memory

| | Rust | React | Svelte |
|---|---|---|---|
| Whole browser RSS, idle | 380 MB | 373 MB | 373 MB |
| Whole browser RSS, 10,000 rows | 551 MB | 548 MB | 549 MB |
| JS heap, 10,000 rows | 3.7 MB | 14.8 MB | 21.1 MB |

The small JS heap for Rust is misleading, because wasm memory isn't counted in it. The whole-process numbers are the real comparison, and they tie.

## What this shows

Better:

- The app code is about 2.7x shorter. The kit ships styled, accessible components. The React and Svelte versions style bare primitives by hand, with 637 lines of CSS each. A React project with shadcn/ui copied in would write less CSS, but the copied components would then be its code to keep.
- Bulk rendering is fastest: 10,000 rows in 703 ms against 1,071 and 1,210.
- Accessibility and keyboard behaviour come out of the box. The Svelte version needed ARIA fixes on Bits UI's select, and React's Radix toast sometimes steals Escape from an open dialog.
- The whole app, including state, events and props, is type-checked by the Rust compiler.

Worse:

- The download is 4.4x React's and 6.3x Svelte's. That is about 1.5 s more on a slow mobile connection, on the first visit only, since wasm is cached afterwards.
- Every component's CSS is a separate file (70 in the dist folder), which shows up as 15 requests where the others make 3.
- Small updates (select, update, clear) take 1.5x to 2x longer. All of them still fit within a couple of frames.
- Release builds are slow: 45 s for a one-line change and about 2 minutes clean. Day-to-day work uses `dx serve`, which hot-reloads markup.
- The kit had one gap: a `Select` trigger can't be given an accessible name directly. The app wraps it in a labelled group instead.

The verdict: the stack suits apps people open and keep using, such as dashboards, internal tools and editors. It is the wrong choice for pages where first-visit download on mobile decides success, such as landing pages and content sites.

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
