//! `atoll acr`: the criteria table covers WCAG 2.2 A and AA exactly, cites only checks that
//! exist, and the keyboard rows follow the contracts of the components an app uses.

use atoll_cli::acr::{components_of, load_criteria, markdown, missing_evidence, report, Status};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// WCAG 2.2 Level A and AA success criteria (4.1.1 Parsing is obsolete in 2.2).
const WCAG22: &[(&str, &str)] = &[
    ("1.1.1", "A"),
    ("1.2.1", "A"),
    ("1.2.2", "A"),
    ("1.2.3", "A"),
    ("1.3.1", "A"),
    ("1.3.2", "A"),
    ("1.3.3", "A"),
    ("1.4.1", "A"),
    ("1.4.2", "A"),
    ("2.1.1", "A"),
    ("2.1.2", "A"),
    ("2.1.4", "A"),
    ("2.2.1", "A"),
    ("2.2.2", "A"),
    ("2.3.1", "A"),
    ("2.4.1", "A"),
    ("2.4.2", "A"),
    ("2.4.3", "A"),
    ("2.4.4", "A"),
    ("2.5.1", "A"),
    ("2.5.2", "A"),
    ("2.5.3", "A"),
    ("2.5.4", "A"),
    ("3.1.1", "A"),
    ("3.2.1", "A"),
    ("3.2.2", "A"),
    ("3.2.6", "A"),
    ("3.3.1", "A"),
    ("3.3.2", "A"),
    ("3.3.7", "A"),
    ("4.1.2", "A"),
    ("1.2.4", "AA"),
    ("1.2.5", "AA"),
    ("1.3.4", "AA"),
    ("1.3.5", "AA"),
    ("1.4.3", "AA"),
    ("1.4.4", "AA"),
    ("1.4.5", "AA"),
    ("1.4.10", "AA"),
    ("1.4.11", "AA"),
    ("1.4.12", "AA"),
    ("1.4.13", "AA"),
    ("2.4.5", "AA"),
    ("2.4.6", "AA"),
    ("2.4.7", "AA"),
    ("2.4.11", "AA"),
    ("2.5.7", "AA"),
    ("2.5.8", "AA"),
    ("3.1.2", "AA"),
    ("3.2.3", "AA"),
    ("3.2.4", "AA"),
    ("3.3.3", "AA"),
    ("3.3.4", "AA"),
    ("3.3.8", "AA"),
    ("4.1.3", "AA"),
];

#[test]
fn every_wcag22_a_and_aa_criterion_is_listed_once() {
    let criteria = load_criteria(&root()).expect("criteria");
    assert_eq!(criteria.len(), WCAG22.len());
    for (id, level) in WCAG22 {
        let found: Vec<_> = criteria.iter().filter(|c| c.id == *id).collect();
        assert_eq!(found.len(), 1, "{id} listed {} times", found.len());
        assert_eq!(found[0].level, *level, "{id} level");
    }
}

#[test]
fn every_cited_check_exists() {
    let criteria = load_criteria(&root()).expect("criteria");
    let missing = missing_evidence(&root(), &criteria);
    assert!(
        missing.is_empty(),
        "evidence that points nowhere:\n{}",
        missing.join("\n")
    );
}

#[test]
fn claims_have_evidence_and_scoped_rows_name_real_components() {
    let reg = atoll_cli::load(&root()).expect("registry");
    for c in load_criteria(&root()).expect("criteria") {
        if c.kit == Status::Supports || c.kit == Status::Partial {
            // A few criteria are met by what the kit leaves out (no flashing, no audio, no
            // orientation lock); everything else must cite a check.
            let by_absence = ["1.3.4", "1.4.5", "2.3.1", "2.5.2", "3.2.2"];
            assert!(
                !c.evidence.is_empty() || by_absence.contains(&c.id.as_str()),
                "{} claims {:?} without evidence",
                c.id,
                c.kit
            );
        }
        for name in &c.components {
            assert!(
                reg.items.iter().any(|i| &i.name == name),
                "{}: no component `{name}`",
                c.id
            );
        }
    }
}

#[test]
fn keyboard_rows_follow_the_used_components_contracts() {
    let root = root();
    let criteria = load_criteria(&root).expect("criteria");
    let reg = atoll_cli::load(&root).expect("registry");
    let used = components_of(&reg, &["dialog".into(), "toolbar".into()]).expect("resolve");
    let r = report(&root, &criteria, &used, "Test app").expect("report");
    let row = |id: &str| r.rows.iter().find(|x| x.id == id).expect("row").clone();

    let keyboard = row("2.1.1");
    assert_eq!(keyboard.status, Status::Partial);
    assert!(keyboard
        .remarks
        .contains("toolbar: Home works, End does not"));
    assert!(keyboard
        .evidence
        .iter()
        .any(|e| e.ends_with("dialog/contract.toml")));

    // Only dialog: its contract has no gaps.
    let r = report(&root, &criteria, &["dialog".into()], "Test app").expect("report");
    let keyboard = r.rows.iter().find(|x| x.id == "2.1.1").expect("row");
    assert_eq!(keyboard.status, Status::Supports);

    // No toast in use: the toast rows don't apply to the kit.
    let timing = r.rows.iter().find(|x| x.id == "2.2.1").expect("row");
    assert_eq!(timing.status, Status::NotApplicable);
    assert!(timing.review, "the app's own time limits still need review");
}

#[test]
fn markdown_has_both_tables_and_the_draft_warning() {
    let root = root();
    let criteria = load_criteria(&root).expect("criteria");
    let reg = atoll_cli::load(&root).expect("registry");
    let all: Vec<String> = reg.items.iter().map(|i| i.name.clone()).collect();
    let used = components_of(&reg, &all).expect("resolve");
    let text = markdown(
        &report(&root, &criteria, &used, "Kit").expect("report"),
        "2026-10-30",
    );
    assert!(text.contains("## Table 1: Success Criteria, Level A"));
    assert!(text.contains("## Table 2: Success Criteria, Level AA"));
    assert!(text.contains("This is a draft, not an audit."));
    assert_eq!(text.matches("(Level A) |").count(), 31);
    assert_eq!(text.matches("(Level AA) |").count(), 24);
}
