# Hard questions

The questions an experienced Rust or frontend engineer asks before adopting a UI kit, with straight answers, including the ones where Atoll loses. Each answer links to the file that backs it. Written 2026-10-31; the numbers are dated where they appear.

## Who builds this, and how old is it?

Atoll started on 2026-10-10. Most of the code was written by an AI coding agent (Claude), one pull request at a time, with CI on every change and one human owner who merges them. As of 2026-10-31 that is 266 of 367 commits by the agent.

We say this first because it is the right thing to worry about: polished READMEs no longer prove anything. What we offer instead is evidence you can rerun:

- 516 Rust tests (render, server rendering, registry, CLI) and 28 browser scripts run on Chromium, Firefox and WebKit ([tests/e2e](https://github.com/Amit-1of1/Rust-UI-/tree/main/tests/e2e)).
- Keyboard contracts in `contract.toml` for the main widgets with a WAI-ARIA keyboard pattern (menus, dialogs, select, tabs and others), checked key by key ([CONTRACTS.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/CONTRACTS.md)). Not every interactive component has one yet: the context menu, calendar, date and color pickers, carousel, OTP input, resizable panels, tag group, data table and sidebar don't.
- Pages that list what does not work: [SCREEN_READERS.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/SCREEN_READERS.md), [NATIVE.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/NATIVE.md), [FORKS.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/FORKS.md).

Age is the one thing tests can't fix. If you need a kit with years of track record, Atoll is not it yet.

## Am I locking my app to Dioxus?

Yes. The components are Dioxus components, and Dioxus is pre-1.0. Since September 2026 the Dioxus team works at Cognition; Dioxus, Blitz and Taffy remain open source under permissive licences.

What limits the cost:

- **You can own the source.** `atoll add` copies a component into your app; it keeps working if Atoll stops.
- **The styling is plain CSS** with documented tokens and `data-*` hooks ([THEMING.md](#theming)), not Tailwind classes or Rust-generated styles. The CSS, the tokens, the keyboard contracts and the browser tests carry over to another framework; the Rust does not.
- **Dioxus upgrades are planned for.** A Dioxus minor (0.7 to 0.8) gets a kit minor of its own with nothing else breaking in it ([VERSIONING.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/VERSIONING.md)). Dioxus 0.8 is in alpha; Atoll is on 0.7.10 today.

## Why are there forks of upstream crates?

To fix bugs we hit without waiting for upstream releases. [FORKS.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/FORKS.md) lists every patch against the upstream commit it sits on.

- **Web and desktop apps use one fork**: `dioxus-primitives`, published as `atoll-primitives` because DioxusLabs has not published it to crates.io. Its changes for the web are bug fixes (a date range freeze, date picker naming, toast roles). Split release builds also use a patched `dx` (a build tool, not a dependency); `dx serve` and debug builds work with stock `dx`.
- **The other five forks are only for the native renderer** (Blitz), which is a preview, and are only compiled with the `native` feature.
- A weekly job checks how far each fork has drifted from upstream. The fixes are drafted for upstream ([upstreaming.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/notes/upstreaming.md)) and each one that lands removes a fork.

## How big is the download compared with React?

Bigger. Measured 2026-10-13/14 with brotli, release builds ([bench/README.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/bench/README.md)):

| App | First screen | Slow 4G (1.6 Mbps), styled first screen |
|---|---|---|
| Atoll, gallery (every component), split wasm | 238 KB | 1.9 s |
| Atoll, dashboard app, split wasm | 316 KB | 2.3 s |
| Same small app in React 19 + Radix | 103 KB | ~0.5 s download |
| Same small app in Svelte 5 + Bits UI | 72 KB | ~0.4 s download |

Once the bytes arrive, startup is a tie (about 450 to 500 ms with the CPU slowed 4x for all three). So the cost is the network, once; it is cached after that.

Choose Atoll for dashboards, internal tools, desktop and agent UIs, where a 300 KB first load is fine. For a content or marketing site where every first-visit kilobyte counts, a JS framework or server-rendered HTML is the better tool.

## How slow is the edit-compile loop?

Markup and CSS are fast; Rust logic is not. Measured 2026-10-31 on a 4-core cloud machine with the gallery app, which uses every component ([bench/README.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/bench/README.md#build-times-edit-compile-see-it)):

| Change | Time to see it |
|---|---|
| Text or markup inside `rsx!`, under `dx serve` | 0.8 s (hot reload, no compile) |
| A component's CSS, under `dx serve` | 0.8 s (hot reload, no compile) |
| Rust code, incremental `cargo build` (wasm, dev) | 19 s |
| Rust code, through `dx serve` (rebuild, bind, reload) | 62 s |
| First build of a fresh checkout (dev) | 125 s |

A Vite app rebuilds a one-line change in 1 to 3.5 s, so for logic-heavy work Rust is slower by an order of magnitude. Keeping logic in small crates and doing layout and styling under `dx serve` keeps most edits on the fast path. Dioxus' `--hot-patch` mode, which patches Rust code without a rebuild, did not work with the gallery when we tried it.

## Is "accessible" a claim or a test?

A test, with a stated limit. On Chromium, Firefox and WebKit, every component in the gallery is checked with axe (light, dark, RTL and every accent preset) and in forced-colors mode (Chromium and Firefox; WebKit cannot emulate it), and its text contrast is proved for every accent preset. The main components are also compared against snapshots of the browser's accessibility tree, and the ones with a keyboard contract (WAI-ARIA APG patterns) are driven key by key against it and by a keyboard fuzzer.

The limit: automated checks find roughly half of real accessibility problems (Deque's own study puts axe at 57% by volume), and **no screen reader has been run by a person yet**. [SCREEN_READERS.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/SCREEN_READERS.md) lists what is automated and what still needs a human pass.

## Does server rendering work?

Yes, with Dioxus fullstack: server rendering then hydration, with ids that match between server and client, media queries that hydrate safely, and a DataTable server mode for large data ([SSR.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/SSR.md), `crates/ui/tests/ssr.rs`). It has been proven on one trial app so far.

## Will 0.x releases break me every month?

Each public item is marked stable, experimental or internal ([STABILITY.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/STABILITY.md)). Stable items only break in a minor release, with a changelog entry and upgrade steps in [MIGRATION.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/MIGRATION.md). If you copied components with `atoll add`, `atoll update` merges upstream fixes into your edited copy with a three-way merge; where you and upstream changed the same lines it writes conflict markers and exits with an error instead of overwriting your edit.

## Does the native renderer really work?

As a preview. All components start and draw on Blitz, and modals, menus and keyboard focus work there, but some browser features are missing and five Blitz crates carry our patches ([NATIVE.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/NATIVE.md)). For a desktop app today, use the Dioxus desktop target (a system webview), which runs the same code as the web.

## Why not the official DioxusLabs components?

Atoll started from them and stays installable with `dx components add`. It adds about 40 components (data table, charts, forms with validation, command palette, chat UI, time pickers and more), the motion layer, the tested contracts above, server-rendering fixes and the `atoll update` merge. If the official set covers what you need, it is a fine choice and you lose nothing by starting there; the two share the same primitives (Atoll's are the lightly patched fork above).

## Why not React and shadcn/ui?

If your team writes TypeScript and your app is a website, use them; they are more mature and lighter to download. Atoll is for teams that already chose Rust for the UI: Dioxus desktop and mobile apps, agent and chat interfaces, internal tools, Rust-only teams who don't want Node in the build. For them the alternative is building and testing a select, a combobox, a date picker and a data table themselves.

## Why pay for Pro if components can be copied?

You don't have to: the kit is MIT or Apache-2.0 and complete on its own. Pro adds premium blocks, motion components and app templates for a one-time price ([docs/pro](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/pro/README.md)). Pro items follow the same rules and checks as the free kit.

## Is copying code from a registry a supply-chain risk?

The registry is this repository: every item is reviewed in a pull request and tested, and there are no third-party uploads. `atoll add` copies plain source you can read before you build it, and `atoll diff` shows what changed before `atoll update` applies it.
