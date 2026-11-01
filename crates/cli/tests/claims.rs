//! The public claims (README, docs home page, docs/CLAIMS.md) checked against the repository,
//! so a number can't drift from what it counts. docs/CLAIMS.md is the ledger: every claim, the
//! evidence for it and the command that reproduces it. This test covers the claims the
//! repository itself can prove without a browser; the browser and size claims point at the
//! scripts that measure them.

use std::fs;
use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &str) -> String {
    fs::read_to_string(repo().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// Registry items that are support code, not components: they are installable, but nobody
/// would call them a component. Keep in step with `UTILITIES` in apps/docs/src/landing.rs.
const UTILITIES: &[&str] = &["hooks", "icons", "ui_strings"];

struct Counts {
    components: usize,
    blocks: usize,
    contracts: usize,
}

fn counts() -> Counts {
    let registry: serde_json::Value = serde_json::from_str(&read("registry.json")).unwrap();
    let items = registry["items"].as_array().unwrap();
    let names: Vec<&str> = items.iter().map(|i| i["name"].as_str().unwrap()).collect();
    for u in UTILITIES {
        assert!(
            names.contains(u),
            "utility `{u}` is no longer in registry.json; update UTILITIES"
        );
    }
    let blocks = items.iter().filter(|i| i["kind"] == "block").count();
    let components = items.len() - blocks - UTILITIES.len();
    let contracts = fs::read_dir(repo().join("crates/ui/src/components"))
        .unwrap()
        .filter(|e| e.as_ref().unwrap().path().join("contract.toml").exists())
        .count();
    Counts {
        components,
        blocks,
        contracts,
    }
}

#[test]
fn readme_counts_match_the_registry() {
    let c = counts();
    let readme = read("README.md");
    let want = format!(
        "**{} components and {} whole-screen blocks**",
        c.components, c.blocks
    );
    assert!(readme.contains(&want), "README.md should say {want}");
    let want = format!("{} components ship a keyboard contract", c.contracts);
    assert!(readme.contains(&want), "README.md should say \"{want}\"");
}

#[test]
fn the_ledger_counts_match_the_registry() {
    let c = counts();
    let ledger = read("docs/CLAIMS.md");
    for want in [
        format!("{} components", c.components),
        format!("{} blocks", c.blocks),
        format!("{} keyboard contracts", c.contracts),
    ] {
        assert!(
            ledger.contains(&want),
            "docs/CLAIMS.md should say \"{want}\""
        );
    }
}

#[test]
fn the_landing_page_counts_components_not_utilities() {
    let landing = read("apps/docs/src/landing.rs");
    let list = UTILITIES
        .iter()
        .map(|u| format!("\"{u}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let want = format!("const UTILITIES: &[&str] = &[{list}];");
    assert!(landing.contains(&want), "landing.rs should define {want}");
}

/// "Contrast is proved for every accent preset" and "axe in every accent": the axe loop in
/// scripts/e2e.sh must scan every preset the theme defines.
#[test]
fn every_accent_preset_is_scanned_by_axe() {
    let theme = read("crates/ui/assets/theme.css");
    let e2e = read("scripts/e2e.sh");
    let line = e2e
        .lines()
        .find(|l| l.contains("for accent in"))
        .expect("scripts/e2e.sh has an accent loop");
    for part in theme.split("[data-accent=\"").skip(1) {
        let name = part.split('"').next().unwrap();
        assert!(
            line.split_whitespace()
                .any(|w| w.trim_end_matches(';') == name),
            "accent preset `{name}` is not in the axe loop of scripts/e2e.sh"
        );
    }
}

/// "axe and keyboard checks in Chromium, Firefox and WebKit in CI".
#[test]
fn ci_runs_the_browser_checks_on_three_engines() {
    let wf = read(".github/workflows/e2e.yml");
    assert!(
        wf.contains("browser: [chromium, firefox, webkit]"),
        ".github/workflows/e2e.yml no longer runs all three engines; fix the claim or the matrix"
    );
}

/// "Motion values are model-checked with TLA+": CI must run the checker, and the spec must
/// keep its mutants (the proof that the checks can fail).
#[test]
fn ci_model_checks_the_tla_specs() {
    let wf = read(".github/workflows/ci.yml");
    assert!(
        wf.contains("scripts/tla.sh"),
        ".github/workflows/ci.yml no longer runs scripts/tla.sh; fix the claim or the workflow"
    );
    for file in [
        "proofs/tla/MotionValue.tla",
        "proofs/tla/MotionValue.cfg",
        "proofs/tla/MotionValue_bug_two_loops.cfg",
        "proofs/tla/MotionValue_bug_frozen.cfg",
    ] {
        assert!(repo().join(file).exists(), "{file} is gone; fix docs/CLAIMS.md");
    }
}

/// "A dialog without a title or an icon button without a label doesn't compile": the
/// `compile_fail` doc tests that prove it must stay.
#[test]
fn the_compile_time_rules_have_compile_fail_tests() {
    for (file, prop) in [
        (
            "crates/ui/src/components/dialog/component.rs",
            "pub title: String",
        ),
        (
            "crates/ui/src/components/button/component.rs",
            "    label: String,",
        ),
    ] {
        let src = read(file);
        assert!(
            src.contains("```compile_fail"),
            "{file} lost its compile_fail doc test"
        );
        assert!(src.contains(prop), "{file}: `{prop}` is no longer required");
    }
}

/// "No Tailwind build step": no manifest, config or workflow in the repo runs Tailwind.
#[test]
fn nothing_runs_tailwind() {
    let mut hits = Vec::new();
    let mut stack = vec![repo()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                if !matches!(name.as_str(), "target" | "node_modules" | ".git" | "dist") {
                    stack.push(path);
                }
                continue;
            }
            let config = name == "Cargo.toml"
                || name == "Dioxus.toml"
                || name == "package.json"
                || name.starts_with("tailwind.config")
                || name.ends_with(".yml");
            if config
                && fs::read_to_string(&path)
                    .unwrap_or_default()
                    .to_lowercase()
                    .contains("tailwind")
            {
                hits.push(path.display().to_string());
            }
        }
    }
    assert!(hits.is_empty(), "Tailwind appears in: {hits:?}");
}

/// The size trade-off on the home page quotes bench/README.md; the two must agree.
#[test]
fn the_landing_size_numbers_come_from_the_benchmark() {
    let landing = read("apps/docs/src/landing.rs");
    let bench = read("bench/README.md");
    let figures = kb_figures(&landing);
    assert!(
        !figures.is_empty(),
        "landing.rs quotes no sizes; drop this test"
    );
    for kb in figures {
        assert!(
            bench.contains(&format!("{kb} KB")),
            "landing.rs quotes {kb} KB, which bench/README.md doesn't have"
        );
    }
}

/// Every "<n> KB" in the text.
fn kb_figures(text: &str) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    words
        .windows(2)
        .filter(|w| w[1].starts_with("KB"))
        .map(|w| w[0].trim_start_matches('(').to_string())
        .filter(|n| n.chars().all(|c| c.is_ascii_digit() || c == ','))
        .collect()
}
