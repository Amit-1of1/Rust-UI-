# Forked dependencies

The native (Blitz) renderer uses three crates with small fixes on top of the exact releases we depend on. The fixes live on branches of forks under `Amit-1of1`; the workspace `Cargo.toml` swaps them in with `[patch.crates-io]`, pinned to a commit. Only builds that pull these crates in (the `native` feature, `apps/native`) are affected; the web and desktop builds never compile them.

| Crate | Release | Upstream commit of that release | Fork branch | Pinned commit |
|---|---|---|---|---|
| `blitz-dom` | 0.2.4 | [DioxusLabs/blitz `0a77560`](https://github.com/DioxusLabs/blitz/tree/0a77560e59440d3c6b5f14b673b9c722a08a3d4c/packages/blitz-dom) | [Amit-1of1/blitz `rust-ui/blitz-dom-0.2.4`](https://github.com/Amit-1of1/blitz/tree/rust-ui/blitz-dom-0.2.4) | `b55b108` |
| `blitz-paint` | 0.2.1 | [DioxusLabs/blitz `a1af09c`](https://github.com/DioxusLabs/blitz/tree/a1af09c0ce99e488f093743c450e9d3678d7c202/packages/blitz-paint) | [Amit-1of1/blitz `rust-ui/blitz-paint-0.2.1`](https://github.com/Amit-1of1/blitz/tree/rust-ui/blitz-paint-0.2.1) | `d69d92a` |
| `dioxus-native-dom` | 0.7.10 | [DioxusLabs/dioxus `57d6794`](https://github.com/DioxusLabs/dioxus/tree/57d6794ad60b949e5bd8aa282f6f8c3dc97a365e/packages/native-dom) | [Amit-1of1/dioxus `rust-ui/dioxus-native-dom-0.7.10`](https://github.com/Amit-1of1/dioxus/tree/rust-ui/dioxus-native-dom-0.7.10) | `9501901` |

[Amit-1of1/dioxus-components](https://github.com/Amit-1of1/dioxus-components) (dioxus-primitives) is forked too but carries no changes yet; see "What to do next" in `docs/NATIVE.md`.

Each branch is three kinds of commit on top of the upstream commit:

1. **Use the published manifest.** The crate's `Cargo.toml` is replaced by the normalized one from the crates.io release. Upstream's workspace manifest uses path dependencies, which would pull a second copy of `blitz-traits` or `dioxus-core` from git and break the types shared with the crates.io copies.
2. **The fixes.** Every changed spot is marked `PATCH(rust-ui)`, so `git grep -n "PATCH(rust-ui)"` on the branch lists them. `git diff <upstream commit>..HEAD -- packages/<crate>/src` shows the whole patch.
3. Anything else needed to ship it (blitz-dom carries `LICENSE-taffy-MIT`, because `src/layout/fixed.rs` adapts Taffy 0.9's absolute-positioning step).

All three crates are `MIT OR Apache-2.0`, and the fork repositories keep upstream's license files.

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

Each row has a test in `apps/native/tests/engine.rs` (renderer level) or `apps/native/tests/interactions.rs` (kit demos).

## Known limits of the patches

- A z-indexed box escapes the `overflow` clip of ancestors between it and its stacking context (browsers clip it unless the clip is outside its containing block).
- Positioned boxes with `z-index: auto` still paint in tree order inside their parent.
- `position: fixed` is computed in the layout pass, so with Blitz's `incremental` feature (not enabled here) a scroll without a relayout would move fixed boxes.
- `getBoundingClientRect`-style rects (`get_client_rect`) ignore transforms.
- Pointer events have one pointer (`pointerId` 1, type `mouse`); no touch or pen, no pointer capture.
- Space activates on keydown, not keyup.

## Changing a fix

1. Clone the fork, check out the branch, make the change (keep the `PATCH(rust-ui)` marker), commit and push the branch.
2. Put the new commit in the `rev` of the matching `[patch.crates-io]` line in `Cargo.toml` and in the table above.
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
