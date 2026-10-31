//! Wiring copied components into an app that already exists: `atoll init` and
//! `atoll add` call these so the app builds without hand edits.
//!
//! - [`ensure_mod_lines`] and [`ensure_root_mod`]: the `pub mod` lines in
//!   `src/components/mod.rs` and the `mod components;` line in `main.rs`/`lib.rs`.
//! - [`cargo_deps`] and [`add_cargo_deps`]: the crates the copied components use (read from
//!   each component's generated `component.json`, the same list `dx components add` uses),
//!   added to the app's `Cargo.toml` with its formatting and comments kept.
//! - [`dioxus_version`]: whether the app is on the Dioxus line the kit supports.
//! - [`clashes`]: CSS in the app that defines the kit's tokens or switches, which is what
//!   `--scoped` is for.
//!
//! Every edit is additive and idempotent: running a command twice changes nothing the
//! second time, and nothing the app already has is rewritten.

use crate::{Item, Kind};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use toml_edit::{value, Array, Document, InlineTable, Item as TomlItem};

/// The Dioxus minor line the kit is built and tested against (docs/GUARDRAILS.md).
pub const DIOXUS_LINE: &str = "0.7";

/// One crate a copied component needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dep {
    /// The key in `[dependencies]`.
    pub name: String,
    /// `package = "..."` when the key differs from the crate's name.
    pub package: Option<String>,
    pub version: Option<String>,
    pub git: Option<String>,
    pub rev: Option<String>,
    pub features: Vec<String>,
}

/// What [`add_cargo_deps`] did with one dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepChange {
    Added(String),
    /// Already there; the missing features were added.
    Features(String, Vec<String>),
    /// Already there from another source or revision; left as it is.
    Differs {
        name: String,
        ours: String,
    },
}

/// The union of the cargo dependencies of the components among `items`, from their
/// `component.json` files under `root` (blocks have none of their own: their components'
/// cover them).
pub fn cargo_deps(root: &Path, items: &[Item]) -> Result<Vec<Dep>, String> {
    let mut by_name: BTreeMap<String, Dep> = BTreeMap::new();
    for item in items.iter().filter(|i| i.kind == Kind::Component) {
        let path = root
            .join(item.kind.dir())
            .join(&item.name)
            .join("component.json");
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let json: Value =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        for d in json["cargoDependencies"].as_array().into_iter().flatten() {
            let s = |k: &str| d[k].as_str().map(str::to_string);
            let Some(name) = s("name") else { continue };
            let features: Vec<String> = d["features"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|f| f.as_str().map(str::to_string))
                .collect();
            let entry = by_name.entry(name.clone()).or_insert_with(|| Dep {
                name,
                package: s("package"),
                version: s("version"),
                git: s("git"),
                rev: s("rev"),
                features: Vec::new(),
            });
            for f in features {
                if !entry.features.contains(&f) {
                    entry.features.push(f);
                }
            }
        }
    }
    Ok(by_name.into_values().collect())
}

impl Dep {
    /// Where the dependency comes from, for messages: `git@rev` or the version.
    pub fn source(&self) -> String {
        match (&self.git, &self.rev, &self.version) {
            (Some(g), Some(r), _) => format!("{g} rev {}", &r[..r.len().min(7)]),
            (Some(g), None, _) => g.clone(),
            (None, _, Some(v)) => v.clone(),
            _ => "*".into(),
        }
    }

    fn toml(&self) -> TomlItem {
        let mut t = InlineTable::new();
        if let Some(p) = &self.package {
            t.insert("package", p.as_str().into());
        }
        if let Some(v) = &self.version {
            if self.git.is_none() && self.package.is_none() && self.features.is_empty() {
                return value(v.as_str());
            }
            t.insert("version", v.as_str().into());
        }
        if let Some(g) = &self.git {
            t.insert("git", g.as_str().into());
        }
        if let Some(r) = &self.rev {
            t.insert("rev", r.as_str().into());
        }
        if !self.features.is_empty() {
            let mut a = Array::new();
            for f in &self.features {
                a.push(f.as_str());
            }
            t.insert("features", a.into());
        }
        value(t)
    }
}

