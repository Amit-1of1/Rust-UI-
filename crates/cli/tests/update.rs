//! `rust-ui update`: lockfile recording and the three-way merge, against a throwaway registry.

use rust_ui_cli::update::{self, Action, Lock, CONFLICT_START};
use rust_ui_cli::{add, build, load, resolve, to_json, COMPONENTS_DIR};
use std::fs;
use std::path::{Path, PathBuf};

const ORIGINAL: &str = "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\n";

/// A registry checkout (`root`) and a project that copied from it (`project`).
struct Setup {
    _dirs: (tempfile::TempDir, tempfile::TempDir),
    root: PathBuf,
    project: PathBuf,
}

impl Setup {
    fn new() -> Setup {
        let (r, p) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let s = Setup {
            root: r.path().to_path_buf(),
            project: p.path().to_path_buf(),
            _dirs: (r, p),
        };
        s.upstream("badge/mod.rs", "mod component;\npub use component::*;\n");
        s.upstream("badge/component.rs", ORIGINAL);
        s.upstream("button/mod.rs", "pub fn button() {}\n");
        s
    }

    /// Write a registry file and regenerate registry.json.
    fn upstream(&self, file: &str, text: &str) {
        let path = self.root.join(COMPONENTS_DIR).join(file);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
        self.regen();
    }

    fn regen(&self) {
        fs::write(
            self.root.join("registry.json"),
            to_json(&build(&self.root).unwrap()),
        )
        .unwrap();
    }

    fn dest(&self) -> PathBuf {
        self.project.join("src/components")
    }

    fn lock(&self) -> PathBuf {
        self.project.join("rust-ui-lock.json")
    }

    fn local(&self, file: &str) -> PathBuf {
        self.dest().join(file)
    }

    /// `rust-ui add <names>`.
    fn add(&self, names: &[&str]) {
        let reg = load(&self.root).unwrap();
        let names: Vec<String> = names.iter().map(|s| s.to_string()).collect();
        let items = resolve(&reg, &names).unwrap();
        add(&self.root, &items, &self.dest(), false).unwrap();
        update::record(
            &self.lock(),
            &self.root,
            "test-source",
            &items,
            &self.dest(),
            &self.project.join("src/blocks"),
        )
        .unwrap();
    }

    fn plan(&self, names: &[&str]) -> update::Plan {
        let names: Vec<String> = names.iter().map(|s| s.to_string()).collect();
        update::plan(
            &self.root,
            &load(&self.root).unwrap(),
            &Lock::load(&self.lock()).unwrap(),
            &self.lock(),
            &names,
            &self.dest(),
            &self.project.join("src/blocks"),
        )
        .unwrap()
    }

    /// `rust-ui update`.
    fn update(&self) -> update::Plan {
        let plan = self.plan(&[]);
        update::apply(&plan, &self.lock(), "test-source").unwrap();
        plan
    }

    fn read(&self, file: &str) -> String {
        fs::read_to_string(self.local(file)).unwrap()
    }
}

fn action(plan: &update::Plan, file: &str) -> Action {
    plan.files
        .iter()
        .find(|f| f.file == file)
        .unwrap_or_else(|| panic!("{file} not in plan: {plan:?}"))
        .action
}

fn edit(path: &Path, from: &str, to: &str) {
    let text = fs::read_to_string(path).unwrap();
    assert!(text.contains(from));
    fs::write(path, text.replacen(from, to, 1)).unwrap();
}

#[test]
fn add_records_files_and_pristine_copies() {
    let s = Setup::new();
    s.add(&["badge"]);
    let lock = Lock::load(&s.lock()).unwrap();
    assert_eq!(lock.source.as_deref(), Some("test-source"));
    let badge = &lock.items["badge"];
    assert_eq!(badge.dir, "src/components");
    assert_eq!(
        badge.files["badge/component.rs"],
        update::hash(ORIGINAL.as_bytes())
    );
    let base = update::base_dir(&s.lock()).join("badge/component.rs");
    assert_eq!(fs::read_to_string(base).unwrap(), ORIGINAL);
}

#[test]
fn untouched_files_stay_up_to_date() {
    let s = Setup::new();
    s.add(&["badge"]);
    let plan = s.update();
    assert!(plan.files.iter().all(|f| f.action == Action::UpToDate));
    assert!(!plan.files.iter().any(|f| f.writes()));
    assert!(!plan.has_conflicts());
}

