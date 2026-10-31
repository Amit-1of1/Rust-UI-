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
fn blocks_are_listed_with_their_components() {
    let reg = load(&repo_root()).unwrap();
    let login = reg.items.iter().find(|i| i.name == "login_01").unwrap();
    assert_eq!(login.kind, Kind::Block);
    for dep in ["card", "form", "link"] {
        assert!(
            login.registry_dependencies.iter().any(|d| d == dep),
            "{dep} missing from {:?}",
            login.registry_dependencies
        );
    }
    // Only blocks carry a `kind` in registry.json, so component entries stay as they were.
    let json = std::fs::read_to_string(repo_root().join("registry.json")).unwrap();
    assert_eq!(
        json.matches("\"kind\"").count(),
        reg.items.iter().filter(|i| i.kind == Kind::Block).count()
    );
}

#[test]
fn add_puts_blocks_and_components_in_their_own_folders() {
    let root = repo_root();
    let items = resolve(&load(&root).unwrap(), &["login_01".into()]).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let (components, blocks) = (tmp.path().join("components"), tmp.path().join("blocks"));
    add_to(&root, &items, &components, &blocks, false).unwrap();
    assert!(blocks.join("login_01/block.rs").is_file());
    assert!(!components.join("login_01").exists());
    assert!(components.join("card/component.rs").is_file());
    assert!(components.join("form/component.rs").is_file());
    // Every component the block imports came along.
    let source = std::fs::read_to_string(blocks.join("login_01/block.rs")).unwrap();
    for name in imports_in(&source) {
        assert!(components.join(&name).is_dir(), "{name} not copied");
    }
    assert_eq!(mod_lines_of(&items, Kind::Block), "pub mod login_01;\n");
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

const ANCHORS: [&str; 1] = ["apps/gallery/src/main.rs"];

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rust-ui-scaffold-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for rel in ANCHORS {
        let to = dir.join(rel);
        std::fs::create_dir_all(to.parent().unwrap()).unwrap();
        std::fs::copy(repo_root().join(rel), to).unwrap();
    }
    dir
}

