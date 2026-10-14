//! Hooks shared by components and apps. `rust-ui add hooks` copies this folder; components
//! that import from it pull it in automatically.

mod clipboard;
mod hotkey;
mod media;
mod selection;

pub use clipboard::*;
pub use hotkey::*;
pub use media::*;
pub use selection::*;
