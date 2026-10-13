//! The kit's icon set: [Lucide](https://lucide.dev) through the `dioxus-icons` crate.
//!
//! Every component and app imports icons from here (`crate::components::icons::Check`, or
//! `ui::icons::Check` in an app), never from `dioxus_icons` directly, so swapping the set
//! means changing this one file (docs/GUARDRAILS.md: wrap outside dependencies).
//!
//! Each icon is its own component, so only the icons you import end up in the build.

/// Every Lucide icon, one component each: `Check { size: "1rem", "aria-hidden": "true" }`.
pub use dioxus_icons::lucide::*;
/// Props shared by every icon: `size` plus any SVG attribute (`stroke`, `class`, `aria_*`).
pub use dioxus_icons::{IconProps, IconSize};
