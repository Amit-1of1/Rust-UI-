use crate::components::dialog::Dialog;
use crate::components::hooks::use_hotkey;
use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/command/style.css")]
struct Styles;

/// One entry in a command palette.
#[derive(Clone, PartialEq, Debug)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    /// Shown as a heading above consecutive items that share it.
    pub group: Option<String>,
    /// Display-only hint such as `Ctrl K`.
    pub shortcut: Option<String>,
}

impl CommandItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            group: None,
            shortcut: None,
        }
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

/// Indices of the items that match `query`: every whitespace-separated word must appear in
/// the label, ignoring case. An empty query matches everything. Order is preserved.
pub fn filter_items(items: &[CommandItem], query: &str) -> Vec<usize> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            let label = item.label.to_lowercase();
            words.iter().all(|word| label.contains(word.as_str()))
        })
        .map(|(index, _)| index)
        .collect()
}

/// Move the highlighted row by `delta`, wrapping at both ends. `len == 0` yields 0.
pub fn step_active(current: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (current as isize + delta).rem_euclid(len as isize) as usize
}

/// Where the highlight goes after `key` in a list of `len` rows, or `None` if the key
/// doesn't move it. Arrows wrap; Home and End jump to the ends.
pub fn command_nav(key: &Key, active: usize, len: usize) -> Option<usize> {
    match key {
        Key::Named(NamedKey::ArrowDown) => Some(step_active(active, 1, len)),
        Key::Named(NamedKey::ArrowUp) => Some(step_active(active, -1, len)),
        Key::Named(NamedKey::Home) => Some(0),
        Key::Named(NamedKey::End) => Some(len.saturating_sub(1)),
        _ => None,
    }
}

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
) -> Element {
    use_hotkey(hotkey.unwrap_or_default(), move || {
        if hotkey.is_some() {
            on_open_change.call(true);
        }
    });
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let base = use_hook(|| format!("dx-command-{}", NEXT.fetch_add(1, Ordering::Relaxed)));
    let mut query = use_signal(String::new);
    let mut active = use_signal(|| 0usize);
    let t = use_ui_strings();
    let placeholder = placeholder.unwrap_or_else(|| t.search_commands_placeholder.clone());

    // Start fresh every time the palette opens.
    use_effect(move || {
        if open {
            query.set(String::new());
            active.set(0);
        }
    });

    let matches = filter_items(&items, &query.read());
    let count = matches.len();
    let active_index = (*active.read()).min(count.saturating_sub(1));
    let active_id = (count > 0).then(|| format!("{base}-option-{active_index}"));

    let choose = {
        let items = items.clone();
        let matches = matches.clone();
        move |row: usize| {
            if let Some(&index) = matches.get(row) {
                on_select.call(items[index].id.clone());
                on_open_change.call(false);
            }
        }
    };
    let choose_on_key = choose.clone();

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        if let Some(next) = command_nav(&key, active_index, count) {
            event.prevent_default();
            active.set(next);
        } else if key == Key::Named(NamedKey::Enter) {
            event.prevent_default();
            choose_on_key(active_index);
        }
    };

    rsx! {
        Dialog {
            open,
            on_open_change: move |v| on_open_change.call(v),
            title: t.command_palette,
            description: t.command_help,
            hide_title: true,
            div { class: Styles::dx_command,
                input {
                    class: Styles::dx_command_input,
                    r#type: "text",
                    role: "combobox",
                    autocomplete: "off",
                    spellcheck: "false",
                    autofocus: true,
                    placeholder,
                    aria_label: t.search_commands,
                    aria_expanded: "true",
                    aria_controls: "{base}-list",
                    aria_activedescendant: active_id,
                    value: "{query}",
                    oninput: move |e| {
                        query.set(e.value());
                        active.set(0);
                    },
                    onkeydown,
                }
                ul { class: Styles::dx_command_list, id: "{base}-list", role: "listbox", aria_label: t.commands,
                    for (row, &index) in matches.iter().enumerate() {
                        {
                            let item = &items[index];
                            let previous_group = row.checked_sub(1).and_then(|r| items[matches[r]].group.clone());
                            let show_heading = item.group.is_some() && item.group != previous_group;
                            let choose = choose.clone();
                            rsx! {
                                Fragment { key: "{item.id}",
                                if show_heading {
                                    li { class: Styles::dx_command_heading, role: "presentation",
                                        "{item.group.clone().unwrap_or_default()}"
                                    }
                                }
                                li {
                                    class: Styles::dx_command_item,
                                    id: "{base}-option-{row}",
                                    role: "option",
                                    aria_selected: (row == active_index).to_string(),
                                    "data-active": (row == active_index).then_some("true"),
                                    onmouseenter: move |_| active.set(row),
                                    onclick: move |_| choose(row),
                                    span { "{item.label}" }
                                    if let Some(shortcut) = &item.shortcut {
                                        span { class: Styles::dx_command_shortcut, "{shortcut}" }
                                    }
                                }
                                }
                            }
                        }
                    }
                }
                if count == 0 {
                    p { class: Styles::dx_command_empty, role: "status", {t.no_results} }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items() -> Vec<CommandItem> {
        ["Open file", "Open folder", "Save file", "Close window"]
            .into_iter()
            .map(|l| CommandItem::new(l.to_lowercase().replace(' ', "-"), l))
            .collect()
    }

    #[test]
    fn empty_query_matches_everything_in_order() {
        assert_eq!(filter_items(&items(), ""), vec![0, 1, 2, 3]);
        assert_eq!(filter_items(&items(), "   "), vec![0, 1, 2, 3]);
    }

    #[test]
    fn filters_case_insensitively_and_requires_every_word() {
        assert_eq!(filter_items(&items(), "OPEN"), vec![0, 1]);
        assert_eq!(filter_items(&items(), "file open"), vec![0]);
        assert!(filter_items(&items(), "zzz").is_empty());
    }

    #[test]
    fn step_wraps_in_both_directions() {
        assert_eq!(step_active(0, 1, 3), 1);
        assert_eq!(step_active(2, 1, 3), 0);
        assert_eq!(step_active(0, -1, 3), 2);
        assert_eq!(step_active(0, 1, 0), 0);
    }
}
