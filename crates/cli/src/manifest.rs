//! `component.json` manifests so the registry also works with Dioxus' own installer:
//! `dx components add button --git <repo-url>`. One manifest per component folder plus a
//! root manifest that lists them. They are generated, like `registry.json`, and a test
//! fails when they are stale.

use crate::{build, COMPONENTS_DIR, THEME_FILE};
use serde_json::{json, Value};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Where this registry lives. `dx` resolves a plain string in `componentDependencies` against
/// its own built-in registry, so our siblings must be named with this URL. The motion crate
/// (needed by drawer) lives here too. Override with `ATOLL_REGISTRY_GIT` to test a fork.
const REGISTRY_GIT: &str = "https://github.com/Amit-1of1/Rust-UI-";

fn registry_git() -> String {
    std::env::var("ATOLL_REGISTRY_GIT").unwrap_or_else(|_| REGISTRY_GIT.to_string())
}

/// Where the kit's primitives come from, read from the workspace manifest's `dioxus-primitives`
/// line. The fork at a pinned commit while `atoll-primitives` is not on crates.io; the registry
/// once that line has no `git` (docs/PUBLISHING.md), and then `atoll-motion` too.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Primitives {
    Git {
        package: String,
        url: String,
        rev: String,
    },
    Registry {
        package: String,
        version: String,
    },
}

fn primitives_source(root: &Path) -> io::Result<Primitives> {
    let text = fs::read_to_string(root.join("Cargo.toml"))?;
    parse_primitives(&text)
}

