//! `atoll icons <words>`: find a Lucide icon by name or by what it is for, using the search
//! words Lucide ships (`tags.json`, kept in the generated rustdoc), and the icons the kit
//! reserves for one meaning (docs/ICONS.md).

/// The generated icon file, relative to the kit root.
pub const ICONS_FILE: &str = "crates/ui/src/components/icons/lucide.rs";

/// Icons the kit's components use for one meaning. Using them for something else in an app
/// teaches people the wrong thing, so pick another icon for another meaning.
pub const RESERVED: &[(&str, &str)] = &[
    ("x", "close or dismiss"),
    ("check", "selected, done"),
    ("chevron-down", "opens a list or menu below"),
    ("chevrons-up-down", "opens a list to choose from"),
    ("chevron-right", "go into, next, submenu"),
    ("chevron-left", "back, previous"),
    ("search", "search"),
    ("ellipsis", "more actions"),
    ("panel-left", "show or hide the sidebar"),
    ("grip-vertical", "drag to reorder"),
    ("loader-circle", "loading"),
    ("circle-check", "success"),
    ("circle-alert", "warning or needs attention"),
    ("circle-x", "error or failed"),
    ("external-link", "opens outside this app"),
    ("plus", "add or create"),
    ("minus", "remove one, decrease"),
    ("clock", "pick a time"),
    ("arrow-up", "sort ascending"),
    ("arrow-down", "sort descending"),
];

/// One icon: its Lucide name (`circle-check`) and search words.
#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    pub name: String,
    pub keywords: Vec<String>,
}

impl Icon {
    /// The Rust component name: `circle-check` -> `CircleCheck`.
    pub fn component(&self) -> String {
        self.name
            .split('-')
            .map(|w| {
                let mut c = w.chars();
                c.next()
                    .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
                    .unwrap_or_default()
            })
            .collect()
    }

    /// What the kit reserves this icon for, if anything.
    pub fn reserved(&self) -> Option<&'static str> {
        RESERVED
            .iter()
            .find(|(n, _)| *n == self.name)
            .map(|(_, m)| *m)
    }
}

/// Reads every icon from the generated file's doc lines: ``/// Lucide `check` (done, tick).``
/// The doc lines of the old-name aliases point at the icon (`[`delete`](super::delete)`) and
/// are skipped, so each icon is listed once.
pub fn parse(source: &str) -> Vec<Icon> {
    let mut seen = std::collections::HashSet::new();
    source
        .lines()
        .filter_map(|l| l.trim().strip_prefix("/// Lucide `"))
        .filter(|l| !l.contains("]("))
        .filter_map(|l| {
            let (name, rest) = l.split_once('`')?;
            let keywords = rest
                .trim()
                .strip_prefix('(')
                .and_then(|r| r.rsplit_once(')'))
                .map(|(k, _)| k.split(',').map(|w| w.trim().to_lowercase()).collect())
                .unwrap_or_default();
            seen.insert(name.to_string()).then(|| Icon {
                name: name.to_string(),
                keywords,
            })
        })
        .collect()
}

/// Icons matching every word of `query`, best first: a word in the name beats a search word,
/// and a whole-word hit beats part of one.
pub fn search<'a>(icons: &'a [Icon], query: &str) -> Vec<&'a Icon> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    if words.is_empty() {
        return Vec::new();
    }
    let mut hits: Vec<(u32, &Icon)> = icons
        .iter()
        .filter_map(|icon| {
            let mut score = 0;
            for w in &words {
                let s = if icon.name == *w {
                    100
                } else if icon.name.split('-').any(|p| p == w) {
                    40
                } else if icon.keywords.iter().any(|k| k == w) {
                    20
                } else if icon.name.contains(w.as_str()) {
                    10
                } else if icon.keywords.iter().any(|k| k.contains(w.as_str())) {
                    5
                } else {
                    return None;
                };
                score += s;
            }
            Some((score, icon))
        })
        .collect();
    hits.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.cmp(&b.1.name)));
    hits.into_iter().map(|(_, i)| i).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "/// Lucide `check` (done, todo, tick, complete, task).\npub const CHECK: IconData = x;\n/// Lucide `circle-check` (done, todo, tick, complete, task, success).\n/// Lucide `trash-2` (garbage, delete, remove, bin).\n";

    #[test]
    fn reads_names_and_search_words() {
        let icons = parse(SAMPLE);
        assert_eq!(icons.len(), 3);
        assert_eq!(icons[2].name, "trash-2");
        assert_eq!(icons[2].keywords, ["garbage", "delete", "remove", "bin"]);
        assert_eq!(icons[1].component(), "CircleCheck");
    }

    #[test]
    fn finds_by_meaning_and_ranks_names_first() {
        let icons = parse(SAMPLE);
        let names: Vec<_> = search(&icons, "delete").iter().map(|i| &i.name).collect();
        assert_eq!(names, ["trash-2"]);
        let names: Vec<_> = search(&icons, "check").iter().map(|i| &i.name).collect();
        assert_eq!(names, ["check", "circle-check"]);
        assert!(search(&icons, "done success")
            .iter()
            .all(|i| i.name == "circle-check"));
    }

    #[test]
    fn every_reserved_icon_exists_in_the_kit() {
        let icons = parse(include_str!("../../ui/src/components/icons/lucide.rs"));
        assert!(icons.len() > 1500);
        for (name, _) in RESERVED {
            assert!(
                icons.iter().any(|i| i.name == *name),
                "{name} is not a Lucide icon"
            );
        }
    }
}