#[test]
fn untouched_files_take_the_new_version() {
    let s = Setup::new();
    s.add(&["badge"]);
    let new = ORIGINAL.replace("line 4", "line four");
    s.upstream("badge/component.rs", &new);
    let plan = s.update();
    assert_eq!(action(&plan, "badge/component.rs"), Action::Updated);
    assert_eq!(s.read("badge/component.rs"), new);
    // The new version is now the base.
    let lock = Lock::load(&s.lock()).unwrap();
    assert_eq!(
        lock.items["badge"].files["badge/component.rs"],
        update::hash(new.as_bytes())
    );
}

#[test]
fn edits_and_upstream_changes_in_different_places_merge_cleanly() {
    let s = Setup::new();
    s.add(&["badge"]);
    edit(
        &s.local("badge/component.rs"),
        "line 1\n",
        "line 1 (mine)\n",
    );
    s.upstream(
        "badge/component.rs",
        &ORIGINAL.replace("line 8\n", "line 8\nline 9\n"),
    );
    let plan = s.update();
    assert_eq!(action(&plan, "badge/component.rs"), Action::Merged);
    let merged = s.read("badge/component.rs");
    assert!(merged.starts_with("line 1 (mine)\n"), "{merged}");
    assert!(merged.ends_with("line 8\nline 9\n"), "{merged}");
    assert!(!plan.has_conflicts());
    // Next time the merged file is the user's edit on top of the new base.
    assert_eq!(
        action(&s.plan(&[]), "badge/component.rs"),
        Action::KeptEdits
    );
}

#[test]
fn overlapping_edits_get_conflict_markers() {
    let s = Setup::new();
    s.add(&["badge"]);
    edit(
        &s.local("badge/component.rs"),
        "line 4\n",
        "line 4 (mine)\n",
    );
    s.upstream(
        "badge/component.rs",
        &ORIGINAL.replace("line 4\n", "line 4 (registry)\n"),
    );
    let plan = s.update();
    assert_eq!(action(&plan, "badge/component.rs"), Action::Conflict);
    assert!(plan.has_conflicts());
    let text = s.read("badge/component.rs");
    for marker in [
        CONFLICT_START,
        "line 4 (mine)",
        "||||||| registry when copied",
        "line 4\n",
        "=======",
        "line 4 (registry)",
        ">>>>>>> registry now",
    ] {
        assert!(text.contains(marker), "{marker:?} missing from\n{text}");
    }
    assert!(text.starts_with("line 1\nline 2\nline 3\n"));
    // Until the markers are gone, later updates leave the file alone and keep failing.
    let again = s.plan(&[]);
    assert_eq!(action(&again, "badge/component.rs"), Action::Unresolved);
    assert!(again.has_conflicts());
    // Resolved: back to a normal edited file.
    fs::write(
        s.local("badge/component.rs"),
        ORIGINAL.replace("line 4\n", "line 4 (both)\n"),
    )
    .unwrap();
    assert_eq!(
        action(&s.plan(&[]), "badge/component.rs"),
        Action::KeptEdits
    );
}

#[test]
fn edits_are_kept_when_the_registry_did_not_change() {
    let s = Setup::new();
    s.add(&["badge"]);
    edit(&s.local("badge/component.rs"), "line 2", "line two");
    s.upstream(
        "badge/mod.rs",
        "mod component;\npub use component::*; // v2\n",
    );
    let plan = s.update();
    assert_eq!(action(&plan, "badge/component.rs"), Action::KeptEdits);
    assert_eq!(action(&plan, "badge/mod.rs"), Action::Updated);
    assert!(s.read("badge/component.rs").contains("line two"));
}

#[test]
fn files_edited_before_recording_are_skipped() {
    let s = Setup::new();
    // Copied by hand (or by an older CLI) and edited: `add` keeps it and records nothing.
    fs::create_dir_all(s.local("badge")).unwrap();
    fs::write(s.local("badge/component.rs"), "mine\n").unwrap();
    s.add(&["badge"]);
    let lock = Lock::load(&s.lock()).unwrap();
    assert!(!lock.items["badge"].files.contains_key("badge/component.rs"));
    s.upstream("badge/component.rs", "new\n");
    let plan = s.update();
    assert_eq!(action(&plan, "badge/component.rs"), Action::NoBase);
    assert!(plan.has_conflicts());
    assert_eq!(s.read("badge/component.rs"), "mine\n");
}

