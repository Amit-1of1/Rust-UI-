//! One stylesheet for the whole app instead of one per component.
//!
//! Each component's `#[css_module]` adds its own `<link rel="stylesheet">` the first time it
//! renders, so an app pays one request per component it shows. After a release build,
//! `atoll postbuild <public dir>` concatenates those files into one, links it from
//! `index.html` and lists the files it holds in a `data-atoll-bundle` attribute.
//! [`install`] (called by [`crate::ThemeStyles`]) reads that list and wraps the app's
//! [`Document`] so the per-component links for bundled files are never created.
//!
//! Without the bundle (`dx serve`, desktop, tests) nothing changes.
//!
//! Internal: not part of the public API (docs/STABILITY.md). Use [`crate::ThemeStyles`] and
//! `atoll postbuild`.

use dioxus::document::{Document, Eval, LinkProps, MetaProps, ScriptProps, StyleProps};
use std::rc::Rc;

/// Attribute on the bundle's `<link>`: the file names it holds, separated by spaces.
pub const BUNDLE_ATTR: &str = "data-atoll-bundle";

/// Stop components from linking stylesheets that the bundle already holds.
/// Does nothing when the page has no bundle.
pub fn install() {
    if let Some(files) = bundled_files() {
        let inner = dioxus::document::document();
        dioxus::core::provide_root_context(Rc::new(Bundled { inner, files }) as Rc<dyn Document>);
    }
}

#[cfg(target_arch = "wasm32")]
fn bundled_files() -> Option<Vec<String>> {
    let link = web_sys::window()?
        .document()?
        .query_selector(&format!("link[{BUNDLE_ATTR}]"))
        .ok()??;
    let list = link.get_attribute(BUNDLE_ATTR)?;
    Some(list.split_whitespace().map(str::to_string).collect())
}

#[cfg(not(target_arch = "wasm32"))]
fn bundled_files() -> Option<Vec<String>> {
    None
}

/// The file name at the end of an asset URL, without any query or fragment.
fn file_name(href: &str) -> &str {
    let path = href.split(['?', '#']).next().unwrap_or(href);
    path.rsplit('/').next().unwrap_or(path)
}

struct Bundled {
    inner: Rc<dyn Document>,
    files: Vec<String>,
}

impl Bundled {
    fn holds(&self, props: &LinkProps) -> bool {
        props.rel.as_deref() == Some("stylesheet")
            && props
                .href
                .as_deref()
                .is_some_and(|href| self.files.iter().any(|f| f == file_name(href)))
    }
}

impl Document for Bundled {
    fn eval(&self, js: String) -> Eval {
        self.inner.eval(js)
    }

    fn set_title(&self, title: String) {
        self.inner.set_title(title)
    }

    fn create_head_element(
        &self,
        name: &str,
        attributes: &[(&str, String)],
        contents: Option<String>,
    ) {
        self.inner.create_head_element(name, attributes, contents)
    }

    fn create_meta(&self, props: MetaProps) {
        self.inner.create_meta(props)
    }

    fn create_script(&self, props: ScriptProps) {
        self.inner.create_script(props)
    }

    fn create_style(&self, props: StyleProps) {
        self.inner.create_style(props)
    }

    fn create_link(&self, props: LinkProps) {
        if !self.holds(&props) {
            self.inner.create_link(props)
        }
    }

    fn create_head_component(&self) -> bool {
        self.inner.create_head_component()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_name_ignores_base_path_and_query() {
        assert_eq!(file_name("/assets/style-dxh12.css"), "style-dxh12.css");
        assert_eq!(file_name("/Rust-UI-/assets/a.css?v=2"), "a.css");
        assert_eq!(file_name("a.css"), "a.css");
    }
}