fn parse_primitives(manifest: &str) -> io::Result<Primitives> {
    let line = manifest
        .lines()
        .find(|l| l.trim_start().starts_with("dioxus-primitives"))
        .ok_or_else(|| io::Error::other("dioxus-primitives not found in Cargo.toml"))?;
    let field = |name: &str| {
        line.split(&format!(" {name} = \""))
            .nth(1)
            .and_then(|r| r.split('"').next())
            .map(str::to_string)
    };
    let package = field("package").unwrap_or_else(|| "dioxus-primitives".into());
    match (field("git"), field("rev"), field("version")) {
        (Some(url), Some(rev), _) => Ok(Primitives::Git { package, url, rev }),
        (Some(_), None, _) => Err(io::Error::other(
            "dioxus-primitives is a git dependency without a rev (docs/GUARDRAILS.md)",
        )),
        (None, _, Some(version)) => Ok(Primitives::Registry { package, version }),
        (None, _, None) => Err(io::Error::other(
            "dioxus-primitives has neither git and rev nor version",
        )),
    }
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
fn cargo_dependencies(source: &str, primitives: &Primitives, registry: &str) -> Vec<Value> {
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
        // The package name, not the kit's `dioxus-primitives` key: `dx components add` runs
        // `cargo add <name>`, and the library is `dioxus_primitives` either way.
        deps.push(match primitives {
            Primitives::Git { package, url, rev } => {
                json!({"name": package, "git": url, "rev": rev, "features": features})
            }
            Primitives::Registry { package, version } => {
                json!({"name": package, "version": version, "features": features})
            }
        });
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
    if uses("dioxus_sdk_time") {
        // Timers that work on the web and native (the async combobox's debounce).
        deps.push(json!({"name": "dioxus-sdk-time", "version": "0.7.0"}));
    }
    if uses("tracing") {
        deps.push(json!({"name": "tracing", "version": "0.1"}));
    }
    if uses("web_sys") {
        // A full version, not "0.3": `cargo add web-sys@0.3 --features Window,Clipboard` (what
        // `dx components add` runs) rejects Clipboard as an unknown feature, while
        // `web-sys@0.3.106` accepts it. Keep in step with Cargo.lock.
        deps.push(
            json!({"name": "web-sys", "version": "0.3.106", "features": web_sys_features(source)}),
        );
    }
    if uses("wasm_bindgen") {
        deps.push(json!({"name": "wasm-bindgen", "version": "0.2"}));
    }
    if uses("wasm_bindgen_futures") {
        deps.push(json!({"name": "wasm-bindgen-futures", "version": "0.4"}));
    }
    if uses("motion") {
        // Published together with the primitives (docs/PUBLISHING.md); the kit's crates share
        // one version (docs/VERSIONING.md).
        deps.push(match primitives {
            Primitives::Git { .. } => json!({"name": "atoll-motion", "git": registry}),
            Primitives::Registry { .. } => {
                json!({"name": "atoll-motion", "version": env!("CARGO_PKG_VERSION")})
            }
        });
    }
    deps
}

/// The web-sys features a source needs: a base set, the types it names, and the types
/// behind the methods it calls (web-sys gates each type behind a feature of its name).
fn web_sys_features(source: &str) -> Vec<String> {
    const BASE: &[&str] = &[
        "Window",
        "Document",
        "Element",
        "HtmlElement",
        "EventTarget",
    ];
    const BY_METHOD: &[(&str, &str)] = &[
        (".navigator()", "Navigator"),
        (".clipboard()", "Clipboard"),
        (".match_media(", "MediaQueryList"),
    ];
    let mut out: Vec<String> = BASE.iter().map(|s| s.to_string()).collect();
    let mut add = |name: &str| {
        if name.starts_with(|c: char| c.is_ascii_uppercase()) && !out.iter().any(|f| f == name) {
            out.push(name.to_string());
        }
    };
    for (method, feature) in BY_METHOD {
        if source.contains(method) {
            add(feature);
        }
    }
    // `web_sys::KeyboardEvent` and `use web_sys::{A, B}`.
    for (i, _) in source.match_indices("web_sys::") {
        let rest = &source[i + "web_sys::".len()..];
        let names = match rest.strip_prefix('{') {
            Some(list) => &list[..list.find('}').unwrap_or(0)],
            None => {
                &rest[..rest
                    .find(|c: char| !c.is_ascii_alphanumeric())
                    .unwrap_or(rest.len())]
            }
        };
        for name in names.split(',') {
            let name: String = name
                .trim()
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            add(&name);
        }
    }
    out
}

/// Every manifest as `(path relative to root, contents)`: one per component and the root.
pub fn build_manifests(root: &Path) -> io::Result<Vec<(PathBuf, String)>> {
    let primitives = primitives_source(root)?;
    let git = registry_git();
    let reg = build(root)?;
    let mut out = Vec::new();
    let mut members = Vec::new();
    // Blocks are installed with `atoll add`; `dx components add` only knows components.
    for item in reg.items.iter().filter(|i| i.kind.is_component()) {
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
        // The keyboard contract drives this repo's tests (docs/CONTRACTS.md); apps don't need it.
        let mut exclude = vec!["docs.md", "component.json"];
        if root.join(&dir).join(crate::contract::FILE).is_file() {
            exclude.push(crate::contract::FILE);
        }
        let manifest = json!({
            "name": item.name,
            "description": description(&docs),
            "exclude": exclude,
            "cargoDependencies": cargo_dependencies(&source, &primitives, &git),
            "componentDependencies": item.registry_dependencies.iter().map(|d| json!({"name": d, "git": git})).collect::<Vec<_>>(),
            "globalAssets": [theme.to_string_lossy()],
        });
        out.push((dir.join("component.json"), pretty(&manifest)));
        members.push(dir.to_string_lossy().into_owned());
    }
    let root_manifest = json!({
        "name": "atoll",
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

    fn git_primitives() -> Primitives {
        Primitives::Git {
            package: "atoll-primitives".into(),
            url: "u".into(),
            rev: "abc".into(),
        }
    }

    #[test]
    fn primitives_come_from_git_until_published() {
        let line =
            |rest: &str| format!("[workspace.dependencies]\ndioxus-primitives = {{ {rest} }}\n");
        assert_eq!(
            parse_primitives(&line(
                r#"package = "atoll-primitives", version = "0.1.0", git = "https://x", rev = "abc""#
            ))
            .unwrap(),
            Primitives::Git {
                package: "atoll-primitives".into(),
                url: "https://x".into(),
                rev: "abc".into()
            }
        );
        // Upstream's name when no `package` is given.
        assert!(matches!(
            parse_primitives(&line(r#"git = "https://x", rev = "abc""#)).unwrap(),
            Primitives::Git { package, .. } if package == "dioxus-primitives"
        ));
        let published =
            parse_primitives(&line(r#"package = "atoll-primitives", version = "0.1.0""#)).unwrap();
        assert_eq!(
            published,
            Primitives::Registry {
                package: "atoll-primitives".into(),
                version: "0.1.0".into()
            }
        );
        // Published: no git anywhere in a component's dependencies.
        let deps = cargo_dependencies("use dioxus_primitives::x;\nuse motion::z;", &published, "g");
        assert!(deps.iter().all(|d| d.get("git").is_none()), "{deps:?}");
        assert!(parse_primitives(&line(r#"git = "https://x", branch = "main""#)).is_err());
    }

    #[test]
    fn dependencies_follow_the_imports() {
        let src = "use dioxus_primitives::x;\nuse motion::z;";
        let names: Vec<_> = cargo_dependencies(src, &git_primitives(), "g")
            .iter()
            .map(|d| d["name"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(names, ["atoll-primitives", "atoll-motion"]);
        assert!(cargo_dependencies("use dioxus::prelude::*;", &git_primitives(), "g").is_empty());
        assert!(cargo_dependencies(
            "use crate::components::time::Clock;",
            &git_primitives(),
            "g"
        )
        .is_empty());
    }

    #[test]
    fn web_sys_features_follow_types_and_methods() {
        let src = "use web_sys::{KeyboardEvent, MediaQueryList};\nweb_sys::window().navigator().clipboard();\nfoo(a, Bar);";
        let f = web_sys_features(src);
        for want in [
            "Window",
            "KeyboardEvent",
            "MediaQueryList",
            "Navigator",
            "Clipboard",
        ] {
            assert!(f.iter().any(|x| x == want), "{want} missing from {f:?}");
        }
        assert_eq!(f.iter().filter(|x| *x == "MediaQueryList").count(), 1);
        assert!(!f.iter().any(|x| x == "Bar"), "{f:?}");
    }

    /// `dx components add` runs `cargo add web-sys@<version> --features ...`, and with a bare
    /// "0.3" cargo rejects `Clipboard` ("unrecognized feature"), so the install stops.
    #[test]
    fn web_sys_has_a_full_version() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../ui/src/components/hooks/component.json"
        );
        let json: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let web_sys = json["cargoDependencies"]
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["name"] == "web-sys")
            .unwrap();
        assert_eq!(web_sys["version"].as_str().unwrap().split('.').count(), 3);
    }
}
