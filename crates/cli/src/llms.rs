//! Text for coding agents: `llms.txt` (an index in the llmstxt.org shape), `llms-full.txt`
//! (every page in one file) and one page per component in `llms/<name>.md`. A page is the
//! component's `docs.md`, the Rust signatures of its public items and its demo, so it is
//! generated from the same sources as the registry and a test fails when it is stale.
//! Blocks (whole screens) get pages too, from their `docs.md` and signatures; their docs carry
//! the example. `rust-ui docs <name>` prints the same page.

use crate::docsgen::title;
use crate::manifest::description;
use crate::{build, Kind, Registry, DEMOS_DIR};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const INDEX: &str = "llms.txt";
pub const FULL: &str = "llms-full.txt";
pub const PAGES_DIR: &str = "llms";
/// Hand-written header of `llms.txt`: what the kit is and the rules for using it.
pub const INTRO: &str = "docs/llms-intro.md";

/// Guides linked from the index, in order.
const GUIDES: &[(&str, &str, &str)] = &[
    (
        "Installing components",
        "docs/INSTALL.md",
        "copy the source with `rust-ui` or `dx`, or depend on the crate from git",
    ),
    (
        "MCP server",
        "docs/MCP.md",
        "`rust-ui mcp` lets MCP clients list, search, read and add components",
    ),
    (
        "Theming",
        "docs/THEMING.md",
        "light and dark mode, accent presets, a brand colour, radius",
    ),
    (
        "Icons",
        "docs/ICONS.md",
        "the Lucide set and when an icon needs a label",
    ),
    (
        "Stability",
        "docs/STABILITY.md",
        "which items are stable, experimental or internal",
    ),
];

/// Files whose public items are not worth listing (generated, or one item per icon).
fn skip_source(path: &Path, text: &str) -> bool {
    path.extension().is_none_or(|e| e != "rs") || text.starts_with("// @generated")
}

/// The public items of one source file, as Rust: doc comments, `#[component]`/`#[props]`
/// attributes and signatures, with bodies dropped (enum and struct bodies are kept, since
/// their variants and fields are the API).
pub fn public_items(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut out = String::new();
    let mut pending: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("///") || line.starts_with("#[component]") {
            pending.push(line);
            i += 1;
            continue;
        }
        if line.starts_with("#[") {
            i += 1;
            continue;
        }
        // Re-exports from other crates (`pub use time::Date;`), not the folder's own modules.
        if let Some(path) = line.strip_prefix("pub use ") {
            let local = path.split("::").count() == 2 && path.ends_with("::*;");
            let relative = ["self::", "super::", "crate::"]
                .iter()
                .any(|p| path.starts_with(p));
            if path.contains("::") && !local && !relative {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(line.trim_end());
                out.push('\n');
            }
            pending.clear();
            i += 1;
            continue;
        }
        let item = [
            "pub fn ",
            "pub enum ",
            "pub struct ",
            "pub type ",
            "pub const ",
        ]
        .iter()
        .find(|p| line.starts_with(*p));
        if let Some(prefix) = item {
            let mut block = Vec::new();
            if *prefix == "pub fn " {
                // The signature runs to the first `{` outside an attribute (the body).
                while i < lines.len() {
                    let l = lines[i].trim_end();
                    i += 1;
                    match l.find('{').filter(|_| !l.contains("#[")) {
                        Some(at) => {
                            block.push(l[..at].trim_end().to_string());
                            break;
                        }
                        None => block.push(l.to_string()),
                    }
                    if l.ends_with(';') {
                        break;
                    }
                }
            } else if matches!(*prefix, "pub const " | "pub type ") {
                // The name and type; a constant's value is not API.
                let head = line
                    .split(" = ")
                    .next()
                    .unwrap_or(line)
                    .trim_end_matches(';');
                block.push(format!("{head};"));
                while i < lines.len() && !lines[i].trim_end().ends_with(';') {
                    i += 1;
                }
                i += 1;
            } else {
                // Up to the closing brace at column 0, or a one-line item.
                while i < lines.len() {
                    let l = lines[i].trim_end();
                    i += 1;
                    block.push(l.to_string());
                    if l == "}" || (block.len() == 1 && l.ends_with(';')) {
                        break;
                    }
                }
            }
            if !out.is_empty() {
                out.push('\n');
            }
            for p in pending.drain(..) {
                out.push_str(p);
                out.push('\n');
            }
            for b in block {
                out.push_str(&b);
                out.push('\n');
            }
            continue;
        }
        pending.clear();
        i += 1;
    }
    out
}

