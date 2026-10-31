### Added

- **ui:** Exhaustive model checking of the keyboard state machines (`crates/ui/tests/model_check.rs`, in `cargo test`): every reachable state of `TimeState`, `OtpState`, `command_nav` and `ChartCursor` (1.18 M states, 33 M transitions over 263 configurations) with every key, checking per-transition invariants, that every value is reachable by keyboard, that no state traps the user, and the worst-case keystroke count. A failure prints the shortest key sequence that causes it. Notes: docs/notes/model-checking.md. (#73)
- **ui:** `TimeState`, `HourCycle`, `Period`, `TimeSegment`, `OtpState` and `ChartCursor` implement `Hash`. (#73)

### Fixed

- **ui:** `TimePicker` with `min`/`max`: an arrow key on the hour no longer leaves the minutes outside the range (Down on the hour of 9:55 in 9:00 to 17:30 gave 17:55, now 17:30), and AM/PM no longer flips to a half of the day with no allowed time. (#73)
