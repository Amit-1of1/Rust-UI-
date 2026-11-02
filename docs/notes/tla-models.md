# TLA+ models of async code

Async bugs depend on the **order** things happen in: a frame firing between two clicks, a stop
landing while the loop waits, a component closing while its timer runs. A test runs one order.
A TLA+ model lists the steps that can interleave, and TLC (the TLA+ model checker) tries
**every** order and reports the shortest one that breaks a rule.

Run them with `scripts/tla.sh` (needs Java 11+; it downloads TLC v1.7.4, MIT, once into
`target/tla/` and checks its SHA-256). CI runs it in the `tla` job of `ci.yml`.

## `proofs/tla/MotionValue.tla`

Models `MotionValue` in `crates/motion/src/value.rs`. Dioxus runs tasks on one thread, so code
only interleaves at `.await`; each TLA+ action is one stretch between awaits:

| Action | Code |
|---|---|
| `Set(x)`, `Stop`, `Animate(t, frames)` | the methods, called from an event handler |
| `ToggleReduced` | the OS reduced-motion setting changes |
| `Wake` | one turn of `run()`'s loop, with or without a frame timer |
| `Unmount` | the owning component goes away |

Numbers are abstracted: a value is one of the targets, or `"between"` mid-flight; an animation
is its target plus the frames it has left. Whether springs and glides settle numerically is
`physics.rs`'s job and is unit-tested there.

Checked over every order (3 targets, animations up to 3 frames, up to 7 calls; 2,136 states,
about 2 s):

| Rule | Meaning |
|---|---|
| `OneLoop` | never two loops moving one value |
| `NoFrozenAnimation` | a running animation always has a loop to drive it |
| `AnimatingIsTrue` | `is_animating()` matches whether an animation is running |
| `LoopingIsTrue` | the `looping` flag is set exactly while a loop is alive |
| `RestsAtGoal` | at rest, the value is where the last call asked |
| `NoWriteAfterUnmount` | nothing writes to a component that is gone |
| `Settles`, `LoopsEnd` | every animation finishes and every loop task ends |

Result: the current code keeps every rule. No bug was found.

**Mutants.** A check that can't fail proves nothing, so each `*_bug_*.cfg` switches on a
deliberate mistake and the script requires TLC to catch it:

- `MotionValue_bug_two_loops.cfg`: `animate()` spawns a loop even when one runs. TLC: two
  `animate` calls give two loops (`OneLoop`).
- `MotionValue_bug_frozen.cfg`: the loop forgets to clear `looping` when it finds the slot
  empty. TLC: animate, `set`, frame, animate again, and the animation never moves
  (`NoFrozenAnimation`). A random test would need that exact four-step order.

**Limit.** TLC checks the model, not the Rust code. `value.rs` points to the model; when that
logic changes, change the model in the same PR and rerun `scripts/tla.sh`.

## Adding a model

Add `proofs/tla/<Name>.tla` and `<Name>.cfg` (must pass). Add at least one
`<Name>_bug_<what>.cfg` whose first lines say `expects <Rule> to fail`. Next candidates: chat
auto-follow (`chat/component.rs` starts `follow.run()` from several places), focus after a
failed form submit, and the sync bot's merge protocol (docs/PROTOCOL.md).
