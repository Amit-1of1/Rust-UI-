//! `atoll font`: put a font of your own into an app. From a file (`atoll font Brand.woff2`)
//! or by Google Fonts name (`atoll font --google "Plus Jakarta Sans"`, downloaded once from
//! the Fontsource packages on npm, the same source as the kit's own fonts). The file goes in
//! the app's `assets/fonts/` with its licence, and the command prints the `CustomFont` call
//! that declares it, so the app serves it itself and nothing is fetched at run time.

use std::path::{Path, PathBuf};
use std::process::Command;

/// What was added, for the printed instructions.
#[derive(Debug, Clone, PartialEq)]
pub struct Added {
    pub family: String,
    /// `(path inside the app, weights, style)` per file.
    pub faces: Vec<(String, String, String)>,
    pub licence: Option<String>,
}

/// `"plus-jakarta-sans"` for `"Plus Jakarta Sans"`.
pub fn slug(family: &str) -> String {
    let mut out = String::new();
    for c in family.trim().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
}

/// A family name from a font file's name: `PlusJakartaSans-VariableFont_wght.ttf` is
/// `Plus Jakarta Sans`, `brand_sans.woff2` is `Brand Sans`.
pub fn family_from_file(path: &Path) -> String {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Custom");
    let base = stem.split(['-', '[']).next().unwrap_or(stem);
    let mut out = String::new();
    let mut prev_lower = false;
    for c in base.chars() {
        if c == '_' || c == ' ' {
            if !out.ends_with(' ') {
                out.push(' ');
            }
            prev_lower = false;
            continue;
        }
        if c.is_ascii_uppercase() && prev_lower {
            out.push(' ');
        }
        let first = out.is_empty() || out.ends_with(' ');
        out.push(if first { c.to_ascii_uppercase() } else { c });
        prev_lower = c.is_ascii_lowercase() || c.is_ascii_digit();
    }
    out.trim().to_string()
}

const FONT_EXTS: [&str; 4] = ["woff2", "woff", "ttf", "otf"];

/// Copies `file` into `<app>/assets/fonts/`.
pub fn from_file(
    app: &Path,
    file: &Path,
    family: Option<&str>,
    weights: &str,
) -> Result<Added, String> {
    let ext = file
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();
    if !FONT_EXTS.contains(&ext.as_str()) {
        return Err(format!(
            "{}: not a font file (woff2, woff, ttf or otf)",
            file.display()
        ));
    }
    let family = family
        .map(str::to_string)
        .unwrap_or_else(|| family_from_file(file));
    let dir = app.join("assets/fonts");
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let name = format!("{}.{ext}", slug(&family));
    std::fs::copy(file, dir.join(&name)).map_err(|e| format!("{}: {e}", file.display()))?;
    Ok(Added {
        family,
        faces: vec![(
            format!("/assets/fonts/{name}"),
            weights.to_string(),
            "normal".into(),
        )],
        licence: None,
    })
}

fn run(cmd: &mut Command) -> Result<Vec<u8>, String> {
    let out = cmd
        .output()
        .map_err(|e| format!("could not run {:?}: {e}", cmd.get_program()))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(out.stdout)
}

fn fetch_json(url: &str) -> Result<serde_json::Value, String> {
    let body = run(Command::new("curl").args(["-fsSL", "-m", "60", url]))?;
    serde_json::from_slice(&body).map_err(|e| format!("{url}: {e}"))
}

/// The `font-weight` of the first `@font-face` in a Fontsource stylesheet.
pub fn weight_in_css(css: &str) -> Option<String> {
    let i = css.find("font-weight:")?;
    let rest = &css[i + "font-weight:".len()..];
    Some(rest[..rest.find(';')?].trim().to_string())
}

