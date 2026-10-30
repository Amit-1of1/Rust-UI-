### Copied components ask for `rust-ui-primitives` (PR #56)

**Who:** apps that copied components with `dx components add` or `rust-ui add` before this change. Apps that depend on `rust-ui-kit` need to do nothing.

**Before** (your `Cargo.toml`):

```toml
dioxus-primitives = { git = "https://github.com/Amit-1of1/dioxus-components", rev = "5b21f56..." }
```

**After:**

```toml
rust-ui-primitives = { git = "https://github.com/Amit-1of1/dioxus-components", rev = "fd533b7a5edc2334ddb5c30bdfa669de3eb5ab1b" }
```

**What to do:** replace the line. Copying a newer component adds the new one, and both crates provide `dioxus_primitives`, so Cargo refuses to build with both. Keep the `features` you had. Your code doesn't change: the library is still `dioxus_primitives`. Once `rust-ui-primitives` is on crates.io, `rust-ui-primitives = "0.1"` replaces the git line.
