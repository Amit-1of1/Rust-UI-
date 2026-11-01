//! `atoll theme --import`: a tweakcn theme becomes an Atoll look that passes the contrast
//! proof, and `atoll font` copies a font file into an app.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn atoll(args: &[&str], dir: &Path) -> (String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_atoll"))
        .args(args)
        .arg("--source")
        .arg(repo_root())
        .current_dir(dir)
        .output()
        .unwrap();
    let (o, e) = (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    );
    assert!(out.status.success(), "atoll {args:?} failed:\n{o}\n{e}");
    (o, e)
}

#[test]
fn imports_a_tweakcn_theme_that_passes_contrast() {
    let fixture = repo_root().join("crates/cli/tests/fixtures/tweakcn-theme.css");
    let (css, notes) = atoll(
        &[
            "theme",
            "--import",
            fixture.to_str().unwrap(),
            "--name",
            "indigo",
        ],
        &repo_root(),
    );
    assert!(css.contains("[data-preset=\"indigo\"] {"), "{css}");
    assert!(
        css.contains(
            ":root,\n:where([data-theme]) {\n  --primary-color: light-dark(#f9fafb, #0f172a);"
        ),
        "{css}"
    );
    assert!(
        css.contains("--accent-color: light-dark(#4f46e5, #929efb);"),
        "{css}"
    );
    assert!(
        notes.contains("every text and focus pair passes"),
        "{notes}"
    );
    assert!(
        notes.contains("atoll font --google \"Plus Jakarta Sans\""),
        "{notes}"
    );
}

#[test]
fn font_copies_a_file_into_the_app() {
    let app = tempfile::tempdir().unwrap();
    let file = app.path().join("BrandSans-Regular.woff2");
    std::fs::write(&file, b"wOF2").unwrap();
    let (out, _) = atoll(
        &[
            "font",
            file.to_str().unwrap(),
            "--weights",
            "400",
            "--role",
            "heading",
        ],
        app.path(),
    );
    assert!(app.path().join("assets/fonts/brand-sans.woff2").exists());
    assert!(
        out.contains("ui::fonts::CustomFont {\n    family: \"Brand Sans\","),
        "{out}"
    );
    assert!(out.contains("role: ui::fonts::FontRole::Heading,"), "{out}");
}
