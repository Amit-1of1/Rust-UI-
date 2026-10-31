# `rust-ui explain`: compiler errors in plain words

`rsx!` turns every component into a props builder, so a one-word mistake in markup comes back
from rustc as an error about a builder type you never wrote. `rust-ui explain` runs
`cargo check`, reads the kit's component source (your copies in `src/components` first, then
the registry), and says what to change.

```sh
rust-ui explain                       # cargo check in the current directory
rust-ui explain -- -p my-app --features web
cargo check --message-format=json > out.json && rust-ui explain --from out.json
rust-ui explain --json                # the same, for editors and agents
```

It exits 1 when there are errors, like `cargo check`. Errors that are not about kit components
are listed at the end by their first line; `cargo check` shows them in full.

## What it explains

| You wrote | rustc says | `rust-ui explain` says |
| --- | --- | --- |
| `Dialog { open: open(), "Hello" }` | this method takes 1 argument but 0 arguments were supplied (argument #1 of type `DialogPropsBuilder_Error_Missing_required_field_title`) | `Dialog` needs the `title` prop, which has no default. `title` (String): The dialog's heading, and its accessible name. Add it: `Dialog { title: "...", ... }` |
| `Button { colour: "red" }` | no method named `colour` found for struct `ButtonPropsBuilder<...>`; help: there is a method `color` | `Button` has no prop `colour`. `Button` props: `variant`, `size`, `onclick`, ... It also takes HTML attributes; rustc's own suggestion may be one of those. |
| `size: ButtonSize::Icon` | no variant, associated function, or constant named `Icon` found for enum `ButtonSize` | `ButtonSize::Icon` does not exist. `ButtonSize` has `Xs`, `Sm`, `Default` and `Lg`. Changelog: `ButtonSize::Icon` ... removed. Icon-only buttons are the new `IconButton` ... |
| `Button { variant: "primary" }` | mismatched types: expected `ButtonVariant`, found `&str`, pointing at the whole `rsx!` block | at the `variant:` line: `variant` takes a `ButtonVariant`, not a `&str`. Write the variant: `ButtonVariant::Primary`. |
| `use crate::components::dialog;` before copying it | unresolved import | `dialog` is a kit component that is not in this project yet. Copy it: `rust-ui add dialog` |

Prop docs come from the `///` comments in the component's source, so the explanation is always
the one for the version you have. `crates/cli/tests/explain.rs` runs it on real `cargo check`
output (`crates/cli/tests/fixtures/explain/`).
