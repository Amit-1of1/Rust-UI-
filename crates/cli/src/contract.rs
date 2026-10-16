//! Component contracts: one `contract.toml` per component saying which WAI-ARIA pattern it
//! follows, its roles, its states and its keyboard map, and for each key a short test that
//! proves it. The docs page, `llms/<name>.md` and the MCP answers show the table generated
//! from it; `tests/e2e/contracts.mjs` (web) and `apps/native/tests/contracts.rs` (Blitz) run
//! every row's test. A row can't be documented without being tested. docs/CONTRACTS.md has
//! the format.
//!
//! This file only uses `serde`, `toml` and `std`, so `crates/ui/build.rs` and the native tests
//! include it with `#[path]` and check the same rules the CLI does.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// The file name inside a component folder.
pub const FILE: &str = "contract.toml";

/// Where an APG pattern slug points.
pub const APG_BASE: &str = "https://www.w3.org/WAI/ARIA/apg/patterns/";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Contract {
    /// Component (folder) name; filled in by `load`.
    #[serde(default)]
    pub name: String,
    /// The pattern's name as the APG writes it, e.g. "Dialog (Modal)".
    pub pattern: String,
    /// The APG pattern's slug (`dialog-modal`), when the component follows one.
    #[serde(default)]
    pub apg: Option<String>,
    /// Roles the component exposes. Each must be asserted by a test through a part whose
    /// selector names it (`[role=dialog]`).
    pub roles: Vec<String>,
    /// What does not work on native yet, shown under the table.
    #[serde(default)]
    pub native_note: Option<String>,
    /// Where the component differs from the APG pattern or a key is missing, shown under the
    /// table. Only absences go here: anything the component does goes in a tested row.
    #[serde(default)]
    pub gaps: Vec<String>,
    /// Named CSS selectors the tests use, relative to the demo.
    pub parts: BTreeMap<String, String>,
    #[serde(default)]
    pub states: Vec<State>,
    pub keys: Vec<KeyRow>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub name: String,
    /// What the state means to the user.
    pub means: String,
    /// One assertion step that holds exactly in this state (`attr trigger aria-expanded=true`).
    pub check: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KeyRow {
    /// One key as `press` takes it: `Enter`, `Space`, `ArrowDown`, `Shift+Tab`, `Escape`.
    pub key: String,
    /// Where focus is (or what is open) for the key to do this.
    pub when: String,
    /// What the key does, as the docs table says it.
    pub does: String,
    /// Whether the row also holds on native (Blitz). The native test checks this both ways.
    pub native: bool,
    /// Steps, one per line (docs/CONTRACTS.md).
    pub test: String,
}

/// A part with an optional 1-based index among its matches (`item@2`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PartRef {
    pub part: String,
    pub selector: String,
    /// 1-based.
    pub nth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "op", rename_all = "kebab-case")]
pub enum Step {
    /// Move keyboard focus to the part.
    Focus { at: PartRef },
    /// Click the middle of the part.
    Click { at: PartRef },
    /// Press a key `times` times.
    Press { key: String, times: usize },
    /// Type text into the focused element.
    Type { text: String },
    /// The part exists and is shown.
    Visible { at: PartRef },
    /// The part is absent or not shown.
    Hidden { at: PartRef },
    /// The part has keyboard focus.
    Focused { at: PartRef },
    /// Keyboard focus is inside the part (or on it).
    FocusIn { at: PartRef },
    /// The part's text contains `text`.
    Text { at: PartRef, text: String },
    /// The part's attribute equals (or, with `negate`, does not equal) a value.
    Attr {
        at: PartRef,
        name: String,
        value: String,
        negate: bool,
    },
}

impl Step {
    pub fn is_assertion(&self) -> bool {
        !matches!(
            self,
            Step::Focus { .. } | Step::Click { .. } | Step::Press { .. } | Step::Type { .. }
        )
    }

    fn part(&self) -> Option<&PartRef> {
        match self {
            Step::Press { .. } | Step::Type { .. } => None,
            Step::Focus { at }
            | Step::Click { at }
            | Step::Visible { at }
            | Step::Hidden { at }
            | Step::Focused { at }
            | Step::FocusIn { at }
            | Step::Text { at, .. }
            | Step::Attr { at, .. } => Some(at),
        }
    }
}

