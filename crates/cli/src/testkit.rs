//! `atoll setup-tests`: get an app ready for `atoll-test` with one command.
//!
//! Two edits to the app's `Cargo.toml`, both additive and idempotent:
//!
//! - `[dev-dependencies] atoll-test`, from the same place as the app's `ui` (atoll-ui) line:
//!   the same git URL and rev/tag/branch, or the `crates/test` folder beside a path kit.
//!   atoll-test reads the kit's CSS from its own checkout, so the two must match.
//! - The kit's `[patch.crates-io]` section: the renderer fixes (docs/FORKS.md) the headless
//!   screen needs. Entries the app already has are left as they are.

use toml_edit::{value, Document, InlineTable, Item, Table};

/// What [`setup`] changed, for the message.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// `atoll-test` was added to `[dev-dependencies]` (false: it was already there).
    pub added_dep: bool,
    /// Patch entries added.
    pub patches: Vec<String>,
}

/// Edit `app` (the app's `Cargo.toml`) using `kit_root` (the kit's root `Cargo.toml`, for
/// its `[patch]` section). Returns the new text and what changed.
pub fn setup(app: &str, kit_root: &str) -> Result<(String, Report), String> {
    let mut doc: Document = app.parse().map_err(|e| format!("Cargo.toml: {e}"))?;
    let kit: Document = kit_root
        .parse()
        .map_err(|e| format!("kit Cargo.toml: {e}"))?;
    let mut report = Report::default();

    let has_dep = doc
        .get("dev-dependencies")
        .and_then(|d| d.get("atoll-test"))
        .is_some();
    if !has_dep {
        let dep = test_dep(&doc)?;
        if doc.get("dev-dependencies").is_none() {
            doc["dev-dependencies"] = Item::Table(Table::new());
        }
        doc["dev-dependencies"]["atoll-test"] = dep;
        report.added_dep = true;
    }

    if let Some(kit_patch) = kit
        .get("patch")
        .and_then(|p| p.get("crates-io"))
        .and_then(Item::as_table_like)
    {
        if doc.get("patch").is_none() {
            let mut t = Table::new();
            t.set_implicit(true);
            doc["patch"] = Item::Table(t);
        }
        if doc["patch"].get("crates-io").is_none() {
            let mut t = Table::new();
            t.decor_mut().set_prefix(
                "\n# The renderer fixes the kit is tested with (docs/FORKS.md in the kit's\n# repository). atoll-test and the native renderer use them; web builds don't.\n",
            );
            doc["patch"]["crates-io"] = Item::Table(t);
        }
        let ours = doc["patch"]["crates-io"]
            .as_table_like_mut()
            .ok_or("[patch.crates-io] in Cargo.toml is not a table")?;
        for (name, entry) in kit_patch.iter() {
            if ours.get(name).is_none() {
                let mut entry = entry.clone();
                if let Some(v) = entry.as_value_mut() {
                    v.decor_mut().clear();
                }
                ours.insert(name, entry);
                report.patches.push(name.to_string());
            }
        }
    }
    Ok((doc.to_string(), report))
}

/// `atoll-test`, from wherever the app's `ui` comes from.
fn test_dep(doc: &Document) -> Result<Item, String> {
    let ui = doc
        .get("dependencies")
        .and_then(|d| d.get("ui").or_else(|| d.get("atoll-ui")))
        .ok_or("the app has no `ui` (atoll-ui) dependency; add the kit first (docs/GETTING_STARTED.md)")?;
    let field = |k: &str| ui.get(k).and_then(Item::as_str).map(str::to_string);
    let mut t = InlineTable::new();
    if let Some(git) = field("git") {
        t.insert("git", git.as_str().into());
        for k in ["rev", "tag", "branch"] {
            if let Some(v) = field(k) {
                t.insert(k, v.as_str().into());
            }
        }
        return Ok(value(t));
    }
    if let Some(path) = field("path") {
        let trimmed = path.trim_end_matches('/');
        let base = trimmed
            .strip_suffix("crates/ui")
            .ok_or_else(|| format!("the kit's path {path:?} doesn't end in crates/ui; add atoll-test by hand (its folder is crates/test beside crates/ui)"))?;
        t.insert("path", format!("{base}crates/test").as_str().into());
        return Ok(value(t));
    }
    Err("atoll-test is not on crates.io yet. Use the kit from git (`atoll create --git`, or `ui = { package = \"atoll-ui\", git = \"...\", rev = \"...\" }`) and run this again, or add `atoll-test = { git = \"https://github.com/Amit-1of1/Rust-UI-\" }` under [dev-dependencies] with the same rev as the kit".into())
}

