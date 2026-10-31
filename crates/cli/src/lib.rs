//! Registry logic for the `rust-ui` CLI.
//!
//! The registry is shaped like shadcn's: one item per component with its files and
//! the other registry items it needs. It is generated from the component folders,
//! so it can't drift from the source.
//!
//! Internal: this library is the `rust-ui` binary's implementation, public for the repo's
//! tests, and may change in any release. The commands are the supported interface; which of
//! them are stable is in docs/STABILITY.md.
#![forbid(unsafe_code)]

pub mod app;
pub mod changelog;
pub mod contract;
#[cfg(test)]
mod contract_tests;
pub mod contrast;
pub mod docsgen;
pub mod doctor;
pub mod explain;
pub mod info;
pub mod llms;
pub mod manifest;
pub mod mcp;
pub mod postbuild;
pub mod scaffold;
pub mod scope;
pub mod theme;
pub mod update;

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// This CLI's version, as `rust-ui --version` prints it.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const COMPONENTS_DIR: &str = "crates/ui/src/components";
/// Whole screens built from components (`login_01`, ...). `rust-ui add` copies them to
/// `src/blocks/` and their components to `src/components/`.
pub const BLOCKS_DIR: &str = "crates/ui/src/blocks";
pub const DEMOS_DIR: &str = "crates/ui/src/demos";
pub const THEME_FILE: &str = "crates/ui/assets/theme.css";

/// What a registry item is. Only blocks say so in `registry.json`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    #[default]
    Component,
    /// A whole screen built from components.
    Block,
}

impl Kind {
    pub fn is_component(&self) -> bool {
        *self == Kind::Component
    }

    /// Where this kind of item lives in the repo.
    pub fn dir(self) -> &'static str {
        match self {
            Kind::Component => COMPONENTS_DIR,
            Kind::Block => BLOCKS_DIR,
        }
    }
}

/// The file that makes an item Pro: it sits in the item's folder, so it is copied along with
/// the item and the licence stays next to the code (docs/pro/README.md).
pub const PRO_LICENSE_FILE: &str = "LICENSE-PRO.md";

/// Whether an item is free (MIT OR Apache-2.0, like the kit) or Pro (sold under a commercial
/// licence from a private registry). Only Pro items say so in `registry.json`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    #[default]
    Free,
    Pro,
}

impl Tier {
    pub fn is_free(&self) -> bool {
        *self == Tier::Free
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    #[serde(default, skip_serializing_if = "Kind::is_component")]
    pub kind: Kind,
    #[serde(default, skip_serializing_if = "Tier::is_free")]
    pub tier: Tier,
    /// Paths relative to the kind's folder (`COMPONENTS_DIR` or `BLOCKS_DIR`).
    pub files: Vec<String>,
    /// Other registry items this one imports from (always components).
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
    // `components::<name>` and sibling imports like `super::super::<name>`.
    for marker in ["components::", "super::super::"] {
        let mut rest = source;
        while let Some(i) = rest.find(marker) {
            rest = &rest[i + marker.len()..];
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_')
                .collect();
            if !name.is_empty() {
                found.insert(name);
            }
        }
    }
    found
}

/// Item folders (those with a `mod.rs`) under `dir`, sorted; none when `dir` is missing.
fn folders(dir: &Path) -> io::Result<Vec<String>> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().join("mod.rs").is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    names.sort();
    Ok(names)
}

/// One item: its files (without `component.json`) and the components its sources import.
fn scan(dir: &Path, name: &str, kind: Kind, components: &[String]) -> io::Result<Item> {
    let mut files = Vec::new();
    let mut deps = BTreeSet::new();
    let mut tier = Tier::Free;
    let mut entries: Vec<_> = fs::read_dir(dir.join(name))?
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let file = e.file_name().to_string_lossy().into_owned();
        if file.ends_with(".rs") {
            deps.extend(imports_in(&fs::read_to_string(e.path())?));
        }
        if file == PRO_LICENSE_FILE {
            tier = Tier::Pro;
        }
        if file != "component.json" {
            files.push(format!("{name}/{file}"));
        }
    }
    deps.remove(name);
    deps.retain(|d| components.contains(d));
    Ok(Item {
        name: name.to_string(),
        kind,
        tier,
        files,
        registry_dependencies: deps.into_iter().collect(),
    })
}

