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

## `proofs/tla/ChatFollow.tla`

Models `Follow` in `crates/ui/src/components/chat/component.rs`, the chat log's "stay at the
newest message unless the reader scrolled up". `Follow::run` is spawned after every render, on
every resize (so on every streamed token) and on mount, so several runs are in flight at once,
each four stretches between awaits (read the height, read the offset, scroll, read the offset
again). Around them: messages arrive, the reader scrolls, and on the web scroll events arrive a
frame late (Blitz sends none). Positions are whole steps, each larger than the code's 24 px slack.

| Rule | Meaning |
|---|---|
| `NoYank` | a run that looks after the reader scrolled above where the log last left them never pulls them back down |
| `Follows` | at rest, a pinned log the reader didn't leave shows the end |
| `TellsOfNew` | at rest, a reader away from the end who missed new content sees "New messages below" |
| `UnpinsOnlyForReader` | the log only lets go because the reader scrolled, never by itself |
| `RunsEnd` | every run finishes |

**TLC found six bugs in the code as it was** (three are `_bug_` mutants that must fail):

- `OldStaleTop`: `top` (where the log last left the reader) was copied out *before* awaiting the
  offset. A run started before another run's first follow still held "nothing yet", skipped the
  check, and pulled down a reader who had scrolled up in between. Fix: read `top` after the await.
- `OldTrustAfter`: after scrolling, whatever offset came back was recorded as where the log left
  the reader. If they scrolled up between the scroll landing and that read (two round trips, so
  often while a reply streams), their own position was recorded, every later run saw no
  movement, and each new token pulled them back down: "I can't scroll up while it's typing". Fix:
  the scroll lands at least at the end of the measured content (and at least where any run last
  left them), so an offset above that is the reader, and the log lets go.
- `OldScrollToSize`: the scroll asked for the height read at the start; a run that read it before
  newer content arrived could finish after a newer run and move the view back up. Fix: scroll far
  past the end, which every engine clamps to the end.
- When the log let go in that last step, content measured meanwhile by another run wasn't
  flagged, so "New messages below" didn't show (`TellsOfNew`). Fix: compare with the latest
  measured height.
- A run started before "Jump to latest" could finish after it, see the reader's old position
  and let go again, undoing the click (`UnpinsOnlyForReader`). Fix: `jump` starts a new epoch,
  and a run from an older epoch stops at its next await.
- A scroll event still queued from before the click was read as the reader moving up and let go
  right after "Jump to latest" (`UnpinsOnlyForReader`). Fix: `jump` forgets where scroll events
  last saw the reader.

Checked over every interleaving, on the web (scroll events on), in two configs that CI runs:
`ChatFollow.cfg`, up to 3 messages and 2 reader scrolls with up to 4 runs in flight (1.26 million
states, about 8 minutes on 4 cores), and `ChatFollowJump.tla`/`.cfg`, the same model on up to 2
messages with one "Jump to latest" click (61,575 states, 20 s). The last two bugs come back as the
mutants `ChatFollowJump_bug_no_epoch.cfg` and `ChatFollowJump_bug_keep_seen_top.cfg`. Not covered: a scroll asked for before
the reader moved still lands (the check and the landing are a round trip apart; no code can
close that), and content that shrinks while pinned.

## Adding a model

Add `proofs/tla/<Name>.tla` and `<Name>.cfg` (must pass). Add at least one
`<Name>_bug_<what>.cfg` whose first lines say `expects <Rule> to fail`. Next candidates: focus after a
failed form submit, and the sync bot's merge protocol (docs/PROTOCOL.md).