/// A contract with every row's test parsed, as the test runners take it (`rust-ui contract
/// --json`).
#[derive(Debug, Clone, Serialize)]
pub struct Runnable {
    pub name: String,
    pub rows: Vec<RunnableRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunnableRow {
    pub key: String,
    pub does: String,
    pub native: bool,
    pub steps: Vec<Step>,
}

/// Keys `press` understands. Anything else must be one character.
pub const NAMED_KEYS: &[&str] = &[
    "Enter",
    "Space",
    "Escape",
    "Tab",
    "ArrowUp",
    "ArrowDown",
    "ArrowLeft",
    "ArrowRight",
    "Home",
    "End",
    "PageUp",
    "PageDown",
    "Backspace",
    "Delete",
];

pub const MODIFIERS: &[&str] = &["Shift", "Control", "Alt", "Meta"];

fn valid_key(key: &str) -> bool {
    let mut parts: Vec<&str> = key.split('+').collect();
    let Some(last) = parts.pop() else {
        return false;
    };
    parts.iter().all(|m| MODIFIERS.contains(m))
        && (NAMED_KEYS.contains(&last) || last.chars().count() == 1)
}

impl Contract {
    pub fn parse(name: &str, text: &str) -> Result<Contract, String> {
        let mut c: Contract = toml::from_str(text).map_err(|e| format!("{name}/{FILE}: {e}"))?;
        c.name = name.to_string();
        Ok(c)
    }

    /// Read `<components>/<name>/contract.toml`, `Ok(None)` when there is none.
    pub fn load(components: &Path, name: &str) -> Result<Option<Contract>, String> {
        let path = components.join(name).join(FILE);
        match fs::read_to_string(&path) {
            Ok(text) => Contract::parse(name, &text).map(Some),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(format!("{}: {e}", path.display())),
        }
    }

    /// Every contract under `components`, in name order. Fails on the first unreadable one.
    pub fn load_all(components: &Path) -> Result<Vec<Contract>, String> {
        let mut names: Vec<String> = fs::read_dir(components)
            .map_err(|e| format!("{}: {e}", components.display()))?
            .filter_map(Result::ok)
            .filter(|e| e.path().join(FILE).is_file())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        let mut out = Vec::new();
        for name in names {
            if let Some(c) = Contract::load(components, &name)? {
                out.push(c);
            }
        }
        Ok(out)
    }

    pub fn part_ref(&self, word: &str) -> Result<PartRef, String> {
        let (part, nth) = match word.split_once('@') {
            Some((p, n)) => (
                p,
                n.parse::<usize>()
                    .ok()
                    .filter(|n| *n > 0)
                    .ok_or_else(|| format!("`{word}`: the index after @ starts at 1"))?,
            ),
            None => (word, 1),
        };
        let selector = self
            .parts
            .get(part)
            .ok_or_else(|| format!("unknown part `{part}` (add it under [parts])"))?;
        Ok(PartRef {
            part: part.to_string(),
            selector: selector.clone(),
            nth,
        })
    }

