//! Theme checks:
//! - every accent preset in `assets/theme.css` keeps its label text readable on the fill
//!   (WCAG AA, 4.5:1) in both light and dark;
//! - the fallback block for browsers without `light-dark()` matches the tokens. Regenerate it
//!   with `UPDATE_THEME_CSS=1 cargo test -p atoll-ui --test theme`;
//! - so does the subtree block that lets `data-theme` on any element switch its own subtree.

const THEME: &str = include_str!("../assets/theme.css");
const THEME_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/theme.css");
const FALLBACK_BEGIN: &str = "/* theme fallback: begin (generated, do not edit) */";
const FALLBACK_END: &str = "/* theme fallback: end */";
const SUBTREE_BEGIN: &str = "/* subtree themes: begin (generated, do not edit) */";
const SUBTREE_END: &str = "/* subtree themes: end */";

fn channel(v: u8) -> f64 {
    let c = f64::from(v) / 255.0;
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn luminance(hex: &str) -> f64 {
    let h = hex.trim_start_matches('#');
    let full = if h.len() == 3 {
        h.chars().flat_map(|c| [c, c]).collect::<String>()
    } else {
        h.to_string()
    };
    let v = |i: usize| u8::from_str_radix(&full[i..i + 2], 16).unwrap();
    0.2126 * channel(v(0)) + 0.7152 * channel(v(2)) + 0.0722 * channel(v(4))
}

fn contrast(a: &str, b: &str) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// Splits `name(a, b)` at its top-level comma: `light-dark(var(--x), #fff)` -> (`var(--x)`, `#fff`).
fn args(value: &str, name: &str) -> Option<(String, String)> {
    let inner = value
        .trim()
        .strip_prefix(name)?
        .strip_prefix('(')?
        .strip_suffix(')')?;
    let mut depth = 0;
    for (i, c) in inner.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                return Some((
                    inner[..i].trim().to_string(),
                    inner[i + 1..].trim().to_string(),
                ))
            }
            _ => {}
        }
    }
    None
}

/// `light-dark(#light, #dark)` -> (dark, light)
fn pair(value: &str) -> (String, String) {
    let (light, dark) =
        args(value, "light-dark").unwrap_or_else(|| panic!("not a light-dark() pair: {value}"));
    (dark, light)
}

/// The value of `--name:` inside the block that starts at `header`.
fn declared(block: &str, name: &str) -> String {
    let line = block
        .lines()
        .find(|l| l.trim_start().starts_with(name))
        .unwrap_or_else(|| panic!("{name} missing"));
    line.split_once(':')
        .unwrap()
        .1
        .trim()
        .trim_end_matches(';')
        .to_string()
}

#[test]
fn accent_presets_meet_aa_contrast_in_both_modes() {
    let mut checked = 0;
    for chunk in outside_fallback(THEME).split("[data-accent=\"").skip(1) {
        let name = &chunk[..chunk.find('"').unwrap()];
        let block = &chunk[..chunk.find('}').unwrap()];
        let (fill_dark, fill_light) = pair(&declared(block, "--accent-color"));
        let (text_dark, text_light) = pair(&declared(block, "--accent-contrast"));
        let dark = contrast(&fill_dark, &text_dark);
        let light = contrast(&fill_light, &text_light);
        assert!(dark >= 4.5, "{name} dark: {dark:.2}:1");
        assert!(light >= 4.5, "{name} light: {light:.2}:1");
        checked += 1;
    }
    assert!(checked >= 5, "expected at least the five shipped presets");
}

/// Chart series are graphics, so WCAG 1.4.11 asks for 3:1 against the page
/// (`--primary-color`: #fff in light, #000 in dark).
#[test]
fn chart_colours_meet_non_text_contrast_in_both_modes() {
    let theme = outside_fallback(THEME);
    for n in 2..=5 {
        let (dark, light) = pair(&declared(&theme, &format!("--chart-{n}:")));
        let on_dark = contrast(&dark, "#000");
        let on_light = contrast(&light, "#fff");
        assert!(on_dark >= 3.0, "--chart-{n} dark: {on_dark:.2}:1");
        assert!(on_light >= 3.0, "--chart-{n} light: {on_light:.2}:1");
    }
}

