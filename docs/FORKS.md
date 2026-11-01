# Forked dependencies

Six crates carry small fixes on top of the exact versions we depend on. The fixes live on branches of forks under `Amit-1of1`, pinned to a commit in the workspace `Cargo.toml`.

- The five renderer crates (Blitz and `dioxus-native-dom`) are swapped in with `[patch.crates-io]`. Only builds that pull them in (the `native` feature, `apps/native`) are affected; the web and desktop builds never compile them.
- `dioxus-primitives` is a git dependency anyway, so its line simply points at the fork. The fork's packages are named `atoll-primitives` and `atoll-attributes` so they can be published (the library names are unchanged; [PUBLISHING.md](PUBLISHING.md#dioxus-primitives)), and the kit's line says `package = "atoll-primitives"`. Its fixes switch on only where `document::eval` can't run scripts (dioxus-native), so the web and desktop behave as upstream, with these exceptions, plain bug fixes that apply everywhere: the date range input freeze, the date pickers' naming and segment advance, and the toast roles below.

| Crate | Release | Upstream commit of that release | Fork branch | Pinned commit |
|---|---|---|---|---|
| `blitz-dom` | 0.2.4 | [DioxusLabs/blitz `0a77560`](https://github.com/DioxusLabs/blitz/tree/0a77560e59440d3c6b5f14b673b9c722a08a3d4c/packages/blitz-dom) | [Amit-1of1/blitz `rust-ui/blitz-dom-0.2.4`](https://github.com/Amit-1of1/blitz/tree/rust-ui/blitz-dom-0.2.4) (`a095949` is on `claude/showcase-multi-target-tc6fhc` until that branch is fast-forwarded) | `a095949` |
| `blitz-traits` | 0.2.0 | [DioxusLabs/blitz `0a77560`](https://github.com/DioxusLabs/blitz/tree/0a77560e59440d3c6b5f14b673b9c722a08a3d4c/packages/blitz-traits) | [Amit-1of1/blitz `rust-ui/blitz-traits-0.2.0`](https://github.com/Amit-1of1/blitz/tree/rust-ui/blitz-traits-0.2.0) | `14f766f` |
| `blitz-net` | 0.2.1 | [DioxusLabs/blitz `0a77560`](https://github.com/DioxusLabs/blitz/tree/0a77560e59440d3c6b5f14b673b9c722a08a3d4c/packages/blitz-net) | [Amit-1of1/blitz `rust-ui/blitz-net-0.2.1`](https://github.com/Amit-1of1/blitz/tree/rust-ui/blitz-net-0.2.1) | `505d30e` |
| `blitz-paint` | 0.2.1 | [DioxusLabs/blitz `a1af09c`](https://github.com/DioxusLabs/blitz/tree/a1af09c0ce99e488f093743c450e9d3678d7c202/packages/blitz-paint) | [Amit-1of1/blitz `rust-ui/blitz-paint-0.2.1`](https://github.com/Amit-1of1/blitz/tree/rust-ui/blitz-paint-0.2.1) | `d69d92a` |
| `dioxus-native-dom` | 0.7.10 | [DioxusLabs/dioxus `57d6794`](https://github.com/DioxusLabs/dioxus/tree/57d6794ad60b949e5bd8aa282f6f8c3dc97a365e/packages/native-dom) | [Amit-1of1/dioxus `rust-ui/dioxus-native-dom-0.7.10`](https://github.com/Amit-1of1/dioxus/tree/rust-ui/dioxus-native-dom-0.7.10) | `8f3d575` |
| `dioxus-cli` (`dx`: its wasm splitter and hot-patching) | 0.7.10 | [DioxusLabs/dioxus `57d6794`](https://github.com/DioxusLabs/dioxus/tree/57d6794ad60b949e5bd8aa282f6f8c3dc97a365e/packages/cli) | [Amit-1of1/dioxus `claude/project-thread-hgbr1g`](https://github.com/Amit-1of1/dioxus/tree/claude/project-thread-hgbr1g) (on top of `claude/project-thread-xwtlc7`, the splitter fix) | `5ab2450` |
| `dioxus-primitives` (published by us as `atoll-primitives`, with `dioxus-attributes` as `atoll-attributes`) | git only | [DioxusLabs/components `9a75825`](https://github.com/DioxusLabs/components/tree/9a758255ea26e2b20c8cecf4c1c946feb9e71da7/primitives) | [Amit-1of1/dioxus-components `rust-ui/primitives-9a75825`](https://github.com/Amit-1of1/dioxus-components/tree/rust-ui/primitives-9a75825) (`21f75f9` is on `claude/project-thread-62cokn` until that branch is fast-forwarded) | `21f75f9` |

`dx` is a tool, not a dependency: `scripts/install-dx.sh` installs it from the pinned commit (`dx --version` prints `0.7.10 (5ab2450)`). Split release builds (`scripts/release-web.sh`, the Pages workflow) and `dx serve --hot-patch` need it; plain `dx serve` and debug builds work with stock 0.7.10. Its branch is the whole upstream workspace with commits on top (three cherry-picked from upstream `main`, the rest ours, each marked `PATCH(rust-ui)`), so it needs no manifest swap. Why hot-patching needs it, and what it gains: [notes/hot-patch.md](notes/hot-patch.md).

The `component.json` manifests that `atoll-cli registry` generates name the same fork and commit, so `dx components add` installs the fixed primitives too.

Each renderer branch is three kinds of commit on top of the upstream commit (the primitives branch only needs the second):

1. **Use the published manifest.** The crate's `Cargo.toml` is replaced by the normalized one from the crates.io release. Upstream's workspace manifest uses path dependencies, which would pull a second copy of `blitz-traits` or `dioxus-core` from git and break the types shared with the crates.io copies.
2. **The fixes.** Every changed spot is marked `PATCH(rust-ui)`, so `git grep -n "PATCH(rust-ui)"` on the branch lists them. `git diff <upstream commit>..HEAD -- packages/<crate>/src` shows the whole patch.
3. Anything else needed to ship it (blitz-dom carries `LICENSE-taffy-MIT`, because `src/layout/fixed.rs` adapts Taffy 0.9's absolute-positioning step).

All six crates are `MIT OR Apache-2.0`, and the fork repositories keep upstream's license files.

## What the patches fix

| Gap (docs/NATIVE.md) | Crate | Fix |
|---|---|---|
| No pointer events | dioxus-native-dom | `pointerdown/move/up` fired alongside the mouse events, with `PointerData` (one mouse pointer) |
| No hover events | dioxus-native-dom | `mouseenter/leave`, `mouseover/out`, `pointerenter/leave/over/out` from Blitz's hover changes |
| Mouse coordinates | dioxus-native-dom | real client, page and element coordinates (upstream panicked on page and element) |
| No `onmounted` | dioxus-native-dom | fired after each render; `get_client_rect`, `set_focus`, `scroll_to`, `scroll`, `get_scroll_offset`, `get_scroll_size` answered by the document between frames. `scroll` goes to a position (clamped, this element only), like `Element.scroll()` on the web |
| No focus events | dioxus-native-dom | `focus`, `blur`, `focusin`, `focusout` whenever Blitz's focus moves |
| `disabled: false` disabled the element, `hidden: false` hid it | dioxus-native-dom | false boolean attributes are removed, the same list as dioxus' web interpreter |
| Listener counting | dioxus-native-dom | counted per event name, so events Blitz does not produce itself are not skipped |
| Click does not focus; focus lost after any click | blitz-dom | mousedown focuses the nearest focusable ancestor (`tabindex="-1"` included) or blurs; the click default action no longer clears focus |
| Keyboard | blitz-dom | Shift+Tab, Tab from nothing focused, wrap-around, skips hidden elements; Enter on buttons and links and Space on buttons click them |
| `position: fixed` relative to the parent | blitz-dom | fixed boxes are re-laid-out against the viewport after the main pass (new `layout/fixed.rs`) |
| Clicks beside text missing inline buttons (padded inline root) | blitz-dom | hit testing no longer offsets inline children by the container's padding |
| Boxes outside their parent unclickable | blitz-dom | hit testing descends into unclipped children even when the point is outside the parent |
| No `pointer-events`, invisible boxes clickable | blitz-dom | `pointer-events: none` and `visibility: hidden` skip the element (descendants can opt back in) |
| Clicks ignore transforms | blitz-dom | hit testing applies the inverse CSS transform |
| `z-index` only orders siblings | blitz-dom + blitz-paint | positioned elements with a z-index are painted and hit tested with their nearest stacking context (root, z-indexed or fixed box, `opacity < 1`, `transform`) |
| Percentage transforms dropped; vertical `transform-origin` used the width | blitz-paint (via blitz-dom `css_transform_2d`) | resolved against the border box |
| Transforms not inherited; text not transformed | blitz-paint | ancestor transforms are combined and applied to children, text, inline SVG and SVG backgrounds |
| An element directly inside a `display: contents` wrapper (`ThemeScope`, `ToastProvider`) loses its box; an app wrapped in them came out as wide as its longest line (found by `apps/showcase`) | blitz-dom | a block whose children are contents wrappers no longer counts as all-inline, and a contents child's own children are collected as the container's layout children (upstream hoisted its grandchildren). `engine.rs` `children_of_display_contents_keep_their_box` |
| Keyed lists lose items when reordered | blitz-dom | a node moved within its own parent is detached before it is inserted (upstream removed the inserted copy too) |
| No `contextmenu` | dioxus-native-dom | a right-button press fires `contextmenu` after `mousedown` |
| No image `load` | blitz-dom + dioxus-native-dom | Blitz records `<img>` nodes whose image arrived; `load` fires for them on the next poll |
| Checkboxes and radios fire `input` only | dioxus-native-dom | they also fire `change`, as in browsers |
| No image `error` | blitz-traits + blitz-net + blitz-dom + dioxus-native-dom | `NetHandler::failed` (a no-op by default) hears fetches that fail; the image handler reports the node for those and for bytes that don't decode (`Resource::ImageFailed`); `error` fires on the next poll |
| No `submit` event | dioxus-native-dom | a click on a submit button and Enter in a single-line field fire `submit` on the form; preventing it also stops Blitz's own submission (navigation) |
| Toast's F6 shortcut needs `eval` | dioxus-primitives | the provider wraps its children in a `display: contents` box that hears F6 where `eval` can't run (rendered everywhere, so server and client markup match) |
| Escape, outside click and focus traps need `eval` | dioxus-primitives | where `eval` can't run: dialog, alert dialog and popover take focus when they open and handle Escape themselves; focus leaving them (a click outside moves or clears it) closes them, and Tab out of a modal one brings focus back; the context menu closes on focus leaving it |
| Slider and color area drags need `eval` | dioxus-primitives | a window-sized `PointerShield` takes the pointer moves and release while dragging |
| Lazy components stay in the main wasm (the gallery's lazy half, the dashboard's Settings page came out as ~450-byte files) | dioxus-cli (wasm-split-cli) | the call graph read type-index relocations (`call_indirect`) as symbol indices; split exports have low symbol indices, so main seemed to call them. Type-index relocations are skipped |
| Date pickers can't be named: the group's English `aria-label` ("Date", "Date Range") sits beside the one you pass, and each segment's `aria-labelledby` points at an id nothing renders (booking gap 13) | dioxus-primitives | the built-in name is used only when the attributes carry no `aria-label`/`aria-labelledby`; segments drop `aria-labelledby` and keep their `aria-label`. `booking.mjs` checks the group name and the segments |
| A date segment stays put after "01" for a month or "03" for a day (booking gap 18) | dioxus-primitives | a segment that holds `max_length` digits moves on, as one that can't take another digit already did. `booking.mjs` types "01" |
| Toasts are `role="alertdialog"` (dashboard gap 13) | dioxus-primitives | a toast is a named, focusable `group`; `ToastContent` is `status` for info and success, `alert` for errors and warnings. `booking.mjs` and `dashboard.mjs` check for the status and that no alertdialog appears |
| `dx build --wasm-split` panics with `assertion failed: !self.dead.contains(&id)` (a lazy command palette) | dioxus-cli (wasm-split-cli) | a function kept in main but in neither graph (a wasm-bindgen closure shim) could call one only a split reached, which was deleted. Callees of kept functions are kept |
| `dx serve --hot-patch` fails before the app starts: "Failed to emit module: failed to parse import section: exceptions proposal not enabled" | dioxus-cli | wasm-bindgen 0.2.129 adds an exception-handling `tag` import (`__wbindgen_jstag`) that dx 0.7.10's walrus 0.23 can't parse. Cherry-picked from upstream `main`: walrus 0.26 and no `-Clink-dead-code` on wasm (DioxusLabs/dioxus#5843), v0 symbol mangling (#5622), skipping workspace crates the fat build never compiled (#5903) |
| A hot-patch of a kit crate fails: "Cycle in workspace dependency graph" | dioxus-cli | `atoll-ui` dev-depends on itself (to turn on its own test features), so it listed itself as a dependent. Self and dev-dependency edges are ignored |
| A hot-patch of a kit crate leaves the change out (and every fat build recompiles the crate) | dioxus-cli | the rustc wrapper saved args under the lib name (`ui`), everything else looks crates up by package name (`atoll-ui`). Lib args are saved by package name, the rlib is found by `--crate-name` |
| A large patch fails in the browser: "WebAssembly.instantiate(): table index is out of bounds" | dioxus-cli | the loader grew the function table by the number of distinct function names in the patch; unnamed and same-named entries need slots too. It reserves one per element-segment entry |
| The page panics right after some patches ("index out of bounds" in `render_with`) | dioxus-cli | a save reported twice was queued during the patch and then resent every template of the new code, which the browser applied before the patch had loaded. Files unchanged since the last build are skipped |
| No nested submenus (complaints gap 1; a feature, see [the decision](decisions/2026-11-01-submenus-in-primitives-fork.md)) | dioxus-primitives | `DropdownMenuSub`, `ContextMenuSub` and `MenubarSub` with `*SubTrigger` and `*SubContent`, in a new `submenu.rs`: the trigger is an item of the parent menu, the content re-provides the menu's context so its items work unchanged; arrow keys, Escape, hover intent and closing on focus leaving use element events only. `apps/native/tests/submenus.rs`, the dropdown menu and menubar contracts, `tests/e2e/submenus.mjs` |
| Drag-and-drop list needs HTML drag and drop | dioxus-primitives | a press that moves more than 4px starts a pointer drag; items under the pointer pick the slot; release drops, leaving the list cancels |
| Typing a complete start date into a date range input froze the page (web too; found by `apps/booking`) | dioxus-primitives | the range input and its two date inputs write each other's signals only when the value changes; a write of the same value used to notify and loop forever |

Each row has a test in `apps/native/tests/engine.rs` (renderer level) or `apps/native/tests/interactions.rs` (kit demos); the date range row is covered by `tests/e2e/booking.mjs`, which types both dates. The hot-patch rows are checked by `bench/hot-patch.mjs` against `dx serve --hot-patch` (by hand; CI has no long-running dev server). The splitter rows are checked by `scripts/release-web.sh`, which fails when a lazy part comes out nearly empty, and by the browser checks on release builds (`scripts/check.sh --release` locally, `.github/workflows/e2e.yml` in CI on each engine), which test the split dashboard and gallery.

## Known limits of the patches

- A z-indexed box escapes the `overflow` clip of ancestors between it and its stacking context (browsers clip it unless the clip is outside its containing block).
- Positioned boxes with `z-index: auto` still paint in tree order inside their parent.
- `position: fixed` is computed in the layout pass, so with Blitz's `incremental` feature (not enabled here) a scroll without a relayout would move fixed boxes.
- `getBoundingClientRect`-style rects (`get_client_rect`) ignore transforms.
- Pointer events have one pointer (`pointerId` 1, type `mouse`); no touch or pen, no pointer capture.
- Space activates on keydown, not keyup.
- An image whose `src` can't be resolved to a URL is never fetched, so it fires neither `load` nor `error`.
- `submit` is fired for Enter in any single-line field of a form (browsers also require a submit button or a single field), and the form's `FormData` is empty: read values from your own state, as the kit's `Form` does.
- Scrolling (wheel and `scroll`) only reaches content Blitz counts in the scroll size: a row of `inline-block` children isn't counted, a flex row is.
- Without `eval`, Escape only reaches an overlay while focus is inside it (it is moved there on open), and Shift+Tab out of a modal brings focus to the modal itself, not its last element.
- Toast's F6 shortcut and the sidebar's Ctrl+B are heard only while focus is inside the app (the provider's box), not with nothing focused.
- The no-script fallbacks are chosen at runtime (`document::eval("").send(())` fails). Server-side rendering also has no `eval`, so its markup carries the fallback `tabindex="-1"` on open overlays; hydrating that on the web is untested.

## Changing a fix

1. Clone the fork, check out the branch, make the change (keep the `PATCH(rust-ui)` marker), commit and push the branch.
2. Put the new commit in the `rev` of the matching line in `Cargo.toml` (`[patch.crates-io]`, or `dioxus-primitives`) and in the table above. To try a change before pushing, point the crates at your checkouts for one command with `--config 'patch.crates-io.blitz-dom.path="../blitz/packages/blitz-dom"'` (and the same for the others). For primitives, also run `cargo run -p atoll-cli -- registry` so the manifests follow.
3. Run `cargo test -p native` (both `engine` and `interactions`) and `scripts/native-smoke.sh --build`.

## Moving to a new upstream release

1. In the fork, fetch upstream and create `atoll/<crate>-<version>` from the commit in the new release's `.cargo_vcs_info.json` (crates.io tarball, or `~/.cargo/registry/src/*/<crate>-<version>`).
2. Copy the release's `Cargo.toml` over `packages/<crate>/Cargo.toml` and commit it.
3. Cherry-pick the fix commits from the previous branch and resolve conflicts.
4. Point `Cargo.toml` at the new commit and run the tests above. Keep the old branch so older commits of this repo still build.

## Using the fixes in an app outside this repo

`[patch]` only applies to the workspace that declares it, so the renderer forks never reach an app that depends on the kit. On the web and desktop that is what you want: those builds never compile the patched crates, and `scripts/consumer-check.sh` (CI, every PR) builds an outside app on stock Dioxus from crates.io to prove it. The primitives fork is the only fork such an app gets, as the ordinary dependency `atoll-primitives`.

An app that uses the `native` feature and wants the renderer fixes copies the `[patch.crates-io]` section from this repository's `Cargo.toml`. Without it, native runs with upstream's gaps (the left column of the table above).

## Keeping up with upstream

`scripts/fork-drift.sh` lists, for each forked crate, the release we patch, the newest compatible release and the newest overall. `--try-rebase` also replays each branch's fix commits onto both of those in throwaway clones. A fix that replays as empty is already upstream and can be dropped. `.github/workflows/fork-drift.yml` runs it every Monday and keeps the result in one issue, "Fork drift". The table at the top of the script must match this page, and the script fails if a pin in `Cargo.toml` or `scripts/install-dx.sh` has no row.

On 2026-10-30 nothing newer is compatible. Blitz `main` is 0.3 and Dioxus `main` is 0.8, and the renderer fixes conflict with both, so the move to Dioxus 0.8 includes porting them. The splitter fix and all the primitives fixes still apply cleanly to upstream `main`.

## Upstream

Nothing is sent upstream (no PRs or issues on DioxusLabs repositories) without the owner's say-so. The fixes are written so they could go upstream as they are. [notes/upstreaming.md](notes/upstreaming.md) groups them into the PRs and issues we would open, in order, with whether each still applies.