/// What to paste into the app after setup.
pub const EXAMPLE: &str = r#"#[cfg(test)]
mod tests {
    use super::*;
    use atoll_test::prelude::*;

    // Written for the `atoll create` starter screen; change the names to your own screen's.
    #[test]
    fn says_hello_by_name() {
        let mut screen = Screen::render(App);
        screen.type_into(label("Your name"), "Ada");
        screen.click(role("button").name("Say hello"));
        screen.assert_visible(text("Hello, Ada.").within(role("dialog")));
    }
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    const KIT: &str = r#"
[workspace]
members = []

[patch.crates-io]
# a comment
blitz-dom = { git = "https://example.com/blitz", rev = "aaa" }
dioxus-native-dom = { git = "https://example.com/dioxus", rev = "bbb" }
"#;

    #[test]
    fn git_kit_gets_the_same_git_and_rev() {
        let app = r#"[package]
name = "a"

[dependencies]
ui = { package = "atoll-ui", git = "https://github.com/x/y", rev = "abc" }
"#;
        let (out, report) = setup(app, KIT).unwrap();
        assert!(report.added_dep);
        assert_eq!(report.patches, ["blitz-dom", "dioxus-native-dom"]);
        assert!(
            out.contains(r#"atoll-test = { git = "https://github.com/x/y", rev = "abc" }"#),
            "{out}"
        );
        assert!(out.contains("[patch.crates-io]"), "{out}");
        assert!(out.contains(r#"rev = "bbb""#), "{out}");
        // Still valid TOML, and running again changes nothing.
        let (again, report) = setup(&out, KIT).unwrap();
        assert_eq!(again, out);
        assert_eq!(report, Report::default());
    }

    #[test]
    fn path_kit_points_at_the_sibling_folder() {
        let app = "[dependencies]\nui = { package = \"atoll-ui\", path = \"../kit/crates/ui\" }\n";
        let (out, _) = setup(app, KIT).unwrap();
        assert!(
            out.contains(r#"atoll-test = { path = "../kit/crates/test" }"#),
            "{out}"
        );
    }

    #[test]
    fn existing_patches_are_kept() {
        let app = "[dependencies]\nui = { git = \"g\" }\n\n[patch.crates-io]\nblitz-dom = { path = \"mine\" }\n";
        let (out, report) = setup(app, KIT).unwrap();
        assert_eq!(report.patches, ["dioxus-native-dom"]);
        assert!(out.contains(r#"blitz-dom = { path = "mine" }"#), "{out}");
    }

    #[test]
    fn crates_io_kit_explains_what_to_do() {
        let app = "[dependencies]\nui = { package = \"atoll-ui\", version = \"0.1\" }\n";
        let e = setup(app, KIT).unwrap_err();
        assert!(e.contains("not on crates.io"), "{e}");
    }

    #[test]
    fn the_repo_root_has_the_patches_the_harness_needs() {
        let root =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.toml"))
                .unwrap();
        let app = "[dependencies]\nui = { git = \"g\", rev = \"r\" }\n";
        let (_, report) = setup(app, &root).unwrap();
        for c in ["blitz-dom", "blitz-paint", "dioxus-native-dom"] {
            assert!(
                report.patches.iter().any(|p| p == c),
                "{c} missing: {report:?}"
            );
        }
    }
}
