# The primitives fork becomes a publishable crate, `rust-ui-primitives`

**Decision:** the fork of DioxusLabs/components on `Amit-1of1/dioxus-components` names its packages `rust-ui-primitives` and `rust-ui-attributes` and keeps the library names `dioxus_primitives` and `dioxus_attributes`. It is the same code with no source changes. The kit depends on it with both `git` + `rev` (what this workspace builds) and `version` (what a packaged kit asks crates.io for). `component.json` names `rust-ui-primitives`. Publishing either crate is still the owner's call ([PUBLISHING.md](../PUBLISHING.md)). This replaces the 2026-10-13 row "Keep `dioxus-primitives` as a pinned git dependency" in GUARDRAILS.md's decision table.

**Why:** being on crates.io is the most common thing teams require before they adopt a crate. Some build and security policies forbid git dependencies outright. The kit's only git dependency is the primitives. The 2026-10-13 decision rejected publishing a fork because we would own about 22,600 lines and port upstream fixes by hand. Since 2026-10-14 we carry that fork anyway. Upstream `main` hasn't moved since `9a75825` (2026-09-07) and has published nothing. So renaming adds a release step and no code. It also closes a gap the old plan had: copied components (`dx components add`) and the crate now use the same primitives.

**How it stays honest:**
- Upstream first. A draft issue asking DioxusLabs to publish `dioxus-primitives` and `dioxus-attributes` is ready for the owner. If they publish, we switch back and keep the fork only as a `[patch]`.
- The fork follows the fork rules in GUARDRAILS.md: fixes only, `PATCH(rust-ui)`, written so they could go upstream as they are. The rename commit is the one exception: it touches manifests only.
- `scripts/fork-drift.sh` (weekly in CI) reports new upstream releases and whether our fixes still replay. `scripts/consumer-check.sh` (every PR) proves an outside app builds from the packages with no git source but this one.

**Who must be told:** apps that copied components before this change have `dioxus-primitives` (git) in their `Cargo.toml`. Copying newer components adds `rust-ui-primitives`, and two crates that both provide `dioxus_primitives` can't sit side by side. The migration note says to swap the line.

**Asked for by:** the owner: "generate fixes for each [adoption barrier] by thinking like a seasoned software engineer" (project thread, 2026-10-30). The coordinator listed "vendor or rename the primitives so the kit can be published" among the options.
