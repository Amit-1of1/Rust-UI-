# rust-ui-motion

Motion helpers for [Dioxus](https://dioxuslabs.com) 0.7, used by the Rust UI kit:

* `SpringPreset` and `Transition`: named spring feels that collapse to an instant change when the OS asks for reduced motion.
* `use_reduced_motion`, `use_spring`, `Pop` with `AnimatePresence`, `FlipList`/`FlipItem` and `SwipeDismiss`.
* `physics`: a closed-form spring solver, momentum glides with bounds and snap points, pointer velocity, rubber banding and zoom maths, in plain Rust with unit tests.
* `use_motion_value`: a value driven by that physics. Follow a pointer with `set`, then `spring_to` or `glide` at the pointer's speed.
* `Draggable` (throw with momentum, bounds, snap points, arrow keys), `PinchZoom` (two fingers, Ctrl + wheel, double click, `+`/`-`/`0`) and `LayoutGroup`/`LayoutItem` (layout and shared-element transitions).
* `css`: the same springs traced as CSS `linear()` easings, for CSS-animated overlays.

Everything honours `prefers-reduced-motion`: animations jump to their end, while dragging still follows the pointer.

It wraps [`dioxus-motion`](https://github.com/wheregmis/dioxus-motion) (pinned `=0.4.0`), so apps never touch the engine directly. Enable the `web` or `desktop` feature to match your Dioxus renderer.

Licensed under MIT or Apache-2.0.
