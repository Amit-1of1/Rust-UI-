# 2026-10-31: the kit is called Atoll

**Decision.** The product, its crates and its CLI are renamed from "Rust UI" to **Atoll**: `atoll-ui` (library `ui`), `atoll-motion`, `atoll-form-derive`, `atoll-cli` (binary `atoll`), and the primitives fork as `atoll-primitives` / `atoll-attributes`. Planned domain: atollui.dev, with atollui.com as the backup.

**Why a rename.** "Rust UI" is the name of rust-ui.com (Rust/UI), a component registry that now has a Dioxus edition, so a search for our name finds a competitor first; `rust-ui` on crates.io is someone else's crate. A name is cheapest to change before the first publish.

**Why Atoll, not Corten.** The 2026-10-30 shortlist (project files, `research/rust-ui-names-2026-10.md`) recommended Corten. On 2026-10-31 crates.io showed `corten-cli`, `corten-feed`, `corten-macros` and `corten-protocol`, published on 2026-10-30 by todd-skelton/corten: a Rust web-app framework, the same field as ours. Atoll was the shortlist's fallback: an atoll is a ring of reef around a lagoon, as the kit is a ring of components around your app.

**What was checked on 2026-10-31.**
- crates.io: `atoll-ui`, `atoll-motion`, `atoll-cli`, `atoll-kit`, `atoll-primitives`, `atoll-attributes`, `atoll-form-derive` are all free. Bare `atoll` is a 0.0.0 placeholder from 2023 ("Automatic transformation of logical layout"), so the CLI crate is `atoll-cli`.
- npm: `atoll-ui` and `@atoll/ui` are free (`atoll` is a small 2022 statistics library).
- GitHub: the best-known neighbour is Ebullioscopic/Atoll, a macOS "Dynamic Island" app (about 5k stars, Swift). A consumer app, not a developer tool, so search results with "Rust", "Dioxus" or "UI kit" stay ours.
- Domains: the 2026-10-30 registrar lookup found atollui.dev, atollui.com and atollkit.com unregistered. It could not be repeated from this environment (no RDAP access), so check again before buying.
- Not checked: trademarks.

**What did not change.** CSS ids (`rui-<component>-N`) and the `data-rui-scope` attribute keep their names: users never see them and renaming them would break apps' CSS. Fork branch names (`rust-ui/blitz-dom-0.2.4` ...) stay, because the pinned commits live on them. The repository is still `Amit-1of1/Rust-UI-`; renaming it to `atoll` is the owner's call, and GitHub redirects the old URLs.
