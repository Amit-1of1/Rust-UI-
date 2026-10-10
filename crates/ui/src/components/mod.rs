//! Styled components. Each is copied from DioxusLabs/components (MIT OR Apache-2.0)
//! at the revision pinned in Cargo.toml; see THIRD_PARTY.md.

pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod calendar;
pub mod card;
pub mod chart;
pub mod checkbox;
pub mod collapsible;
pub mod color_picker;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod date_picker;
pub mod dialog;
pub mod drag_and_drop_list;
pub mod dropdown_menu;
pub mod field;
pub mod hover_card;
pub mod input;
pub mod item;
pub mod kbd;
pub mod label;
pub mod menubar;
pub mod navbar;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod tag_group;
pub mod textarea;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod toolbar;
pub mod tooltip;
pub mod virtual_list;

/// A neutral placeholder image (an inline SVG) for avatar demos, so demos never hit the network.
pub(crate) const DEMO_AVATAR_SRC: &str = concat!(
    "data:image/svg+xml;utf8,",
    "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'>",
    "<rect width='64' height='64' fill='%23888'/>",
    "<circle cx='32' cy='25' r='11' fill='%23fff'/>",
    "<path d='M10 58c3-15 13-21 22-21s19 6 22 21z' fill='%23fff'/></svg>"
);
