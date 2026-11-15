### Added

- **motion:** `crates/motion/tests/value_replay.rs` runs every sequence of up to six actions of the TLA+ model of `MotionValue` (137,257 sequences) against the real `MotionValue` in a `VirtualDom` and checks the model's rules after every step, so the model and the code can't drift apart unnoticed. Notes: docs/notes/motion-value-replay.md.

### Fixed

- **motion:** An app no longer panics when a component closes in the same event as an animation starts on one of its values (for example a button that starts an exit spring and hides its panel). The animation loop is now spawned on the component's scope, so it is dropped with the component; before, dioxus-core kept the task and panicked on its first poll.
