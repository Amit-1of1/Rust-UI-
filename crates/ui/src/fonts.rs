//! The fonts the theme presets use, shipped with the kit so nothing is fetched from a font
//! service. All are under the SIL Open Font License 1.1; each licence sits beside its file in
//! `assets/fonts/` and is listed in THIRD_PARTY.md. Latin subsets of the variable fonts from
//! Fontsource 5.3 (one file per family covers every weight).
//!
//! [`ThemeStyles`](crate::ThemeStyles) declares them with `@font-face`. A browser downloads a
//! file only when text on the page uses that family, so declaring all of them costs a few
//! hundred bytes of CSS, and the `classic` preset downloads none.

use dioxus::prelude::*;

/// One `@font-face`: a family, its file and the weights the variable file spans.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontFace {
    /// The `font-family` name the theme tokens use (`--font-sans`, `--font-heading`, ...).
    pub family: &'static str,
    pub file: Asset,
    /// `font-weight` range, e.g. `"300 900"`.
    pub weights: &'static str,
}

/// Every font the presets use.
pub const FONT_FACES: [FontFace; 5] = [
    FontFace {
        family: "Inter Variable",
        file: asset!("/assets/fonts/inter.woff2"),
        weights: "100 900",
    },
    FontFace {
        family: "Geist Variable",
        file: asset!("/assets/fonts/geist.woff2"),
        weights: "100 900",
    },
    FontFace {
        family: "IBM Plex Sans Variable",
        file: asset!("/assets/fonts/ibm-plex-sans.woff2"),
        weights: "100 700",
    },
    FontFace {
        family: "Source Serif 4 Variable",
        file: asset!("/assets/fonts/source-serif-4.woff2"),
        weights: "200 900",
    },
    FontFace {
        family: "JetBrains Mono Variable",
        file: asset!("/assets/fonts/jetbrains-mono.woff2"),
        weights: "100 800",
    },
];

/// The Latin characters the files hold; other scripts fall through to the system fonts at
/// the end of each stack.
const LATIN: &str = "U+0000-00FF,U+0131,U+0152-0153,U+02BB-02BC,U+02C6,U+02DA,U+02DC,U+0304,U+0308,U+0329,U+2000-206F,U+20AC,U+2122,U+2191,U+2193,U+2212,U+2215,U+FEFF,U+FFFD";

/// The `@font-face` rules for [`FONT_FACES`]. `font-display: swap`: text shows at once in the
/// system font and switches when the file arrives, so fonts never hold up the first screen.
pub fn font_face_css() -> String {
    FONT_FACES
        .iter()
        .map(|f| {
            format!(
                "@font-face{{font-family:\"{}\";font-style:normal;font-display:swap;font-weight:{};src:url(\"{}\") format(\"woff2\");unicode-range:{LATIN}}}",
                f.family, f.weights, f.file
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