    /// Parse one step line. `state <name>` expands to that state's check.
    pub fn step(&self, line: &str) -> Result<Step, String> {
        let line = line.trim();
        let (op, rest) = line.split_once(' ').unwrap_or((line, ""));
        let rest = rest.trim();
        let one = |what: &str| -> Result<PartRef, String> {
            if rest.is_empty() || rest.contains(' ') {
                return Err(format!("`{line}`: `{op}` takes one {what}"));
            }
            self.part_ref(rest)
        };
        Ok(match op {
            "focus" => Step::Focus { at: one("part")? },
            "click" => Step::Click { at: one("part")? },
            "visible" => Step::Visible { at: one("part")? },
            "hidden" => Step::Hidden { at: one("part")? },
            "focused" => Step::Focused { at: one("part")? },
            "focus-in" => Step::FocusIn { at: one("part")? },
            "type" if !rest.is_empty() => Step::Type {
                text: rest.to_string(),
            },
            "press" => {
                let (key, times) = match rest.rsplit_once('*') {
                    Some((k, n)) => (
                        k,
                        n.parse::<usize>()
                            .ok()
                            .filter(|n| *n > 0)
                            .ok_or_else(|| format!("`{line}`: bad repeat count"))?,
                    ),
                    None => (rest, 1),
                };
                if !valid_key(key) {
                    return Err(format!(
                        "`{line}`: unknown key `{key}` (one of {}, or one character, with optional Shift+/Control+/Alt+/Meta+)",
                        NAMED_KEYS.join(", ")
                    ));
                }
                Step::Press {
                    key: key.to_string(),
                    times,
                }
            }
            "text" => {
                let (part, text) = rest
                    .split_once(' ')
                    .ok_or_else(|| format!("`{line}`: write `text <part> <some text>`"))?;
                Step::Text {
                    at: self.part_ref(part)?,
                    text: text.trim().to_string(),
                }
            }
            "attr" => {
                let (part, cond) = rest
                    .split_once(' ')
                    .ok_or_else(|| format!("`{line}`: write `attr <part> <name>=<value>`"))?;
                let (name, value, negate) = match cond.split_once("!=") {
                    Some((n, v)) => (n, v, true),
                    None => {
                        let (n, v) = cond.split_once('=').ok_or_else(|| {
                            format!("`{line}`: write `attr <part> <name>=<value>`")
                        })?;
                        (n, v, false)
                    }
                };
                Step::Attr {
                    at: self.part_ref(part)?,
                    name: name.trim().to_string(),
                    value: value.trim().to_string(),
                    negate,
                }
            }
            "state" => {
                let state = self
                    .states
                    .iter()
                    .find(|s| s.name == rest)
                    .ok_or_else(|| format!("`{line}`: no state `{rest}` under [[states]]"))?;
                let step = self.step(&state.check)?;
                if !step.is_assertion() || state.check.trim_start().starts_with("state") {
                    return Err(format!(
                        "state `{}`: its check must be one assertion step",
                        state.name
                    ));
                }
                step
            }
            _ => return Err(format!("`{line}`: unknown step `{op}` (docs/CONTRACTS.md)")),
        })
    }

    /// The steps of one row's test.
    pub fn steps(&self, row: &KeyRow) -> Result<Vec<Step>, String> {
        row.test
            .lines()
            .map(|l| l.split('#').next().unwrap_or("").trim())
            .filter(|l| !l.is_empty())
            .map(|l| self.step(l))
            .collect()
    }

    pub fn runnable(&self) -> Result<Runnable, String> {
        let mut rows = Vec::new();
        for row in &self.keys {
            rows.push(RunnableRow {
                key: row.key.clone(),
                does: row.does.clone(),
                native: row.native,
                steps: self
                    .steps(row)
                    .map_err(|e| format!("{} key `{}`: {e}", self.name, row.key))?,
            });
        }
        Ok(Runnable {
            name: self.name.clone(),
            rows,
        })
    }

