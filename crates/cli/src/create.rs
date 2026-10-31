//! `atoll create <dir>`: a new Dioxus app that already uses the kit, so the getting-started
//! guide (docs/GETTING_STARTED.md) is three commands: install the CLI, create, `dx serve`.
//!
//! It writes a `Cargo.toml` (Dioxus 0.7 with `web` and `desktop` features, the kit as
//! `ui`), `Dioxus.toml`, `src/main.rs` with a small screen built from kit components,
//! and a `.gitignore`. It refuses to write into a folder that already has a `Cargo.toml`.
//! `scripts/getting-started-check.sh` builds what it writes.

use std::fs;
use std::path::{Path, PathBuf};

/// The repository the git dependency points at by default.
pub const REPO: &str = "https://github.com/Amit-1of1/Rust-UI-";
/// The branch releases come from. Written into the dependency for [`REPO`] when no rev or tag
/// is given, so the app doesn't depend on which branch GitHub serves by default.
pub const REPO_BRANCH: &str = "main";

/// Where the generated app gets the kit from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KitSource {
    /// crates.io, at this CLI's version (the default once the kit is published).
    Registry,
    /// A git repository, pinned to `rev` (or a `tag`) when given.
    Git {
        url: String,
        rev: Option<String>,
        tag: Option<String>,
    },
    /// A local checkout of this repository (for testing the kit itself).
    Path(PathBuf),
}

/// `my-app` or `my_app`: what Cargo accepts as a package name, kept simple.
pub fn validate_package_name(name: &str) -> Result<(), String> {
    let ok = name.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_');
    if ok {
        Ok(())
    } else {
        Err(format!(
            "`{name}` is not a valid package name: use lowercase letters, digits, `-` and `_`, starting with a letter (or pass --name)"
        ))
    }
}

fn kit_dep(source: &KitSource) -> String {
    let version = env!("CARGO_PKG_VERSION");
    match source {
        KitSource::Registry => {
            format!("ui = {{ package = \"atoll-ui\", version = \"{version}\" }}")
        }
        KitSource::Git { url, rev, tag } => {
            let pin = match (rev, tag) {
                (Some(rev), _) => format!(", rev = \"{rev}\""),
                (None, Some(tag)) => format!(", tag = \"{tag}\""),
                (None, None) if url == REPO => format!(", branch = \"{REPO_BRANCH}\""),
                (None, None) => String::new(),
            };
            format!("ui = {{ package = \"atoll-ui\", git = \"{url}\"{pin} }}")
        }
        KitSource::Path(p) => {
            let p = p.join("crates/ui");
            format!(
                "ui = {{ package = \"atoll-ui\", path = \"{}\" }}",
                p.display().to_string().replace('\\', "/")
            )
        }
    }
}

/// Every file `create` writes, as (path relative to the app, contents).
pub fn files(name: &str, source: &KitSource) -> Vec<(&'static str, String)> {
    let title = name.replace(['-', '_'], " ");
    let cargo = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
# The kit supports one Dioxus minor at a time (0.7 now); stay on it.
dioxus = "0.7.10"
{kit}

[features]
default = ["web"]
web = ["dioxus/web", "ui/web"]
desktop = ["dioxus/desktop", "ui/desktop"]

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
"#,
        kit = kit_dep(source),
    );
    let dioxus_toml = format!(
        r#"[application]
name = "{name}"

[web.app]
title = "{title}"
"#
    );
    let main_rs = MAIN_RS.replace("{title}", &title);
    let gitignore = "/target\n/dist\n".to_string();
    vec![
        ("Cargo.toml", cargo),
        ("Dioxus.toml", dioxus_toml),
        ("src/main.rs", main_rs),
        (".gitignore", gitignore),
    ]
}

