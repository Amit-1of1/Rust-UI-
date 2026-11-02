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

/// Which theme token a [`CustomFont`] takes over.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum FontRole {
    /// `--font-sans`: all text (and headings, unless a look sets its own heading font).
    #[default]
    Sans,
    /// `--font-heading`: `h1`-`h4` and titles only.
    Heading,
    /// `--font-mono`: code and keyboard keys.
    Mono,
    /// Only declare the face (another weight or style of a family a first `CustomFont`
    /// already put in a role).
    FaceOnly,
}

impl FontRole {
    fn token(self) -> Option<&'static str> {
        match self {
            FontRole::Sans => Some("--font-sans"),
            FontRole::Heading => Some("--font-heading"),
            FontRole::Mono => Some("--font-mono"),
            FontRole::FaceOnly => None,
        }
    }
}

/// The `@font-face` rule and token override for a font of your own (experimental).
/// `atoll font` writes the call for you.
pub fn custom_font_css(
    family: &str,
    file: &str,
    weights: &str,
    style: &str,
    role: FontRole,
) -> String {
    let format = match file.rsplit('.').next().map(|e| e.to_ascii_lowercase()) {
        Some(e) if e == "woff" => "woff",
        Some(e) if e == "ttf" => "truetype",
        Some(e) if e == "otf" => "opentype",
        _ => "woff2",
    };
    let mut css = format!(
        "@font-face{{font-family:\"{family}\";font-style:{style};font-display:swap;font-weight:{weights};src:url(\"{file}\") format(\"{format}\")}}"
    );
    if let Some(token) = role.token() {
        let fallback = if role == FontRole::Mono {
            "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace"
        } else {
            "ui-sans-serif, system-ui, -apple-system, \"Segoe UI\", Roboto, sans-serif"
        };
        // `:root:root` outranks a look's `[data-preset]` tokens on <html>, so your font wins
        // whichever look is on.
        css.push_str(&format!("\n:root:root{{{token}:\"{family}\", {fallback}}}"));
    }
    css
}

/// Use a font of your own in place of the look's (experimental). Put the file in your app's
/// `assets/` folder and render this once, after [`ThemeStyles`](crate::ThemeStyles):
///
/// ```rust,ignore
/// CustomFont {
///     family: "Plus Jakarta Sans Variable",
///     file: asset!("/assets/fonts/plus-jakarta-sans.woff2"),
///     weights: "200 800",
/// }
/// ```
///
/// The file is served from your app like any asset, so nothing is fetched from a font
/// service. Text shows at once in the system font and swaps when the file arrives.
/// `atoll font <file>` or `atoll font --google "<name>"` copies the file in and prints this
/// call. Check the font's licence allows web use (Google Fonts are OFL or Apache).
#[component]
pub fn CustomFont(
    /// The `font-family` name to declare.
    family: String,
    /// The font file: `asset!("/assets/fonts/x.woff2")` (woff2, woff, ttf or otf).
    file: Asset,
    /// The weights the file covers: `"100 900"` for a variable font, `"400"` for a static one.
    #[props(default = "100 900".to_string())]
    weights: String,
    /// `"normal"` or `"italic"`.
    #[props(default = "normal".to_string())]
    style: String,
    /// Which token it takes over.
    #[props(default)]
    role: FontRole,
) -> Element {
    let css = custom_font_css(&family, &file.to_string(), &weights, &style, role);
    rsx! {
        document::Style { {css} }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_font_declares_the_face_and_takes_the_token() {
        let css = custom_font_css(
            "Brand Sans",
            "/assets/brand.woff2",
            "200 800",
            "normal",
            FontRole::Sans,
        );
        assert!(css.contains("font-family:\"Brand Sans\";font-style:normal;font-display:swap;font-weight:200 800;src:url(\"/assets/brand.woff2\") format(\"woff2\")"), "{css}");
        assert!(
            css.contains(":root:root{--font-sans:\"Brand Sans\", ui-sans-serif"),
            "{css}"
        );
        let mono = custom_font_css("Code", "/a/code.ttf", "400", "normal", FontRole::Mono);
        assert!(
            mono.contains("format(\"truetype\")")
                && mono.contains("--font-mono:\"Code\", ui-monospace"),
            "{mono}"
        );
        let face = custom_font_css(
            "Brand Sans",
            "/a/b.woff2",
            "700",
            "italic",
            FontRole::FaceOnly,
        );
        assert!(!face.contains(":root"), "{face}");
    }
}
