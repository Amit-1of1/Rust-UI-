//! Key names as tests write them (`"Enter"`, `"Shift+Tab"`, `"Control+a"`), turned into the
//! key, code and modifiers the shell would send.

use keyboard_types::{Code, Key, Modifiers};

/// Parse `"Shift+Tab"`, `"Escape"`, `"ArrowDown"`, `"Space"`, `"a"`. Modifiers are `Shift`,
/// `Control` (or `Ctrl`), `Alt` and `Meta` (or `Cmd`), joined with `+`. Any other single name
/// is sent as a character key. Panics on an unknown multi-letter name, so a typo fails loudly.
pub fn parse(name: &str) -> (Key, Code, Modifiers) {
    let mut mods = Modifiers::empty();
    let mut parts: Vec<&str> = name.split('+').collect();
    // "Control++" is Control and "+".
    let last = if name.ends_with("++") {
        parts.truncate(parts.len().saturating_sub(2));
        "+"
    } else {
        parts.pop().unwrap_or(name)
    };
    for m in parts {
        mods |= match m {
            "Shift" => Modifiers::SHIFT,
            "Control" | "Ctrl" => Modifiers::CONTROL,
            "Alt" => Modifiers::ALT,
            "Meta" | "Cmd" => Modifiers::META,
            other => panic!("atoll-test: unknown modifier {other:?} in key {name:?}"),
        };
    }
    let (k, c) = match last {
        "Enter" => (Key::Enter, Code::Enter),
        "Space" | " " => (Key::Character(" ".into()), Code::Space),
        "Escape" | "Esc" => (Key::Escape, Code::Escape),
        "Tab" => (Key::Tab, Code::Tab),
        "ArrowUp" => (Key::ArrowUp, Code::ArrowUp),
        "ArrowDown" => (Key::ArrowDown, Code::ArrowDown),
        "ArrowLeft" => (Key::ArrowLeft, Code::ArrowLeft),
        "ArrowRight" => (Key::ArrowRight, Code::ArrowRight),
        "Home" => (Key::Home, Code::Home),
        "End" => (Key::End, Code::End),
        "PageUp" => (Key::PageUp, Code::PageUp),
        "PageDown" => (Key::PageDown, Code::PageDown),
        "Backspace" => (Key::Backspace, Code::Backspace),
        "Delete" => (Key::Delete, Code::Delete),
        c if c.chars().count() == 1 => (Key::Character(c.into()), Code::Unidentified),
        other => panic!("atoll-test: unknown key {other:?} (in {name:?})"),
    };
    (k, c, mods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_modifiers() {
        assert_eq!(parse("Enter").0, Key::Enter);
        let (k, c, m) = parse("Shift+Tab");
        assert_eq!((k, c, m), (Key::Tab, Code::Tab, Modifiers::SHIFT));
        let (k, _, m) = parse("Ctrl+a");
        assert_eq!((k, m), (Key::Character("a".into()), Modifiers::CONTROL));
        assert_eq!(parse("Control++").0, Key::Character("+".into()));
        assert_eq!(parse("Space").1, Code::Space);
    }

    #[test]
    #[should_panic(expected = "unknown key")]
    fn typo_fails() {
        parse("Enterr");
    }
}
