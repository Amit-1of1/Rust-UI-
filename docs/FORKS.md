# Forked dependencies

Four crates carry small fixes on top of the exact versions we depend on. The fixes live on branches of forks under `Amit-1of1`, pinned to a commit in the workspace `Cargo.toml`.

- The three renderer crates (Blitz and `dioxus-native-dom`) are swapped in with `[patch.crates-io]`. Only builds that pull them in (the `native` feature, `apps/native`) are affected; the web and desktop builds never compile them.
- `dioxus-primitives` is a git dependency anyway, so its line simply points at the fork. Its fixes only switch on where `document::eval` can't run scripts (dioxus-native), so the web and desktop behave exactly as upstream.

| Crate | Release | Upstream commit of that release | Fork branch | Pinned commit |
|---|---|---|---|---|
| `blitz-dom` | 0.2.4 | [DioxusLabs/blitz `0a77560`](https://github.com/DioxusLabs/blitz/tree/0a77560e59440d3c6b5f14b673b9c722a08a3d4c/packages/blitz-dom) | [Amit-1of1/blitz `rust-ui/blitz-dom-0.2.4`](https://github.com/Amit-1of1/blitz/tree/rust-ui/blitz-dom-0.2.4) | `ceca47f` |
| `blitz-paint` | 0.2.1 | [DioxusLabs/blitz `a1af09c`](https://github.com/DioxusLabs/blitz/tree/a1af09c0ce99e488f093743c450e9d3678d7c202/packages/blitz-paint) | [Amit-1of1/blitz `rust-ui/blitz-paint-0.2.1`](https://github.com/Amit-1of1/blitz/tree/rust-ui/blitz-paint-0.2.1) | `d69d92a` |
| `dioxus-native-dom` | 0.7.10 | [DioxusLabs/dioxus `57d6794`](https://github.com/DioxusLabs/dioxus/tree/57d6794ad60b949e5bd8aa282f6f8c3dc97a365e/packages/native-dom) | [Amit-1of1/dioxus `rust-ui/dioxus-native-dom-0.7.10`](https://github.com/Amit-1of1/dioxus/tree/rust-ui/dioxus-native-dom-0.7.10) | `e7eae74` |
| `dioxus-primitives` | git only | [DioxusLabs/components `9a75825`](https://github.com/DioxusLabs/components/tree/9a758255ea26e2b20c8cecf4c1c946feb9e71da7/primitives) | [Amit-1of1/dioxus-components `rust-ui/primitives-9a75825`](https://github.com/Amit-1of1/dioxus-components/tree/rust-ui/primitives-9a75825) | `a110015` |

The `component.json` manifests that `rust-ui-cli registry` generates name the same fork and commit, so `dx components add` installs the fixed primitives too.

Each renderer branch is three kinds of commit on top of the upstream commit (the primitives branch only needs the second):

1. **Use the published manifest.** The crate's `Cargo.toml` is replaced by the normalized one from the crates.io release. Upstream's workspace manifest uses path dependencies, which would pull a second copy of `blitz-traits` or `dioxus-core` from git and break the types shared with the crates.io copies.
2. **The fixes.** Every changed spot is marked `PATCH(rust-ui)`, so `git grep -n "PATCH(rust-ui)"` on the branch lists them. `git diff <upstream commit>..HEAD -- packages/<crate>/src` shows the whole patch.
3. Anything else needed to ship it (blitz-dom carries `LICENSE-taffy-MIT`, because `src/layout/fixed.rs` adapts Taffy 0.9's absolute-positioning step).

All four crates are `MIT OR Apache-2.0`, and the fork repositories keep upstream's license files.

## What the patches fix

| Gap (docs/NATIVE.md) | Crate | Fix |
|---|---|---|
| No pointer events | dioxus-native-dom | `pointerdown/move/up` fired alongside the mouse events, with `PointerData` (one mouse pointer) |
| No hover events | dioxus-native-dom | `mouseenter/leave`, `mouseover/out`, `pointerenter/leave/over/out` from Blitz's hover changes |
| Mouse coordinates | dioxus-native-dom | real client, page and element coordinates (upstream panicked on page and element) |
| No `onmounted` | dioxus-native-dom | fired after each render; `get_client_rect`, `set_focus`, `scroll_to`, `scroll`, `get_scroll_offset`, `get_scroll_size` answered by the document between frames |
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
| Keyed lists lose items when reordered | blitz-dom | a node moved within its own parent is detached before it is inserted (upstream removed the inserted copy too) |
| No `contextmenu` | dioxus-native-dom | a right-button press fires `contextmenu` after `mousedown` |
| No image `load` | blitz-dom + dioxus-native-dom | Blitz records `<img>` nodes whose image arrived; `load` fires for them on the next poll |
| Checkboxes and radios fire `input` only | dioxus-native-dom | they also fire `change`, as in browsers |
| Escape, outside click and focus traps need `eval` | dioxus-primitives | where `eval` can't run: dialog, alert dialog and popover take focus when they open and handle Escape themselves; focus leaving them (a click outside moves or clears it) closes them, and Tab out of a modal one brings focus back; the context menu closes on focus leaving it |
| Slider and color area drags need `eval` | dioxus-primitives | a window-sized `PointerShield` takes the pointer moves and release while dragging |
| Drag-and-drop list needs HTML drag and drop | dioxus-primitives | a press that moves more than 4px starts a pointer drag; items under the pointer pick the slot; release drops, leaving the list cancels |

Each row has a test in `apps/native/tests/engine.rs` (renderer level) or `apps/native/tests/interactions.rs` (kit demos).

## Known limits of the patches

- A z-indexed box escapes the `overflow` clip of ancestors between it and its stacking context (browsers clip it unless the clip is outside its containing block).
- Positioned boxes with `z-index: auto` still paint in tree order inside their parent.
- `position: fixed` is computed in the layout pass, so with Blitz's `incremental` feature (not enabled here) a scroll without a relayout would move fixed boxes.
- `getBoundingClientRect`-style rects (`get_client_rect`) ignore transforms.
- Pointer events have one pointer (`pointerId` 1, type `mouse`); no touch or pen, no pointer capture.
- Space activates on keydown, not keyup.
- Image `error` never fires: Blitz reports failed fetches without the node, so an avatar whose image fails keeps showing its fallback in the loading state.
- Without `eval`, Escape only reaches an overlay while focus is inside it (it is moved there on open), and Shift+Tab out of a modal brings focus to the modal itself, not its last element.
- Toast's F6 shortcut is still a document-level key listener and does nothing on native.
- The no-script fallbacks are chosen at runtime (`document::eval("").send(())` fails). Server-side rendering also has no `eval`, so its markup carries the fallback `tabindex="-1"` on open overlays; hydrating that on the web is untested.

## Changing a fix

1. Clone the fork, check out the branch, make the change (keep the `PATCH(rust-ui)` marker), commit and push the branch.
2. Put the new commit in the `rev` of the matching line in `Cargo.toml` (`[patch.crates-io]`, or `dioxus-primitives`) and in the table above. For primitives, also run `cargo run -p rust-ui-cli -- registry` so the manifests follow.
3. Run `cargo test -p native` (both `engine` and `interactions`) and `scripts/native-smoke.sh --build`.

## Moving to a new upstream release

1. In the fork, fetch upstream and create `rust-ui/<crate>-<version>` from the commit in the new release's `.cargo_vcs_info.json` (crates.io tarball, or `~/.cargo/registry/src/*/<crate>-<version>`).
2. Copy the release's `Cargo.toml` over `packages/<crate>/Cargo.toml` and commit it.
3. Cherry-pick the fix commits from the previous branch and resolve conflicts.
4. Point `Cargo.toml` at the new commit and run the tests above. Keep the old branch so older commits of this repo still build.

## Using the fixes in an app outside this repo

`[patch]` only applies to the workspace that declares it. An app that git-installs the kit and uses the `native` feature needs the same `[patch.crates-io]` section, copied from this repository's `Cargo.toml` (not yet tried from an outside app).

## Upstream

Nothing is sent upstream (no PRs or issues on DioxusLabs repositories) without the owner's say-so. The fixes are written so they could go upstream as they are.