/// Downloads the Google font `family` (Latin subset) from its Fontsource package on npm:
/// the variable package when there is one (one file, every weight), else the regular and
/// bold files of the static package. Needs `curl` and `tar` (both ship with Windows 10+,
/// macOS and Linux).
pub fn from_google(app: &Path, family: &str, cache: &Path) -> Result<Added, String> {
    let id = slug(family);
    let tries = [
        (format!("@fontsource-variable/{id}"), true),
        (format!("@fontsource/{id}"), false),
    ];
    let mut last = String::new();
    for (pkg, variable) in tries {
        let meta = match fetch_json(&format!("https://registry.npmjs.org/{pkg}/latest")) {
            Ok(m) => m,
            Err(e) => {
                last = e;
                continue;
            }
        };
        let Some(tarball) = meta["dist"]["tarball"].as_str() else {
            continue;
        };
        let licence = meta["license"].as_str().map(str::to_string);
        let dir = cache.join(format!("{}-{}", pkg.replace('/', "-"), std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let tgz = dir.join("pkg.tgz");
        run(Command::new("curl")
            .args(["-fsSL", "-m", "120", "-o"])
            .arg(&tgz)
            .arg(tarball))?;
        run(Command::new("tar")
            .arg("-xzf")
            .arg(&tgz)
            .arg("-C")
            .arg(&dir))?;
        let pkg_dir = dir.join("package");
        let files = pkg_dir.join("files");
        let fonts = app.join("assets/fonts");
        std::fs::create_dir_all(&fonts).map_err(|e| format!("{}: {e}", fonts.display()))?;
        let mut faces = Vec::new();
        let wanted: Vec<(PathBuf, String, String)> = if variable {
            let css = std::fs::read_to_string(pkg_dir.join("index.css")).unwrap_or_default();
            let weights = weight_in_css(&css).unwrap_or_else(|| "100 900".into());
            vec![(
                files.join(format!("{id}-latin-wght-normal.woff2")),
                weights,
                format!("{id}.woff2"),
            )]
        } else {
            ["400", "700"]
                .iter()
                .map(|w| {
                    (
                        files.join(format!("{id}-latin-{w}-normal.woff2")),
                        w.to_string(),
                        format!("{id}-{w}.woff2"),
                    )
                })
                .filter(|(p, _, _)| p.exists())
                .collect()
        };
        for (src, weights, name) in wanted {
            if !src.exists() {
                continue;
            }
            std::fs::copy(&src, fonts.join(&name))
                .map_err(|e| format!("{}: {e}", src.display()))?;
            faces.push((
                format!("/assets/fonts/{name}"),
                weights,
                "normal".to_string(),
            ));
        }
        if faces.is_empty() {
            last = format!("{pkg} has no Latin woff2 file");
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(pkg_dir.join("LICENSE")) {
            let _ = std::fs::write(fonts.join(format!("{id}-LICENSE.txt")), text);
        }
        let _ = std::fs::remove_dir_all(&dir);
        let name = meta["keywords"]
            .as_array()
            .and_then(|k| {
                k.iter()
                    .filter_map(|v| v.as_str())
                    .find(|v| slug(v) == id && v.chars().any(|c| c.is_ascii_uppercase()))
            })
            .unwrap_or(family)
            .to_string();
        return Ok(Added {
            family: if variable {
                format!("{name} Variable")
            } else {
                name
            },
            faces,
            licence,
        });
    }
    Err(format!(
        "no Google font `{family}` found (looked for @fontsource-variable/{id} and @fontsource/{id} on npm): {last}"
    ))
}

/// The Rust to paste into the app's root component.
pub fn snippet(added: &Added, role: &str) -> String {
    let mut out = String::from("// After ThemeStyles in your root component:\n");
    for (i, (file, weights, style)) in added.faces.iter().enumerate() {
        let role = if i == 0 { role } else { "FaceOnly" };
        out.push_str(&format!(
            "ui::fonts::CustomFont {{\n    family: \"{}\",\n    file: asset!(\"{file}\"),\n    weights: \"{weights}\",\n{}{}}}\n",
            added.family,
            if style == "normal" { String::new() } else { format!("    style: \"{style}\",\n") },
            if role == "Sans" { String::new() } else { format!("    role: ui::fonts::FontRole::{role},\n") },
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_from_files() {
        assert_eq!(slug("Plus Jakarta Sans"), "plus-jakarta-sans");
        assert_eq!(
            family_from_file(Path::new("PlusJakartaSans-VariableFont_wght.ttf")),
            "Plus Jakarta Sans"
        );
        assert_eq!(
            family_from_file(Path::new("brand_sans.woff2")),
            "Brand Sans"
        );
        assert_eq!(family_from_file(Path::new("Inter[wght].woff2")), "Inter");
    }

    #[test]
    fn copies_a_file_and_prints_the_call() {
        let app = tempfile::tempdir().unwrap();
        let src = app.path().join("BrandSans-Regular.woff2");
        std::fs::write(&src, b"wOF2").unwrap();
        let added = from_file(app.path(), &src, None, "400").unwrap();
        assert_eq!(added.family, "Brand Sans");
        assert!(app.path().join("assets/fonts/brand-sans.woff2").exists());
        let code = snippet(&added, "Sans");
        assert!(code.contains("family: \"Brand Sans\",\n    file: asset!(\"/assets/fonts/brand-sans.woff2\"),\n    weights: \"400\",\n}"), "{code}");
        assert!(snippet(&added, "Heading").contains("role: ui::fonts::FontRole::Heading"));
        assert!(from_file(app.path(), Path::new("x.txt"), None, "400").is_err());
    }

    #[test]
    fn reads_the_weight_range() {
        assert_eq!(
            weight_in_css("@font-face {\n  font-family: 'X Variable';\n  font-weight: 200 800;\n}")
                .as_deref(),
            Some("200 800")
        );
    }
}
