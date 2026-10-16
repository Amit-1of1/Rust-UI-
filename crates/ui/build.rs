//! Generates the module lists that used to be edited by hand, so two branches that each add a
//! component no longer conflict over the same lines (docs/GUARDRAILS.md, "Parallel sessions").
//!
//! - `components.rs`: `pub mod <name>;` for every folder in `src/components` with a `mod.rs`.
//! - `prelude.rs`: `pub use crate::components::<name>::*;` for the same, minus the names in
//!   `not_in_prelude.txt`.
//! - `demos.rs`: `pub mod <name>;` for every `src/demos/<name>.rs`, and `ALL`, the list the
//!   tests, gallery checks and snapshots walk.
//! - `component_tests.rs`: `mod <name>;` for every `tests/components/<name>.rs`.
//!
//! Adding a component is now "add files": nothing else needs a new line.
//!
//! It also checks every `contract.toml` (docs/CONTRACTS.md) with the same rules as
//! `rust-ui doctor`, so a contract that documents a key its test never presses, or a role no
//! test looks at, fails the build instead of reaching the docs.

#[path = "../cli/src/contract.rs"]
#[allow(dead_code)]
mod contract;

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let components_dir = root.join("src/components");
    let demos_dir = root.join("src/demos");
    let tests_dir = root.join("tests/components");
    let exclude_file = root.join("not_in_prelude.txt");
    for dir in [&components_dir, &demos_dir, &tests_dir, &exclude_file] {
        println!("cargo::rerun-if-changed={}", dir.display());
    }
    let excluded = fs::read_to_string(&exclude_file).expect("not_in_prelude.txt");
    let not_in_prelude: Vec<&str> = excluded
        .lines()
        .filter_map(|l| l.split('#').next())
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    let components: Vec<String> = names(&components_dir, |p| {
        p.is_dir() && p.join("mod.rs").is_file()
    });
    let demos: Vec<String> = names(&demos_dir, |p| is_rs(p) && !p.ends_with("mod.rs"))
        .into_iter()
        .map(|n| n.trim_end_matches(".rs").to_string())
        .collect();
    let tests: Vec<String> = names(&tests_dir, |p| is_rs(p) && !p.ends_with("main.rs"))
        .into_iter()
        .map(|n| n.trim_end_matches(".rs").to_string())
        .collect();

    let mut broken = Vec::new();
    for name in &components {
        match contract::Contract::load(&components_dir, name) {
            Ok(Some(c)) => broken.extend(
                c.problems()
                    .into_iter()
                    .map(|p| format!("{name}/{}: {p}", contract::FILE)),
            ),
            Ok(None) => {}
            Err(e) => broken.push(e),
        }
    }
    assert!(
        broken.is_empty(),
        "component contracts are not sound (docs/CONTRACTS.md):\n  {}",
        broken.join("\n  ")
    );

    let mut mods = String::new();
    let mut prelude = String::new();
    for name in &components {
        let path = components_dir.join(name).join("mod.rs");
        mods.push_str(&format!(
            "#[path = {:?}]\npub mod {name};\n",
            path.display().to_string()
        ));
        if !not_in_prelude.contains(&name.as_str()) {
            prelude.push_str(&format!("pub use crate::components::{name}::*;\n"));
        }
    }

    let mut demo_mods = String::new();
    let mut all = String::from(
        "/// A component name and its demo.\n\
         pub type Entry = (&'static str, fn() -> dioxus::prelude::Element);\n\
         /// Every demo by component name, in name order. The tests walk this list.\n\
         pub const ALL: &[Entry] = &[\n",
    );
    for name in &demos {
        let path = demos_dir.join(format!("{name}.rs"));
        demo_mods.push_str(&format!(
            "#[path = {:?}]\npub mod {name};\n",
            path.display().to_string()
        ));
        all.push_str(&format!("    ({name:?}, {name}::Demo),\n"));
    }
    all.push_str("];\n");
    demo_mods.push_str(&all);

    let mut test_mods = String::new();
    for name in &tests {
        let path = tests_dir.join(format!("{name}.rs"));
        test_mods.push_str(&format!(
            "#[path = {:?}]\nmod {name};\n",
            path.display().to_string()
        ));
    }

    write(&out.join("components.rs"), &mods);
    write(&out.join("prelude.rs"), &prelude);
    write(&out.join("demos.rs"), &demo_mods);
    write(&out.join("component_tests.rs"), &test_mods);
}

fn is_rs(p: &Path) -> bool {
    p.is_file() && p.extension().is_some_and(|e| e == "rs")
}

/// Sorted file names in `dir` that pass `keep`; empty if `dir` does not exist.
fn names(dir: &Path, keep: impl Fn(&Path) -> bool) -> Vec<String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<String> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| keep(p))
        .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string))
        .collect();
    out.sort();
    out
}

/// Write only when the content changed, so an unchanged list does not trigger a rebuild.
fn write(path: &Path, body: &str) {
    if fs::read_to_string(path).ok().as_deref() != Some(body) {
        fs::write(path, body).expect("write generated module list");
    }
}
