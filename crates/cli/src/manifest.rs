//! `component.json` manifests so the registry also works with Dioxus' own installer:
//! `dx components add button --git <repo-url>`. One manifest per component folder plus a
//! root manifest that lists them. They are generated, like `registry.json`, and a test
//! fails when they are stale.

use crate::{build, COMPONENTS_DIR, THEME_FILE};
use serde_json::{json, Value};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const PRIMITIVES_GIT: &str = "https://github.com/DioxusLabs/components";
/// Where this registry lives. `dx` resolves a plain string in `componentDependencies` against
/// its own built-in registry, so our siblings must be named with this URL. The motion crate
/// (needed by drawer) lives here too. Override with `RUST_UI_REGISTRY_GIT` to test a fork.
const REGISTRY_GIT: &str = "https://github.com/Amit-1of1/Rust-UI-";

fn registry_git() -> String {
    std::env::var("RUST_UI_REGISTRY_GIT").unwrap_or_else(|_| REGISTRY_GIT.to_string())
}

/// The pinned `dioxus-primitives` revision, read from the workspace manifest.
fn primitives_rev(root: &Path) -> io::Result<String> {
    let text = fs::read_to_string(root.join("Cargo.toml"))?;
    text.lines()
        .find(|l| l.trim_start().starts_with("dioxus-primitives"))
        .and_then(|l| l.split("rev = \"").nth(1))
        .and_then(|r| r.split('"').next())
        .map(str::to_string)
        .ok_or_else(|| io::Error::other("dioxus-primitives rev not found in Cargo.toml"))
}

/// First paragraph after the title in `docs.md`, on one line.
pub fn description(docs: &str) -> String {
    docs.lines()
        .skip_while(|l| l.trim().is_empty() || l.starts_with('#'))
        .take_while(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Crates a component's source uses, as `dx` cargo dependencies.
fn cargo_dependencies(source: &str, rev: &str, registry: &str) -> Vec<Value> {
    // `krate::` as a crate root, not as a path segment like `components::time::`.
    let uses = |krate: &str| {
        let pat = format!("{krate}::");
        source.match_indices(&pat).any(|(i, _)| {
            !source[..i]
                .chars()
                .next_back()
                .is_some_and(|c| c == ':' || c == '_' || c.is_alphanumeric())
        })
    };
    let mut deps = Vec::new();
    if uses("dioxus_primitives") {
        // `navbar` only exists with the `router` feature.
        let features: &[&str] = if source.contains("dioxus_primitives::navbar") {
            &["router"]
        } else {
            &[]
        };
        deps.push(json!({"name": "dioxus-primitives", "git": PRIMITIVES_GIT, "rev": rev, "features": features}));
    }
    if uses("dioxus_icons") {
        deps.push(json!({"name": "dioxus-icons", "version": "0.1.0"}));
    }
    if uses("time") {
        deps.push(json!({"name": "time", "version": "0.3.44", "features": ["std", "macros"]}));
    }
    if uses("palette") {
        deps.push(json!({"name": "palette", "version": "0.7.6"}));
    }
    if uses("strum") {
        deps.push(json!({"name": "strum", "version": "0.27.2", "features": ["derive"]}));
    }
    if uses("tracing") {
        deps.push(json!({"name": "tracing", "version": "0.1"}));
    }
    if uses("web_sys") {
        deps.push(json!({"name": "web-sys", "version": "0.3", "features": ["Window", "Document", "Element", "HtmlElement"]}));
    }
    if uses("wasm_bindgen") {
        deps.push(json!({"name": "wasm-bindgen", "version": "0.2"}));
    }
    if uses("motion") {
        deps.push(json!({"name": "rust-ui-motion", "git": registry}));
    }
    deps
}

/// Every manifest as `(path relative to root, contents)`: one per component and the root.
pub fn build_manifests(root: &Path) -> io::Result<Vec<(PathBuf, String)>> {
    let rev = primitives_rev(root)?;
    let git = registry_git();
    let reg = build(root)?;
    let mut out = Vec::new();
    let mut members = Vec::new();
    for item in &reg.items {
        let dir = Path::new(COMPONENTS_DIR).join(&item.name);
        let mut source = String::new();
        for f in &item.files {
            if f.ends_with(".rs") {
                source.push_str(&fs::read_to_string(root.join(COMPONENTS_DIR).join(f))?);
            }
        }
        let docs = fs::read_to_string(root.join(&dir).join("docs.md")).unwrap_or_default();
        // `crates/ui/src/components/<name>/../../../assets/theme.css`
        let theme =
            Path::new("../../..").join(Path::new(THEME_FILE).strip_prefix("crates/ui").unwrap());
        let manifest = json!({
            "name": item.name,
            "description": description(&docs),
            "exclude": ["docs.md", "component.json"],
            "cargoDependencies": cargo_dependencies(&source, &rev, &git),
            "componentDependencies": item.registry_dependencies.iter().map(|d| json!({"name": d, "git": git})).collect::<Vec<_>>(),
            "globalAssets": [theme.to_string_lossy()],
        });
        out.push((dir.join("component.json"), pretty(&manifest)));
        members.push(dir.to_string_lossy().into_owned());
    }
    let root_manifest = json!({
        "name": "rust-ui",
        "description": "Accessible, animated components for Dioxus",
        "members": members,
    });
    out.push((PathBuf::from("component.json"), pretty(&root_manifest)));
    Ok(out)
}

fn pretty(v: &Value) -> String {
    let mut s = serde_json::to_string_pretty(v).expect("manifest serialises");
    s.push('\n');
    s
}

/// Write every manifest.
pub fn write_manifests(root: &Path) -> io::Result<usize> {
    let all = build_manifests(root)?;
    for (path, text) in &all {
        fs::write(root.join(path), text)?;
    }
    Ok(all.len())
}

/// Paths whose committed manifest differs from the generated one.
pub fn stale_manifests(root: &Path) -> io::Result<Vec<PathBuf>> {
    Ok(build_manifests(root)?
        .into_iter()
        .filter(|(path, text)| fs::read_to_string(root.join(path)).ok().as_deref() != Some(text))
        .map(|(path, _)| path)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_is_the_first_paragraph() {
        let docs = "# Callout\n\nAn inline note.\nSecond line.\n\n## Accessibility\n";
        assert_eq!(description(docs), "An inline note. Second line.");
        assert_eq!(description("# Only a title\n"), "");
    }

    #[test]
    fn dependencies_follow_the_imports() {
        let src = "use dioxus_primitives::x;\nuse dioxus_icons::y;\nuse motion::z;";
        let names: Vec<_> = cargo_dependencies(src, "abc", "g")
            .iter()
            .map(|d| d["name"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(
            names,
            ["dioxus-primitives", "dioxus-icons", "rust-ui-motion"]
        );
        assert!(cargo_dependencies("use dioxus::prelude::*;", "abc", "g").is_empty());
        assert!(cargo_dependencies("use crate::components::time::Clock;", "abc", "g").is_empty());
    }
}