#[test]
fn scaffold_only_adds_files_and_a_gallery_section() {
    let dir = scratch("ok");
    let files = scaffold::scaffold(&dir, "status_dot").unwrap();
    assert_eq!(files.len(), 7);
    let comp = dir.join("crates/ui/src/components/status_dot");
    for f in ["mod.rs", "component.rs", "style.css", "docs.md"] {
        assert!(comp.join(f).is_file(), "{f}");
    }
    let read = |rel: &str| std::fs::read_to_string(dir.join(rel)).unwrap();
    assert!(dir.join("crates/ui/src/demos/status_dot.rs").is_file());
    // Module lists, prelude, demo list and test list come from crates/ui/build.rs.
    assert!(read("crates/ui/tests/components/status_dot.rs")
        .contains("status_dot_renders_its_children"));
    assert!(read("apps/gallery/src/main.rs").contains("ui::demos::status_dot::Demo"));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn scaffold_rejects_bad_and_duplicate_names() {
    let dir = scratch("err");
    for bad in ["", "Bad", "two words", "9lives", "a-b"] {
        assert!(scaffold::scaffold(&dir, bad).is_err(), "{bad:?}");
    }
    scaffold::scaffold(&dir, "status_dot").unwrap();
    assert!(scaffold::scaffold(&dir, "status_dot").is_err());
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn committed_component_manifests_are_up_to_date() {
    let stale = manifest::stale_manifests(&repo_root()).unwrap();
    assert!(
        stale.is_empty(),
        "run `cargo run -p rust-ui-cli -- registry`: {stale:?}"
    );
}

#[test]
fn committed_llms_files_are_up_to_date() {
    let stale = llms::stale(&repo_root()).unwrap();
    assert!(
        stale.is_empty(),
        "run `cargo run -p rust-ui-cli -- registry`: {stale:?}"
    );
}

#[test]
fn every_component_has_an_agent_page_with_its_example() {
    let root = repo_root();
    let reg = load(&root).unwrap();
    for item in &reg.items {
        let text = llms::page(&root, &reg, &item.name).unwrap().unwrap();
        assert!(text.contains(&format!("rust-ui add {}", item.name)));
        if root
            .join(DEMOS_DIR)
            .join(format!("{}.rs", item.name))
            .is_file()
        {
            assert!(
                text.contains("## Example"),
                "{} page has no example",
                item.name
            );
        }
    }
    assert!(llms::page(&root, &reg, "no_such_component")
        .unwrap()
        .is_none());
}

#[test]
fn blocks_have_agent_pages_from_their_own_folder() {
    let root = repo_root();
    let reg = load(&root).unwrap();
    let page = llms::page(&root, &reg, "login_01").unwrap().unwrap();
    assert!(page.starts_with("# Login 01\n"));
    assert!(page.contains("use crate::blocks::login_01::*;"));
    assert!(
        page.contains("pub fn Login01("),
        "signatures come from the block's source"
    );
    assert!(!page.contains("dx components add"), "dx can't copy blocks");
    let index = llms::index(&root, &reg).unwrap();
    let blocks = index
        .find("## Blocks")
        .expect("llms.txt has a Blocks section");
    assert!(index[blocks..].contains("(llms/login_01.md)"));
    assert!(!index[..blocks].contains("(llms/login_01.md)"));
}

#[test]
fn every_component_is_listed_in_the_root_manifest() {
    let root: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(repo_root().join("component.json")).unwrap())
            .unwrap();
    let members = root["members"].as_array().unwrap().len();
    let components = load(&repo_root())
        .unwrap()
        .items
        .iter()
        .filter(|i| i.kind.is_component())
        .count();
    assert_eq!(members, components);
}

#[test]
fn pro_items_are_marked_by_their_licence_file() {
    let tmp = tempfile::tempdir().unwrap();
    let components = tmp.path().join(COMPONENTS_DIR);
    let blocks = tmp.path().join(BLOCKS_DIR);
    for (dir, name) in [(&components, "button"), (&blocks, "pricing_02")] {
        std::fs::create_dir_all(dir.join(name)).unwrap();
        std::fs::write(dir.join(name).join("mod.rs"), "").unwrap();
    }
    std::fs::write(
        blocks.join("pricing_02/block.rs"),
        "use crate::components::button::Button;",
    )
    .unwrap();
    std::fs::write(blocks.join("pricing_02").join(PRO_LICENSE_FILE), "licence").unwrap();

    let reg = build(tmp.path()).unwrap();
    let tiers: Vec<_> = reg
        .items
        .iter()
        .map(|i| (i.name.as_str(), i.tier))
        .collect();
    assert_eq!(tiers, [("button", Tier::Free), ("pricing_02", Tier::Pro)]);
    // Only Pro items carry the field, so the free registry stays as it was.
    let json = to_json(&reg);
    assert_eq!(json.matches("\"tier\"").count(), 1, "{json}");
    assert!(json.contains("\"tier\": \"pro\""));
    assert_eq!(load_str(&json), reg);

    // `add` copies the licence with the block and names it in its reminder.
    let items = resolve(&reg, &["pricing_02".into()]).unwrap();
    let out = tempfile::tempdir().unwrap();
    add(tmp.path(), &items, &out.path().join("components"), false).unwrap();
    assert!(out
        .path()
        .join("blocks/pricing_02")
        .join(PRO_LICENSE_FILE)
        .is_file());
    let notice = pro_notice(&items).unwrap();
    assert!(
        notice.contains("pricing_02") && notice.contains(PRO_LICENSE_FILE),
        "{notice}"
    );
    assert!(!notice.contains("button"), "{notice}");
}

#[test]
fn the_public_registry_has_no_pro_items() {
    // Pro items live in the private Pro repository only (docs/pro/README.md), which marks
    // itself with a `.rust-ui-pro` file at its root.
    if repo_root().join(".rust-ui-pro").exists() {
        return;
    }
    let reg = load(&repo_root()).unwrap();
    assert!(pro_items(&reg.items).is_empty());
    assert_eq!(pro_notice(&reg.items), None);
}

fn load_str(json: &str) -> Registry {
    serde_json::from_str(json).unwrap()
}
