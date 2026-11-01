//! The maintenance docs (docs/MAINTENANCE_CHECKLIST.md and the pages it points to) checked
//! against the repository: every file they link to exists, the issue templates are there, and
//! the list of components without a "Component Structure" snippet only shrinks.

use std::fs;
use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &str) -> String {
    fs::read_to_string(repo().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The pages written for the maintenance checklist, plus the checklist itself.
const PAGES: &[&str] = &[
    "docs/MAINTENANCE_CHECKLIST.md",
    "docs/PRINCIPLES.md",
    "docs/BEST_PRACTICES.md",
    "docs/ANATOMY.md",
    "docs/COMPOSITION.md",
    "docs/SUPPORT_MATRIX.md",
    "docs/LOCAL_LIBRARIES.md",
    "docs/ROADMAP.md",
    "docs/MAINTAINING.md",
    "docs/decisions/README.md",
    "docs/decisions/TEMPLATE.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
];

/// The relative link targets in a Markdown file (`[text](target)`), without anchors.
fn relative_links(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find("](") {
        rest = &rest[at + 2..];
        let Some(end) = rest.find(')') else { break };
        let target = &rest[..end];
        let path = target.split('#').next().unwrap_or_default();
        if !path.is_empty() && !target.contains("://") && !target.starts_with("mailto:") {
            out.push(path.to_string());
        }
    }
    out
}

#[test]
fn every_linked_file_exists() {
    let mut missing = Vec::new();
    for page in PAGES {
        let dir = repo().join(page);
        let dir = dir.parent().unwrap();
        for link in relative_links(&read(page)) {
            if !dir.join(&link).exists() {
                missing.push(format!("{page} -> {link}"));
            }
        }
    }
    assert!(missing.is_empty(), "broken links: {missing:#?}");
}

#[test]
fn issue_and_pr_templates_exist() {
    for file in [
        ".github/ISSUE_TEMPLATE/config.yml",
        ".github/ISSUE_TEMPLATE/bug_report.yml",
        ".github/ISSUE_TEMPLATE/feature_request.yml",
        ".github/ISSUE_TEMPLATE/component_proposal.yml",
        ".github/pull_request_template.md",
    ] {
        assert!(repo().join(file).is_file(), "{file} is missing");
    }
}

/// Every decision record is in the index, so the log stays browsable.
#[test]
fn decision_index_lists_every_record() {
    let index = read("docs/decisions/README.md");
    for entry in fs::read_dir(repo().join("docs/decisions")).unwrap() {
        let name = entry.unwrap().file_name().to_string_lossy().to_string();
        if name == "README.md" || name == "TEMPLATE.md" || !name.ends_with(".md") {
            continue;
        }
        assert!(
            index.contains(&format!("({name})")),
            "docs/decisions/README.md should link {name}"
        );
    }
}

/// Components whose docs.md has no "Component Structure" section yet (2026-11-01). Remove a
/// name when its section is written; a new component must ship with one.
const NO_STRUCTURE_YET: &[&str] = &[
    "alert",
    "bottom_nav",
    "breadcrumb",
    "button_group",
    "callout",
    "carousel",
    "chart",
    "chat",
    "chat_composer",
    "chips",
    "command",
    "data_table",
    "date_time_picker",
    "drawer",
    "dropzone",
    "empty",
    "field",
    "form",
    "input_group",
    "input_otp",
    "kbd",
    "link",
    "marquee",
    "multi_select",
    "native_select",
    "number_field",
    "resizable",
    "shimmer",
    "skip_link",
    "spinner",
    "status",
    "stepper",
    "table",
    "theme_customizer",
    "time_picker",
    "tool_call",
];

/// Support code, not components (as in tests/claims.rs).
const UTILITIES: &[&str] = &["hooks", "icons", "ui_strings"];

#[test]
fn component_structure_list_only_shrinks() {
    let dir = repo().join("crates/ui/src/components");
    let mut lacking = Vec::new();
    let mut gained = Vec::new();
    for entry in fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let docs = path.join("docs.md");
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !docs.is_file() || UTILITIES.contains(&name.as_str()) {
            continue;
        }
        let text = fs::read_to_string(&docs).unwrap();
        let has = text
            .lines()
            .any(|l| l == "## Component Structure" || l == "## Structure" || l == "## Anatomy");
        let listed = NO_STRUCTURE_YET.contains(&name.as_str());
        if !has && !listed {
            lacking.push(name);
        } else if has && listed {
            gained.push(name);
        }
    }
    assert!(
        lacking.is_empty(),
        "add a `## Component Structure` section to docs.md (docs/ANATOMY.md): {lacking:?}"
    );
    assert!(
        gained.is_empty(),
        "these now have a structure section; remove them from NO_STRUCTURE_YET: {gained:?}"
    );
}

#[test]
fn anatomy_page_counts_match() {
    let without = NO_STRUCTURE_YET.len();
    let anatomy = read("docs/ANATOMY.md");
    assert!(
        anatomy.contains(&format!("The {without} still without")),
        "docs/ANATOMY.md should say \"The {without} still without\""
    );
}