/// Scan `<root>/crates/ui/src/components` and `<root>/crates/ui/src/blocks` and build the
/// registry: components first, then blocks.
pub fn build(root: &Path) -> io::Result<Registry> {
    let dir = root.join(COMPONENTS_DIR);
    let names = folders(&dir)?;
    let mut items = Vec::new();
    for name in &names {
        items.push(scan(&dir, name, Kind::Component, &names)?);
    }
    let blocks = root.join(BLOCKS_DIR);
    for name in folders(&blocks)? {
        if names.contains(&name) {
            return Err(io::Error::other(format!(
                "block `{name}` has the same name as a component"
            )));
        }
        items.push(scan(&blocks, &name, Kind::Block, &names)?);
    }
    Ok(Registry {
        name: "rust-ui".into(),
        theme: THEME_FILE.into(),
        cargo_dependencies: ["dioxus", "dioxus-primitives"].map(String::from).to_vec(),
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
    let reg: Registry =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    validate(&reg).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(reg)
}

/// Whether `name` can be an item: a Rust module name, so it is also a plain folder name.
pub fn is_item_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('a'..='z' | '_'))
        && chars.all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_'))
}

/// Whether `file` is `<name>/<plain file name>`, the only shape an item's file can have.
pub fn is_item_file(name: &str, file: &str) -> bool {
    file.split_once('/').is_some_and(|(dir, f)| {
        dir == name && !f.is_empty() && f != "." && f != ".." && !f.contains(['/', '\\', ':', '\0'])
    })
}

/// Whether `dir` is a relative path that stays inside the folder it is joined onto.
pub fn is_inside(dir: &Path) -> bool {
    dir.components().all(|c| {
        matches!(
            c,
            std::path::Component::Normal(_) | std::path::Component::CurDir
        )
    })
}

/// Refuse a registry whose names or paths would leave the item folders.
///
/// A `--source` can be any git URL, so its `registry.json` is untrusted input: `add` and the
/// MCP server join its file paths onto the app's folders and write `pub mod <name>;` lines
/// into its source. Every file must be `<item name>/<file name>` and every name a module name.
pub fn validate(reg: &Registry) -> Result<(), String> {
    for item in &reg.items {
        if !is_item_name(&item.name) {
            return Err(format!("`{}` is not a valid item name", item.name));
        }
        for dep in &item.registry_dependencies {
            if !is_item_name(dep) {
                return Err(format!(
                    "`{}` depends on `{dep}`, not a valid item name",
                    item.name
                ));
            }
        }
        for file in &item.files {
            if !is_item_file(&item.name, file) {
                return Err(format!(
                    "`{}` lists `{file}`; files must be `{}/<file name>`",
                    item.name, item.name
                ));
            }
        }
    }
    Ok(())
}

/// The requested items plus everything they depend on, in a stable order.
pub fn resolve(reg: &Registry, wanted: &[String]) -> Result<Vec<Item>, String> {
    let mut out: Vec<Item> = Vec::new();
    let mut stack: Vec<String> = wanted.iter().rev().cloned().collect();
    while let Some(name) = stack.pop() {
        if out.iter().any(|i| i.name == name) {
            continue;
        }
        let item =
            reg.items.iter().find(|i| i.name == name).ok_or_else(|| {
                format!("unknown component or block `{name}` (try `rust-ui list`)")
            })?;
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

/// The Pro items among `items`, for the licence reminder `add` prints.
pub fn pro_items(items: &[Item]) -> Vec<&Item> {
    items.iter().filter(|i| i.tier == Tier::Pro).collect()
}

/// What `add` prints after copying Pro items: which ones, and where their licence is.
pub fn pro_notice(items: &[Item]) -> Option<String> {
    let pro = pro_items(items);
    if pro.is_empty() {
        return None;
    }
    let names = pro
        .iter()
        .map(|i| i.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "Pro: {names} {} under the commercial licence in each folder's {PRO_LICENSE_FILE} \
         (one licence per developer; keep the file with the code). The rest of the kit is \
         MIT OR Apache-2.0.",
        if pro.len() == 1 { "is" } else { "are" }
    ))
}

/// Copy items into `dest`, and blocks into a `blocks` folder beside it (`src/components` ->
/// `src/blocks`). Existing files that differ are kept unless `force`.
/// Returns (written, skipped) relative paths.
pub fn add(
    root: &Path,
    items: &[Item],
    dest: &Path,
    force: bool,
) -> io::Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    add_to(root, items, dest, &dest.with_file_name("blocks"), force)
}

