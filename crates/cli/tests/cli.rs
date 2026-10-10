use rust_ui_cli::*;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn committed_registry_is_up_to_date() {
    let built = to_json(&build(&repo_root()).unwrap());
    let committed = std::fs::read_to_string(repo_root().join("registry.json")).unwrap();
    assert_eq!(
        built, committed,
        "run `cargo run -p rust-ui-cli -- registry`"
    );
}

#[test]
fn finds_imports() {
    let src = "use crate::components::button::Button;\nuse crate::components::dropdown_menu::*;";
    let found: Vec<_> = imports_in(src).into_iter().collect();
    assert_eq!(found, ["button", "dropdown_menu"]);
}

#[test]
fn resolves_dependencies_and_rejects_unknown() {
    let reg = load(&repo_root()).unwrap();
    let names: Vec<_> = resolve(&reg, &["command".into()])
        .unwrap()
        .into_iter()
        .map(|i| i.name)
        .collect();
    assert!(names.contains(&"command".to_string()));
    assert!(names.contains(&"dialog".to_string()), "{names:?}");
    assert!(resolve(&reg, &["nope".into()]).is_err());
}

#[test]
fn add_copies_then_keeps_edits() {
    let root = repo_root();
    let reg = load(&root).unwrap();
    let items = resolve(&reg, &["badge".into()]).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let (written, skipped) = add(&root, &items, tmp.path(), false).unwrap();
    assert!(!written.is_empty() && skipped.is_empty());
    // A second run changes nothing.
    let (written, _) = add(&root, &items, tmp.path(), false).unwrap();
    assert!(written.is_empty());
    // A local edit is kept unless forced.
    let file = tmp.path().join(&items[0].files[0]);
    std::fs::write(&file, "// mine").unwrap();
    let (written, skipped) = add(&root, &items, tmp.path(), false).unwrap();
    assert!(written.is_empty() && skipped.len() == 1);
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "// mine");
    let (written, _) = add(&root, &items, tmp.path(), true).unwrap();
    assert_eq!(written.len(), 1);
}

#[test]
fn recognises_remotes_and_names_their_cache() {
    assert!(is_remote("https://github.com/Amit-1of1/Rust-UI-"));
    assert!(is_remote("git@github.com:Amit-1of1/Rust-UI-.git"));
    assert!(!is_remote("../Rust-UI-"));
    assert!(!is_remote("."));
    assert_eq!(
        cache_name("https://github.com/a/b.git"),
        "https---github-com-a-b-git"
    );
}

#[test]
fn clones_a_remote_once_and_reuses_it() {
    let src = tempfile::tempdir().unwrap();
    let git = |args: &[&str], dir: &Path| {
        let ok = std::process::Command::new("git")
            .args(args)
            .current_dir(dir)
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@example.com")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@example.com")
            .status()
            .unwrap()
            .success();
        assert!(ok, "git {args:?}");
    };
    git(&["init", "-q", "."], src.path());
    std::fs::write(src.path().join("registry.json"), "{}").unwrap();
    git(&["add", "."], src.path());
    git(&["commit", "-q", "-m", "x"], src.path());
    let url = format!("file://{}", src.path().display());
    let cache = tempfile::tempdir().unwrap();
    let first = materialize(&url, cache.path()).unwrap();
    assert!(first.join("registry.json").is_file());
    let second = materialize(&url, cache.path()).unwrap();
    assert_eq!(first, second);
    // Local paths pass straight through.
    assert_eq!(materialize(".", cache.path()).unwrap(), PathBuf::from("."));
}
