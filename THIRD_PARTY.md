# Third-party code

| What | From | License | Where it lives here |
|---|---|---|---|
| Styled components (all 43 under `crates/ui/src/components`): `component.rs`, `style.css`, `docs.md`; demos adapted into `<name>_demo.rs` (external images replaced by an inline placeholder) | [DioxusLabs/components](https://github.com/DioxusLabs/components) `preview/src/components`, commit `9a758255ea26e2b20c8cecf4c1c946feb9e71da7` | MIT OR Apache-2.0 | `crates/ui/src/components/*` |
| Theme tokens (`dx-components-theme.css`) | same repo and commit, `preview/assets` | MIT OR Apache-2.0 | `crates/ui/assets/theme.css` (reduced-motion block added) |
| `dioxus-primitives` (dependency, not copied) | same repo and commit, `primitives/` | MIT OR Apache-2.0 | `Cargo.toml` git dependency |
| `dioxus-motion` 0.4.0 (dependency, not copied; behind `crates/motion`) | [wheregmis/dioxus-motion](https://github.com/wheregmis/dioxus-motion) | MIT | `crates/motion/Cargo.toml`, exact pin `=0.4.0` |
| Week 4 components `alert`, `breadcrumb`, `chart`, `command`, `field`, `kbd`, `spinner`, `table` | Original code, written here (shadcn/ui used only as a visual reference, nothing copied) | n/a | `crates/ui/src/components/*` |
| Structure, props and behaviour of `callout`, `empty`, `status`, `shimmer`, `marquee`, `button_group`, `stepper`, `bottom_nav`, `input_group` (styling and accessibility rewritten; no Tailwind classes kept) | [rust-ui/ui](https://github.com/rust-ui/ui) `app_crates/registry/src/ui/<name>.rs` (+ `hooks/use_stepper.rs`), commit `43ddc65`, Copyright (c) 2026 Max Wells | MIT | `crates/ui/src/components/{callout,empty,status,shimmer,marquee,button_group,stepper,bottom_nav,input_group}` |
| Ideas and props of `chips`, `link` (`TextLink`), and the hooks `use_media_query`, `use_copy_clipboard` (rewritten: focusable checkbox, no `force_reload` button, listeners removed on unmount, copy reported only after it succeeds; no code or Tailwind classes kept) | [rust-ui/ui](https://github.com/rust-ui/ui) `app_crates/registry/src/ui/{chips,link}.rs`, `registry/src/hooks/{use_media_query,use_copy_clipboard}.rs`, commit `43ddc65`, Copyright (c) 2026 Max Wells | MIT | `crates/ui/src/components/{chips,link,hooks}` |
| `dioxus-icons` 0.1.0 (dependency, not copied; behind `crates/ui/src/components/icons`) | [dioxuslabs/dioxus-icons](https://github.com/dioxuslabs/dioxus-icons), Lucide 1.14.0 icon data | MIT AND ISC (Lucide) | `Cargo.toml` workspace dependency |

The upstream MIT license text is kept at `crates/ui/LICENSE-dioxus-components-MIT`; the Rust/UI license is at `crates/ui/LICENSE-rust-ui-MIT`.
Add a row here before copying anything else (docs/GUARDRAILS.md).