/// Copy components into `dest` and blocks into `blocks_dest`. Existing files that differ are
/// kept unless `force`. Returns (written, skipped) paths, relative to their destination.
pub fn add_to(
    root: &Path,
    items: &[Item],
    dest: &Path,
    blocks_dest: &Path,
    force: bool,
) -> io::Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let (mut written, mut skipped) = (Vec::new(), Vec::new());
    for item in items {
        let target = match item.kind {
            Kind::Component => dest,
            Kind::Block => blocks_dest,
        };
        for file in &item.files {
            let from = root.join(item.kind.dir()).join(file);
            let to = target.join(file);
            // A link in a remote checkout could point anywhere (`~/.ssh/...`); copy only
            // regular files, and never write through a link the app already has.
            refuse_links(&from)?;
            refuse_links(&to)?;
            if let Some(dir) = to.parent() {
                refuse_links(dir)?;
            }
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

/// An error if `path` exists and is a symbolic link.
pub fn refuse_links(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(m) if m.file_type().is_symlink() => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is a symbolic link; refusing to copy it", path.display()),
        )),
        _ => Ok(()),
    }
}

/// `pub mod` lines for the items of one kind, to paste into the target's `components/mod.rs`
/// (or `blocks/mod.rs`).
pub fn mod_lines_of(items: &[Item], kind: Kind) -> String {
    items
        .iter()
        .filter(|i| i.kind == kind)
        .map(|i| format!("pub mod {};\n", i.name))
        .collect()
}

/// `pub mod` lines for the components among the items, to paste into `components/mod.rs`.
pub fn mod_lines(items: &[Item]) -> String {
    mod_lines_of(items, Kind::Component)
}

/// Whether a `--source` value is a git remote rather than a local checkout.
pub fn is_remote(source: &str) -> bool {
    ["https://", "http://", "ssh://", "git@", "file://"]
        .iter()
        .any(|p| source.starts_with(p))
}

/// A stable, filesystem-safe folder name for a remote's cache.
pub fn cache_name(source: &str) -> String {
    source
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

/// Make `source` usable as a local checkout: local paths are returned as they are; remotes are
/// shallow-cloned into `cache_root` (and fast-forwarded if already there) with the `git` command.
pub fn materialize(source: &str, cache_root: &Path) -> Result<PathBuf, String> {
    if !is_remote(source) {
        return Ok(PathBuf::from(source));
    }
    let dir = cache_root.join(cache_name(source));
    let git = |args: &[&str], cwd: Option<&Path>| -> Result<(), String> {
        let mut cmd = std::process::Command::new("git");
        cmd.args(args);
        if let Some(cwd) = cwd {
            cmd.current_dir(cwd);
        }
        let out = cmd
            .output()
            .map_err(|e| format!("could not run git: {e}"))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(format!(
                "git {} failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr).trim()
            ))
        }
    };
    if dir.join(".git").exists() {
        // Offline or diverged: keep using what is cached.
        let _ = git(&["pull", "--ff-only", "--depth", "1", "-q"], Some(&dir));
    } else {
        fs::create_dir_all(cache_root).map_err(|e| e.to_string())?;
        let target = dir.to_string_lossy().into_owned();
        // `--` keeps a source from being read as an option; only the URL schemes in
        // `is_remote` get here, so `ext::` and other command-running transports never do.
        git(
            &["clone", "--depth", "1", "-q", "--", source, &target],
            None,
        )?;
    }
    Ok(dir)
}