/// The `docs.md` body without its `# Title` line.
fn body(docs: &str) -> &str {
    match docs.strip_prefix("# ") {
        Some(rest) => rest.split_once('\n').map_or("", |(_, tail)| tail),
        None => docs,
    }
    .trim()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

/// The agent page for one component or block, or `None` when there is no such item.
pub fn page(root: &Path, reg: &Registry, name: &str) -> io::Result<Option<String>> {
    let Some(item) = reg.items.iter().find(|i| i.name == name) else {
        return Ok(None);
    };
    let dir = root.join(item.kind.dir()).join(name);
    let docs = read(&dir.join("docs.md"));
    let mut out = format!("# {}\n\n", title(&docs, name));
    // `dx components add` reads component.json files, which blocks don't have.
    out.push_str(&match item.kind {
        Kind::Component => format!(
            "- Copy into your app: `rust-ui add {name}` or `dx components add {name} --git https://github.com/Amit-1of1/Rust-UI-`"
        ),
        Kind::Block => format!("- Copy into your app: `rust-ui add {name}`"),
    });
    if !item.registry_dependencies.is_empty() {
        out.push_str(&format!(
            " (also copies {})",
            item.registry_dependencies
                .iter()
                .map(|d| format!("`{d}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    let lib = read(&root.join("crates/ui/src/lib.rs"));
    let from_crate = if item.kind == Kind::Block {
        format!("`use ui::blocks::{name}::*;` with the `rust-ui-kit` crate's `blocks` feature (internal)")
    } else if lib.contains(&format!("pub use components::{name};")) {
        format!("`use ui::{name}::{{...}};` (by path, no glob)")
    } else if !not_in_prelude(root).iter().any(|n| n == name) {
        // The prelude is generated from every component folder (crates/ui/build.rs).
        "`use ui::prelude::*;`".to_string()
    } else {
        format!("`use ui::components::{name}::*;`")
    };
    out.push_str(&format!(
        ".\n- Import: `use crate::{}::{name}::*;` after copying, or {from_crate}{}.\n\n",
        if item.kind == Kind::Block {
            "blocks"
        } else {
            "components"
        },
        if item.kind == Kind::Block {
            ""
        } else {
            " with the `rust-ui-kit` crate"
        },
    ));
    let text = body(&docs);
    if !text.is_empty() {
        out.push_str(text);
        out.push_str("\n\n");
    }

    let mut api = String::new();
    let mut sources: Vec<PathBuf> = item
        .files
        .iter()
        .map(|f| root.join(item.kind.dir()).join(f))
        .collect();
    sources.sort();
    for path in sources {
        let source = read(&path);
        if skip_source(&path, &source) {
            continue;
        }
        let items = public_items(&source);
        if !items.is_empty() {
            if !api.is_empty() {
                api.push('\n');
            }
            api.push_str(&items);
        }
    }
    if !api.is_empty() {
        out.push_str(&format!("## API\n\n```rust\n{api}```\n\n"));
    }

    // A block's docs.md already shows it in use; a component's demo is its example.
    let demo = match item.kind {
        Kind::Component => read(&root.join(DEMOS_DIR).join(format!("{name}.rs"))),
        Kind::Block => String::new(),
    };
    if !demo.is_empty() {
        out.push_str(&format!(
            "## Example\n\n```rust\n{}\n```\n",
            demo.trim_end()
        ));
    }
    Ok(Some(out.trim_end().to_string() + "\n"))
}

/// `llms.txt`: the intro, the guides and one line per component, then one per block.
pub fn index(root: &Path, reg: &Registry) -> io::Result<String> {
    let mut out = fs::read_to_string(root.join(INTRO))?.trim_end().to_string();
    out.push_str("\n\nPaths are relative to the repository root. `rust-ui docs <name>` prints a component page.\n\n## Docs\n\n");
    for (name, path, what) in GUIDES {
        out.push_str(&format!("- [{name}]({path}): {what}\n"));
    }
    out.push_str(&format!(
        "- [Everything]({FULL}): this index plus every component page in one file\n\n## Components\n\n"
    ));
    let mut blocks_heading = false;
    for item in &reg.items {
        if item.kind == Kind::Block && !blocks_heading {
            blocks_heading = true;
            out.push_str("\n## Blocks\n\n");
        }
        let docs = read(&root.join(item.kind.dir()).join(&item.name).join("docs.md"));
        let summary = description(&docs);
        let summary = summary
            .split(". ")
            .next()
            .unwrap_or("")
            .trim_end_matches('.');
        out.push_str(&format!(
            "- [{}]({PAGES_DIR}/{}.md)",
            title(&docs, &item.name),
            item.name
        ));
        if !summary.is_empty() {
            out.push_str(&format!(": {summary}"));
        }
        out.push('\n');
    }
    Ok(out)
}

/// Every generated file, as (path relative to the root, contents).
pub fn generate(root: &Path) -> io::Result<Vec<(PathBuf, String)>> {
    let reg = build(root)?;
    let index = index(root, &reg)?;
    let mut full = index.clone();
    let mut files = Vec::new();
    for item in &reg.items {
        let text = page(root, &reg, &item.name)?.unwrap_or_default();
        full.push_str("\n---\n\n");
        full.push_str(&text);
        files.push((
            PathBuf::from(PAGES_DIR).join(format!("{}.md", item.name)),
            text,
        ));
    }
    files.push((PathBuf::from(INDEX), index));
    files.push((PathBuf::from(FULL), full));
    Ok(files)
}

/// Write every file and remove pages of components that no longer exist.
pub fn write(root: &Path) -> io::Result<usize> {
    let files = generate(root)?;
    let pages = root.join(PAGES_DIR);
    fs::create_dir_all(&pages)?;
    for entry in fs::read_dir(&pages)? {
        let path = entry?.path();
        if !files.iter().any(|(p, _)| root.join(p) == path) {
            fs::remove_file(path)?;
        }
    }
    for (path, text) in &files {
        fs::write(root.join(path), text)?;
    }
    Ok(files.len())
}

/// Generated files that are missing, differ, or should no longer exist.
pub fn stale(root: &Path) -> io::Result<Vec<PathBuf>> {
    let files = generate(root)?;
    let mut stale: Vec<PathBuf> = files
        .iter()
        .filter(|(p, text)| fs::read_to_string(root.join(p)).ok().as_deref() != Some(text))
        .map(|(p, _)| p.clone())
        .collect();
    if let Ok(entries) = fs::read_dir(root.join(PAGES_DIR)) {
        for entry in entries.filter_map(Result::ok) {
            let rel = PathBuf::from(PAGES_DIR).join(entry.file_name());
            if !files.iter().any(|(p, _)| *p == rel) {
                stale.push(rel);
            }
        }
    }
    Ok(stale)
}

/// Components `ui::prelude` leaves out (`crates/ui/not_in_prelude.txt`, shared with build.rs).
fn not_in_prelude(root: &Path) -> Vec<String> {
    read(&root.join("crates/ui/not_in_prelude.txt"))
        .lines()
        .filter_map(|l| l.split('#').next())
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_signatures_and_drops_bodies() {
        let src = "use x;\n\n/// A button.\n#[component]\npub fn Button(\n    #[props(default)] size: Size,\n    children: Element,\n) -> Element {\n    rsx! {}\n}\n\nfn private() {}\n\n#[derive(Clone)]\npub enum Size {\n    Sm,\n    Lg,\n}\n\n/// Not attached.\nstruct Hidden;\n";
        assert_eq!(
            public_items(src),
            "/// A button.\n#[component]\npub fn Button(\n    #[props(default)] size: Size,\n    children: Element,\n) -> Element\n\npub enum Size {\n    Sm,\n    Lg,\n}\n"
        );
    }

    #[test]
    fn body_drops_the_title() {
        assert_eq!(body("# Button\n\nText.\n"), "Text.");
        assert_eq!(body("Text.\n"), "Text.");
    }
}