#[test]
fn a_missing_pristine_copy_falls_back_to_the_hash() {
    let s = Setup::new();
    s.add(&["badge"]);
    fs::remove_dir_all(update::base_dir(&s.lock())).unwrap();
    s.upstream("badge/component.rs", "new\n");
    let plan = s.update();
    assert_eq!(action(&plan, "badge/component.rs"), Action::Updated);
    assert_eq!(s.read("badge/component.rs"), "new\n");
}

#[test]
fn new_files_and_dependencies_are_added_and_removed_files_untracked() {
    let s = Setup::new();
    s.add(&["badge"]);
    // badge gains a file and a dependency on button; later it drops the file again.
    s.upstream("badge/extra.rs", "pub fn extra() {}\n");
    s.upstream(
        "badge/component.rs",
        &format!("use crate::components::button::*;\n{ORIGINAL}"),
    );
    let plan = s.update();
    assert_eq!(action(&plan, "badge/extra.rs"), Action::Added);
    assert_eq!(action(&plan, "button/mod.rs"), Action::Added);
    assert_eq!(
        plan.new_items
            .iter()
            .map(|(i, _)| i.name.as_str())
            .collect::<Vec<_>>(),
        ["button"]
    );
    assert!(s.local("button/mod.rs").is_file());
    let lock = Lock::load(&s.lock()).unwrap();
    assert!(lock.items.contains_key("button"));

    fs::remove_file(s.root.join(COMPONENTS_DIR).join("badge/extra.rs")).unwrap();
    s.regen();
    let plan = s.update();
    assert_eq!(action(&plan, "badge/extra.rs"), Action::RemovedUpstream);
    assert!(s.local("badge/extra.rs").is_file(), "local copy is kept");
    let lock = Lock::load(&s.lock()).unwrap();
    assert!(!lock.items["badge"].files.contains_key("badge/extra.rs"));
}

#[test]
fn deleted_files_are_not_restored() {
    let s = Setup::new();
    s.add(&["badge"]);
    fs::remove_file(s.local("badge/component.rs")).unwrap();
    s.upstream("badge/component.rs", "new\n");
    let plan = s.update();
    assert_eq!(action(&plan, "badge/component.rs"), Action::DeletedLocally);
    assert!(!s.local("badge/component.rs").exists());
}

#[test]
fn dry_run_shows_a_diff_and_writes_nothing() {
    let s = Setup::new();
    s.add(&["badge"]);
    let lock_before = fs::read_to_string(s.lock()).unwrap();
    s.upstream(
        "badge/component.rs",
        &ORIGINAL.replace("line 4", "line four"),
    );
    let plan = s.plan(&[]);
    let diff = update::render_diff(&plan);
    assert!(diff.contains("-line 4\n+line four\n"), "{diff}");
    assert!(diff.contains("badge/component.rs"), "{diff}");
    assert_eq!(s.read("badge/component.rs"), ORIGINAL);
    assert_eq!(fs::read_to_string(s.lock()).unwrap(), lock_before);
}

#[test]
fn only_named_items_are_updated_and_unknown_names_are_refused() {
    let s = Setup::new();
    s.add(&["badge", "button"]);
    s.upstream("badge/component.rs", "new\n");
    s.upstream("button/mod.rs", "pub fn button2() {}\n");
    let plan = s.plan(&["button"]);
    assert!(plan.files.iter().all(|f| f.item == "button"));
    let names = vec!["card".to_string()];
    let err = update::plan(
        &s.root,
        &load(&s.root).unwrap(),
        &Lock::load(&s.lock()).unwrap(),
        &s.lock(),
        &names,
        &s.dest(),
        &s.project.join("src/blocks"),
    )
    .unwrap_err();
    assert!(err.contains("rust-ui add card"), "{err}");
}

#[test]
fn merge_relabels_conflict_sides() {
    let err = update::merge(b"a\n", b"b\n", b"c\n", "base", "new").unwrap_err();
    assert_eq!(
        String::from_utf8(err).unwrap(),
        "<<<<<<< yours\nb\n||||||| base\na\n=======\nc\n>>>>>>> new\n"
    );
    assert_eq!(
        update::merge(b"a\nb\nc\n", b"A\nb\nc\n", b"a\nb\nC\n", "base", "new").unwrap(),
        b"A\nb\nC\n"
    );
}
