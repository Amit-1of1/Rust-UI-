# Size and startup budgets

`tests/e2e/budgets.mjs` checks every release web app (gallery, site, docs, dashboard, booking, showcase, as `scripts/release-web.sh` builds them) against `bench/budgets.json`. `scripts/e2e.sh --release` runs it in the Chromium job, so CI fails a PR that makes an app:

- **first screen** more than 10% bigger: brotli (quality 11) of the files that finished loading before the app's ready selector matched and every stylesheet loaded, in a Chromium load at slow 4G (1.6 Mbps, 150 ms RTT). A fast local load would count lazy parts that happen to arrive before the check;
- **main wasm** more than 10% bigger: the largest wasm file of the first screen;
- **total** more than 10% bigger: every `.wasm`, `.js` and `.css` the build ships, lazy parts included;
- need more than one extra request for the first screen;
- more than twice as slow to ready with the CPU slowed 4x (median of 5 loads). Startup is loose because shared CI machines are noisy; sizes are brotli of the files on disk, so they are the same on every machine.

A failure lists the files that grew the most (names without their content hashes), which points at the lazy part or the main file that took the new code. A change that ends up 10% under a budget is told to lower it, so the budgets ratchet down. Raise or lower them on purpose with `BUDGETS_UPDATE=1 node tests/e2e/budgets.mjs` (from `tests/e2e`, after `scripts/release-web.sh <app>`), and say why in the PR.

Baseline, main at e303195, 2026-10-31 (4-core cloud VM, headless Chromium 153). It agrees with bench/size.mjs on the same builds (gallery 294.5 KB, dashboard 346.6 KB first screen):

| App | First screen | Requests | Main wasm | Total | Ready, CPU 4x |
|---|---|---|---|---|---|
| gallery | 292.4 KB | 4 | 257.8 KB | 1,314.9 KB | 2,982 ms |
| site | 398.9 KB | 5 | 380.3 KB | 441.1 KB | 749 ms |
| docs | 1,002.0 KB | 6 | 967.9 KB | 1,045.9 KB | 687 ms |
| dashboard | 344.6 KB | 4 | 320.3 KB | 791.3 KB | 733 ms |
| booking | 570.7 KB | 4 | 547.8 KB | 613.4 KB | 1,047 ms |
| showcase | 416.0 KB | 4 | 395.3 KB | 458.7 KB | 508 ms |

Verified: with the gallery's main-wasm budget set to 220 KB the check fails with `mainWasmKB 257.8 KB is 17% over its budget of 220 KB` and `grew: assets/gallery_bg.wasm 206.24 -> 257.8 KB`.

The docs site was split on 2026-11-01 (first screen 1,089 -> 326 KB); booking was tried and not kept, because the router already makes its first page a lazy part of its own. Numbers and reasons: [split-docs-and-render-cost.md](split-docs-and-render-cost.md).
