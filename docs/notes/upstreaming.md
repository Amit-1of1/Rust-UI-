# Getting the fork fixes upstream

Every fork in [FORKS.md](../FORKS.md) is a cost: each upstream release means a rebase, and apps that want the native fixes have to copy a `[patch]` section. A fix that lands upstream disappears from that list. Nothing is filed without the owner's word (GUARDRAILS.md). The owner has drafts with titles and bodies. This page holds the plan and the order. Applies = cherry-picks onto upstream's default branch (`scripts/fork-drift.sh --try-rebase`, 2026-10-30).

| # | Where | What | Fork commits | Applies | Shape |
|---|---|---|---|---|---|
| 1 | DioxusLabs/components | Ask them to publish `dioxus-primitives` and `dioxus-attributes` | none | n/a | issue |
| 2 | DioxusLabs/dioxus | wasm-split: skip type-index relocations; keep callees of kept functions | `73a027c` | yes | PR |
| 3 | DioxusLabs/components | Date range input freezes after a complete start date | `71fed69` | yes | PR |
| 4 | DioxusLabs/components | Date pickers can be named, full segments advance, toasts aren't alertdialogs | `5b21f56` | yes | PR (or three) |
| 5 | DioxusLabs/components | Escape, outside click, focus trap, drags, F6 and drag-and-drop without `eval` | `9a1543b`, `a110015`, `68c3117` | yes | issue first (design) |
| 6 | DioxusLabs/blitz | Keyed reorder; `display: contents`; hit testing; focus and keyboard; transforms; `position: fixed`; z-index; image load/error | `bfe28fb`, `129b500`, `ceca47f`, `a55f290`, `a095949`, `d69d92a`, `14f766f`, `505d30e` | no (0.3 on `main`) | port, then one PR each; issues first for fixed, z-index and image events |
| 7 | DioxusLabs/dioxus `native-dom` | False boolean attributes, listener counting, mouse coordinates, `change`, pointer/hover/focus events, `onmounted`, `contextmenu`/`load`/`error`/`submit` | `95019012a`, `b2e6ae5e5`, `e7eae74cf`, `8f3d57528` | no (0.8 on `main`) | port, then small PRs; issue first for `onmounted` |

Numbers 1 to 4 are worth doing now: they're small, they still apply, and number 1 alone would let the kit depend on upstream's primitives from crates.io. Numbers 6 and 7 are best done during the move to Dioxus 0.8, since porting them to upstream `main` is the same work as rebasing the forks. The five accessibility issues found earlier against the primitives (tabs, tooltip, date segments, select, toast) were drafted separately and are not fork patches.

When a fix lands: drop its commit from the next fork branch, delete its row in FORKS.md, and keep the test (`apps/native/tests/engine.rs`) as a regression check against upstream.
