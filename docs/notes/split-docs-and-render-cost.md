# Splitting the docs site, why booking stays whole, and the gallery's render cost

Measured 2026-11-01 on a 4-core cloud VM, headless Chromium, `main` at `97df0eb` (before) and
this branch (after), same session. Sizes are brotli; slow 4G is 1.6 Mbps and 150 ms RTT, as in
`tests/e2e/budgets.mjs`. The research behind these changes is in the project files,
`research/atoll-performance-research.md`.

## Docs site: split, home page in the main file

`DocPage` (apps/docs/src/main.rs) holds every page but the home page, as one lazy part
(`wasm-split` feature, turned on by `scripts/release-web.sh`). The home page fetches it once it
is up. The landing page names `ui::demos::dialog::Demo` directly: calling the generated
`demo(name)` from the home page would pull all 80 demos into the main file, since the match
references every one of them.

| | before | after |
|---|---|---|
| First screen | 1,089.2 KB | 325.8 KB (-70%) |
| Main wasm | 1,054.4 KB | 290.4 KB |
| Total download | 1,133.3 KB | 1,275.7 KB (+13%: code shared by both parts is copied into each) |
| Home page ready, slow 4G (median of 3) | 6,361 ms | 2,373 ms |
| A component page opened by a link, slow 4G | 6,081 ms | 7,019 ms (two files one after the other) |
| Click from the home page after 3 s, slow 4G | 34 ms | 1,644 ms (the prefetch was still arriving) |

So the home page, which most visitors see first, is 2.7x faster on a slow connection; a deep link
is about 0.9 s slower and the first click soon after landing can wait up to a second or two. A
page waits for its heading (`#page-title`) before focus moves to it, so screen readers still
land on the new page. `scripts/e2e.sh` waits for `.docs-main #page-title` before scanning a
docs route with axe (`.docs-main` alone matches while the page's code is still arriving).

Reproduce: `scripts/release-web.sh docs`, then from `tests/e2e`: `node budgets.mjs`; the slow-4G
navigation timings came from a one-off script described in the research report.

## Booking: tried, not kept

With the router, `dx --wasm-split` already makes every route a lazy part, so Explore (the first
screen) became a second download after the main file. Result: first screen 571.0 -> 533.6 KB
(-7%), 4 -> 6 requests, total 613.8 -> 873.8 KB (+42%), home page on slow 4G 3,788 -> 3,829 ms
(no gain). Not worth it. Also found: dx 0.7.10's loader throws "Cannot read properties of
undefined (reading 'listening')" and the page never becomes ready when a second lazy part
finishes downloading before the route's own part; only start a prefetch after the first lazy
part has mounted.

## Gallery: content-visibility (tried, not kept)

The gallery's startup budget (ready at 4x CPU) counts the whole page, because the lazy part
with 70 sections arrives before the stylesheets it adds finish loading. A Chromium trace of
that load: about 2.1 s running wasm (building the DOM), 0.4-0.5 s layout and 0.2-0.3 s style
recalculation. `content-visibility: auto` on `.gallery-section` cut ready at 4x CPU from
3,213 / 3,020 / 3,192 ms to 2,205 / 2,199 / 2,325 ms (layout ~400 -> ~80 ms) in Chromium.

Not kept, because containment breaks overlays that are not portaled. `content-visibility: auto`
always applies layout and paint containment, so each section becomes the containing block of
the menus, popovers, dialogs and drawers rendered inside it: anchored overlays stopped flipping
at the screen edge (7 anchor checks in Chromium), and in WebKit CI the drawer never became
visible and the table and multi-select checks failed. Rendering a section normally while it is
hovered, focused or holds `[data-state="open"]` fixed Chromium but not Firefox or WebKit.
axe also reported a false contrast failure on never-rendered sections, and `innerText` of a
skipped section is empty. Revisit once overlays render at the document root (a portal), which
is what makes containment safe.
