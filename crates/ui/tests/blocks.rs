//! Blocks render on the server with the structure their docs promise. Behaviour (submit,
//! focus, the code step) is checked in the browser by tests/e2e/blocks.mjs.

use common::{render, BLOCKS};

mod common;

#[test]
fn every_block_renders_one_h1_and_its_marker() {
    for (name, block) in BLOCKS {
        let html = render(*block);
        let id = name.trim_start_matches("block_");
        assert!(
            html.contains(&format!("data-block=\"{id}\"")),
            "{name} has no data-block marker"
        );
        assert_eq!(
            html.matches("<h1").count(),
            1,
            "{name} needs exactly one h1"
        );
    }
}

#[test]
fn every_block_folder_is_listed() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src/blocks");
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        // Pro blocks (a LICENSE-PRO.md in the folder) live only in the Pro repository, which
        // tests them with its own list (docs/pro/README.md).
        if path.is_dir() && !path.join("LICENSE-PRO.md").is_file() {
            let name = format!("block_{}", path.file_name().unwrap().to_string_lossy());
            assert!(
                BLOCKS.iter().any(|(n, _)| *n == name),
                "{name} is not in BLOCKS in tests/common/mod.rs"
            );
        }
    }
}

#[test]
fn login_fields_are_named_for_password_managers() {
    let html = render(BLOCKS[1].1);
    assert!(html.contains("autocomplete=\"email\""));
    assert!(html.contains("autocomplete=\"current-password\""));
    assert!(html.contains("aria-label=\"Sign in\""));
}

#[test]
fn signup_asks_for_a_new_password_twice() {
    let html = render(BLOCKS[3].1);
    assert_eq!(html.matches("autocomplete=\"new-password\"").count(), 2);
}

#[test]
fn dashboard_marks_the_current_page_and_has_one_main() {
    let html = render(BLOCKS[0].1);
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
    assert_eq!(html.matches("<main").count(), 1);
    assert!(html.contains(">Overview</h1>"));
}

#[test]
fn settings_has_four_tabs() {
    let html = render(BLOCKS[2].1);
    assert_eq!(html.matches("role=\"tab\"").count(), 4);
}
