### Breaking

- **all:** The kit is now **Atoll**. Crates: `rust-ui-kit` is `atoll-ui`, `rust-ui-motion` is `atoll-motion`, `rust-ui-form-derive` is `atoll-form-derive`, `rust-ui-cli` is `atoll-cli` (binary `atoll`), and the primitives fork is `atoll-primitives` / `atoll-attributes`. Library names (`ui`, `motion`, `dioxus_primitives`) are unchanged, so Rust code compiles as before. The lockfile is `atoll-lock.json` with pristine copies in `.atoll/base/`, the environment variables are `ATOLL_SOURCE` and `ATOLL_REGISTRY_GIT`, the agent skill installs into `.claude/skills/atoll/`, and the MCP server is named `atoll`. CSS ids (`rui-*`) and `data-rui-scope` keep their names. (#68)

### Added

- **cli:** `atoll create <dir>` makes a new Dioxus 0.7 app that already uses the kit (Cargo.toml, Dioxus.toml, a starter screen), from crates.io by default or `--git [url] [--tag|--rev]` / `--path`. Experimental. (#68)
- **docs:** Getting started guide (docs/GETTING_STARTED.md, first page of the docs site), checked from a fresh app by `scripts/getting-started-check.sh` and in CI. Release plan for the first tag in docs/RELEASE.md. (#68)
