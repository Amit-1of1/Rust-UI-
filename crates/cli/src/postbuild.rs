//! `atoll postbuild`: shrink a `dx build --release` web build in place.
//!
//! - [`strip_wasm`] drops the `name` and debug sections from every `.wasm`. `dx build
//!   --wasm-split` keeps them (the splitter needs them) and never removes them afterwards, which
//!   leaves about a fifth of the main module as dead weight.
//! - [`bundle_css`] turns one-stylesheet-per-component into one file.
//!
//! Each component's `#[css_module]` ships as its own `style-dxh….css` and is linked the first
//! time the component renders, so an app makes one request per component it shows. This
//! concatenates the theme and the component stylesheets the app actually uses into one
//! hashed file, links it from `index.html`, and lists the files it replaces in the link's
//! `data-atoll-bundle` attribute; `ui::css_bundle` reads that list at startup and skips
//! those links.
//!
//! A stylesheet counts as used when one of its hashed class names (`name-<8 hex>`) appears
//! in the app's wasm: class names reach the binary only through the `Styles` constants the
//! components actually read. Stylesheets with no hashed class are kept, to be safe.

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// The attribute `ui::css_bundle` looks for. Keep the two in sync.
pub const BUNDLE_ATTR: &str = "data-atoll-bundle";

#[derive(Debug, PartialEq)]
pub struct CssReport {
    /// The bundle's file name, inside `assets/`.
    pub file: String,
    /// Stylesheets copied into the bundle, in bundle order.
    pub held: Vec<String>,
    /// Component stylesheets the wasm never uses, left out.
    pub unused: Vec<String>,
}

/// Bundle the stylesheets of the dx web build in `public` (the folder with `index.html`).
pub fn bundle_css(public: &Path) -> Result<CssReport, String> {
    bundle_css_with(public, false)
}

/// [`bundle_css`] for an app that renders `ThemeStyles { scoped: true }`: the scoped theme
/// (`theme.scoped-….css`) goes into the bundle instead of the page-wide one, which such an
/// app never links. Both are in every build (one `ThemeStyles` serves both modes), so the
/// choice can't be read from the wasm. The reset stylesheets are never bundled; the page
/// links them itself when it asks for them.
pub fn bundle_css_with(public: &Path, scoped: bool) -> Result<CssReport, String> {
    let theme = if scoped { "theme.scoped-" } else { "theme-" };
    let assets = public.join("assets");
    let index_path = public.join("index.html");
    let index =
        fs::read_to_string(&index_path).map_err(|e| format!("{}: {e}", index_path.display()))?;
    if index.contains(BUNDLE_ATTR) {
        return Err(format!(
            "{} already links a bundle; rebuild before bundling again",
            index_path.display()
        ));
    }

    let mut wasm = Vec::new();
    let mut css = Vec::new();
    for path in files(&assets)? {
        let name = file_name(&path);
        if name.ends_with(".wasm") {
            wasm.extend(fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?);
        } else if name.ends_with(".css") && (name.starts_with("style-") || name.starts_with(theme))
        {
            css.push(path);
        }
    }
    if wasm.is_empty() {
        return Err(format!("no .wasm file in {}", assets.display()));
    }
    // The theme first, so components can override its element rules; then components by name.
    css.sort_by_key(|p| (!file_name(p).starts_with(theme), file_name(p).to_string()));

    let mut out = String::new();
    let mut held = Vec::new();
    let mut unused = Vec::new();
    for path in css {
        let name = file_name(&path).to_string();
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if name.starts_with("style-") && !used(&text, &wasm) {
            unused.push(name);
            continue;
        }
        out.push_str(&format!("/* {name} */\n{}\n", text.trim_end()));
        held.push(name);
    }
    if held.is_empty() {
        return Err("no stylesheets to bundle".into());
    }

    let mut hasher = DefaultHasher::new();
    out.hash(&mut hasher);
    let file = format!("atoll-{:016x}.css", hasher.finish());
    fs::write(assets.join(&file), &out).map_err(|e| e.to_string())?;

    // Link it the way dx links its own script, so a base path carries over.
    let prefix = asset_prefix(&index);
    let link = format!(
        "<link rel=\"stylesheet\" href=\"{prefix}{file}\" {BUNDLE_ATTR}=\"{}\">",
        held.join(" ")
    );
    let Some(head_end) = index.find("</head>") else {
        return Err(format!("no </head> in {}", index_path.display()));
    };
    let index = format!(
        "{}    {link}\n    {}",
        &index[..head_end],
        &index[head_end..]
    );
    fs::write(&index_path, index).map_err(|e| e.to_string())?;

    Ok(CssReport { file, held, unused })
}