/// `theme` without the generated block between `begin` and `end`.
fn without(theme: &str, begin: &str, end: &str) -> String {
    match (theme.find(begin), theme.find(end)) {
        (Some(a), Some(b)) => format!("{}{}", &theme[..a], &theme[b + end.len()..]),
        _ => theme.to_string(),
    }
}

/// The theme without either generated block: what both are generated from.
fn outside_fallback(theme: &str) -> String {
    without(
        &without(theme, FALLBACK_BEGIN, FALLBACK_END),
        SUBTREE_BEGIN,
        SUBTREE_END,
    )
}

/// The subtree block. Browsers resolve `light-dark()` in a custom property on the element
/// that declares it, so tokens declared on `:root` reach a `[data-theme="dark"]` box inside a
/// light page already resolved to their light values. Declaring the colour tokens again on
/// every `[data-theme]` element (and the accent presets on `[data-theme]` elements inside a
/// `[data-accent]`) makes them resolve there. `:where()` keeps them at zero specificity, so
/// an element's own `data-accent` still wins; plain values such as `--radius` are left out,
/// so a value set on `<html>` still reaches the subtree.
fn subtree(tokens: &str) -> String {
    let mut out = format!(
        "{SUBTREE_BEGIN}\n/* data-theme on any element switches its subtree: the colour tokens again, resolved there.\n * Regenerate: UPDATE_THEME_CSS=1 cargo test -p atoll-ui --test theme */\n"
    );
    let mut selector = None;
    let mut body = String::new();
    for line in tokens.lines() {
        if let Some(sel) = line.strip_suffix(" {").filter(|_| !line.starts_with(' ')) {
            selector = Some(sel.to_string());
            body.clear();
        } else if line == "}" {
            let Some(sel) = selector.take().filter(|_| !body.is_empty()) else {
                continue;
            };
            let sel = if sel == ":root" {
                ":where([data-theme])".to_string()
            } else if sel.starts_with("[data-accent=") {
                format!(":where({sel} [data-theme])")
            } else {
                continue;
            };
            out.push_str(&format!("{sel} {{\n{body}}}\n"));
        } else if selector.is_some() {
            let t = line.trim();
            if t.starts_with("--") && (t.contains("light-dark(") || t.contains("var(")) {
                body.push_str(&format!("  {t}\n"));
            }
        }
    }
    out.push_str(SUBTREE_END);
    out
}

#[test]
fn subtree_block_is_current() {
    let theme = std::fs::read_to_string(THEME_PATH).expect("read theme.css");
    let start = theme.find(SUBTREE_BEGIN).expect("subtree begin marker");
    let end = theme.find(SUBTREE_END).expect("subtree end marker") + SUBTREE_END.len();
    let fresh = subtree(&outside_fallback(&theme));
    if theme[start..end] == fresh {
        return;
    }
    if std::env::var_os("UPDATE_THEME_CSS").is_some() {
        let updated = format!("{}{}{}", &theme[..start], fresh, &theme[end..]);
        std::fs::write(THEME_PATH, updated).expect("write theme.css");
        return;
    }
    panic!("the subtree block in theme.css is stale; rerun with UPDATE_THEME_CSS=1");
}

#[test]
fn subtree_redeclares_colours_not_plain_values() {
    let css = ":root {\n  color-scheme: light dark;\n}\n:root {\n  --a: light-dark(#fff, #000);\n  --b: var(--a);\n  --r: 1rem;\n}\n[data-accent=\"x\"] {\n  --a: light-dark(#111, #eee);\n}\n[data-density=\"compact\"] {\n  --r: 2rem;\n}\n";
    let out = subtree(css);
    assert!(
        out.contains(":where([data-theme]) {\n  --a: light-dark(#fff, #000);\n  --b: var(--a);\n}"),
        "{out}"
    );
    assert!(
        out.contains(
            ":where([data-accent=\"x\"] [data-theme]) {\n  --a: light-dark(#111, #eee);\n}"
        ),
        "{out}"
    );
    assert!(!out.contains("--r") && !out.contains("density"), "{out}");
    // The root block comes first, so a preset inside an accent outranks it by order.
    assert!(out.find(":where([data-theme])") < out.find(":where([data-accent"));
}

/// Repeats a selector so it outranks the plain one wherever it sits in the file (presets
/// appended after the block, too): `:root` -> `:root:root`.
fn doubled(selector: &str) -> String {
    format!("{selector}{selector}")
}

