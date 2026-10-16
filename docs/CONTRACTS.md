# Component contracts

A component's `contract.toml` (next to its `component.rs`) says which WAI-ARIA pattern it follows, which roles it exposes, its states and its keyboard map, and gives every key a short test. Everything that shows the keyboard map is generated from it, and the keyboard tests run it, so the docs can't claim a key the tests don't press:

| Reads it | What it does with it |
|---|---|
| `rust-ui registry` | the "Keyboard and accessibility" section of the docs site page (`apps/docs/src/generated.rs`), of `llms/<name>.md` and `llms-full.txt`, and of the MCP `get_component` answer |
| `tests/e2e/contracts.mjs` | runs every row in the browser on the component's docs page (Chromium, Firefox and WebKit in CI), through `scripts/e2e.sh` |
| `apps/native/tests/contracts.rs` | runs every row on headless Blitz in `cargo test -p native`: a row marked `native = true` must pass there and a row marked `native = false` must still fail, so the docs' Native column is right both ways |
| `crates/ui/build.rs` and `rust-ui doctor` | refuse an unsound contract (below); doctor also warns about keyboard-operable demos that have no contract yet |
| `rust-ui contract [name] [--json]` | prints the table, or the parsed steps the runners use |

## Format

```toml
pattern = "Dialog (Modal)"   # the pattern's name as the APG writes it
apg = "dialog-modal"         # its slug: https://www.w3.org/WAI/ARIA/apg/patterns/<slug>/ (optional)
roles = ["dialog"]           # roles the component exposes
native_note = "..."          # optional: what is different or missing on native

[parts]                      # CSS selectors the tests use, inside the demo
trigger = "button"
dialog = "[role=dialog]"

[[states]]                   # optional
name = "open"
means = "The dialog is shown over the page."
check = "visible dialog"     # one assertion step that holds in this state

[[keys]]                     # one row of the docs table per entry
key = "Escape"
when = "Inside the dialog"
does = "Closes the dialog and returns focus to the button that opened it."
native = false
test = """
focus trigger
press Enter
press Escape
state closed
focused trigger
"""
```

The tests drive the component's demo (`crates/ui/src/demos/<name>.rs`), starting fresh for each row. On the web the selectors are scoped to the docs page's preview, on native to the rendered demo.

## Steps

One per line; `#` starts a comment. A part is a name from `[parts]`, optionally with a 1-based index among its matches: `option@2`.

| Step | Does or checks |
|---|---|
| `focus <part>` | moves keyboard focus to the part |
| `click <part>` | clicks its middle |
| `press <key>` | presses a key: `Enter`, `Space`, `Escape`, `Tab`, arrows, `Home`, `End`, `PageUp`, `PageDown`, `Backspace`, `Delete` or one character, with optional `Shift+`, `Control+`, `Alt+`, `Meta+`; `press Tab*6` repeats |
| `type <text>` | types text into the focused element |
| `visible <part>` | the part exists and is shown |
| `hidden <part>` | the part is absent or not shown |
| `focused <part>` | the part has focus |
| `focus-in <part>` | focus is on or inside the part |
| `text <part> <text>` | the part's text contains `<text>` |
| `attr <part> <name>=<value>` | an attribute's value (`!=` for "anything else", including missing) |
| `state <name>` | that state's `check` |

In the browser an assertion waits up to 1.5 s (overlays animate); on native the harness settles after every input, so it checks at once.

## Rules (build.rs and doctor)

- Every row's `key` is pressed in its own test, and something is checked after the last press of it. A table row is therefore never untested.
- Every role in `roles` is asserted on through a part whose selector contains `[role=<role>]`.
- Every state is checked by at least one test (`state <name>`).
- Steps parse, parts exist, keys are known, and no key appears twice for the same `when`.
- The component has a demo, and its `docs.md` has no hand-written keyboard section (`## Keyboard` or a `- Keyboard:` line) beside the generated one.

## Adding one

1. Write `contract.toml` from the APG pattern's "Keyboard Interaction" section, one row per key the component supports.
2. `cargo test -p native --test contracts` (with `CONTRACT=<name>` it prints each row's outcome) and set `native` to what Blitz does.
3. `scripts/e2e.sh` (or `node tests/e2e/contracts.mjs <name>` against a running docs site) must pass every row.
4. `cargo run -p rust-ui-cli -- registry` regenerates the docs and llms pages.

A row the component doesn't meet on the web is a bug to fix, not a row to leave out quietly; if it can't be fixed now, leave the key out of the contract and note the gap in the component's `docs.md`.
