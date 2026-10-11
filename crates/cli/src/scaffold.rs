//! `rust-ui new <name>`: create a component with everything CLAUDE.md requires
//! (folder, prelude export, gallery demo, smoke and render tests), so only the real
//! component code is left to write.

use std::fs;
use std::path::{Path, PathBuf};

const MOD_RS: &str = "crates/ui/src/components/mod.rs";
const DEMOS_MOD: &str = "crates/ui/src/demos/mod.rs";
const DEMOS_DIR_REL: &str = "crates/ui/src/demos";
const COMMON_RS: &str = "crates/ui/tests/common/mod.rs";
const LIB_RS: &str = "crates/ui/src/lib.rs";
const DEMOS_RS: &str = "crates/ui/tests/demos.rs";
const RENDER_RS: &str = "crates/ui/tests/render.rs";
const GALLERY_RS: &str = "apps/gallery/src/main.rs";
/// New gallery sections go before this one (the motion sections come last).
const GALLERY_ANCHOR: &str = "title: \"Spring motion\"";

pub fn validate_name(name: &str) -> Result<(), String> {
    let mut chars = name.chars();
    let ok = chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && !name.ends_with('_')
        && !name.contains("__");
    if ok {
        Ok(())
    } else {
        Err(format!(
            "`{name}` is not a valid component name: use snake_case like `stat_card`"
        ))
    }
}

/// `stat_card` -> `StatCard`
pub fn pascal(name: &str) -> String {
    name.split('_')
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
                .unwrap_or_default()
        })
        .collect()
}

/// `stat_card` -> `Stat card`
fn title(name: &str) -> String {
    let words = name.replace('_', " ");
    let mut c = words.chars();
    c.next()
        .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
        .unwrap_or_default()
}

/// Insert `new_line` among consecutive lines that start with `prefix`, keeping them sorted.
fn insert_sorted(text: &str, prefix: &str, new_line: &str) -> Result<String, String> {
    let lines: Vec<&str> = text.lines().collect();
    let positions: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.starts_with(prefix))
        .map(|(i, _)| i)
        .collect();
    let Some(&last) = positions.last() else {
        return Err(format!("no `{prefix}` lines found to insert next to"));
    };
    let at = positions
        .iter()
        .copied()
        .find(|&i| lines[i] > new_line)
        .unwrap_or(last + 1);
    let mut out: Vec<&str> = lines;
    out.insert(at, new_line);
    let mut s = out.join("\n");
    if text.ends_with('\n') {
        s.push('\n');
    }
    Ok(s)
}

