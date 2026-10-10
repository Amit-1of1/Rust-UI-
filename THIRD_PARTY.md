# Third-party code

| What | From | License | Where it lives here |
|---|---|---|---|
| Styled components (all 43 under `crates/ui/src/components`): `component.rs`, `style.css`, `docs.md`; demos adapted into `<name>_demo.rs` (external images replaced by an inline placeholder) | [DioxusLabs/components](https://github.com/DioxusLabs/components) `preview/src/components`, commit `9a758255ea26e2b20c8cecf4c1c946feb9e71da7` | MIT OR Apache-2.0 | `crates/ui/src/components/*` |
| Theme tokens (`dx-components-theme.css`) | same repo and commit, `preview/assets` | MIT OR Apache-2.0 | `crates/ui/assets/theme.css` (reduced-motion block added) |
| `dioxus-primitives` (dependency, not copied) | same repo and commit, `primitives/` | MIT OR Apache-2.0 | `Cargo.toml` git dependency |
| `dioxus-motion` 0.4.0 (dependency, not copied; behind `crates/motion`) | [wheregmis/dioxus-motion](https://github.com/wheregmis/dioxus-motion) | MIT | `crates/motion/Cargo.toml`, exact pin `=0.4.0` |

The upstream MIT license text is kept at `crates/ui/LICENSE-dioxus-components-MIT`.
Add a row here before copying anything else (docs/GUARDRAILS.md).
