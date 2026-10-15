//! Blocks: whole screens (sign-in, sign-up, a dashboard shell, settings) built only from kit
//! components. Copy one with `rust-ui add login_01` and edit it like your own code; it lands in
//! `src/blocks/` and pulls in the components it uses. They are starting points, not fixed
//! components, so their props are few and their look is meant to be changed.
//!
//! Internal: not part of the public API (docs/STABILITY.md). Feature `blocks`.

pub mod dashboard_01;
pub mod login_01;
pub mod settings_01;
pub mod signup_01;
