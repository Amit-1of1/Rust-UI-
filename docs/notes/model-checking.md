# Exhaustive model checking of keyboard state machines

`crates/ui/tests/model_check.rs` walks **every** state the kit's pure keyboard reducers can reach from their start state, presses **every** key in each one, and checks invariants on every transition. Proptest (`keyboard_state.rs`, `time_picker.rs`) samples random sequences; this enumerates the whole reachable graph breadth-first, so a pass is a proof for that configuration and a failure comes with the *shortest* key sequence that breaks it.

| Machine | Configurations | Reachable states | Transitions checked |
|---|---|---|---|
| `TimeState` (hours, minutes, AM/PM, steps 1/5/7/10/15, ranges incl. past midnight) | 9 | 63,197 | 1,629,590 |
| `TimeState` with seconds (24 h and 12 h) | 2 | 1,073,844 | 28,685,250 |
| `OtpState` (length 1 to 6, numeric and alphanumeric) | 12 | 36,963 | 3,094,761 |
| `command_nav` (0 to 64 rows) | 65 | 2,081 | 14,567 |
| `ChartCursor` (0 to 24 categories x 0 to 6 series) | 175 | 6,331 | 37,986 |
| **Total** | **263** | **1,182,416** | **33,462,154** |

Debug build, ~65 s on a 4-core cloud VM (the seconds configurations are 60 s of it). `cargo test -p atoll-ui --test model_check -- --nocapture` prints these counts.

## What is checked

- Per transition: no panic; focus index in range; every value inside its segment's range; `time()` is `Some` exactly when every segment is filled; `Ignored` changed nothing and `Moved` changed no value; an arrow key always leaves a value; with a range, an arrow key never takes an allowed time outside it; without one, Up then Down returns to the same value on the step grid. OTP: no rejected character stored, `is_complete` agrees with `code()`, a typed character lands in its box. Palette and chart: the highlight or cursor stays on a row or mark, and chart moves stop at the ends.
- Completeness: every time of day (all 1,440, or all 86,400 with seconds) is reachable from empty; every code over the alphabet, gaps included, is reachable; every palette row and chart mark is reachable. With a range, every allowed time on the minute grid is reachable with arrow keys and focus moves alone, no typing.
- No trap: from every reachable state a fixed sequence clears the picker (focus each segment, Delete), and End then holding Backspace clears an OTP within length + 1 presses.
- Cost: worst-case keystrokes to enter any time are 4 (24 h), 5 (12 h), 6 (24 h with seconds) and 7 (12 h with seconds); to reach any of 64 palette rows, 32.

## Found and fixed

Both were one arrow key from an allowed time to one outside the picker's `min`/`max`, and passed the existing proptests, which never stated that property. The checker printed each with its 4-key repro.

1. Range 9:00 to 17:30, step 5: type `9`, Down, focus the hour, Down: 9:55 became **17:55**. The hour step checked only the hour. Now the minutes (and seconds) below are pulled to the nearest allowed time: 17:30.
2. 12-hour, range 9:00 to 17:30: at 2:06 PM, any arrow key on AM/PM gave **2:06 AM**. When no step lands on an allowed value the key stepped anyway; now an allowed current value stays.

Random walks of 60 keys find each bug within a median of 2 to 14 walks once the invariant is written down, so the gain is the property list plus a guarantee that nothing else is left, not raw bug-finding speed.

## Adding a machine

Make its state `Clone + Eq + Hash`, list its keys, and call `explore(name, init, &keys, step)` where `step` applies one key and returns `Err(reason)` on a broken invariant. Keep the state finite: abstract open inputs (OTP uses the alphabet `1`, `2` plus one rejected character, which is enough because the reducer never looks at which accepted character it holds). Keep configurations under ~1 M states in debug builds.
