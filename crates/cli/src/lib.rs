//! Registry logic for the `rust-ui` CLI.
//!
//! The registry is shaped like shadcn's: one item per component with its files and
//! the other registry items it needs. It is generated from the component folders,
//! so it can't drift from the source.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const COMPONENTS_DIR: &str = "crates/ui/src/components";
pub const THEME_FILE: &str = "crates/ui/assets/theme.css";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    /// Paths relative to `COMPONENTS_DIR`.
    pub files: Vec<String>,
    /// Other registry items this one imports from.
    pub registry_dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    pub name: String,
    pub theme: String,
    /// Crates every copied component needs in the target `Cargo.toml`.
    pub cargo_dependencies: Vec<String>,
    pub items: Vec<Item>,
}

/// Names of the components a source file imports via `components::<name>`.
pub fn imports_in(source: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut rest = source;
    while let Some(i) = rest.find("components::") {
        rest = &rest[i + "components::".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_')
            .collect();
        if !name.is_empty() {
            found.insert(name);
        }
    }
    found
}

/// Scan `<root>/crates/ui/src/components` and build the registry.
pub fn build(root: &Path) -> io::Result<Registry> {
    let dir = root.join(COMPONENTS_DIR);
    let mut names: Vec<String> = fs::read_dir(&dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().join("mod.rs").is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    names.sort();
    let mut items = Vec::new();
    for name in &names {
        let mut files = Vec::new();
        let mut deps = BTreeSet::new();
        let mut entries: Vec<_> = fs::read_dir(dir.join(name))?
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .collect();
        entries.sort_by_key(|e| e.file_name());
        for e in entries {
            let file = e.file_name().to_string_lossy().into_owned();
            if file.ends_with(".rs") {
                deps.extend(imports_in(&fs::read_to_string(e.path())?));
            }
            files.push(format!("{name}/{file}"));
        }
        deps.remove(name);
        deps.retain(|d| names.contains(d));
        items.push(Item {
            name: name.clone(),
            files,
            registry_dependencies: deps.into_iter().collect(),
        });
    }
    Ok(Registry {
        name: "rust-ui".into(),
        theme: THEME_FILE.into(),
        cargo_dependencies: ["dioxus", "dioxus-primitives", "dioxus-icons"]
            .map(String::from)
            .to_vec(),
        items,
    })
}

pub fn to_json(reg: &Registry) -> String {
    let mut s = serde_json::to_string_pretty(reg).expect("registry serialises");
    s.push('\n');
    s
}

pub fn load(root: &Path) -> Result<Registry, String> {
    let path = root.join("registry.json");
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// The requested items plus everything they depend on, in a stable order.
pub fn resolve(reg: &Registry, wanted: &[String]) -> Result<Vec<Item>, String> {
    let mut out: Vec<Item> = Vec::new();
    let mut stack: Vec<String> = wanted.iter().rev().cloned().collect();
    while let Some(name) = stack.pop() {
        if out.iter().any(|i| i.name == name) {
            continue;
        }
        let item = reg
            .items
            .iter()
            .find(|i| i.name == name)
            .ok_or_else(|| format!("unknown component `{name}` (try `rust-ui list`)"))?;
        stack.extend(item.registry_dependencies.iter().rev().cloned());
        out.push(item.clone());
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileState {
    Missing,
    Same,
    Changed,
}

pub fn compare(source: &Path, dest: &Path) -> io::Result<FileState> {
    if !dest.exists() {
        return Ok(FileState::Missing);
    }
    Ok(if fs::read(source)? == fs::read(dest)? {
        FileState::Same
    } else {
        FileState::Changed
    })
}

/// Copy items into `dest`. Existing files that differ are kept unless `force`.
/// Returns (written, skipped) relative paths.
pub fn add(
    root: &Path,
    items: &[Item],
    dest: &Path,
    force: bool,
) -> io::Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let (mut written, mut skipped) = (Vec::new(), Vec::new());
    for item in items {
        for file in &item.files {
            let from = root.join(COMPONENTS_DIR).join(file);
            let to = dest.join(file);
            match compare(&from, &to)? {
                FileState::Same => continue,
                FileState::Changed if !force => {
                    skipped.push(PathBuf::from(file));
                    continue;
                }
                _ => {}
            }
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&from, &to)?;
            written.push(PathBuf::from(file));
        }
    }
    Ok((written, skipped))
}

/// `pub mod` lines for the items, to paste into the target's `components/mod.rs`.
pub fn mod_lines(items: &[Item]) -> String {
    items
        .iter()
        .map(|i| format!("pub mod {};\n", i.name))
        .collect()
}
