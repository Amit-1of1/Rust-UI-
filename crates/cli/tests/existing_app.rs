//! `atoll init` and `atoll add` inside an app that already exists: the commands wire
//! the copied code in (mod lines, crate root, Cargo.toml) without touching anything else,
//! and running them twice changes nothing.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

const MANIFEST: &str = r#"[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# my own comment, kept
dioxus = { version = "0.7.10", features = ["router"] }
serde = "1"

[features]
default = ["web"]
web = ["dioxus/web"]
"#;

const MAIN: &str = r#"//! My app.
#![allow(dead_code)]

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "hi" }
}
"#;

fn existing_app() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("assets")).unwrap();
    fs::write(dir.path().join("Cargo.toml"), MANIFEST).unwrap();
    fs::write(dir.path().join("src/main.rs"), MAIN).unwrap();
    fs::write(
        dir.path().join("assets/main.css"),
        ":root {\n  --radius: 4px;\n  --primary-color: hotpink;\n}\n[data-theme=\"dark\"] body { color: white; }\n",
    )
    .unwrap();
    dir
}

fn atoll(app: &Path, args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_atoll"))
        .args(args)
        .arg("--source")
        .arg(repo_root())
        .current_dir(app)
        .output()
        .unwrap();
    let text =
        String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "atoll {args:?} failed:\n{text}");
    text
}

/// Every file under `dir` (except the lockfile's base copies), with its contents.
fn snapshot(dir: &Path) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in fs::read_dir(&d).unwrap().filter_map(Result::ok) {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                out.push((p.clone(), fs::read_to_string(&p).unwrap_or_default()));
            }
        }
    }
    out.sort();
    out
}

#[test]
fn init_reports_clashes_and_writes_the_scoped_theme() {
    let app = existing_app();
    let out = atoll(app.path(), &["init", "--scoped", "--reset"]);
    assert!(out.contains("dioxus 0.7.10: supported"), "{out}");
    for clash in ["--radius", "--primary-color", "data-theme"] {
        assert!(out.contains(clash), "clash {clash} not reported:\n{out}");
    }
    assert!(
        !out.contains("theme.css:"),
        "the kit's own copy is not a clash:\n{out}"
    );
    let theme = fs::read_to_string(app.path().join("assets/theme.css")).unwrap();
    assert!(theme.contains("[data-rui-scope] {\n  /* Primary colors */\n  --primary-color"));
    assert!(!theme.contains("\n:root"), "scoped theme still has :root");
    let reset = fs::read_to_string(app.path().join("assets/reset.css")).unwrap();
    assert!(reset.contains("[data-rui-scope] *"));
    // The app's own stylesheet is untouched.
    assert!(fs::read_to_string(app.path().join("assets/main.css"))
        .unwrap()
        .contains("hotpink"));

    let plain = atoll(app.path(), &["init"]);
    assert!(plain.contains("--scoped"), "suggests --scoped:\n{plain}");
}

#[test]
fn init_warns_about_another_dioxus_line() {
    let app = existing_app();
    let manifest = app.path().join("Cargo.toml");
    let text = fs::read_to_string(&manifest).unwrap();
    fs::write(&manifest, text.replace("0.7.10", "0.6.3")).unwrap();
    let out = atoll(app.path(), &["init"]);
    assert!(out.contains("warn: this app is on dioxus 0.6.3"), "{out}");
}

#[test]
fn add_wires_components_into_the_app() {
    let app = existing_app();
    let out = atoll(app.path(), &["add", "command"]);

    // Modules declared, in the components folder and the crate root.
    let mod_rs = fs::read_to_string(app.path().join("src/components/mod.rs")).unwrap();
    assert!(mod_rs.contains("pub mod dialog;"), "{mod_rs}");
    assert!(mod_rs.contains("pub mod command;"), "{mod_rs}");
    let main = fs::read_to_string(app.path().join("src/main.rs")).unwrap();
    assert!(
        main.starts_with("//! My app.\n#![allow(dead_code)]\nmod components;\n"),
        "{main}"
    );

    // The crates the components need, as their component.json lists them.
    let manifest = fs::read_to_string(app.path().join("Cargo.toml")).unwrap();
    let json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(repo_root().join("crates/ui/src/components/dialog/component.json"))
            .unwrap(),
    )
    .unwrap();
    for dep in json["cargoDependencies"].as_array().unwrap() {
        let name = dep["name"].as_str().unwrap();
        assert!(
            manifest.contains(&format!("{name} = {{")),
            "{name} missing:\n{manifest}"
        );
        if let Some(rev) = dep["rev"].as_str() {
            assert!(manifest.contains(rev), "rev {rev} missing:\n{manifest}");
        }
    }
    assert!(manifest.contains("# my own comment, kept"));
    assert!(manifest.contains("dioxus = { version = \"0.7.10\", features = [\"router\"] }"));
    assert!(out.contains("added "), "{out}");

    // A second run changes nothing.
    let before = snapshot(app.path());
    atoll(app.path(), &["add", "command"]);
    assert_eq!(before, snapshot(app.path()));
}

#[test]
fn add_reports_a_different_pin_instead_of_changing_it() {
    let app = existing_app();
    atoll(app.path(), &["add", "dialog"]);
    let manifest = app.path().join("Cargo.toml");
    let text = fs::read_to_string(&manifest).unwrap();
    // Point the app's copy of every git dependency at another commit.
    let mut changed = String::new();
    for line in text.lines() {
        if let Some(i) = line.find("rev = \"") {
            let end = line[i + 7..].find('"').unwrap() + i + 7;
            changed.push_str(&format!(
                "{}rev = \"0000000\"{}\n",
                &line[..i],
                &line[end + 1..]
            ));
        } else {
            changed.push_str(line);
            changed.push('\n');
        }
    }
    fs::write(&manifest, &changed).unwrap();
    let out = atoll(app.path(), &["add", "dialog"]);
    assert!(out.contains("kept your"), "{out}");
    assert_eq!(fs::read_to_string(&manifest).unwrap(), changed);
}

#[test]
fn no_wire_only_copies() {
    let app = existing_app();
    let out = atoll(app.path(), &["add", "badge", "--no-wire"]);
    assert!(out.contains("Add to src/components/mod.rs"), "{out}");
    assert_eq!(
        fs::read_to_string(app.path().join("Cargo.toml")).unwrap(),
        MANIFEST
    );
    assert_eq!(
        fs::read_to_string(app.path().join("src/main.rs")).unwrap(),
        MAIN
    );
}
