//! `atoll theme --preset <name>`: one of the kit's theme presets (`[data-preset]` in
//! theme.css) as a plain `:root` block, for an app that wants one look baked into its own
//! stylesheet instead of the attribute.

/// The preset names in `theme_css`, in file order (`lagoon` first: the default).
pub fn names(theme_css: &str) -> Vec<String> {
    blocks(theme_css).into_iter().map(|(n, _)| n).collect()
}

/// `(name, declarations)` of every `[data-preset="name"] { ... }` rule at the top level.
fn blocks(theme_css: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for chunk in theme_css.split("\n[data-preset=\"").skip(1) {
        let Some(end) = chunk.find('"') else { continue };
        let name = &chunk[..end];
        let rest = &chunk[end..];
        if !rest.starts_with("\"] {") {
            continue;
        }
        let Some(close) = rest.find("\n}") else {
            continue;
        };
        let body = &rest[rest.find('{').unwrap_or(0) + 1..close];
        out.push((name.to_string(), body.trim_matches('\n').to_string()));
    }
    out
}

/// The preset `name` as `:root` and subtree tokens. Load the result after the kit's
/// theme.css. The tokens also go on `:where([data-theme])`, so a light or dark island inside
/// the page resolves them too (the colours are `light-dark()` pairs, resolved where declared).
pub fn css(theme_css: &str, name: &str) -> Result<String, String> {
    let all = blocks(theme_css);
    let Some((_, body)) = all.iter().find(|(n, _)| n == name) else {
        let known: Vec<&str> = all.iter().map(|(n, _)| n.as_str()).collect();
        return Err(format!(
            "no theme preset `{name}`; the kit has: {}",
            known.join(", ")
        ));
    };
    // An explicit data-accent / data-density on <html> must still win, as it does over a
    // preset attribute: those tokens go on `:root:not([...])`, which outranks the kit's
    // `[data-accent]` rules only where there is none.
    let (mut plain, mut accent, mut density) = (String::new(), String::new(), String::new());
    for line in body.lines() {
        let t = line.trim_start();
        let target = if ACCENT_TOKENS
            .iter()
            .any(|a| t.starts_with(&format!("{a}:")))
        {
            &mut accent
        } else if t.starts_with("--density:") {
            &mut density
        } else {
            &mut plain
        };
        target.push_str(line);
        target.push('\n');
    }
    let mut out = format!(
        "/* Atoll theme preset \"{name}\" as the page default (atoll theme --preset {name}).\n * Load after the kit's theme.css. data-accent and data-density still override it. */\n:root,\n:where([data-theme]) {{\n{plain}}}\n"
    );
    if !accent.is_empty() {
        out.push_str(&format!(
            ":root:not([data-accent]),\n:where(:root:not([data-accent]) [data-theme]) {{\n{accent}}}\n"
        ));
    }
    if !density.is_empty() {
        out.push_str(&format!(":root:not([data-density]) {{\n{density}}}\n"));
    }
    Ok(out)
}

/// The tokens an accent preset sets.
const ACCENT_TOKENS: [&str; 3] = [
    "--accent-color",
    "--accent-contrast",
    "--focused-border-color",
];

#[cfg(test)]
mod tests {
    use super::*;

    const THEME: &str = ":root {\n  --a: 1;\n}\n[data-preset=\"lagoon\"] {\n  --a: 1;\n}\n[data-preset=\"reef\"] {\n  --a: light-dark(#fff, #000);\n  --font-sans: \"Nunito Variable\", sans-serif;\n}\n:where([data-preset=\"reef\"] [data-theme]) {\n  --a: 2;\n}\n";

    #[test]
    fn lists_presets_in_file_order() {
        assert_eq!(names(THEME), ["lagoon", "reef"]);
    }

    #[test]
    fn prints_a_preset_as_the_root_block() {
        let out = css(THEME, "reef").unwrap();
        assert!(out.contains(":root,\n:where([data-theme]) {\n  --a: light-dark(#fff, #000);\n  --font-sans: \"Nunito Variable\", sans-serif;\n}"), "{out}");
        let kit = css(include_str!("../../ui/assets/theme.css"), "reef").unwrap();
        assert!(
            kit.contains(":root:not([data-accent]),\n:where(:root:not([data-accent]) [data-theme]) {\n  --accent-color:"),
            "{kit}"
        );
        assert!(
            kit.contains(":root:not([data-density]) {\n  --density: 1.08;\n}"),
            "{kit}"
        );
        assert!(css(THEME, "nope").unwrap_err().contains("lagoon, reef"));
    }

    #[test]
    fn the_kit_ships_its_presets() {
        let kit = include_str!("../../ui/assets/theme.css");
        assert_eq!(
            names(kit),
            ["lagoon", "reef", "abyss", "driftwood", "classic"]
        );
        assert!(css(kit, "abyss").unwrap().contains("Space Grotesk"));
    }
}
