//! `rust-ui info`: what a coding agent needs to know about the app it is working in, read
//! from the app itself (no network): the Dioxus version, the theme, which kit items are
//! installed from where, which of them you edited, and whether the skill and MCP server are
//! set up. The agent skill (`skills/rust-ui/SKILL.md`) runs `rust-ui info --json` first.

use crate::update::{hash, Lock};
use crate::Kind;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Where `rust-ui init` puts the theme by default.
pub const THEME_DEST: &str = "assets/theme.css";
/// The skill in this repo, and where `rust-ui skill` installs it in an app.
pub const SKILL_FILE: &str = "skills/rust-ui/SKILL.md";
pub const SKILL_DEST: &str = ".claude/skills/rust-ui/SKILL.md";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub cli_version: &'static str,
    /// The `dioxus` requirement in the app's Cargo.toml (`0.7`, or `git <url>`), if any.
    pub dioxus: Option<String>,
    /// The app depends on the `rust-ui-kit` crate instead of (or as well as) copied source.
    pub kit_crate: bool,
    pub theme: Option<Theme>,
    /// The `--source` the lockfile recorded.
    pub source: Option<String>,
    pub lockfile: Option<String>,
    pub installed: Vec<Installed>,
    /// Path of the installed skill, if any.
    pub skill: Option<String>,
    /// `.mcp.json` (or `.cursor/mcp.json`, `.vscode/mcp.json`) runs `rust-ui ... mcp`.
    pub mcp: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub path: String,
    /// The `data-accent` presets the stylesheet defines.
    pub accents: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Installed {
    pub name: String,
    pub kind: Kind,
    pub dir: String,
    pub rev: Option<String>,
    /// Files whose content no longer matches what `add`/`update` recorded.
    pub edited: Vec<String>,
    /// Recorded files that are gone from the app.
    pub missing: Vec<String>,
    /// Files still holding `<<<<<<< yours` markers from `rust-ui update`.
    pub conflicts: Vec<String>,
}

/// Inspect the app in `project`; `lock` is the lockfile path as given on the command line.
pub fn gather(project: &Path, lock: &Path) -> Result<Info, String> {
    let lock_path = if lock.is_absolute() {
        lock.to_path_buf()
    } else {
        project.join(lock)
    };
    let locked = Lock::load(&lock_path)?;
    let cargo = fs::read_to_string(project.join("Cargo.toml"))
        .ok()
        .and_then(|s| s.parse::<toml::Table>().ok());
    let deps = cargo
        .as_ref()
        .and_then(|t| t.get("dependencies"))
        .and_then(|d| d.as_table());
    let dioxus = deps.and_then(|d| d.get("dioxus")).map(requirement);
    let kit_crate = deps.is_some_and(|d| d.contains_key("rust-ui-kit") || d.contains_key("ui"));

    let theme = fs::read_to_string(project.join(THEME_DEST))
        .ok()
        .map(|css| Theme {
            path: THEME_DEST.into(),
            accents: accents(&css),
        });

    let mut installed = Vec::new();
    for (name, item) in &locked.items {
        let dir = project.join(&item.dir);
        let (mut edited, mut missing, mut conflicts) = (Vec::new(), Vec::new(), Vec::new());
        for (file, recorded) in &item.files {
            match fs::read(dir.join(file)) {
                Err(_) => missing.push(file.clone()),
                Ok(bytes) => {
                    if String::from_utf8_lossy(&bytes).contains(crate::update::CONFLICT_START) {
                        conflicts.push(file.clone());
                    } else if &hash(&bytes) != recorded {
                        edited.push(file.clone());
                    }
                }
            }
        }
        installed.push(Installed {
            name: name.clone(),
            kind: item.kind,
            dir: item.dir.clone(),
            rev: item.rev.clone(),
            edited,
            missing,
            conflicts,
        });
    }

    let mcp = [".mcp.json", ".cursor/mcp.json", ".vscode/mcp.json"]
        .iter()
        .filter_map(|f| fs::read_to_string(project.join(f)).ok())
        .any(|s| s.contains("rust-ui") && s.contains("\"mcp\""));

    Ok(Info {
        cli_version: crate::VERSION,
        dioxus,
        kit_crate,
        theme,
        source: locked.source,
        lockfile: lock_path.exists().then(|| lock.display().to_string()),
        installed,
        skill: project
            .join(SKILL_DEST)
            .exists()
            .then(|| SKILL_DEST.to_string()),
        mcp,
    })
}

fn requirement(dep: &toml::Value) -> String {
    match dep {
        toml::Value::String(v) => v.clone(),
        toml::Value::Table(t) => match (t.get("version"), t.get("git")) {
            (Some(v), _) => v.as_str().unwrap_or_default().to_string(),
            (None, Some(g)) => format!("git {}", g.as_str().unwrap_or_default()),
            _ => "path or workspace".into(),
        },
        _ => String::new(),
    }
}

/// `[data-accent="x"]` selectors in the theme, each name once, in order.
fn accents(css: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for part in css.split("[data-accent=\"").skip(1) {
        if let Some(name) = part.split('"').next() {
            if !name.is_empty() && !out.iter().any(|n| n == name) {
                out.push(name.to_string());
            }
        }
    }
    out
}

/// The human-readable report.
pub fn render(info: &Info) -> String {
    let mut s = format!("rust-ui {}\n", info.cli_version);
    let or_none = |v: &Option<String>| v.clone().unwrap_or_else(|| "not found".into());
    s += &format!("dioxus:    {}\n", or_none(&info.dioxus));
    s += &format!(
        "theme:     {}\n",
        match &info.theme {
            Some(t) if t.accents.is_empty() => t.path.clone(),
            Some(t) => format!("{} (accents: {})", t.path, t.accents.join(", ")),
            None => format!("not found (run `rust-ui init` to copy it to {THEME_DEST})"),
        }
    );
    s += &format!("source:    {}\n", or_none(&info.source));
    s += &format!("skill:     {}\n", or_none(&info.skill));
    s += &format!(
        "mcp:       {}\n",
        if info.mcp {
            "configured"
        } else {
            "not configured (docs/MCP.md)"
        }
    );
    if info.installed.is_empty() {
        s += "installed: nothing yet (`rust-ui add <name>`)\n";
    } else {
        s += &format!("installed: {} item(s)\n", info.installed.len());
        for item in &info.installed {
            let kind = match item.kind {
                Kind::Block => " (block)",
                Kind::Component => "",
            };
            let mut notes = Vec::new();
            if !item.edited.is_empty() {
                notes.push(format!("edited: {}", item.edited.join(", ")));
            }
            if !item.conflicts.is_empty() {
                notes.push(format!("CONFLICTS: {}", item.conflicts.join(", ")));
            }
            if !item.missing.is_empty() {
                notes.push(format!("missing: {}", item.missing.join(", ")));
            }
            let notes = if notes.is_empty() {
                String::new()
            } else {
                format!("  [{}]", notes.join("; "))
            };
            s += &format!("  {}{kind} in {}{notes}\n", item.name, item.dir);
        }
    }
    s
}
