use rust_ui_cli::info::{gather, render, SKILL_FILE};
use rust_ui_cli::*;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn empty_app_reports_what_is_missing() {
    let app = tempfile::tempdir().unwrap();
    let info = gather(app.path(), Path::new(update::LOCK_FILE)).unwrap();
    assert!(info.dioxus.is_none() && info.theme.is_none() && info.lockfile.is_none());
    assert!(info.installed.is_empty() && !info.mcp && info.skill.is_none());
    let text = render(&info);
    assert!(text.contains("rust-ui init"), "{text}");
    assert!(text.contains("nothing yet"), "{text}");
}

#[test]
fn reports_dioxus_theme_items_edits_and_conflicts() {
    let root = repo_root();
    let app = tempfile::tempdir().unwrap();
    let p = app.path();
    std::fs::write(
        p.join("Cargo.toml"),
        "[package]\nname = \"app\"\n\n[dependencies]\ndioxus = { version = \"0.7.1\", features = [\"web\"] }\n",
    )
    .unwrap();
    std::fs::create_dir_all(p.join("assets")).unwrap();
    std::fs::copy(root.join(THEME_FILE), p.join("assets/theme.css")).unwrap();
    std::fs::write(
        p.join(".mcp.json"),
        r#"{"mcpServers":{"rust-ui":{"command":"rust-ui","args":["mcp"]}}}"#,
    )
    .unwrap();

    let items = resolve(&load(&root).unwrap(), &["button".into(), "badge".into()]).unwrap();
    let (dest, blocks) = (p.join("src/components"), p.join("src/blocks"));
    add_to(&root, &items, &dest, &blocks, false).unwrap();
    let lock = p.join(update::LOCK_FILE);
    update::record(&lock, &root, "kit", &items, &dest, &blocks).unwrap();

    let button = items.iter().find(|i| i.name == "button").unwrap();
    let edited = &button.files[0];
    let path = dest.join(edited);
    let mut text = std::fs::read_to_string(&path).unwrap();
    text.push_str("\n// my edit\n");
    std::fs::write(&path, text).unwrap();
    let badge = items.iter().find(|i| i.name == "badge").unwrap();
    std::fs::write(
        dest.join(&badge.files[0]),
        "<<<<<<< yours\na\n=======\nb\n>>>>>>> registry\n",
    )
    .unwrap();

    let info = gather(p, Path::new(update::LOCK_FILE)).unwrap();
    assert_eq!(info.dioxus.as_deref(), Some("0.7.1"));
    let theme = info.theme.as_ref().unwrap();
    assert!(theme.accents.iter().any(|a| a == "violet"), "{theme:?}");
    assert!(info.mcp);
    assert_eq!(info.source.as_deref(), Some("kit"));
    let b = info.installed.iter().find(|i| i.name == "button").unwrap();
    assert_eq!(b.edited, [edited.clone()]);
    assert!(b.dir.ends_with("src/components"), "{}", b.dir);
    let g = info.installed.iter().find(|i| i.name == "badge").unwrap();
    assert_eq!(g.conflicts, [badge.files[0].clone()]);

    let json = serde_json::to_value(&info).unwrap();
    assert!(json["installed"].is_array() && json["cliVersion"].is_string());
    assert!(render(&info).contains("CONFLICTS"));
}

#[test]
fn skill_file_has_frontmatter_and_names_real_items() {
    let text = std::fs::read_to_string(repo_root().join(SKILL_FILE)).unwrap();
    assert!(text.starts_with("---\nname: rust-ui\ndescription: "), "frontmatter");
    // Every snake_case item named in backticks in the skill must exist in the registry, so the
    // skill can't send agents to a component that was renamed or removed.
    let reg = load(&repo_root()).unwrap();
    let known: Vec<&str> = reg.items.iter().map(|i| i.name.as_str()).collect();
    let table = text.split("| Need | Use |").nth(1).unwrap().split("\n\n").next().unwrap();
    let mut checked = 0;
    for name in table.split('`').skip(1).step_by(2) {
        if name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            assert!(known.contains(&name), "SKILL.md names `{name}`, which the registry lacks");
            checked += 1;
        }
    }
    assert!(checked > 20, "{checked}");
}