/// Write the app into `dir`. Fails when `dir` already holds a `Cargo.toml`.
pub fn create(dir: &Path, name: &str, source: &KitSource) -> Result<Vec<PathBuf>, String> {
    validate_package_name(name)?;
    if dir.join("Cargo.toml").exists() {
        return Err(format!(
            "{} already has a Cargo.toml; to add the kit to an existing app use `atoll init` and `atoll add` (docs/EXISTING_APP.md)",
            dir.display()
        ));
    }
    let mut written = Vec::new();
    for (rel, text) in files(name, source) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        written.push(path);
    }
    Ok(written)
}

/// The starter screen: a card with a form, a dialog and a dark-mode switch.
const MAIN_RS: &str = r#"use dioxus::prelude::*;
use ui::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let theme = use_theme();
    let mut name = use_signal(String::new);
    let mut open = use_signal(|| false);
    rsx! {
        // Loads the kit's theme and component styles; `reset` adds the page font, colours and
        // box sizing a new app wants. Render it once, at the root.
        ThemeStyles { reset: true }
        main { style: "min-height: 100vh; display: grid; place-items: center; padding: 1.5rem;",
            Card { style: "width: 100%; max-width: 26rem;",
                CardHeader {
                    CardTitle { "{title}" }
                    CardDescription { "Built with Atoll. Edit src/main.rs and save to see changes." }
                }
                CardContent {
                    div { style: "display: grid; gap: 0.5rem;",
                        Label { html_for: "name", "Your name" }
                        Input {
                            id: "name",
                            placeholder: "Ada Lovelace",
                            value: name,
                            oninput: move |e: FormEvent| name.set(e.value()),
                        }
                    }
                    div { style: "display: flex; align-items: center; gap: 0.5rem; margin-top: 1rem;",
                        Switch {
                            checked: theme.is_dark(),
                            aria_label: "Dark mode",
                            on_checked_change: move |dark: bool| {
                                theme.set_mode(if dark { ThemeMode::Dark } else { ThemeMode::Light })
                            },
                        }
                        span { "Dark mode" }
                    }
                }
                CardFooter { style: "justify-content: flex-end;",
                    Button { onclick: move |_| open.set(true), "Say hello" }
                }
            }
        }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Hello",
            description: if name().trim().is_empty() { "Hello, stranger.".to_string() } else { format!("Hello, {}.", name().trim()) },
            div { style: "display: flex; justify-content: flex-end; margin-top: 1rem;",
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Close" }
            }
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert!(validate_package_name("my-app").is_ok());
        assert!(validate_package_name("app_2").is_ok());
        assert!(validate_package_name("My App").is_err());
        assert!(validate_package_name("2app").is_err());
        assert!(validate_package_name("").is_err());
    }

    #[test]
    fn dependency_lines() {
        let v = env!("CARGO_PKG_VERSION");
        assert_eq!(
            kit_dep(&KitSource::Registry),
            format!("ui = {{ package = \"atoll-ui\", version = \"{v}\" }}")
        );
        let git = KitSource::Git {
            url: REPO.into(),
            rev: Some("abc".into()),
            tag: None,
        };
        assert!(kit_dep(&git)
            .contains("git = \"https://github.com/Amit-1of1/Rust-UI-\", rev = \"abc\""));
        let tag = KitSource::Git {
            url: REPO.into(),
            rev: None,
            tag: Some("v0.1.0".into()),
        };
        assert!(kit_dep(&tag).ends_with("tag = \"v0.1.0\" }"));
        let unpinned = KitSource::Git {
            url: REPO.into(),
            rev: None,
            tag: None,
        };
        assert!(kit_dep(&unpinned).ends_with("branch = \"main\" }"));
        let fork = KitSource::Git {
            url: "https://example.com/fork".into(),
            rev: None,
            tag: None,
        };
        assert!(kit_dep(&fork).ends_with("git = \"https://example.com/fork\" }"));
    }

    #[test]
    fn refuses_an_existing_app() {
        let dir = std::env::temp_dir().join(format!("atoll-create-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("Cargo.toml"), "[package]").unwrap();
        let err = create(&dir, "x", &KitSource::Registry).unwrap_err();
        assert!(err.contains("atoll init"));
        fs::remove_dir_all(&dir).unwrap();
    }
}
