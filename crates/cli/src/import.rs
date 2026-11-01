//! `atoll theme --import <file>`: a theme made with another tool, as an Atoll look.
//!
//! Reads:
//! - shadcn/ui and tweakcn CSS: `:root { --background: ...; }` and `.dark { ... }` blocks,
//!   colours as `oklch()`, `hsl()`, bare `H S% L%` triplets, `rgb()` or hex;
//! - shadcn registry JSON: `{ "cssVars": { "theme": {...}, "light": {...}, "dark": {...} } }`;
//! - Reshaped theme definitions (JSON): `{ "color": { "backgroundPage": { "hex", "hexDark" } },
//!   "fontFamily": { "body": { "family" } }, "radius": { "medium": { "px" } } }`.
//!
//! Every format is read into shadcn's names first ([`Source`]), then mapped onto the kit's
//! tokens ([`body`]). Colours become `light-dark()` pairs; tokens the file doesn't set keep
//! the kit's value. The result goes through the contrast proof like any other theme.

use crate::contrast::{color, from_oklab, oklab, Rgba};
use std::collections::BTreeMap;

/// A theme in shadcn's variable names (without `--`): colours per mode, plus the rest.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Source {
    pub light: BTreeMap<String, Rgba>,
    pub dark: BTreeMap<String, Rgba>,
    /// Non-colour values (`radius`, `font-sans`, `font-serif`, `font-mono`).
    pub other: BTreeMap<String, String>,
    /// What the file was read as, for the printed header.
    pub format: &'static str,
}

impl Source {
    fn pair(&self, name: &str) -> Option<(Rgba, Rgba)> {
        let l = self.light.get(name).copied();
        let d = self.dark.get(name).copied();
        match (l, d) {
            (Some(l), Some(d)) => Some((l, d)),
            (Some(c), None) | (None, Some(c)) => Some((c, c)),
            _ => None,
        }
    }
}

/// Reads any supported file.
pub fn parse(text: &str) -> Result<Source, String> {
    let t = text.trim_start();
    let src = if t.starts_with('{') {
        let json: serde_json::Value =
            serde_json::from_str(t).map_err(|e| format!("not valid JSON: {e}"))?;
        if json.get("cssVars").is_some() {
            from_shadcn_json(&json)
        } else if json.get("color").is_some() {
            from_reshaped(&json)
        } else {
            return Err("JSON with neither `cssVars` (shadcn) nor `color` (Reshaped)".into());
        }
    } else {
        from_css(text)
    };
    if !src.light.contains_key("background") && !src.light.contains_key("primary") {
        return Err(
            "found no theme colours: expected shadcn variables (--background, --primary, ...) or a Reshaped `color` map"
                .into(),
        );
    }
    Ok(src)
}

fn put(src: &mut Source, dark: bool, name: &str, value: &str) {
    let name = name.trim().trim_start_matches("--");
    let value = value.trim();
    if matches!(name, "radius" | "font-sans" | "font-serif" | "font-mono") {
        src.other.insert(name.into(), value.into());
    } else if let Some(c) = parse_color(value) {
        let map = if dark { &mut src.dark } else { &mut src.light };
        map.insert(name.into(), c);
    }
}

fn from_css(text: &str) -> Source {
    let mut src = Source {
        format: "shadcn CSS",
        ..Default::default()
    };
    let text = strip_comments(text);
    let mut rest = text.as_str();
    while let Some(open) = rest.find('{') {
        let selector = rest[..open].rsplit(['}', ';']).next().unwrap_or("").trim();
        let Some(close) = rest[open..].find('}') else {
            break;
        };
        let body = &rest[open + 1..open + close];
        // `@layer base {` and `@media` wrappers: step inside.
        if selector.starts_with('@') && body.contains('{') {
            rest = &rest[open + 1..];
            continue;
        }
        let dark = selector.contains(".dark") || selector.contains("dark");
        let root = selector.contains(":root") || selector.contains(".light") || dark;
        if root && !selector.starts_with("@theme") {
            for decl in body.split(';') {
                if let Some((k, v)) = decl.split_once(':') {
                    if k.trim().starts_with("--") {
                        put(&mut src, dark, k, v);
                    }
                }
            }
        }
        rest = &rest[open + close + 1..];
    }
    src
}