/// Add `deps` to the `[dependencies]` of the manifest at `cargo_toml`, keeping its layout.
/// A dependency the app already has is never rewritten: missing features are added, and a
/// different source or revision is reported, not changed.
pub fn add_cargo_deps(cargo_toml: &Path, deps: &[Dep]) -> Result<Vec<DepChange>, String> {
    let text =
        fs::read_to_string(cargo_toml).map_err(|e| format!("{}: {e}", cargo_toml.display()))?;
    let mut doc: Document = text
        .parse()
        .map_err(|e| format!("{}: {e}", cargo_toml.display()))?;
    if !doc.contains_key("dependencies") {
        doc["dependencies"] = toml_edit::table();
    }
    let table = doc["dependencies"]
        .as_table_like_mut()
        .ok_or_else(|| format!("{}: [dependencies] is not a table", cargo_toml.display()))?;
    let mut changes = Vec::new();
    for dep in deps {
        let Some(existing) = table.get_mut(&dep.name) else {
            table.insert(&dep.name, dep.toml());
            changes.push(DepChange::Added(dep.name.clone()));
            continue;
        };
        let get = |k: &str| {
            existing
                .as_table_like()
                .and_then(|t| t.get(k))
                .and_then(|v| v.as_str())
                .map(str::to_string)
        };
        let same_source = match (&dep.git, &dep.rev) {
            (Some(g), rev) => {
                get("git").as_deref() == Some(g.as_str()) && (rev.is_none() || get("rev") == *rev)
            }
            (None, _) => true,
        };
        if !same_source {
            changes.push(DepChange::Differs {
                name: dep.name.clone(),
                ours: dep.source(),
            });
            continue;
        }
        if dep.features.is_empty() {
            continue;
        }
        // A plain `name = "1"` becomes `{ version = "1", features = [...] }`.
        if let Some(v) = existing.as_str().map(str::to_string) {
            let mut t = InlineTable::new();
            t.insert("version", v.as_str().into());
            *existing = value(t);
        }
        let Some(t) = existing.as_table_like_mut() else {
            continue;
        };
        let have: Vec<String> = t
            .get("features")
            .and_then(|f| f.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let missing: Vec<String> = dep
            .features
            .iter()
            .filter(|f| !have.contains(f))
            .cloned()
            .collect();
        if missing.is_empty() {
            continue;
        }
        let mut a = Array::new();
        for f in have.iter().chain(&missing) {
            a.push(f.as_str());
        }
        t.insert("features", value(a));
        changes.push(DepChange::Features(dep.name.clone(), missing));
    }
    let out = doc.to_string();
    if out != text {
        fs::write(cargo_toml, out).map_err(|e| format!("{}: {e}", cargo_toml.display()))?;
    }
    Ok(changes)
}

/// Make sure `mod_rs` declares `pub mod <name>;` for each name, creating the file if needed.
/// Returns the names it added.
pub fn ensure_mod_lines(mod_rs: &Path, names: &[String]) -> io::Result<Vec<String>> {
    let mut text = fs::read_to_string(mod_rs).unwrap_or_default();
    let declared = |text: &str, name: &str| {
        text.lines().any(|l| {
            let l = l.trim();
            let l = l.strip_prefix("pub ").unwrap_or(l);
            let l = l.strip_prefix("pub(crate) ").unwrap_or(l);
            l == format!("mod {name};")
        })
    };
    let mut added = Vec::new();
    for name in names {
        if !declared(&text, name) {
            if !text.is_empty() && !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&format!("pub mod {name};\n"));
            added.push(name.clone());
        }
    }
    if !added.is_empty() {
        if let Some(parent) = mod_rs.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(mod_rs, text)?;
    }
    Ok(added)
}

/// Make sure the crate root under `app/src` (`main.rs`, else `lib.rs`) declares `mod
/// <module>;`. The line goes after the inner attributes and doc comments at the top.
/// Returns the file it changed, or `None` when the line was there or there is no root.
pub fn ensure_root_mod(app: &Path, module: &str) -> io::Result<Option<PathBuf>> {
    let Some(root) = ["src/main.rs", "src/lib.rs"]
        .iter()
        .map(|f| app.join(f))
        .find(|p| p.is_file())
    else {
        return Ok(None);
    };
    let text = fs::read_to_string(&root)?;
    let has = text.lines().any(|l| {
        let l = l.trim();
        ["mod ", "pub mod ", "pub(crate) mod "]
            .iter()
            .any(|p| l == format!("{p}{module};") || l.starts_with(&format!("{p}{module} {{")))
    });
    if has {
        return Ok(None);
    }
    let mut lines: Vec<&str> = text.lines().collect();
    // After the header: the leading inner doc comments and inner attributes.
    let at = lines
        .iter()
        .take_while(|l| {
            let l = l.trim_start();
            l.starts_with("//!") || l.starts_with("#![") || l.is_empty()
        })
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty())
        .map(|(i, _)| i + 1)
        .last()
        .unwrap_or(0);
    let line = format!("mod {module};");
    lines.insert(at, &line);
    let mut out = lines.join("\n");
    out.push('\n');
    fs::write(&root, out)?;
    Ok(Some(root))
}

/// The `dioxus` requirement in the manifest at `cargo_toml` (a plain version or the
/// `version` key), if any.
pub fn dioxus_version(cargo_toml: &Path) -> Option<String> {
    let doc: Document = fs::read_to_string(cargo_toml).ok()?.parse().ok()?;
    let req = |d: &TomlItem| {
        d.as_str().map(str::to_string).or_else(|| {
            d.as_table_like()?
                .get("version")?
                .as_str()
                .map(str::to_string)
        })
    };
    let workspace = doc
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.get("dioxus"))
        .and_then(req);
    // `dioxus = { workspace = true }` (or a workspace root): the workspace table has it.
    doc.get("dependencies")
        .and_then(|d| d.get("dioxus"))
        .and_then(req)
        .or(workspace)
}

