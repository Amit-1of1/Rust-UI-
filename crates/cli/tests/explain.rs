//! `rust-ui explain` on real compiler output: `fixtures/explain/cargo.jsonl` is what
//! `cargo check --message-format=json` printed for `fixtures/explain/src/main.rs`.

use rust_ui_cli::explain::{render, Api, Hint};
use std::path::{Path, PathBuf};

fn kit() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/explain")
}

fn hints() -> Vec<Hint> {
    let json = std::fs::read_to_string(fixture().join("cargo.jsonl")).expect("fixture");
    let report = Api::load(&kit(), &[]).explain(&json, &fixture());
    assert!(
        report.other.is_empty(),
        "every fixture error is about the kit: {:?}",
        report.other
    );
    report.hints
}

fn hint(at: &str) -> Hint {
    hints()
        .into_iter()
        .find(|h| h.at.starts_with(at))
        .unwrap_or_else(|| panic!("no hint at {at}"))
}

fn says(h: &Hint, text: &str) {
    let all = format!("{}\n{}", h.title, h.fix.join("\n"));
    assert!(all.contains(text), "expected {text:?} in:\n{all}");
}

#[test]
fn a_missing_required_prop_names_the_prop_and_its_docs() {
    // rustc: "this method takes 1 argument but 0 arguments were supplied".
    let h = hint("src/main.rs:20:");
    says(&h, "`Dialog` needs the `title` prop");
    says(&h, "accessible name");
    says(&h, r#"Dialog { title: "...", ... }"#);
    says(
        &hint("src/main.rs:21:"),
        "`AvatarImage` needs the `alt` prop",
    );
    says(
        &hint("src/main.rs:22:"),
        "`IconButton` needs the `label` prop",
    );
}

#[test]
fn an_unknown_prop_lists_the_real_ones() {
    // rustc suggests the HTML attribute `color`, which is not what was meant.
    let h = hint("src/main.rs:18:");
    says(&h, "`Button` has no prop `colour`");
    says(&h, "`variant`, `size`");
    says(&h, "HTML attributes");
}

#[test]
fn a_removed_variant_points_at_the_changelog() {
    let h = hint("src/main.rs:17:68");
    says(&h, "`ButtonSize::Icon` does not exist");
    says(&h, "`Xs`, `Sm`, `Default` and `Lg`");
    says(&h, "IconButton");
    let typo = hint("src/main.rs:17:42");
    says(&typo, "Did you mean `ButtonVariant::Primary`?");
}

#[test]
fn a_string_for_an_enum_is_found_on_its_own_line() {
    // rustc points at the whole `rsx!` block (line 16); the prop is on line 19.
    let h = hint("src/main.rs:19:18");
    says(&h, "`variant` takes a `ButtonVariant`, not a `&str`");
    says(&h, "`ButtonVariant::Primary`");
}

#[test]
fn missing_modules_say_whether_to_add_them() {
    says(
        &hint("src/components/mod.rs:1:1"),
        "Copy it: `rust-ui add button`",
    );
    says(
        &hint("src/main.rs:7:"),
        "there is no kit component `fancy_widget`",
    );
    says(&hint("src/main.rs:10:"), "Did you mean `dialog`?");
}

#[test]
fn project_copies_override_the_registry() {
    let dir = tempfile::tempdir().expect("tempdir");
    let button = dir.path().join("button");
    std::fs::create_dir_all(&button).expect("mkdir");
    std::fs::write(
        button.join("component.rs"),
        "#[component]\npub fn Button(\n    /// Your own prop.\n    #[props(default)] tone: u8,\n) -> Element { todo!() }\n",
    )
    .expect("write");
    let api = Api::load(&kit(), &[dir.path().to_path_buf()]);
    let props: Vec<&str> = api.components["Button"]
        .props
        .iter()
        .map(|p| p.name.as_str())
        .collect();
    assert_eq!(props, ["tone"]);
}

#[test]
fn every_kit_component_with_props_is_read() {
    let api = Api::load(&kit(), &[]);
    for name in [
        "Dialog",
        "IconButton",
        "AvatarImage",
        "Button",
        "Select",
        "Tabs",
    ] {
        assert!(api.components.contains_key(name), "{name} not found");
    }
    let dialog = &api.components["Dialog"];
    let title = dialog
        .props
        .iter()
        .find(|p| p.name == "title")
        .expect("title");
    assert!(!title.optional);
    assert_eq!(title.ty, "String");
    assert!(dialog
        .props
        .iter()
        .any(|p| p.name == "description" && p.optional));
    assert!(api.enums["ButtonVariant"].1.contains(&"Ghost".to_string()));
}

#[test]
fn rendered_text_is_one_block_per_error() {
    let json = std::fs::read_to_string(fixture().join("cargo.jsonl")).expect("fixture");
    let text = render(&Api::load(&kit(), &[]).explain(&json, &fixture()));
    assert_eq!(text.matches("\n\n").count(), 10, "{text}");
}
