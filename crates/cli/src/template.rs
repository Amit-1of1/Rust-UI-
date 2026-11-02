//! `atoll create <dir> --template <name>`: start a new app from one of the kit's own example
//! apps instead of the one-screen starter. The examples (`apps/dashboard`, `apps/booking`,
//! `apps/site`, `apps/showcase`) are built in CI, checked with axe in three browsers and used
//! to find the kit's gaps, so they are working, tested starting points rather than snippets.
//!
//! The app's files are copied from a checkout of the kit (`--path`, the `--source` checkout,
//! or a clone of the repository). Its `Cargo.toml` is rebuilt from the one `atoll create`
//! writes (same Dioxus pin, kit dependency and release profile) plus the example's own extra
//! dependencies and features, so nothing points back into the kit's workspace.
//! `scripts/template-check.sh` builds every template the way a user would.

use crate::create::{self, KitSource};
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{value, Array, Document, InlineTable, Item, Value};

/// One example app that can be used as a template.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Template {
    /// The name given to `--template`.
    pub name: &'static str,
    /// Its folder under `apps/` in the kit's repository.
    pub app: &'static str,
    /// One line for `--help` and the docs.
    pub summary: &'static str,
}

/// Every template, in the order the docs list them.
pub const TEMPLATES: &[Template] = &[
    Template {
        name: "dashboard",
        app: "dashboard",
        summary: "Admin dashboard: sidebar shell, stat cards, charts, sortable and filterable tables, edit sheet, settings form, Ctrl+K palette",
    },
    Template {
        name: "booking",
        app: "booking",
        summary: "Multi-page booking app: router, search and filters, detail pages, a long checkout form with validation, trips list",
    },
    Template {
        name: "marketing",
        app: "site",
        summary: "Marketing site: header, hero, features, pricing, FAQ, contact form, command palette and motion",
    },
    Template {
        name: "showcase",
        app: "showcase",
        summary: "One codebase for the web, desktop and phones (webview or the native Blitz renderer), with a phone layout",
    },
];

/// The template called `name`, or an error that lists the ones there are.
pub fn find(name: &str) -> Result<&'static Template, String> {
    TEMPLATES.iter().find(|t| t.name == name).ok_or_else(|| {
        let names: Vec<&str> = TEMPLATES.iter().map(|t| t.name).collect();
        format!(
            "unknown template `{name}`; the templates are {}",
            names.join(", ")
        )
    })
}

/// Files and folders of an example that a new app doesn't get: its manifest is rewritten,
/// and `GAPS.md` is the kit's own to-do list.
const SKIP: &[&str] = &["Cargo.toml", "GAPS.md", "target", "dist"];

/// The new app's `Cargo.toml`: what `atoll create` writes, plus the example's extra
/// dependencies, the extra features of `dioxus` and `ui` it turns on, and its own Cargo
/// features. With `patches` (the kit's root `Cargo.toml`), its `[patch]` section is copied
/// too, so a native build gets the same renderer fixes the kit is tested with.
pub fn cargo_toml(
    example: &str,
    name: &str,
    source: &KitSource,
    patches: Option<&str>,
) -> Result<String, String> {
    let base = create::files(name, source)
        .into_iter()
        .find(|(path, _)| *path == "Cargo.toml")
        .map(|(_, text)| text)
        .expect("create writes a Cargo.toml");
    let mut doc: Document = base
        .parse()
        .map_err(|e| format!("starter Cargo.toml: {e}"))?;
    let example: Document = example
        .parse()
        .map_err(|e| format!("example Cargo.toml: {e}"))?;

    if let Some(description) = example
        .get("package")
        .and_then(|p| p.get("description"))
        .and_then(Item::as_str)
    {
        // "..., built only from the kit, to find gaps (see GAPS.md)": keep the first part.
        let short = description
            .split(" built only")
            .next()
            .unwrap_or(description);
        doc["package"]["description"] = value(short.trim_end_matches([',', ':', ' ']));
    }

    if let Some(deps) = example.get("dependencies").and_then(Item::as_table_like) {
        for (key, dep) in deps.iter() {
            match key {
                "dioxus" | "ui" => add_features(&mut doc["dependencies"][key], dep),
                _ => {
                    if is_workspace(dep) {
                        return Err(format!(
                            "the example's `{key}` comes from the kit's workspace; a template can't use it"
                        ));
                    }
                    doc["dependencies"][key] = dep.clone();
                }
            }
        }
    }
    if let Some(target) = example.get("target") {
        doc["target"] = target.clone();
        if let Some(t) = doc["target"].as_table_mut() {
            t.set_implicit(true);
        }
    }
    if let Some(features) = example.get("features").and_then(Item::as_table_like) {
        for (key, list) in features.iter() {
            doc["features"][key] = list.clone();
        }
    }
    if example.get("lib").is_some() {
        doc["lib"] = example["lib"].clone();
    }

    let mut text = doc.to_string();
    if let Some(root) = patches {
        let root: Document = root.parse().map_err(|e| format!("kit Cargo.toml: {e}"))?;
        if let Some(patch) = root.get("patch") {
            let mut only = Document::new();
            only["patch"] = patch.clone();
            if let Some(t) = only["patch"].as_table_mut() {
                t.set_implicit(true);
            }
            text.push_str(
                "\n# The renderer fixes the kit is tested with (docs/FORKS.md in the kit's repository).\n# Only the native renderer uses these crates.\n",
            );
            text.push_str(&only.to_string());
        }
    }
    Ok(text)
}