fn component_rs(name: &str) -> String {
    let p = pascal(name);
    format!(
        r#"use dioxus::prelude::*;

#[css_module("/src/components/{name}/style.css")]
struct Styles;

/// TODO: say what `{p}` is for in one line.
#[component]
pub fn {p}(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {{
    rsx! {{
        div {{ class: Styles::dx_{name}, ..attributes, {{children}} }}
    }}
}}
"#
    )
}

fn style_css(name: &str) -> String {
    let class = name.replace('_', "-");
    format!(
        "/* Use theme tokens (var(--...)), never literal colors, so light and dark both work.\n   Any animation needs a prefers-reduced-motion fallback (docs/GUARDRAILS.md). */\n.dx-{class} {{\n  padding: 0.75rem 1rem;\n  border: 1px solid var(--primary-color-6);\n  border-radius: 0.5rem;\n  color: var(--secondary-color-4);\n}}\n"
    )
}

fn demo_rs(name: &str) -> String {
    let p = pascal(name);
    format!(
        "use crate::components::{name}::*;\nuse dioxus::prelude::*;\n\n#[component]\npub fn Demo() -> Element {{\n    rsx! {{\n        {p} {{ \"{} works\" }}\n    }}\n}}\n",
        title(name)
    )
}

fn docs_md(name: &str) -> String {
    format!(
        "# {p}\n\nTODO: what it is, when to use it.\n\n## Accessibility\n\n- Role and name: TODO\n- Keyboard: TODO\n- Reduced motion: TODO\n\n## Source\n\nOriginal to this repo.\n",
        p = pascal(name)
    )
}

/// Create the component and wire it in. Returns the paths written or changed.
pub fn scaffold(root: &Path, name: &str) -> Result<Vec<PathBuf>, String> {
    validate_name(name)?;
    let dir = root.join("crates/ui/src/components").join(name);
    if dir.exists() {
        return Err(format!("{} already exists", dir.display()));
    }
    let io = |e: std::io::Error| e.to_string();
    let read = |p: &str| fs::read_to_string(root.join(p)).map_err(|e| format!("{p}: {e}"));
    let p = pascal(name);

    // Compute every edit first so a bad anchor leaves the repo untouched.
    let mod_rs = insert_sorted(&read(MOD_RS)?, "pub mod ", &format!("pub mod {name};"))?;
    let lib_rs = insert_sorted(
        &read(LIB_RS)?,
        "    pub use crate::components::",
        &format!("    pub use crate::components::{name}::*;"),
    )?;
    let demos_mod = insert_sorted(&read(DEMOS_MOD)?, "pub mod ", &format!("pub mod {name};"))?;
    let common = read(COMMON_RS)?;
    let common_anchor = common
        .find("\n];\n\n/// Fails if")
        .ok_or("DEMOS list end not found in tests/common/mod.rs")?;
    let mut common_new = common.clone();
    common_new.insert_str(
        common_anchor + 1,
        &format!("    (\"{name}\", ui::demos::{name}::Demo),\n"),
    );
    let mut demos = read(DEMOS_RS)?;
    demos.push_str(&format!(
        "\n#[test]\nfn {name}_demo_renders() {{\n    let html = render(ui::demos::{name}::Demo);\n    assert!(!html.is_empty());\n}}\n"
    ));
    let mut render = read(RENDER_RS)?;
    render.push_str(&format!(
        "\n#[test]\nfn {name}_renders_its_children() {{\n    let html = render(|| rsx! {{ {p} {{ \"hello\" }} }});\n    assert!(html.contains(\"hello\"), \"{{html}}\");\n}}\n"
    ));
    let gallery = read(GALLERY_RS)?;
    let anchor = gallery
        .find(GALLERY_ANCHOR)
        .ok_or("gallery anchor `Spring motion` not found")?;
    let section_start = gallery[..anchor]
        .rfind("                Section {")
        .ok_or("gallery Section before the anchor not found")?;
    let line = format!(
        "                Section {{ title: \"{}\", blurb: \"TODO: describe {p}.\", ui::demos::{name}::Demo {{}} }}\n",
        title(name)
    );
    let mut gallery_new = gallery.clone();
    gallery_new.insert_str(section_start, &line);

    fs::create_dir_all(&dir).map_err(io)?;
    let files = [
        (
            "mod.rs",
            "mod component;\npub use component::*;\n".to_string(),
        ),
        ("component.rs", component_rs(name)),
        ("style.css", style_css(name)),
        ("docs.md", docs_md(name)),
    ];
    let mut touched = Vec::new();
    for (file, body) in files {
        fs::write(dir.join(file), body).map_err(io)?;
        touched.push(dir.join(file));
    }
    let demo_path = root.join(DEMOS_DIR_REL).join(format!("{name}.rs"));
    fs::write(&demo_path, demo_rs(name)).map_err(io)?;
    touched.push(demo_path);
    for (path, body) in [
        (MOD_RS, mod_rs),
        (DEMOS_MOD, demos_mod),
        (COMMON_RS, common_new),
        (LIB_RS, lib_rs),
        (DEMOS_RS, demos),
        (RENDER_RS, render),
        (GALLERY_RS, gallery_new),
    ] {
        fs::write(root.join(path), body).map_err(io)?;
        touched.push(root.join(path));
    }
    Ok(touched)
}
