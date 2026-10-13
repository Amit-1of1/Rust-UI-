# rust-ui-motion

Motion helpers for [Dioxus](https://dioxuslabs.com) 0.7, used by the Rust UI kit:

* `SpringPreset` and `Transition`: named spring feels that collapse to an instant change when the OS asks for reduced motion.
* `use_reduced_motion`, `use_spring`, `Pop` with `AnimatePresence`, `FlipList`/`FlipItem` and `SwipeDismiss`.
* `css`: the same springs traced as CSS `linear()` easings, for CSS-animated overlays.

It wraps [`dioxus-motion`](https://github.com/wheregmis/dioxus-motion) (pinned `=0.4.0`), so apps never touch the engine directly. Enable the `web` or `desktop` feature to match your Dioxus renderer.

Licensed under MIT or Apache-2.0.
