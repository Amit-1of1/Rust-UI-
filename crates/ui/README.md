# rust-ui-kit

A shadcn-inspired component kit for [Dioxus](https://dioxuslabs.com) 0.7: styled components on top of [`dioxus-primitives`](https://github.com/DioxusLabs/components), light and dark themes, and spring motion from `rust-ui-motion`.

```rust
use ui::prelude::*;

rsx! {
    ThemeStyles {}
    Button { "Hello" }
}
```

Not on crates.io yet: `dioxus-primitives` is git-only, and crates.io refuses git dependencies. Install it from git or copy the components into your app; see [docs/INSTALL.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/INSTALL.md).

Licensed under MIT or Apache-2.0. Parts are adapted from DioxusLabs/components and rust-ui/ui (both MIT); their notices are in this folder and listed in [THIRD_PARTY.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/THIRD_PARTY.md).