    /// Every way this contract could claim something its tests do not check. Empty when the
    /// contract is sound.
    pub fn problems(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.keys.is_empty() {
            out.push("no [[keys]] rows".to_string());
        }
        if let Some(slug) = &self.apg {
            if slug.is_empty()
                || !slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                out.push(format!(
                    "apg `{slug}` must be the pattern's slug, as in {APG_BASE}<slug>/"
                ));
            }
        }
        for s in &self.states {
            if let Err(e) = self.step(&format!("state {}", s.name)) {
                out.push(e);
            }
        }
        // Parts that assertions look at, for the roles check.
        let mut asserted: Vec<String> = Vec::new();
        let mut used_states: Vec<&str> = Vec::new();
        for row in &self.keys {
            let tag = format!("key `{}`", row.key);
            if !valid_key(&row.key) {
                out.push(format!("{tag}: not a key `press` understands"));
            }
            if row.when.trim().is_empty() || row.does.trim().is_empty() {
                out.push(format!("{tag}: `when` and `does` must say something"));
            }
            for line in row.test.lines() {
                if let Some(s) = line.trim().strip_prefix("state ") {
                    used_states.push(
                        self.states
                            .iter()
                            .find(|st| st.name == s.trim())
                            .map_or("", |st| st.name.as_str()),
                    );
                }
            }
            let steps = match self.steps(row) {
                Ok(s) => s,
                Err(e) => {
                    out.push(format!("{tag}: {e}"));
                    continue;
                }
            };
            // The row's own key must be pressed, and something must be checked after it.
            match steps
                .iter()
                .rposition(|s| matches!(s, Step::Press { key, .. } if *key == row.key))
            {
                None => out.push(format!(
                    "{tag}: its test never presses {} (the table would claim an untested key)",
                    row.key
                )),
                Some(i) if !steps[i + 1..].iter().any(Step::is_assertion) => out.push(format!(
                    "{tag}: nothing is checked after pressing {}",
                    row.key
                )),
                Some(_) => {}
            }
            for s in steps.iter().filter(|s| s.is_assertion()) {
                if let Some(p) = s.part() {
                    asserted.push(p.selector.clone());
                }
            }
        }
        for s in &self.states {
            if !used_states.contains(&s.name.as_str()) {
                out.push(format!(
                    "state `{}` is listed but no test checks it (`state {}`)",
                    s.name, s.name
                ));
            }
        }
        for role in &self.roles {
            let needle = [
                format!("[role={role}]"),
                format!("[role=\"{role}\"]"),
                format!("[role='{role}']"),
            ];
            if !asserted
                .iter()
                .any(|sel| needle.iter().any(|n| sel.contains(n.as_str())))
            {
                out.push(format!(
                    "role `{role}` is listed but no test asserts on a part selected by [role={role}]"
                ));
            }
        }
        for (part, sel) in &self.parts {
            if sel.trim().is_empty() {
                out.push(format!("part `{part}` has an empty selector"));
            }
        }
        let mut seen: Vec<(&str, &str)> = Vec::new();
        for row in &self.keys {
            let k = (row.key.as_str(), row.when.as_str());
            if seen.contains(&k) {
                out.push(format!(
                    "key `{}` appears twice for \"{}\"; merge the rows",
                    row.key, row.when
                ));
            }
            seen.push(k);
        }
        out
    }

    /// Whether the component works on native: every row, some rows or none.
    pub fn native_summary(&self) -> &'static str {
        let yes = self.keys.iter().filter(|k| k.native).count();
        if yes == self.keys.len() {
            "yes"
        } else if yes == 0 {
            "not yet"
        } else {
            "partly"
        }
    }

    /// The `## Keyboard and accessibility` section for the docs page and the llms page.
    pub fn markdown(&self) -> String {
        let mut out = String::from("## Keyboard and accessibility\n\n");
        match &self.apg {
            Some(slug) => out.push_str(&format!(
                "Follows the WAI-ARIA [{}]({APG_BASE}{slug}/) pattern. ",
                self.pattern
            )),
            None => out.push_str(&format!("Pattern: {}. ", self.pattern)),
        }
        let roles: Vec<String> = self.roles.iter().map(|r| format!("`{r}`")).collect();
        if !roles.is_empty() {
            out.push_str(&format!("Roles: {}. ", roles.join(", ")));
        }
        out.push_str(&format!(
            "Works on native (Blitz): {}.\n\n",
            self.native_summary()
        ));
        out.push_str("| Key | When | What it does | Native |\n|---|---|---|---|\n");
        for row in &self.keys {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                display_key(&row.key),
                cell(&row.when),
                cell(&row.does),
                if row.native { "yes" } else { "not yet" }
            ));
        }
        if !self.states.is_empty() {
            out.push_str("\nStates:\n\n");
            for s in &self.states {
                out.push_str(&format!("- `{}`: {}\n", s.name, s.means.trim()));
            }
        }
        if !self.gaps.is_empty() {
            out.push_str("\nKnown gaps:\n\n");
            for g in &self.gaps {
                out.push_str(&format!("- {}\n", g.trim()));
            }
        }
        if let Some(note) = &self.native_note {
            out.push_str(&format!("\nNative: {}\n", note.trim()));
        }
        out.push_str(
            "\nEvery row is run as a keyboard test on the web, and on native where it says yes. \
             Generated from the component's `contract.toml`.\n",
        );
        out
    }
}

fn cell(text: &str) -> String {
    text.trim().replace('|', "\\|").replace('\n', " ")
}

/// `Shift+Tab` -> `Shift + Tab`, `Space` stays.
pub fn display_key(key: &str) -> String {
    key.split('+').collect::<Vec<_>>().join(" + ")
}
