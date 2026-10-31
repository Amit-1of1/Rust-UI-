### Copied components ask for `atoll-primitives` (PR #62)

**Who:** apps that copied components with `dx components add` or `atoll add` before this change. Apps that depend on `atoll-ui` need to do nothing.

**Before** (your `Cargo.toml`):

```toml
dioxus-primitives = { git = "https://github.com/Amit-1of1/dioxus-components", rev = "5b21f56..." }
```

**After:**

```toml
atoll-primitives = { git = "https://github.com/Amit-1of1/dioxus-components", rev = "cda2513bb8ea0e9d2f72cc85ed966e3b83d4e1ee" }
```

**What to do:** replace the line. Copying a newer component adds the new one, and both crates provide `dioxus_primitives`, so Cargo refuses to build with both. Keep the `features` you had. Your code doesn't change: the library is still `dioxus_primitives`. Once `atoll-primitives` is on crates.io, `atoll-primitives = "0.1"` replaces the git line.