/// Whether a version requirement is on [`DIOXUS_LINE`] (`0.7`, `0.7.10`, `^0.7`, `=0.7.3`, `~0.7`).
pub fn on_supported_line(req: &str) -> bool {
    let v = req.trim().trim_start_matches(['=', '^', '~', ' ']);
    v == DIOXUS_LINE || v.starts_with(&format!("{DIOXUS_LINE}."))
}

/// One place where the app's CSS defines something the kit's theme also defines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clash {
    pub file: PathBuf,
    pub line: usize,
    /// The token (`--radius`) or rule (`[data-theme]`, `.sr-only`).
    pub what: String,
}

/// Custom properties the kit's theme defines (`--name:` at the start of a declaration).
pub fn defined_tokens(css: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in css.lines() {
        let l = line.trim_start();
        if let Some(rest) = l.strip_prefix("--") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .collect();
            if !name.is_empty() && rest[name.len()..].trim_start().starts_with(':') {
                let token = format!("--{name}");
                if !out.contains(&token) {
                    out.push(token);
                }
            }
        }
    }
    out
}

/// Folders never scanned for the app's own CSS.
const SKIP_DIRS: [&str; 6] = ["target", "node_modules", ".git", "dist", ".atoll", "vendor"];

/// The app's CSS files under `app` (at most 6 levels deep), except `skip` (the kit's own
/// copies) and build output.
fn css_files(app: &Path, skip: &[PathBuf]) -> Vec<PathBuf> {
    fn walk(dir: &Path, depth: usize, skip: &[PathBuf], out: &mut Vec<PathBuf>) {
        if depth > 6 {
            return;
        }
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        let mut entries: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
        entries.sort();
        for p in entries {
            let name = p
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            if p.is_dir() {
                if !SKIP_DIRS.contains(&name.as_str()) {
                    walk(&p, depth + 1, skip, out);
                }
            } else if (name.ends_with(".css") || name.ends_with(".scss"))
                && !skip.iter().any(|s| same_file(s, &p))
            {
                out.push(p);
            }
        }
    }
    let mut out = Vec::new();
    walk(app, 0, skip, &mut out);
    out
}

fn same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

/// Places in the app's CSS (`.css`/`.scss` under `app`, minus `skip`) that define one of the
/// kit's tokens from `theme_css`, or the kit's page-wide switches and helpers
/// (`[data-theme`, `[data-accent`, `[data-density`, `.sr-only`). Each is something the
/// page-wide theme would fight with; the scoped theme keeps them apart.
pub fn clashes(app: &Path, theme_css: &str, skip: &[PathBuf]) -> Vec<Clash> {
    let tokens = defined_tokens(theme_css);
    let switches = ["[data-theme", "[data-accent", "[data-density", ".sr-only"];
    let mut out = Vec::new();
    for file in css_files(app, skip) {
        let Ok(text) = fs::read_to_string(&file) else {
            continue;
        };
        for (i, line) in text.lines().enumerate() {
            let l = line.trim_start();
            for t in &tokens {
                if l.strip_prefix(t.as_str())
                    .is_some_and(|r| r.trim_start().starts_with(':'))
                {
                    out.push(Clash {
                        file: file.clone(),
                        line: i + 1,
                        what: t.clone(),
                    });
                }
            }
            for s in switches {
                if l.contains(s) && !l.starts_with("/*") && !l.starts_with('*') {
                    out.push(Clash {
                        file: file.clone(),
                        line: i + 1,
                        what: s.trim_start_matches('[').to_string(),
                    });
                }
            }
        }
    }
    out
}

/// Whether the app's CSS uses Tailwind (its preflight is a reset of its own).
pub fn uses_tailwind(app: &Path) -> bool {
    app.join("tailwind.config.js").is_file()
        || app.join("tailwind.config.ts").is_file()
        || css_files(app, &[]).iter().any(|f| {
            fs::read_to_string(f)
                .is_ok_and(|t| t.contains("@tailwind") || t.contains("@import \"tailwindcss\""))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_line() {
        for ok in ["0.7", "0.7.10", "^0.7.1", "=0.7.10", "~0.7"] {
            assert!(on_supported_line(ok), "{ok}");
        }
        for bad in ["0.6.3", "0.8.0-alpha.1", "0.70", "1"] {
            assert!(!on_supported_line(bad), "{bad}");
        }
    }

    #[test]
    fn tokens_are_read_from_declarations_only() {
        let css =
            ":root {\n  --radius: 0.5rem;\n  --a-b: x; --c: y;\n  color: var(--not-defined);\n}";
        assert_eq!(defined_tokens(css), ["--radius", "--a-b"]);
    }
}
