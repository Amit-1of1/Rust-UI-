# Command

- Copy into your app: `rust-ui add command` or `dx components add command --git https://github.com/Amit-1of1/Rust-UI-` (also copies `dialog`, `hooks`, `ui_strings`).
- Import: `use crate::components::command::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A command palette: a search box over a filtered list, inside a dialog. `filter_items` and `step_active` are pure helpers.

Pass `hotkey: "k"` to open it with Ctrl+K (Cmd+K on macOS); it uses `use_hotkey` from the `hooks` item, which `rust-ui add command` installs with it. That works on the web only, so keep a visible button too.

## Accessibility

- Role and name: a `combobox` input controlling a labelled `listbox`, with `aria-activedescendant` on the highlighted option
- Keyboard: Ctrl+K opens it (web, with `hotkey: "k"`); type to filter, Arrow keys move the highlight, Enter runs it, Escape closes
- Reduced motion: none

## API

```rust
/// One entry in a command palette.
pub struct CommandItem {
    pub id: String,
    pub label: String,
    /// Shown as a heading above consecutive items that share it.
    pub group: Option<String>,
    /// Display-only hint such as `Ctrl K`.
    pub shortcut: Option<String>,
}

/// Indices of the items that match `query`: every whitespace-separated word must appear in
/// the label, ignoring case. An empty query matches everything. Order is preserved.
pub fn filter_items(items: &[CommandItem], query: &str) -> Vec<usize>

/// Move the highlighted row by `delta`, wrapping at both ends. `len == 0` yields 0.
pub fn step_active(current: usize, delta: isize, len: usize) -> usize

/// Where the highlight goes after `key` in a list of `len` rows, or `None` if the key
/// doesn't move it. Arrows wrap; Home and End jump to the ends.
pub fn command_nav(key: &Key, active: usize, len: usize) -> Option<usize>

/// A searchable list of actions in a modal dialog. Type to filter, arrow keys to move,
/// Enter to choose, Escape to close. Focus stays in the input; the highlighted row is
/// exposed through `aria-activedescendant`.
#[component]
pub fn CommandDialog(
    open: bool,
    on_open_change: EventHandler<bool>,
    items: Vec<CommandItem>,
    on_select: EventHandler<String>,
    /// Defaults to "Search commands..." ([`UiStrings`](crate::components::ui_strings::UiStrings)).
    #[props(into)]
    placeholder: Option<String>,
    /// Open with Ctrl+`hotkey` (Cmd on macOS) anywhere on the page, e.g. `hotkey: "k"`.
    /// Web only ([`use_hotkey`]); keep a visible button that opens it too. Read once, on mount.
    hotkey: Option<&'static str>,
) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::command::*;
use crate::components::kbd::{Kbd, KbdGroup};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut last = use_signal(|| None::<String>);
    let items = vec![
        CommandItem::new("new-file", "New file")
            .group("File")
            .shortcut("Ctrl N"),
        CommandItem::new("open-file", "Open file")
            .group("File")
            .shortcut("Ctrl O"),
        CommandItem::new("save-file", "Save file")
            .group("File")
            .shortcut("Ctrl S"),
        CommandItem::new("toggle-theme", "Toggle theme").group("View"),
        CommandItem::new("zoom-in", "Zoom in")
            .group("View")
            .shortcut("Ctrl +"),
        CommandItem::new("zoom-out", "Zoom out")
            .group("View")
            .shortcut("Ctrl -"),
    ];
    rsx! {
        div { style: "display: flex; align-items: center; gap: 1rem; flex-wrap: wrap;",
            Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true),
                "Open command palette"
                KbdGroup {
                    Kbd { "Ctrl" }
                    Kbd { "K" }
                }
            }
            if let Some(id) = last() {
                span { style: "font-size: 0.875rem;", "Ran: {id}" }
            }
        }
        CommandDialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            items,
            on_select: move |id| last.set(Some(id)),
            // Ctrl+K (Cmd+K) anywhere on the page; the button does the same off the web.
            hotkey: "k",
        }
    }
}
```
