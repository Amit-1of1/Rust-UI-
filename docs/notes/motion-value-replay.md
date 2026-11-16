# Replaying the MotionValue model against the real code

`proofs/tla/MotionValue.tla` (docs/notes/tla-models.md) is checked by TLC, but TLC checks the
model, not the Rust. `crates/motion/tests/value_replay.rs` narrows that gap: it runs every
sequence of up to six of the model's actions against the real `MotionValue` in a `VirtualDom`
and checks the model's rules that can be seen from outside after every step.

| Model action | In the test |
|---|---|
| `Set(x)`, `Stop`, `Animate(t)` | `set(1.0)`, `set(2.0)`, `stop()`, `spring_to(1.0)`, `spring_to(2.0)`, called inside the runtime |
| `Wake` | one non-blocking poll of the spawned tasks, then a render |
| `Unmount` | the component that owns the value is removed |

That is 7 actions and 1 + 7 + ... + 7^6 = 137,257 sequences, about 20 s in a debug build. It
runs in plain `cargo test`. Off the web without a Tokio runtime the loop has no frame timer and
finishes an animation in one turn, so a `Wake` is a whole animation; the interleavings that
matter (calls between `animate` spawning the loop and its first turn, a loop finding the slot
emptied by `set` or `stop`, an unmount while a loop is pending) are all reached.

Rules checked: `AnimatingIsTrue` (`is_animating()` is true exactly while an animation runs),
`RestsAtGoal` (with nothing pending, the value is where the last call asked), `Settles` (every
animation finishes), `NoFrozenAnimation` (an animation started afterwards, whatever state the
loop flag was left in, still finishes) and `NoWriteAfterUnmount` (nothing panics when tasks run
after the component is gone). Set `REPLAY_DEBUG=1` to print each sequence as it runs.

## What it found

The first run crashed on an `animate` followed by an unmount before the loop ran. `animate()` started its loop with
`Runtime::current().spawn(scope, ...)`. In dioxus-core 0.7 that call queues the task but does
not record it on the scope (only the scope's own `spawn`, which the global `spawn()` uses,
adds it to the scope's task list), so removing the component does not remove the task. On the
task's first poll the scope is gone: a `debug_assert` in the scheduler fails in debug builds,
and an `unwrap` in `Runtime::get_state` panics in release. In an app: a button that starts an
exit spring and hides its panel in the same click. The TLA+ model could not see this, since it
is about the runtime's bookkeeping, not the loop's logic.

Fix: the loop is spawned with `spawn()` inside `Runtime::in_scope(scope, ...)`, so the task
belongs to the component and is dropped with it. With the old line back, the replay fails at
once. The same `Runtime::spawn` behaviour is still in dioxus-core; a draft upstream issue is in
the PR description (not filed).

## When to change it

When `MotionValue`'s loop logic changes, change the TLA+ model and this test together and run
both `scripts/tla.sh` and `cargo test -p atoll-motion --test value_replay`.
