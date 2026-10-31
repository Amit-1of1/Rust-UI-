//! A `--source` can be any git URL and a lockfile is just a file in the repo, so both are
//! untrusted input: nothing they say may make `add`, `update` or the MCP server touch files
//! outside the item folders.

use rust_ui_cli::update::Lock;
use rust_ui_cli::*;
use std::fs;
use std::path::Path;

fn registry_with(name: &str, file: &str) -> String {
    format!(
        r#"{{"name":"evil","theme":"theme.css","cargoDependencies":[],
        "items":[{{"name":"{name}","files":["{file}"],"registryDependencies":[]}}]}}"#
    )
}

fn load_from(json: &str) -> Result<Registry, String> {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("registry.json"), json).unwrap();
    load(tmp.path())
}

#[test]
fn the_committed_registry_is_valid() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    validate(&build(&root).unwrap()).unwrap();
}

#[test]
fn registry_paths_that_leave_the_item_folder_are_refused() {
    for file in [
        "../../.bashrc",
        "badge/../../x.rs",
        "badge/../x.rs",
        "/etc/passwd",
        "badge/sub/x.rs",
        "badge/..",
        "badge/",
        "other/x.rs",
        r"badge/..\\x.rs",
    ] {
        assert!(
            load_from(&registry_with("badge", file)).is_err(),
            "{file} was accepted"
        );
    }
    assert!(load_from(&registry_with("badge", "badge/mod.rs")).is_ok());
}

#[test]
fn item_names_must_be_module_names() {
    for name in ["..", "a/b", "Badge", "badge;\\nfn evil(){}", "", "1x"] {
        assert!(
            load_from(&registry_with(name, &format!("{name}/mod.rs"))).is_err(),
            "{name} was accepted"
        );
    }
}

#[cfg(unix)]
#[test]
fn add_does_not_copy_links_or_write_through_them() {
    let src = tempfile::tempdir().unwrap();
    let dir = src.path().join(COMPONENTS_DIR).join("badge");
    fs::create_dir_all(&dir).unwrap();
    let secret = src.path().join("secret");
    fs::write(&secret, "key").unwrap();
    std::os::unix::fs::symlink(&secret, dir.join("mod.rs")).unwrap();
    let reg = load_from(&registry_with("badge", "badge/mod.rs")).unwrap();
    let app = tempfile::tempdir().unwrap();
    let dest = app.path().join("components");
    assert!(add(src.path(), &reg.items, &dest, true).is_err());
    assert!(!dest.join("badge/mod.rs").exists());

    // A real file, but the app's folder is a link out of the app.
    fs::remove_file(dir.join("mod.rs")).unwrap();
    fs::write(dir.join("mod.rs"), "// badge").unwrap();
    let outside = tempfile::tempdir().unwrap();
    fs::create_dir_all(&dest).unwrap();
    std::os::unix::fs::symlink(outside.path(), dest.join("badge")).unwrap();
    assert!(add(src.path(), &reg.items, &dest, true).is_err());
    assert!(!outside.path().join("mod.rs").exists());
}

#[test]
fn lockfiles_that_point_outside_the_project_are_refused() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("rust-ui-lock.json");
    for (dir, file) in [
        ("../elsewhere", "badge/mod.rs"),
        ("/tmp", "badge/mod.rs"),
        ("src/components", "badge/../../../x"),
    ] {
        fs::write(
            &path,
            format!(
                r#"{{"version":1,"items":{{"badge":{{"dir":"{dir}","files":{{"{file}":"0"}}}}}}}}"#
            ),
        )
        .unwrap();
        assert!(Lock::load(&path).is_err(), "{dir} {file} was accepted");
    }
    fs::write(
        &path,
        r#"{"version":1,"items":{"badge":{"dir":"src/components","files":{"badge/mod.rs":"0"}}}}"#,
    )
    .unwrap();
    assert!(Lock::load(&path).is_ok());
}
