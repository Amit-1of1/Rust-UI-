### Renamed to Atoll (PR #PR)

**Who:** every app that used the kit under its old name.

**Before:**

```toml
ui = { git = "https://github.com/Amit-1of1/Rust-UI-", rev = "...", package = "rust-ui-kit" }
```

```sh
rust-ui add dialog
```

**After:**

```toml
ui = { git = "https://github.com/Amit-1of1/Rust-UI-", rev = "...", package = "atoll-ui" }
```

```sh
cargo install --git https://github.com/Amit-1of1/Rust-UI- atoll-cli --locked
atoll add dialog
```

**What to do:**
- `Cargo.toml`: `package = "rust-ui-kit"` becomes `package = "atoll-ui"`; `rust-ui-motion` becomes `atoll-motion`; a copied component's `dioxus-primitives`/`rust-ui-primitives` line becomes `atoll-primitives` (see the next section). Your `use ui::...` lines stay.
- The CLI: `cargo uninstall rust-ui-cli`, then install `atoll-cli`. Every command keeps its name and flags: `rust-ui add` is `atoll add`.
- Copied components: rename `rust-ui-lock.json` to `atoll-lock.json` and `.rust-ui/` to `.atoll/` (`git mv` both) so `atoll update` finds them. Environment: `RUST_UI_SOURCE` is `ATOLL_SOURCE`.
- Agents: `atoll skill` installs the skill into `.claude/skills/atoll/`; delete `.claude/skills/rust-ui/`. In `.mcp.json`, rename the `rust-ui` server to `atoll` and its command to `atoll mcp`.
- Nothing in your CSS changes: the `rui-` ids and `data-rui-scope` kept their names.
