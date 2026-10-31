//! `atoll changelog`: fragments in `changes/` fold into CHANGELOG.md and docs/MIGRATION.md.

use atoll_cli::changelog::{fold, load, parse, CHANGELOG, MIGRATION};
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("atoll-changelog-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("changes")).unwrap();
    fs::create_dir_all(dir.join("docs")).unwrap();
    for f in [CHANGELOG, MIGRATION] {
        fs::copy(repo_root().join(f), dir.join(f)).unwrap();
    }
    dir
}

#[test]
fn fold_puts_newest_entries_first_and_removes_the_fragments() {
    let dir = scratch("fold");
    let w = |name: &str, body: &str| fs::write(dir.join("changes").join(name), body).unwrap();
    w("older.md", "### Added\n\n- **ui:** Older thing. (#90)\n");
    w(
        "newer.md",
        "### Added\n\n- **ui:** Newer thing. (#91)\n\n### Deprecated\n\n- **ui:** `Old` is deprecated. (#91)\n",
    );
    w(
        "newer.migration.md",
        "### `Old` is deprecated (PR #91)\n\n**What to do:** use `New`.\n",
    );

    let folded = fold(&dir).unwrap();
    assert_eq!(folded.len(), 2);
    let log = fs::read_to_string(dir.join(CHANGELOG)).unwrap();
    let added = log.find("### Added").unwrap();
    let newer = log.find("Newer thing. (#91)").unwrap();
    let older = log.find("Older thing. (#90)").unwrap();
    let previous_top = log.find("- **ui:** `router` feature").unwrap();
    assert!(
        added < newer && newer < older && older < previous_top,
        "{log}"
    );
    // A section the changelog did not have yet goes in its Keep a Changelog place.
    let changed = log.find("### Changed").unwrap();
    let deprecated = log.find("### Deprecated").unwrap();
    let fixed = log.find("### Fixed").unwrap();
    assert!(changed < deprecated && deprecated < fixed, "{log}");

    let mig = fs::read_to_string(dir.join(MIGRATION)).unwrap();
    let note = mig.find("### `Old` is deprecated (PR #91)").unwrap();
    let previous = mig.find("### Toasts are not dialogs").unwrap();
    assert!(
        mig.find("## Unreleased").unwrap() < note && note < previous,
        "{mig}"
    );

    assert!(load(&dir).unwrap().is_empty(), "fragments are removed");
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn fold_refuses_fragments_without_a_pr_number() {
    let dir = scratch("unnamed");
    fs::write(dir.join("changes/x.md"), "### Fixed\n\n- **ui:** A fix.\n").unwrap();
    let before = fs::read_to_string(dir.join(CHANGELOG)).unwrap();
    assert!(fold(&dir).is_err());
    assert_eq!(fs::read_to_string(dir.join(CHANGELOG)).unwrap(), before);
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn parse_rejects_unknown_sections_and_stray_text() {
    let p = Path::new("changes/x.md");
    assert!(parse(p, "### Added\n\n- ok (#1)\n").is_ok());
    assert!(parse(p, "### Improvements\n\n- no (#1)\n").is_err());
    assert!(parse(p, "- no heading (#1)\n").is_err());
    assert!(parse(p, "## Unreleased\n### Added\n- no (#1)\n").is_err());
    assert!(parse(p, "### Added\n\n").is_err());
}

#[test]
fn committed_fragments_are_valid() {
    load(&repo_root()).unwrap();
}
