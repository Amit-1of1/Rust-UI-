# Kani proofs

[Kani](https://github.com/model-checking/kani) (Apache-2.0/MIT, AWS) compiles the real Rust code
and checks a harness for **every** value of its `kani::any()` inputs with a SAT solver (CBMC),
including every overflow, division by zero and panic on the way. Where `tests/model_check.rs`
walks every state of a keyboard reducer up to a size (chart up to 24 × 6), Kani proves the same
rules for every `usize`.

- Harnesses: `crates/ui/src/kani_proofs.rs` (`#[cfg(kani)]`, outside every component folder so a
  copied component never carries it). `crates/ui/Cargo.toml` declares the `kani` cfg so normal
  builds don't warn.
- Run: `scripts/kani.sh` (all, one at a time) or `scripts/kani.sh <harness>`. It installs Kani
  0.68.0 if missing. CI: `.github/workflows/kani.yml`.
- In a cloud container GitHub release *pages* may be blocked while downloads work: install with
  `cargo install --locked kani-verifier --version 0.68.0`, download
  `https://github.com/model-checking/kani/releases/download/kani-0.68.0/kani-0.68.0-x86_64-unknown-linux-gnu.tar.gz`
  and run `cargo kani setup --use-local-bundle <file>`.

| Harness | Proves | Found |
|---|---|---|
| `chart_cursor_stays_on_a_mark` | any cursor, any key, any chart size: the cursor lands on a real mark, moves one step at most, Home/End reach the ends | nothing (the reducer was already right) |
| `command_palette_wraps_for_every_length` | arrow keys wrap for every list length a `Vec` can have; Home/End; other keys do nothing | nothing |
| `pagination_survives_any_siblings` | any page and any `siblings` (a prop) | `page + siblings` overflowed for a huge `siblings` (fixed: saturating). The list's shape is proved exhaustively instead (`tests/components/pagination.rs`): Kani ran out of memory walking it. |

Writing one: keep inputs bounded where the code allocates or loops (`kani::assume(n <= 12)` plus
`#[kani::unwind(n)]`): an unbounded `Vec` makes CBMC run out of memory. A harness should assert
what the code promises (its doc comment), not just that it doesn't panic.