/// Remove the `name` and `.debug*` custom sections from every `.wasm` under `public/assets`.
/// Returns the bytes saved. The file names stay: the result depends only on the original
/// contents, so a content-hashed name still means one content.
pub fn strip_wasm(public: &Path) -> Result<u64, String> {
    let mut saved = 0;
    for path in files(&public.join("assets"))? {
        if !file_name(&path).ends_with(".wasm") {
            continue;
        }
        let bytes = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let stripped = strip_sections(&bytes)
            .ok_or_else(|| format!("{}: not a wasm module", path.display()))?;
        if stripped.len() < bytes.len() {
            saved += (bytes.len() - stripped.len()) as u64;
            fs::write(&path, stripped).map_err(|e| e.to_string())?;
        }
    }
    Ok(saved)
}

/// A copy of the module without its `name` and `.debug*` custom sections.
fn strip_sections(wasm: &[u8]) -> Option<Vec<u8>> {
    if wasm.get(..4)? != b"\0asm" {
        return None;
    }
    let mut out = wasm.get(..8)?.to_vec();
    let mut at = 8;
    while at < wasm.len() {
        let start = at;
        let id = wasm[at];
        let (size, len) = leb128(wasm.get(at + 1..)?)?;
        let body = at + 1 + len;
        let end = body.checked_add(size)?;
        let section = wasm.get(body..end)?;
        let drop = id == 0 && {
            let (name_len, len) = leb128(section)?;
            let name = section.get(len..len + name_len)?;
            name == b"name" || name.starts_with(b".debug")
        };
        if !drop {
            out.extend_from_slice(&wasm[start..end]);
        }
        at = end;
    }
    Some(out)
}

/// An unsigned LEB128 number and how many bytes it took.
fn leb128(bytes: &[u8]) -> Option<(usize, usize)> {
    let mut value = 0usize;
    for (i, b) in bytes.iter().enumerate().take(5) {
        value |= usize::from(b & 0x7f) << (7 * i);
        if b & 0x80 == 0 {
            return Some((value, i + 1));
        }
    }
    None
}

fn files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_dir() {
            out.extend(files(&path)?);
        } else {
            out.push(path);
        }
    }
    Ok(out)
}

fn file_name(path: &Path) -> &str {
    path.file_name().and_then(|n| n.to_str()).unwrap_or("")
}

/// The URL prefix dx puts before `assets/` (e.g. `/./assets/` or `/Rust-UI-/assets/`).
fn asset_prefix(index: &str) -> String {
    for attr in ["src=\"", "href=\""] {
        for (at, _) in index.match_indices(attr) {
            let value = &index[at + attr.len()..];
            let value = &value[..value.find('"').unwrap_or(0)];
            if let Some(i) = value.find("assets/") {
                return value[..i + "assets/".len()].to_string();
            }
        }
    }
    "assets/".to_string()
}

