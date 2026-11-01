# Hot-patching Rust code (`dx serve --hot-patch`)

`dx serve --hot-patch` (Dioxus' Subsecond) recompiles only the crate you edited and swaps the new
functions into the running page, so a Rust change shows without a rebuild or a reload and the
page keeps its state. It needs `dx` from `scripts/install-dx.sh`; stock dx 0.7.10 fails on every
app that uses wasm-bindgen 0.2.129, the kit included.

```sh
scripts/install-dx.sh                      # dx --version: 0.7.10 (5ab2450)
dx serve --platform web --hot-patch        # in the app's folder
```

## Measured (2026-10-31)

4-core cloud VM, Rust 1.97, wasm-bindgen 0.2.129, headless Chromium, time from saving the file to
the change on screen ([`bench/hot-patch.mjs`](../../bench/hot-patch.mjs); medians of 2 or 3 runs
after the first patch).

| App, Rust edit (not hot-reloadable `rsx!`) | `dx serve` (rebuild, reload) | `dx serve --hot-patch` |
|---|---|---|
| `atoll create` starter (59 lines, uses the kit from a path) | 15.6 s | **0.87 s** |
| Gallery: expression in the app crate | 14.8 s (42 s for the first edit) | **4.2 s** |
| Gallery: new helper function in the app crate | 14.8 s | **4.3 s** |
| Gallery: a kit component in `crates/ui` | full rebuild | 57 s |
| Stock dx 0.7.10, any of the above | as above | fails before the app starts |

Where the time goes on the gallery: about 0.5 s until dx sees the save, about 3.4 s in rustc for
the app crate (type checking 1.3 s, code generation 1.3 s, coherence 0.6 s), about 0.2 s to link
and rewrite the patch, about 0.7 s for the browser to fetch, compile and run it. The starter's
crate is small, so rustc takes 0.65 s. The time grows with the size of the crate you edit, not
with the size of the app: keep the code you iterate on in a small crate, or in the app crate.

A kit-crate edit recompiles all of `atoll-ui` (every component and demo), so it is no faster than
a rebuild; the gain there is only that the page keeps its state. Apps that copied components in
with `atoll add` own that code in their own crate and get the app-crate numbers.

## What was broken, and the fixes (all in the dx fork, docs/FORKS.md)

1. **The app never started.** wasm-bindgen 0.2.129 imports an exception-handling tag
   (`__wbindgen_jstag`); dx 0.7.10 reads the wasm with walrus 0.23, which rejects it: "Failed to
   emit module: failed to parse import section: exceptions proposal not enabled". Upstream fixed
   this on `main` (walrus 0.26, DioxusLabs/dioxus#5843) but not in a 0.7 release; the fork
   cherry-picks it with #5622 (v0 symbol mangling) and #5903.
2. **The page panicked after some patches** ("index out of bounds" in `render_with`). Editors
   report a save twice; the second event was queued during the patch and then resent every
   template of the new code, which the browser applied before the patch had loaded. Fixed in dx:
   a file unchanged since the last build is skipped.
3. **Kit-crate patches failed** with "Cycle in workspace dependency graph": `atoll-ui`
   dev-depends on itself. Fixed in dx: self and dev-dependency edges are ignored.
4. **Kit-crate patches were empty** (they "succeeded" without the change), because dx stored
   rustc args under the lib name `ui` and looked them up by the package name `atoll-ui`. Fixed in
   dx; this also stops every fat build from recompiling `atoll-ui`.
5. **A large patch trapped** in the browser ("table index is out of bounds"): dx sized the
   function table by distinct function names instead of table entries. Fixed in dx.

## Limits

- `rsx!` hot reload (text, attributes) works in the app crate, but a literal attribute added to a
  kit crate's component did not show in our test: dx picked up the edit and sent it, and the page
  did not change. Make kit-crate markup edits Rust edits (they are patched) or rebuild. Not
  investigated further.
- A reload starts from the code of the last full build; dx then sends the patches made since
  (seen once, not timed).
- Not checked in CI (it needs a long-running dev server); rerun `bench/hot-patch.mjs` after a dx
  or wasm-bindgen bump.
- Desktop and native were not measured; they use a different patch loader (no wasm table).