/// The fallback block: every `light-dark(l, d)` token again as the space toggle
/// `var(--dark, d) var(--light, l)`, for engines that do not parse `light-dark()`, and for
/// Blitz (dioxus-native), whose Stylo parses it but always picks the light value. Blitz is
/// told apart by `accent-color`, which its Stylo lacks and every browser with `light-dark()`
/// has (`color-scheme` would not do: Blitz turns on Stylo's unimplemented properties). Root tokens are also re-declared on `[data-theme]`, so a subtree can
/// switch theme on its own there too.
fn fallback(tokens: &str) -> String {
    let mut out = format!(
        "{FALLBACK_BEGIN}\n/* For browsers without light-dark(): the same tokens as space toggles.\n * Regenerate: UPDATE_THEME_CSS=1 cargo test -p atoll-ui --test theme */\n@supports not ((color: light-dark(#000, #fff)) and (accent-color: auto)) {{\n"
    );
    let mut selector = None;
    let mut body = String::new();
    for line in tokens.lines() {
        if let Some(sel) = line.strip_suffix(" {").filter(|_| !line.starts_with(' ')) {
            selector = Some(sel.to_string());
            body.clear();
        } else if line == "}" {
            if let Some(sel) = selector.take().filter(|_| !body.is_empty()) {
                let sel = match sel.as_str() {
                    ":root" => format!("{}, {}", doubled(":root"), doubled("[data-theme]")),
                    _ => doubled(&sel),
                };
                out.push_str(&format!("  {sel} {{\n{body}  }}\n"));
            }
        } else if let Some((name, value)) = line.trim().split_once(':') {
            let value = value.trim().trim_end_matches(';');
            if let Some((light, dark)) = args(value, "light-dark") {
                body.push_str(&format!(
                    "    {name}: var(--dark, {dark}) var(--light, {light});\n"
                ));
            }
        }
    }
    out.push_str("}\n");
    out.push_str(FALLBACK_END);
    out
}

#[test]
fn light_dark_fallback_is_current() {
    let theme = std::fs::read_to_string(THEME_PATH).expect("read theme.css");
    let start = theme.find(FALLBACK_BEGIN).expect("fallback begin marker");
    let end = theme.find(FALLBACK_END).expect("fallback end marker") + FALLBACK_END.len();
    let fresh = fallback(&outside_fallback(&theme));
    if theme[start..end] == fresh {
        return;
    }
    if std::env::var_os("UPDATE_THEME_CSS").is_some() {
        let updated = format!("{}{}{}", &theme[..start], fresh, &theme[end..]);
        std::fs::write(THEME_PATH, updated).expect("write theme.css");
        return;
    }
    panic!("the light-dark() fallback in theme.css is stale; rerun with UPDATE_THEME_CSS=1");
}

#[test]
fn colour_tokens_use_light_dark_not_space_toggles() {
    let theme = outside_fallback(THEME);
    let tokens = &theme[..theme
        .find("/* Space toggles")
        .expect("space toggle section")];
    for line in tokens.lines().filter(|l| l.trim_start().starts_with("--")) {
        assert!(
            !line.contains("var(--dark") && !line.contains("var(--light"),
            "use light-dark() for colour tokens: {line}"
        );
    }
}

#[test]
fn fallback_converts_pairs_and_nested_vars() {
    let css = ":root {\n  --a: light-dark(#fff, #000);\n  --b: light-dark(var(--a), var(--c, #111));\n  --r: 1rem;\n}\n[data-accent=\"x\"] {\n  --r: 2rem;\n}\n";
    let out = fallback(css);
    assert!(
        out.contains("--a: var(--dark, #000) var(--light, #fff);"),
        "{out}"
    );
    assert!(
        out.contains("--b: var(--dark, var(--c, #111)) var(--light, var(--a));"),
        "{out}"
    );
    assert!(
        out.contains(":root:root, [data-theme][data-theme] {"),
        "{out}"
    );
    assert!(
        !out.contains("--r") && !out.contains("data-accent"),
        "{out}"
    );
}

#[test]
fn theme_ships_a_global_sr_only_class() {
    let start = THEME.find("\n.sr-only {").expect(".sr-only in theme.css");
    let block = &THEME[start..start + THEME[start..].find('}').unwrap()];
    for rule in [
        "position: absolute;",
        "clip-path: inset(50%);",
        "overflow: hidden;",
    ] {
        assert!(block.contains(rule), "{rule} missing from {block}");
    }
}