fn strip_comments(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find("/*") {
        out.push_str(&rest[..i]);
        match rest[i..].find("*/") {
            Some(j) => rest = &rest[i + j + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

fn from_shadcn_json(json: &serde_json::Value) -> Source {
    let mut src = Source {
        format: "shadcn registry JSON",
        ..Default::default()
    };
    let vars = &json["cssVars"];
    for (key, dark) in [("theme", false), ("light", false), ("dark", true)] {
        if let Some(map) = vars[key].as_object() {
            for (k, v) in map {
                if let Some(v) = v.as_str() {
                    put(&mut src, dark, k, v);
                }
            }
        }
    }
    src
}

/// Reshaped's semantic colour names, as the shadcn names they correspond to.
const RESHAPED: [(&str, &str); 14] = [
    ("backgroundPage", "background"),
    ("backgroundElevationBase", "card"),
    ("backgroundElevationOverlay", "popover"),
    ("backgroundNeutralFaded", "muted"),
    ("backgroundNeutral", "secondary"),
    ("foregroundNeutral", "foreground"),
    ("foregroundNeutralFaded", "muted-foreground"),
    ("borderNeutral", "border"),
    ("backgroundPrimary", "primary"),
    ("onBackgroundPrimary", "primary-foreground"),
    ("borderFocus", "ring"),
    ("backgroundCritical", "destructive"),
    ("onBackgroundCritical", "destructive-foreground"),
    ("borderNeutralFaded", "input"),
];

fn from_reshaped(json: &serde_json::Value) -> Source {
    let mut src = Source {
        format: "Reshaped theme definition",
        ..Default::default()
    };
    for (theirs, ours) in RESHAPED {
        let c = &json["color"][theirs];
        if let Some(hex) = c["hex"].as_str() {
            put(&mut src, false, ours, hex);
            put(&mut src, true, ours, c["hexDark"].as_str().unwrap_or(hex));
        }
    }
    if !src.light.contains_key("ring") {
        if let Some((l, d)) = src.pair("primary") {
            src.light.insert("ring".into(), l);
            src.dark.insert("ring".into(), d);
        }
    }
    for (theirs, ours) in [
        ("body", "font-sans"),
        ("title", "font-serif"),
        ("monospace", "font-mono"),
    ] {
        if let Some(f) = json["fontFamily"][theirs]["family"].as_str() {
            src.other.insert(ours.into(), f.into());
        }
    }
    // The kit's medium radius is 0.6 of --radius, as Reshaped's medium is its control radius.
    if let Some(px) = json["radius"]["medium"]["px"].as_f64() {
        src.other
            .insert("radius".into(), format!("{}rem", round3(px / 16.0 / 0.6)));
    }
    src
}

fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

/// Every colour syntax shadcn themes use: what the contrast proof reads (hex, `rgb()`,
/// `color-mix()`), plus `oklch()`, `hsl()` and bare `H S% L%` triplets.
pub fn parse_color(v: &str) -> Option<Rgba> {
    let v = v.trim().to_ascii_lowercase();
    if let Some(c) = color(&v) {
        return Some(c);
    }
    let (func, inner) = match v.find('(') {
        Some(i) if v.ends_with(')') => (&v[..i], &v[i + 1..v.len() - 1]),
        None => ("hsl", v.as_str()),
        _ => return None,
    };
    let (main, alpha) = match inner.split_once('/') {
        Some((m, a)) => (m, Some(a.trim())),
        None => (inner, None),
    };
    let nums: Vec<&str> = main
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();
    if nums.len() != 3 {
        return None;
    }
    let num = |s: &str, pct_scale: f64| -> Option<f64> {
        let s = s.trim_end_matches("deg");
        match s.strip_suffix('%') {
            Some(p) => p.parse::<f64>().ok().map(|p| p / 100.0 * pct_scale),
            None => s.parse().ok(),
        }
    };
    let a = match alpha {
        Some(s) => num(s, 1.0)?,
        None => 1.0,
    };
    let rgb = match func {
        "oklch" => {
            let l = num(nums[0], 1.0)?;
            let c = num(nums[1], 0.4)?;
            let h = num(nums[2], 1.0)?.to_radians();
            from_oklab([l, c * h.cos(), c * h.sin()])
        }
        "oklab" => from_oklab([num(nums[0], 1.0)?, num(nums[1], 0.4)?, num(nums[2], 0.4)?]),
        "hsl" | "hsla" => {
            let h = num(nums[0], 1.0)?;
            let s = num(nums[1], 1.0)?;
            let l = num(nums[2], 1.0)?;
            // Bare numbers in hsl() are percentages too.
            let (s, l) = if nums[1].ends_with('%') {
                (s, l)
            } else {
                (s / 100.0, l / 100.0)
            };
            hsl(h, s, l)
        }
        _ => return None,
    };
    Some(Rgba {
        r: rgb[0],
        g: rgb[1],
        b: rgb[2],
        a,
    })
}

fn hsl(h: f64, s: f64, l: f64) -> [f64; 3] {
    let h = h.rem_euclid(360.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    [(r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0]
}

/// `a` mixed with `b` in OKLab, `t` of the way to `b` (opaque colours).
fn mix(a: Rgba, b: Rgba, t: f64) -> Rgba {
    let (x, y) = (oklab(a), oklab(b));
    let m = from_oklab([
        x[0] + (y[0] - x[0]) * t,
        x[1] + (y[1] - x[1]) * t,
        x[2] + (y[2] - x[2]) * t,
    ]);
    Rgba {
        r: m[0],
        g: m[1],
        b: m[2],
        a: 1.0,
    }
}

/// `fill` moved just far enough from `text` (darker under light text, lighter under dark
/// text, same hue) for body text on it to pass 4.5:1, as the theme builder does for a brand.
pub fn readable(fill: Rgba, text: Rgba) -> Rgba {
    readable_at(fill, text, 4.5)
}

/// [`readable`] to at least `min`.
pub fn readable_at(fill: Rgba, text: Rgba, min: f64) -> Rgba {
    use crate::contrast::ratio;
    if ratio(fill, text) >= min {
        return fill;
    }
    let toward = if text.luminance() > 0.5 {
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    } else {
        Rgba {
            r: 255.0,
            g: 255.0,
            b: 255.0,
            a: 1.0,
        }
    };
    let mut lab = oklab(fill);
    let target = oklab(toward)[0];
    for _ in 0..60 {
        lab[0] += (target - lab[0]) * 0.03;
        let m = from_oklab(lab);
        let c = Rgba {
            r: m[0],
            g: m[1],
            b: m[2],
            a: 1.0,
        };
        // Compare the colour as written out, so rounding can't undo the pass.
        let c = crate::contrast::color(&c.hex()).unwrap_or(c);
        if ratio(c, text) >= min + 0.05 {
            return c;
        }
    }
    fill
}

/// The colour as drawn on `page` (shadcn borders are often translucent in dark mode).
fn flat(c: Rgba, page: Rgba) -> Rgba {
    if c.a >= 1.0 {
        c
    } else {
        c.over(page)
    }
}

const SANS_FALLBACK: &str =
    "ui-sans-serif, system-ui, -apple-system, \"Segoe UI\", Roboto, sans-serif";
const MONO_FALLBACK: &str = "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace";

/// The declarations of an Atoll look (the body of a `[data-preset]` block) for `src`.
pub fn body(src: &Source) -> String {
    let white = Rgba {
        r: 255.0,
        g: 255.0,
        b: 255.0,
        a: 1.0,
    };
    let black = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    let page = src.pair("background").unwrap_or((white, black));
    let fg = src
        .pair("foreground")
        .unwrap_or((mix(page.0, black, 0.9), mix(page.1, white, 0.9)));
    // Each mode on its own: a variable a `.dark` block leaves out is derived from that mode's
    // page and text, not copied from light (where it would be the wrong way round).
    let derive = |name: &str, fallback: &dyn Fn(Rgba, Rgba) -> Rgba| -> (Rgba, Rgba) {
        let l = src.light.get(name).copied();
        let d = src.dark.get(name).copied();
        (
            l.map(|c| flat(c, page.0))
                .unwrap_or_else(|| fallback(page.0, fg.0)),
            d.map(|c| flat(c, page.1))
                .unwrap_or_else(|| fallback(page.1, fg.1)),
        )
    };
    let muted = derive("muted", &|p, f| mix(p, f, 0.05));
    let card = derive("card", &|p, _| p);
    let secondary = derive("secondary", &|p, f| mix(p, f, 0.08));
    let border = derive("border", &|p, f| mix(p, f, 0.12));
    let input = derive("input", &|p, f| mix(p, f, 0.14));
    let muted_fg = derive("muted-foreground", &|p, f| mix(f, p, 0.4));
    let card_fg = derive("card-foreground", &|_, f| f);

    let per_mode = |f: &dyn Fn(usize) -> Rgba| -> String {
        format!("light-dark({}, {})", f(0).hex(), f(1).hex())
    };
    let pick = |p: (Rgba, Rgba), i: usize| if i == 0 { p.0 } else { p.1 };
    let mut out: Vec<(String, String)> = Vec::new();
    let mut set = |k: &str, v: String| out.push((k.to_string(), v));

    set("--primary-color", per_mode(&|i| pick(page, i)));
    set(
        "--primary-color-1",
        per_mode(&|i| mix(pick(page, i), pick(muted, i), 0.5)),
    );
    set("--primary-color-2", per_mode(&|i| pick(card, i)));
    set(
        "--primary-color-3",
        per_mode(&|i| mix(pick(card, i), pick(muted, i), 0.6)),
    );
    set("--primary-color-4", per_mode(&|i| pick(muted, i)));
    set("--primary-color-5", per_mode(&|i| pick(secondary, i)));
    set("--primary-color-6", per_mode(&|i| pick(border, i)));
    set(
        "--primary-color-7",
        per_mode(&|i| mix(pick(input, i), pick(muted_fg, i), 0.15)),
    );
    set(
        "--secondary-color",
        per_mode(&|i| mix(pick(fg, i), if i == 0 { black } else { white }, 0.3)),
    );
    set("--secondary-color-1", per_mode(&|i| pick(fg, i)));
    set("--secondary-color-2", per_mode(&|i| pick(card_fg, i)));
    set(
        "--secondary-color-3",
        per_mode(&|i| mix(pick(fg, i), pick(muted_fg, i), 0.4)),
    );
    set("--secondary-color-4", per_mode(&|i| pick(fg, i)));
    // Secondary text sits on the quiet fills too (keyboard keys, calendar ranges), and the
    // kit fills selected days with --secondary-color-6 under body text: keep both readable.
    let muted_fg = (
        readable(muted_fg.0, secondary.0),
        readable(muted_fg.1, secondary.1),
    );
    set("--secondary-color-5", per_mode(&|i| pick(muted_fg, i)));
    set(
        "--secondary-color-6",
        per_mode(&|i| readable(mix(pick(border, i), pick(muted_fg, i), 0.3), pick(fg, i))),
    );
    if let Some(primary) = src.pair("primary") {
        let primary = (flat(primary.0, page.0), flat(primary.1, page.1));
        let on = src.pair("primary-foreground").unwrap_or_else(|| {
            let best = |c: Rgba| {
                if crate::contrast::ratio(white, c) >= crate::contrast::ratio(black, c) {
                    white
                } else {
                    black
                }
            };
            (best(primary.0), best(primary.1))
        });
        // Headroom for the hover fill, which mixes the accent towards grey.
        let primary = (
            readable_at(primary.0, on.0, 5.0),
            readable_at(primary.1, on.1, 5.0),
        );
        set("--accent-color", per_mode(&|i| pick(primary, i)));
        set("--accent-contrast", per_mode(&|i| pick(on, i)));
        let ring = src.pair("ring").unwrap_or(primary);
        set("--focused-border-color", per_mode(&|i| pick(ring, i)));
    }
    if let Some(d) = src.pair("destructive") {
        let d = (flat(d.0, page.0), flat(d.1, page.1));
        let on = src.pair("destructive-foreground").unwrap_or((white, white));
        let d = (readable(d.0, on.0), readable(d.1, on.1));
        set("--primary-error-color", per_mode(&|i| pick(d, i)));
        set("--contrast-error-color", per_mode(&|i| pick(on, i)));
    }
    for n in 2..=5 {
        if let Some(c) = src.pair(&format!("chart-{n}")) {
            set(&format!("--chart-{n}"), per_mode(&|i| pick(c, i)));
        }
    }
    let family = |v: &str, fallback: &str| {
        let v = v.trim().trim_end_matches(';');
        if v.contains(',') {
            v.to_string()
        } else {
            format!("{v}, {fallback}")
        }
    };
    if let Some(f) = src.other.get("font-sans") {
        set("--font-sans", family(f, SANS_FALLBACK));
        set("--font-heading", "var(--font-sans)".into());
    }
    if let Some(f) = src.other.get("font-mono") {
        set("--font-mono", family(f, MONO_FALLBACK));
    }
    if let Some(r) = src.other.get("radius") {
        set("--radius", r.clone());
    }
    out.iter()
        .map(|(k, v)| format!("  {k}: {v};"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The fonts the theme names that the kit doesn't ship, to tell the user to add them.
pub fn missing_fonts(src: &Source) -> Vec<String> {
    let shipped = [
        "inter",
        "geist",
        "ibm plex sans",
        "source serif 4",
        "jetbrains mono",
    ];
    let mut out = Vec::new();
    for key in ["font-sans", "font-mono"] {
        if let Some(v) = src.other.get(key) {
            let first = v
                .split(',')
                .next()
                .unwrap_or("")
                .trim()
                .trim_matches(['"', '\''])
                .to_string();
            let generic = first.starts_with("ui-")
                || first.starts_with("var(")
                || matches!(
                    first.as_str(),
                    "system-ui" | "sans-serif" | "serif" | "monospace" | ""
                );
            if !generic
                && !shipped
                    .iter()
                    .any(|s| first.to_ascii_lowercase().starts_with(s))
            {
                out.push(first);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_every_colour_syntax() {
        let hex = |v: &str| parse_color(v).unwrap().hex();
        assert_eq!(hex("oklch(1 0 0)"), "#ffffff");
        assert_eq!(hex("oklch(0.145 0 0)"), "#0a0a0a");
        assert_eq!(hex("oklch(62.8% 0.2577 29.23)"), "#ff0000");
        assert_eq!(hex("222.2 84% 4.9%"), "#020817");
        assert_eq!(hex("hsl(0 0% 100%)"), "#ffffff");
        assert_eq!(hex("hsl(210, 40%, 96.1%)"), "#f1f5f9");
        assert_eq!(hex("#7c3aed"), "#7c3aed");
        assert_eq!(hex("oklch(1 0 0 / 10%)"), "#ffffff1a");
    }

    const TWEAKCN: &str = r#"
@import "tailwindcss";
:root {
  --radius: 0.5rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --primary: oklch(0.546 0.245 262.881);
  --primary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.97 0 0);
  --muted-foreground: oklch(0.556 0 0);
  --border: oklch(0.922 0 0);
  --destructive: oklch(0.577 0.245 27.325);
  --font-sans: Nunito, sans-serif;
}
.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --primary: oklch(0.707 0.165 254.624);
  --primary-foreground: oklch(0.205 0 0);
  --border: oklch(1 0 0 / 10%);
}
@theme inline {
  --color-background: var(--background);
}
"#;

    #[test]
    fn maps_a_tweakcn_theme() {
        let src = parse(TWEAKCN).unwrap();
        assert_eq!(src.format, "shadcn CSS");
        let b = body(&src);
        assert!(
            b.contains("--primary-color: light-dark(#ffffff, #0a0a0a);"),
            "{b}"
        );
        assert!(b.contains("--accent-color: light-dark(#"), "{b}");
        assert!(
            b.contains("--font-sans: Nunito, sans-serif;"),
            "a full stack is kept: {b}"
        );
        assert!(b.contains("--radius: 0.5rem;"), "{b}");
        // The dark border is 10% white over the dark page, flattened.
        assert!(
            b.contains("--primary-color-6: light-dark(#e5e5e5, #232323);"),
            "{b}"
        );
        assert_eq!(missing_fonts(&src), ["Nunito"]);
        // @theme inline is Tailwind's mapping, not colours.
        assert!(!src.light.contains_key("color-background"));
    }

    #[test]
    fn maps_shadcn_json_and_reshaped() {
        let json = r#"{"name":"x","cssVars":{"theme":{"radius":"0.75rem"},"light":{"background":"0 0% 100%","primary":"262 83% 58%"},"dark":{"background":"240 10% 4%","primary":"263 70% 50%"}}}"#;
        let src = parse(json).unwrap();
        assert_eq!(src.format, "shadcn registry JSON");
        let b = body(&src);
        assert!(b.contains("--radius: 0.75rem;"), "{b}");
        assert!(
            b.contains("--accent-contrast: light-dark(#ffffff, #ffffff);"),
            "{b}"
        );

        let reshaped = r##"{"color":{"backgroundPage":{"hex":"#ffffff","hexDark":"#101010"},"foregroundNeutral":{"hex":"#14181f","hexDark":"#eff1f4"},"backgroundPrimary":{"hex":"#5a4cdb","hexDark":"#6b5ef0"},"onBackgroundPrimary":{"hex":"#ffffff"}},"fontFamily":{"body":{"family":"Inter, sans-serif"}},"radius":{"medium":{"px":6}}}"##;
        let src = parse(reshaped).unwrap();
        assert_eq!(src.format, "Reshaped theme definition");
        let b = body(&src);
        assert!(
            b.contains("--accent-color: light-dark(#5a4cdb, #6b5ef0);"),
            "{b}"
        );
        assert!(
            b.contains("--focused-border-color: light-dark(#5a4cdb, #6b5ef0);"),
            "{b}"
        );
        assert!(b.contains("--radius: 0.625rem;"), "{b}");
        assert!(missing_fonts(&src).is_empty());
    }

    #[test]
    fn says_what_it_cannot_read() {
        assert!(parse("body { color: red }")
            .unwrap_err()
            .contains("no theme colours"));
        assert!(parse("{\"a\":1}").unwrap_err().contains("cssVars"));
    }
}