fn is_workspace(dep: &Item) -> bool {
    dep.get("workspace").and_then(Item::as_bool) == Some(true)
}

/// Turn on the features `example` asks for in the starter's dependency `target`, keeping the
/// starter's version, git or path.
fn add_features(target: &mut Item, example: &Item) {
    let Some(features) = example.get("features").and_then(Item::as_array) else {
        return;
    };
    if features.is_empty() {
        return;
    }
    // `dioxus = "0.7.10"` becomes `dioxus = { version = "0.7.10", features = [...] }`.
    if let Some(version) = target.as_str().map(str::to_string) {
        let mut t = InlineTable::new();
        t.insert("version", Value::from(version));
        *target = Item::Value(Value::InlineTable(t));
    }
    let list: Array = features.iter().cloned().collect();
    match target {
        Item::Value(Value::InlineTable(t)) => {
            t.insert("features", Value::Array(list));
        }
        Item::Table(t) => {
            t.insert("features", value(list));
        }
        _ => {}
    }
}

/// Write the example `template` from the kit checkout `kit` into `dir` as the app `name`.
/// Fails, writing nothing, when `dir` already has a `Cargo.toml`.
pub fn create(
    dir: &Path,
    name: &str,
    source: &KitSource,
    template: &Template,
    kit: &Path,
) -> Result<Vec<PathBuf>, String> {
    create::validate_package_name(name)?;
    if dir.join("Cargo.toml").exists() {
        return Err(format!(
            "{} already has a Cargo.toml; to add the kit to an existing app use `atoll init` and `atoll add` (docs/EXISTING_APP.md)",
            dir.display()
        ));
    }
    let app = kit.join("apps").join(template.app);
    let read = |p: &Path| fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()));
    let example = read(&app.join("Cargo.toml")).map_err(|e| {
        format!("{e}\n(the template is copied from a checkout of the kit; pass --path <checkout> or --git <url>)")
    })?;
    // Only the showcase builds the native renderer, the one place the patches matter.
    let patches = if example.contains("dioxus/native") {
        Some(read(&kit.join("Cargo.toml"))?)
    } else {
        None
    };
    let cargo = cargo_toml(&example, name, source, patches.as_deref())?;

    let mut files = Vec::new();
    collect(&app, &app, &mut files).map_err(|e| format!("{}: {e}", app.display()))?;
    files.sort();
    let has_lib = app.join("src/lib.rs").exists();
    let old_ident = template.app.replace('-', "_");
    let new_ident = name.replace('-', "_");

    let mut written = Vec::new();
    let mut write = |rel: &Path, bytes: Vec<u8>| -> Result<(), String> {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        written.push(path);
        Ok(())
    };
    write(Path::new("Cargo.toml"), cargo.into_bytes())?;
    for rel in files {
        let from = app.join(&rel);
        let mut bytes = fs::read(&from).map_err(|e| format!("{}: {e}", from.display()))?;
        if rel == Path::new("Dioxus.toml") {
            let text = String::from_utf8_lossy(&bytes).replace(
                &format!("name = \"{}\"", template.app),
                &format!("name = \"{name}\""),
            );
            bytes = text.into_bytes();
        } else if has_lib && rel == Path::new("src/main.rs") && old_ident != new_ident {
            // The binary calls into the library by its crate name, which is now `name`.
            let text = String::from_utf8_lossy(&bytes)
                .replace(&format!("{old_ident}::"), &format!("{new_ident}::"));
            bytes = text.into_bytes();
        }
        write(&rel, bytes)?;
    }
    if !dir.join(".gitignore").exists() {
        write(Path::new(".gitignore"), b"/target\n/dist\n".to_vec())?;
    }
    Ok(written)
}

