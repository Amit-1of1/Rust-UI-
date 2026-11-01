//! The stylesheets a screen is drawn with. `dx` bundles CSS at build time and a test has no
//! bundler, so the harness reads the same files and hashes `#[css_module]` class names exactly
//! the way the macro does (the hash is taken from the file's absolute path).

use std::path::{Path, PathBuf};

/// The kit's source folder: `crates/ui`, next to this crate in the same checkout (the git
/// checkout Cargo made, or this repository). That is why `atoll-test` has to come from the
/// same git URL and `rev` as `atoll-ui`.
pub fn kit_dir() -> PathBuf {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("../ui");
    here.canonicalize().unwrap_or(here)
}

/// The kit's CSS as `dx` serves it: `theme.css` (and `reset.css` when `reset` is set) as they
/// are, then every component's `style.css` with its class names hashed like `#[css_module]`.
pub fn kit_css(reset: bool) -> String {
    let root = kit_dir();
    let mut css = std::fs::read_to_string(root.join("assets/theme.css")).unwrap_or_else(|e| {
        panic!(
            "atoll-test: cannot read the kit's theme.css under {} ({e}). atoll-test must come \
             from the same git URL and rev as atoll-ui.",
            root.display()
        )
    });
    if reset {
        if let Ok(r) = std::fs::read_to_string(root.join("assets/reset.css")) {
            css.push('\n');
            css.push_str(&r);
        }
    }
    css.push_str(&css_modules_in(&root.join("src/components")));
    css
}

/// Every `<dir>/<name>/style.css`, hashed like `#[css_module]`. Use it for components copied
/// into an app with `atoll add` (they keep their CSS in `src/components/<name>/style.css`).
pub fn css_modules_in(dir: &Path) -> String {
    let mut out = String::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    let mut dirs: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    dirs.sort();
    for d in dirs {
        let file = d.join("style.css");
        if file.is_file() {
            out.push_str(&css_module(&file));
        }
    }
    out
}

/// One CSS module, hashed like `#[css_module("/path/style.css")]` hashes it. Files the macro
/// never sees (no `#[css_module]`, comments only) may not parse; those give an empty string.
pub fn css_module(file: &Path) -> String {
    let file = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());
    let Ok(src) = std::fs::read_to_string(&file) else {
        return String::new();
    };
    let hash = manganis_core::create_module_hash(&file);
    match manganis_core::transform_css(&src, &hash) {
        Ok(css) => format!("\n{css}"),
        Err(_) => String::new(),
    }
}

/// A path relative to the package under test (`CARGO_MANIFEST_DIR` of the test binary, which
/// `cargo test` sets), or the path itself when it is absolute.
pub fn app_path(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    if path.is_absolute() {
        return path.to_path_buf();
    }
    match std::env::var_os("CARGO_MANIFEST_DIR") {
        Some(root) => Path::new(&root).join(path),
        None => path.to_path_buf(),
    }
}
