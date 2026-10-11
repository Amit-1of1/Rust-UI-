//! The spring easings in theme.css are generated from the Rust presets
//! (`motion::css`); this fails if they drift. Regenerate with
//! `UPDATE_MOTION_CSS=1 cargo test -p rust-ui-kit --test motion_css`.

use motion::css::{stylesheet, BEGIN_MARKER, END_MARKER};

const THEME: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/theme.css");

#[test]
fn theme_css_motion_tokens_are_current() {
    let theme = std::fs::read_to_string(THEME).expect("read theme.css");
    let start = theme
        .find(BEGIN_MARKER)
        .expect("motion begin marker in theme.css");
    let end = theme
        .find(END_MARKER)
        .expect("motion end marker in theme.css")
        + END_MARKER.len();
    let current = &theme[start..end];
    let fresh = stylesheet();
    if current == fresh {
        return;
    }
    if std::env::var_os("UPDATE_MOTION_CSS").is_some() {
        let updated = format!("{}{}{}", &theme[..start], fresh, &theme[end..]);
        std::fs::write(THEME, updated).expect("write theme.css");
        return;
    }
    panic!("motion tokens in theme.css are stale; rerun with UPDATE_MOTION_CSS=1");
}