/// Whether any `<class>-<8 hex>` name from this css module appears in the wasm.
fn used(css: &str, wasm: &[u8]) -> bool {
    let mut hashes: Vec<&str> = Vec::new();
    for (at, _) in css.match_indices('.') {
        let ident: &str = css[at + 1..]
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
            .next()
            .unwrap_or("");
        if let Some((_, hash)) = ident.rsplit_once('-') {
            if hash.len() == 8
                && hash.bytes().all(|b| b.is_ascii_hexdigit())
                && !hashes.contains(&hash)
            {
                hashes.push(hash);
            }
        }
    }
    hashes.is_empty()
        || hashes.iter().any(|h| {
            let needle = format!("-{h}");
            wasm.windows(needle.len()).any(|w| w == needle.as_bytes())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_counts_as_used_only_when_its_hash_is_in_the_wasm() {
        let css = ".badge-1a2b3c4d{color:red}.badge-1a2b3c4d.primary-1a2b3c4d{}";
        assert!(used(css, b"..badge-1a2b3c4d.."));
        assert!(!used(css, b"..badge-ffffffff.."));
        // No hashed class at all: keep it.
        assert!(used("button{color:red}", b""));
        // Decimals in declarations aren't class names.
        assert!(!used(".a-1a2b3c4d{opacity:.5}", b""));
    }

    #[test]
    fn prefix_follows_the_script_tag() {
        assert_eq!(
            asset_prefix(r#"<script src="/./assets/app-dxh1.js"></script>"#),
            "/./assets/"
        );
        assert_eq!(
            asset_prefix(r#"<link href="/Rust-UI-/assets/app.js">"#),
            "/Rust-UI-/assets/"
        );
        assert_eq!(asset_prefix("<html></html>"), "assets/");
    }

    #[test]
    fn bundles_used_stylesheets_and_links_them() {
        let dir = tempfile::tempdir().unwrap();
        let assets = dir.path().join("assets");
        fs::create_dir(&assets).unwrap();
        fs::write(
            dir.path().join("index.html"),
            "<html><head><script src=\"/./assets/app.js\"></script></head></html>",
        )
        .unwrap();
        fs::write(assets.join("app_bg.wasm"), "x card-0000000a y").unwrap();
        fs::write(assets.join("theme-dxh1.css"), ":root{--a:1}").unwrap();
        fs::write(assets.join("style-dxh2.css"), ".card-0000000a{}").unwrap();
        fs::write(assets.join("style-dxh3.css"), ".chart-0000000b{}").unwrap();
        fs::write(assets.join("app-dxh4.css"), ".mine{}").unwrap();

        let report = bundle_css(dir.path()).unwrap();
        assert_eq!(report.held, ["theme-dxh1.css", "style-dxh2.css"]);
        assert_eq!(report.unused, ["style-dxh3.css"]);
        let out = fs::read_to_string(assets.join(&report.file)).unwrap();
        assert!(out.find(":root").unwrap() < out.find(".card").unwrap());
        assert!(!out.contains(".mine"));
        let index = fs::read_to_string(dir.path().join("index.html")).unwrap();
        assert!(index.contains(&format!(
            "href=\"/./assets/{}\" data-atoll-bundle=\"theme-dxh1.css style-dxh2.css\"",
            report.file
        )));
        assert!(bundle_css(dir.path()).is_err(), "bundling twice is refused");
    }

    #[test]
    fn scoped_bundles_the_scoped_theme_only() {
        let dir = tempfile::tempdir().unwrap();
        let assets = dir.path().join("assets");
        fs::create_dir(&assets).unwrap();
        fs::write(dir.path().join("index.html"), "<html><head></head></html>").unwrap();
        fs::write(assets.join("app_bg.wasm"), "x card-0000000a y").unwrap();
        fs::write(assets.join("theme-dxh1.css"), ":root{--a:1}").unwrap();
        fs::write(
            assets.join("theme.scoped-dxh5.css"),
            "[data-rui-scope]{--a:1}",
        )
        .unwrap();
        fs::write(assets.join("reset-dxh6.css"), "html{}").unwrap();
        fs::write(assets.join("style-dxh2.css"), ".card-0000000a{}").unwrap();

        let report = bundle_css_with(dir.path(), true).unwrap();
        assert_eq!(report.held, ["theme.scoped-dxh5.css", "style-dxh2.css"]);
        let out = fs::read_to_string(assets.join(&report.file)).unwrap();
        assert!(!out.contains(":root"), "the page-wide theme stays out");
    }

    fn custom(name: &str, body: &[u8]) -> Vec<u8> {
        let mut payload = vec![name.len() as u8];
        payload.extend_from_slice(name.as_bytes());
        payload.extend_from_slice(body);
        let mut section = vec![0, payload.len() as u8];
        section.extend(payload);
        section
    }

    #[test]
    fn strips_names_and_debug_info_only() {
        let header = b"\0asm\x01\0\0\0".to_vec();
        let types = vec![1, 1, 0]; // a type section holding zero types
        let module = [
            header.clone(),
            types.clone(),
            custom("name", &[1, 2, 3]),
            custom(".debug_info", &[4]),
            custom("producers", &[5]),
        ]
        .concat();
        let stripped = strip_sections(&module).unwrap();
        assert_eq!(
            stripped,
            [header, types, custom("producers", &[5])].concat()
        );
        assert_eq!(strip_sections(&stripped).unwrap(), stripped);
        assert!(strip_sections(b"not wasm").is_none());
        // A section that claims more bytes than the file holds.
        assert!(strip_sections(b"\0asm\x01\0\0\0\x01\x7f").is_none());
    }

    #[test]
    fn leb128_reads_multi_byte_sizes() {
        assert_eq!(leb128(&[0x05]), Some((5, 1)));
        assert_eq!(leb128(&[0xe5, 0x8e, 0x26]), Some((624_485, 3)));
        assert_eq!(leb128(&[0x80]), None);
    }
}
