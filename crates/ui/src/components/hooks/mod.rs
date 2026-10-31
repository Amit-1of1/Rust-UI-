//! Hooks shared by components and apps. `rust-ui add hooks` copies this folder; components
//! that import from it pull it in automatically.

mod clipboard;
mod focus_return;
mod hotkey;
mod id;
mod media;
mod router;
mod selection;

pub use clipboard::*;
pub use focus_return::*;
pub use hotkey::*;
pub use id::*;
pub use media::*;
pub use router::*;
pub use selection::*;
