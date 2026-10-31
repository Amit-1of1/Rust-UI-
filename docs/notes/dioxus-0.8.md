# Dioxus 0.8 (alpha) branch

Dioxus 0.8 is in alpha: `0.8.0-alpha.3` on crates.io since 2026-10-16, with no date for a stable
release. `main` stays on Dioxus 0.7.10 (GUARDRAILS: "Pin Dioxus to a minor version ... upgrade
on purpose, on its own branch, with the gallery as the test"). This branch is that upgrade: the
whole kit on `=0.8.0-alpha.3`, kept next to `main` so 0.7 users see no change, and merged only
when 0.8 is stable.

## Using it

Pin the branch's commit; the kit's crates are not published for 0.8:

```toml
[dependencies]
dioxus = "=0.8.0-alpha.3"
ui = { package = "atoll-ui", git = "https://github.com/Amit-1of1/Rust-UI-", rev = "<commit>" }
```

and the matching CLI: `cargo install dioxus-cli --version 0.8.0-alpha.3 --locked`.

## What changed for 0.8

| Area | Change |
|---|---|
| Dependencies | `dioxus`, `dioxus-sdk-time`, `dioxus-ssr` at `=0.8.0-alpha.3` (an exact pin: alphas break between releases) |
| Primitives | `Amit-1of1/dioxus-components` branch `claude/project-thread-hgbr1g`: the 0.7 fork (`cda2513`, all the fixes in docs/FORKS.md) ported to 0.8, the same API changes as upstream's `0.8` branch |
| Motion engine | dioxus-motion has no 0.8 release. Its 0.4.0 is ported (presence code for 0.8's flat templates) and lives in the same fork commit, a git dependency of `crates/motion` (so apps need no patch) until it has a fork of its own. Publishing needs a 0.8 release of it, so the `consumer` and `semver` CI jobs fail on this branch |
| Keys | keyboard-types 0.8: `Key::Escape` is `Key::Named(NamedKey::Escape)` (98 places in the kit, 125 in the primitives) |
| Event attributes | No free `dioxus_elements::events::onfocus(...)` functions; build lists with `Vec::<Attribute>::new().onfocus(...)` (the focus-return hooks) |
| Nodes | `VNode::key()` and `template()` are methods; no `DynamicNode::Placeholder` (an empty fragment instead) |
| Toasts | No `PropsWithOwner`: `ToastProvider { render_toast }` takes `Callback<toast::ToastProps, Element>` (was `ToastPropsWithOwner`). The only kit API change |
| `attributes!` | dioxus-rsx 0.8 renders a lone attribute with its Rust name (`aria_label="Send"`, not `aria-label`) and as `Box<[Attribute; 1]>`. The primitives' macro now sets built-in attributes through the typed builder methods, as `rsx!` does for elements (this also fixed the dropdown menu's Escape focus return, which looks for `[aria-haspopup]`) |
| Snapshots | Dioxus 0.8 writes attributes in another order; `crates/ui/tests/snapshots` were re-recorded (checked: the same attributes and values on every tag) and two `render.rs` asserts check attributes one at a time |
| Native | `apps/native` and the Blitz 0.2 forks stay on 0.7 (the fixes don't apply to Blitz 0.3; `scripts/fork-drift.sh` says so); this branch builds the web and desktop targets |

## Checked

2026-10-31, on `0.8.0-alpha.3`: `cargo fmt`, `cargo clippy --workspace --all-targets -D warnings`
and `cargo test --workspace` (489 tests) clean; `cargo build -p gallery --target
wasm32-unknown-unknown` builds. The gallery under `dx serve` 0.8.0-alpha.3 in headless Chromium
passes 17 browser suites: a11y_tree, anchor, axe (light, 0 violations), blocks, chat, density,
drawer, forced_colors, form_fields, hotkey, keyboard, motion, otp, pickers, resizable, rtl, table,
theme, time_picker. Not run: the apps that are not the gallery (docs, site, booking, dashboard,
showcase), Firefox and WebKit, the split release build, and `apps/native`.

dx 0.8 downloads esbuild 0.27.3 itself; behind a proxy with its own CA put the binary in
`~/.local/share/.dx/tools/esbuild-0.27.3/esbuild` first.

## Keeping it current

- After a change on `main`, merge `main` into this branch; most conflicts are `Key::` lines (rerun
  the rewrite: `Key::X` to `Key::Named(NamedKey::X)` outside `Character`).
- On a new alpha: bump the three pins here and in the fork, rebuild the fork's port.
- When 0.8 is stable: rebase the primitives port onto the fork's release branch, publish, merge
  this branch, and write the migration note (`render_toast` type, key matching in app code).

`atoll create` writes `dioxus = "=0.8.0-alpha.3"` on this branch, and `atoll init` and `doctor`
treat 0.8 as the supported line.