/// Every file under `dir`, relative to `root`, minus [`SKIP`].
fn collect(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        if dir == root && SKIP.iter().any(|s| name == *s) {
            continue;
        }
        if entry.file_type()?.is_dir() {
            collect(root, &path, out)?;
        } else {
            out.push(path.strip_prefix(root).expect("under root").to_path_buf());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kit() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn every_template_is_an_app_in_the_repo() {
        for t in TEMPLATES {
            assert!(
                kit().join("apps").join(t.app).join("Cargo.toml").exists(),
                "{}",
                t.name
            );
        }
        assert!(find("nope").unwrap_err().contains("dashboard, booking"));
    }

    #[test]
    fn the_docs_list_every_template() {
        // apps/docs/src/examples.rs can't depend on this crate, so check its list here.
        let page = fs::read_to_string(kit().join("apps/docs/src/examples.rs")).unwrap();
        let listed: Vec<&str> = page
            .lines()
            .filter_map(|l| l.trim().strip_prefix("template: \""))
            .filter_map(|l| l.strip_suffix("\","))
            .collect();
        let names: Vec<&str> = TEMPLATES.iter().map(|t| t.name).collect();
        assert_eq!(
            listed, names,
            "update EXAMPLES in apps/docs/src/examples.rs"
        );
        for t in TEMPLATES {
            assert!(page.contains(&format!("app: \"{}\"", t.app)), "{}", t.name);
        }
    }

    #[test]
    fn manifests_leave_the_workspace_behind() {
        let source = KitSource::Git {
            url: create::REPO.into(),
            rev: Some("abc123".into()),
            tag: None,
        };
        for t in TEMPLATES {
            let example =
                fs::read_to_string(kit().join("apps").join(t.app).join("Cargo.toml")).unwrap();
            let text = cargo_toml(&example, "my-app", &source, None).unwrap();
            assert!(!text.contains("workspace"), "{}:\n{text}", t.name);
            let doc: Document = text.parse().unwrap();
            assert_eq!(doc["package"]["name"].as_str(), Some("my-app"));
            let ui = &doc["dependencies"]["ui"];
            assert_eq!(ui["package"].as_str(), Some("atoll-ui"), "{}", t.name);
            assert_eq!(ui["rev"].as_str(), Some("abc123"));
            assert!(doc["features"]["web"].as_array().is_some(), "{}", t.name);
            assert!(doc["profile"]["release"]["lto"].as_bool() == Some(true));
        }
    }

    #[test]
    fn extra_features_and_dependencies_carry_over() {
        let example = fs::read_to_string(kit().join("apps/booking/Cargo.toml")).unwrap();
        let text = cargo_toml(&example, "trips", &KitSource::Registry, None).unwrap();
        let doc: Document = text.parse().unwrap();
        let dioxus = &doc["dependencies"]["dioxus"];
        assert!(dioxus["version"]
            .as_str()
            .is_some_and(|v| v.contains("0.8")));
        let has = |item: &Item, f: &str| {
            item["features"]
                .as_array()
                .is_some_and(|a| a.iter().any(|v| v.as_str() == Some(f)))
        };
        assert!(has(dioxus, "router"), "{text}");
        assert!(has(&doc["dependencies"]["ui"], "router"), "{text}");
        assert!(doc["dependencies"]["time"].is_value() || doc["dependencies"]["time"].is_table());
        assert!(text.contains("[target.'cfg(target_arch = \"wasm32\")'.dependencies]"));
        assert!(doc["package"]["description"]
            .as_str()
            .is_some_and(|d| !d.contains("GAPS")));
    }

    #[test]
    fn writes_a_whole_app_and_renames_it() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("phone-app");
        let t = find("showcase").unwrap();
        let written = create(&dir, "phone-app", &KitSource::Path(kit()), t, &kit()).unwrap();
        assert!(written.iter().any(|p| p.ends_with("src/lib.rs")));
        assert!(dir.join("assets/showcase.css").exists());
        let main = fs::read_to_string(dir.join("src/main.rs")).unwrap();
        assert!(main.contains("phone_app::App") && !main.contains("showcase::App"));
        let dioxus = fs::read_to_string(dir.join("Dioxus.toml")).unwrap();
        assert!(dioxus.contains("name = \"phone-app\""));
        // Dioxus 0.8 branch: the native renderer forks are 0.7-only and the kit's root has no
        // patches, so none come along (on `main` they do, pinned like the kit's).
        let cargo = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
        assert!(
            cargo.parse::<Document>().is_ok() && !cargo.contains("blitz"),
            "{cargo}"
        );
        // A second run refuses to overwrite the app.
        assert!(create(&dir, "phone-app", &KitSource::Registry, t, &kit()).is_err());
    }

    #[test]
    fn gaps_lists_stay_in_the_kit() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("admin");
        create(
            &dir,
            "admin",
            &KitSource::Registry,
            find("dashboard").unwrap(),
            &kit(),
        )
        .unwrap();
        assert!(!dir.join("GAPS.md").exists());
        assert!(dir.join("src/main.rs").exists() && dir.join(".gitignore").exists());
        let cargo = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
        assert!(!cargo.contains("[patch"), "{cargo}");
    }
}
