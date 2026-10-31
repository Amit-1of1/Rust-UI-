//! Hooks shared by components and apps. `atoll add hooks` copies this folder; components
//! that import from it pull it in automatically.

mod clipboard;
mod focus_return;
mod hotkey;
mod media;
mod router;
mod selection;

pub use clipboard::*;
pub use focus_return::*;
pub use hotkey::*;
pub use media::*;
pub use router::*;
pub use selection::*;
