### Added

- **cli:** `rust-ui explain [-- <cargo check args>]` runs `cargo check` and explains the errors that come from kit components in plain words: a missing required prop (with its docs and an example) instead of "this method takes 1 argument but 0 arguments were supplied", the real props for a misspelt one, the variants of an enum and the changelog line for a removed one, the exact line in an `rsx!` block where a string was passed for an enum, and `rust-ui add <name>` for a component not copied yet. Reads your own copies first; `--json` for editors and agents. Experimental.
