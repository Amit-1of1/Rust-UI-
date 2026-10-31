//! Contrast proof: every text colour a component's CSS draws, on the background it is drawn
//! on, in light and dark and under every accent preset, meets WCAG 2 AA.
//!
//! The proof works on the stylesheets, not on screenshots, so it also covers states no demo
//! shows by default (hover, selected, checked, invalid, open overlays). For each rule that sets
//! a text colour or a background it works out the pair the element ends up with:
//!
//! - **Same element.** A state rule (`.item:hover`, `.item[data-state="on"]`) inherits what
//!   its base rule (`.item`) sets: any rule whose selector is a subset of this one counts.
//! - **Ancestors.** `.content .item` sits on whatever `.content` has.
//! - **Unknown host.** Text with no background of its own is checked on the page and on every
//!   surface its own component paints (its plain, state-free rules), which over-approximates:
//!   a pair that passes everywhere it could sit is proven. [`Pair::hosts`] lists them.
//!
//! Colours resolve like the browser does: `var()` with fallbacks, `light-dark()`, the
//! `--light`/`--dark` space toggles, `color-mix()` (sRGB and OKLab), alpha composited over
//! what is underneath. Disabled states are exempt (WCAG 1.4.3 exempts inactive controls), as
//! are `forced-colors` blocks (the system picks the colours there). Text needs 4.5:1, large
//! text (24px, or 18.66px bold) 3:1, and a focus indicator 3:1 against the surface it is drawn
//! on (WCAG 1.4.11).
//!
//! `cargo test -p atoll-cli --test contrast` runs it on the kit; `atoll contrast --theme
//! my-theme.css` runs it on a theme of your own.

use std::collections::{BTreeMap, HashMap};

/// WCAG AA for body text.
pub const TEXT: f64 = 4.5;
/// WCAG AA for large text and for non-text UI (focus indicators).
pub const LARGE: f64 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Rgba {
    const fn rgb(r: f64, g: f64, b: f64) -> Rgba {
        Rgba { r, g, b, a: 1.0 }
    }

    pub fn hex(self) -> String {
        let c = |v: f64| v.round().clamp(0.0, 255.0) as u8;
        if self.a < 1.0 {
            format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                c(self.r),
                c(self.g),
                c(self.b),
                c(self.a * 255.0)
            )
        } else {
            format!("#{:02x}{:02x}{:02x}", c(self.r), c(self.g), c(self.b))
        }
    }

    /// `self` drawn over an opaque `under`.
    pub fn over(self, under: Rgba) -> Rgba {
        let m = |f: f64, u: f64| f * self.a + u * (1.0 - self.a);
        Rgba::rgb(m(self.r, under.r), m(self.g, under.g), m(self.b, under.b))
    }

    pub fn luminance(self) -> f64 {
        let c = |v: f64| {
            let c = v / 255.0;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * c(self.r) + 0.7152 * c(self.g) + 0.0722 * c(self.b)
    }
}

/// WCAG 2 contrast ratio of two opaque colours.
pub fn ratio(a: Rgba, b: Rgba) -> f64 {
    let (la, lb) = (a.luminance(), b.luminance());
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Light,
    Dark,
}

impl Mode {
    pub const ALL: [Mode; 2] = [Mode::Light, Mode::Dark];

    pub fn name(self) -> &'static str {
        match self {
            Mode::Light => "light",
            Mode::Dark => "dark",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// CSS

/// One style rule with its nesting and at-rules flattened: `.a { &:hover { .. } }` becomes
/// `.a:hover`, and the declarations of a nested `@supports` count for the rule around it.
#[derive(Debug, Clone)]
pub struct Rule {
    pub selector: String,
    pub decls: Vec<(String, String)>,
    /// Innermost enclosing `@media` condition, if any.
    pub media: Option<String>,
}

impl Rule {
    pub fn get(&self, prop: &str) -> Option<&str> {
        self.decls
            .iter()
            .rev()
            .find(|(p, _)| p == prop)
            .map(|(_, v)| v.as_str())
    }
}

fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(i) = rest.find("/*") {
        out.push_str(&rest[..i]);
        rest = match rest[i + 2..].find("*/") {
            Some(j) => &rest[i + 2 + j + 2..],
            None => "",
        };
    }
    out.push_str(rest);
    out
}

/// Splits on `sep` outside brackets, parentheses and quotes.
fn split_top(s: &str, sep: char) -> Vec<&str> {
    let (mut out, mut depth, mut start, mut quote) = (Vec::new(), 0i32, 0, None);
    for (i, c) in s.char_indices() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '(' | '[') => depth += 1,
            (None, ')' | ']') => depth -= 1,
            (None, c) if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

/// Parses a stylesheet into flat rules. `@keyframes`, `@font-face` and other blocks without
/// selectors are skipped.
pub fn parse(css: &str) -> Vec<Rule> {
    let css = strip_comments(css);
    let mut rules = Vec::new();
    let chars: Vec<char> = css.chars().collect();
    let mut i = 0;
    block(&chars, &mut i, &[], None, &mut rules);
    rules
}

/// Reads a block body from `i` up to its closing `}` (or the end), appending rules. The
/// declarations directly in it belong to `parents`.
fn block(
    chars: &[char],
    i: &mut usize,
    parents: &[String],
    media: Option<&str>,
    rules: &mut Vec<Rule>,
) {
    // The block's own declarations come before its nested rules in the cascade.
    let slot = rules.len();
    let mut own = Vec::new();
    let mut buf = String::new();
    let mut depth = 0i32;
    while *i < chars.len() {
        let c = chars[*i];
        *i += 1;
        match c {
            '(' | '[' => {
                depth += 1;
                buf.push(c);
            }
            ')' | ']' => {
                depth -= 1;
                buf.push(c);
            }
            ';' if depth == 0 => {
                if let Some((p, v)) = buf.split_once(':') {
                    own.push((p.trim().to_ascii_lowercase(), v.trim().to_string()));
                }
                buf.clear();
            }
            '{' if depth == 0 => {
                let prelude = buf.trim().to_string();
                buf.clear();
                if let Some(at) = prelude.strip_prefix('@') {
                    let name = at.split_whitespace().next().unwrap_or("");
                    let cond = at[name.len()..].trim();
                    match name {
                        "media" => block(chars, i, parents, Some(cond), rules),
                        "supports" | "layer" | "container" | "scope" => {
                            block(chars, i, parents, media, rules)
                        }
                        _ => skip(chars, i),
                    }
                } else {
                    let selectors: Vec<String> = split_top(&prelude, ',')
                        .into_iter()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .flat_map(|s| nest(parents, s))
                        .collect();
                    block(chars, i, &selectors, media, rules);
                }
            }
            '}' if depth == 0 => break,
            _ => buf.push(c),
        }
    }
    if let Some((p, v)) = buf.split_once(':') {
        if !p.trim().is_empty() && !p.contains('{') {
            own.push((p.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }
    if !own.is_empty() && !parents.is_empty() {
        for (k, selector) in parents.iter().enumerate() {
            rules.insert(
                slot + k,
                Rule {
                    selector: selector.clone(),
                    decls: own.clone(),
                    media: media.map(str::to_string),
                },
            );
        }
    }
}

fn skip(chars: &[char], i: &mut usize) {
    let mut depth = 1;
    while *i < chars.len() && depth > 0 {
        match chars[*i] {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        *i += 1;
    }
}

fn nest(parents: &[String], child: &str) -> Vec<String> {
    if parents.is_empty() {
        return vec![child.to_string()];
    }
    parents
        .iter()
        .map(|p| {
            if child.contains('&') {
                child.replace('&', p)
            } else {
                format!("{p} {child}")
            }
        })
        .collect()
}

/// A selector as its compounds, outermost first; each compound is a set of simple selectors.
/// Combinators (` `, `>`, `+`, `~`) are all treated as "inside".
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub compounds: Vec<Vec<String>>,
}

impl Selector {
    pub fn parse(s: &str) -> Selector {
        let mut compounds = Vec::new();
        let mut cur = String::new();
        let mut depth = 0;
        let flush = |cur: &mut String, out: &mut Vec<Vec<String>>| {
            if !cur.trim().is_empty() {
                out.push(simples(cur.trim()));
            }
            cur.clear();
        };
        for c in s.chars() {
            match c {
                '(' | '[' => {
                    depth += 1;
                    cur.push(c)
                }
                ')' | ']' => {
                    depth -= 1;
                    cur.push(c)
                }
                ' ' | '>' | '+' | '~' if depth == 0 => flush(&mut cur, &mut compounds),
                _ => cur.push(c),
            }
        }
        flush(&mut cur, &mut compounds);
        Selector { compounds }
    }

    fn last(&self) -> &[String] {
        self.compounds.last().map(Vec::as_slice).unwrap_or(&[])
    }

    /// `self` applies to every element `other` matches, as far as a stylesheet can tell:
    /// its subject is a subset of the other's, and so are its ancestors.
    pub fn is_base_of(&self, other: &Selector) -> bool {
        let subset = |a: &[String], b: &[String]| a.iter().all(|s| b.contains(s));
        subset(self.last(), other.last())
            && self.compounds[..self.compounds.len().saturating_sub(1)]
                .iter()
                .all(|a| {
                    other.compounds[..other.compounds.len().saturating_sub(1)]
                        .iter()
                        .any(|b| subset(a, b))
                })
    }
}

/// `.a.b[x="1"]:hover::before` -> [".a", ".b", "[x=\"1\"]", ":hover", "::before"]
fn simples(compound: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut depth = 0;
    let chars: Vec<char> = compound.chars().collect();
    for (k, &c) in chars.iter().enumerate() {
        let starts =
            depth == 0 && (c == '.' || c == '[' || (c == ':' && (k == 0 || chars[k - 1] != ':')));
        if starts && !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            _ => {}
        }
        cur.push(c);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out.retain(|s| s != "&" && s != "*");
    out
}

/// The selector with its `:not(...)` parts removed.
fn without_not(selector: &str) -> String {
    let mut s = selector.to_string();
    while let Some(i) = s.find(":not(") {
        let end = matching(&s, i + 4).map(|j| j + 1).unwrap_or(s.len());
        s.replace_range(i..end, "");
    }
    s
}

/// Disabled states are exempt from contrast (WCAG 1.4.3).
fn is_disabled(selector: &str) -> bool {
    let s = without_not(selector);
    s.contains("disabled") || s.contains("inert") || s.contains("unavailable")
}

// ---------------------------------------------------------------------------------------------
// Values

/// Custom properties in effect: the theme's, an accent preset's, a component's own.
#[derive(Debug, Clone, Default)]
pub struct Tokens {
    pub vars: HashMap<String, String>,
}

/// What a value resolved to.
#[derive(Debug, Clone, PartialEq)]
pub enum Paint {
    Color(Rgba),
    /// Nothing painted (`transparent`, `none`, an empty space toggle).
    None,
    /// Something this check can't evaluate (gradients, images, system colours).
    Unknown(String),
    /// `inherit` / `currentColor`.
    Inherit,
}

pub struct Ctx<'a> {
    pub mode: Mode,
    pub layers: Vec<&'a Tokens>,
}

impl Ctx<'_> {
    fn lookup(&self, name: &str) -> Option<&str> {
        self.layers
            .iter()
            .find_map(|t| t.vars.get(name).map(String::as_str))
    }

    /// Substitutes `var()` and `light-dark()` until only plain CSS is left. `None` means the
    /// value is invalid at computed-value time (a missing variable with no fallback).
    pub fn substitute(&self, value: &str, depth: usize) -> Option<String> {
        if depth > 32 {
            return None;
        }
        let mut out = String::new();
        let mut rest = value;
        loop {
            let v = rest.find("var(");
            let ld = rest.find("light-dark(");
            let (at, is_var) = match (v, ld) {
                (Some(a), Some(b)) if b < a => (b, false),
                (Some(a), _) => (a, true),
                (None, Some(b)) => (b, false),
                (None, None) => break,
            };
            // `var(` also matches the tail of an identifier like `xvar(`: not in CSS we read.
            out.push_str(&rest[..at]);
            let open = at + if is_var { 4 } else { 11 };
            let close = matching(rest, open - 1)?;
            let inner = &rest[open..close];
            let parts = split_top(inner, ',');
            if is_var {
                let name = parts[0].trim();
                let fallback = (parts.len() > 1).then(|| inner[parts[0].len() + 1..].trim());
                let toggled = match (name, self.mode) {
                    ("--light", Mode::Light) | ("--dark", Mode::Dark) => Some(true),
                    ("--light", Mode::Dark) | ("--dark", Mode::Light) => Some(false),
                    _ => None,
                };
                match toggled {
                    Some(true) => {
                        out.push_str(&self.substitute(fallback.unwrap_or(""), depth + 1)?)
                    }
                    Some(false) => {}
                    None => match self.lookup(name) {
                        Some(v) => out.push_str(&self.substitute(v, depth + 1)?),
                        None => out.push_str(&self.substitute(fallback?, depth + 1)?),
                    },
                }
            } else {
                if parts.len() != 2 {
                    return None;
                }
                let pick = match self.mode {
                    Mode::Light => parts[0],
                    Mode::Dark => parts[1],
                };
                out.push_str(&self.substitute(pick.trim(), depth + 1)?);
            }
            rest = &rest[close + 1..];
        }
        out.push_str(rest);
        Some(out)
    }

    pub fn paint(&self, value: &str) -> Paint {
        match self.substitute(value, 0) {
            None => Paint::None,
            Some(v) => paint_of(v.trim()),
        }
    }
}

/// Index of the `)` matching the `(` at `open`.
fn matching(s: &str, open: usize) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + i);
                }
            }
            _ => {}
        }
    }
    None
}

/// A colour (or a `background` shorthand holding just one) after substitution.
fn paint_of(v: &str) -> Paint {
    let lower = v.to_ascii_lowercase();
    match lower.as_str() {
        "" | "transparent" | "none" | "initial" | "unset" => return Paint::None,
        "inherit" | "currentcolor" => return Paint::Inherit,
        _ => {}
    }
    if let Some(c) = color(&lower) {
        return if c.a == 0.0 {
            Paint::None
        } else {
            Paint::Color(c)
        };
    }
    // `background: <colour> <something else>`: take the colour layer if there is exactly one.
    let words: Vec<&str> = split_top(&lower, ' ')
        .into_iter()
        .filter(|w| !w.is_empty())
        .collect();
    if !lower.contains("gradient") && !lower.contains("url(") {
        let colours: Vec<Rgba> = words.iter().filter_map(|w| color(w)).collect();
        if colours.len() == 1 && words.len() <= 3 {
            return Paint::Color(colours[0]);
        }
        if words.iter().all(|w| *w == "none" || *w == "transparent") {
            return Paint::None;
        }
    }
    Paint::Unknown(v.to_string())
}

/// Parses `#rgb[a]`, `#rrggbb[aa]`, `rgb()`/`rgba()`, `color-mix()` and a few keywords.
pub fn color(v: &str) -> Option<Rgba> {
    let v = v.trim().to_ascii_lowercase();
    match v.as_str() {
        "white" => return Some(Rgba::rgb(255.0, 255.0, 255.0)),
        "black" => return Some(Rgba::rgb(0.0, 0.0, 0.0)),
        "transparent" => {
            return Some(Rgba {
                a: 0.0,
                ..Rgba::rgb(0.0, 0.0, 0.0)
            })
        }
        _ => {}
    }
    if let Some(h) = v.strip_prefix('#') {
        let full: String = match h.len() {
            3 | 4 => h.chars().flat_map(|c| [c, c]).collect(),
            6 | 8 => h.to_string(),
            _ => return None,
        };
        let n = |i: usize| {
            u8::from_str_radix(full.get(i..i + 2)?, 16)
                .ok()
                .map(f64::from)
        };
        let a = if full.len() == 8 { n(6)? / 255.0 } else { 1.0 };
        return Some(Rgba {
            r: n(0)?,
            g: n(2)?,
            b: n(4)?,
            a,
        });
    }
    if let Some(inner) = v
        .strip_prefix("rgba(")
        .or_else(|| v.strip_prefix("rgb("))
        .and_then(|s| s.strip_suffix(')'))
    {
        let (rgb, alpha) = match inner.split_once('/') {
            Some((a, b)) => (a, Some(b.trim())),
            None => (inner, None),
        };
        let mut nums: Vec<&str> = rgb
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty())
            .collect();
        let alpha = alpha.or_else(|| (nums.len() == 4).then(|| nums.pop()).flatten());
        if nums.len() != 3 {
            return None;
        }
        let ch = |s: &str| -> Option<f64> {
            match s.strip_suffix('%') {
                Some(p) => p.parse::<f64>().ok().map(|p| p * 2.55),
                None => s.parse().ok(),
            }
        };
        let a = match alpha {
            Some(s) => match s.strip_suffix('%') {
                Some(p) => p.parse::<f64>().ok()? / 100.0,
                None => s.parse().ok()?,
            },
            None => 1.0,
        };
        return Some(Rgba {
            r: ch(nums[0])?,
            g: ch(nums[1])?,
            b: ch(nums[2])?,
            a,
        });
    }
    if let Some(inner) = v
        .strip_prefix("color-mix(")
        .and_then(|s| s.strip_suffix(')'))
    {
        return color_mix(inner);
    }
    None
}

/// `color-mix(in srgb|oklab, a [p%], b [q%])`, premultiplied as CSS Color 5 specifies.
fn color_mix(inner: &str) -> Option<Rgba> {
    let parts = split_top(inner, ',');
    if parts.len() != 3 {
        return None;
    }
    let space = parts[0].trim().strip_prefix("in ")?.trim();
    let item = |s: &str| -> Option<(Rgba, Option<f64>)> {
        let s = s.trim();
        let words = split_top(s, ' ');
        let last = words.last()?.trim();
        match last.strip_suffix('%').and_then(|p| p.parse::<f64>().ok()) {
            Some(p) if words.len() > 1 => {
                Some((color(s[..s.len() - last.len()].trim())?, Some(p / 100.0)))
            }
            _ => Some((color(s)?, None)),
        }
    };
    let (a, pa) = item(parts[1])?;
    let (b, pb) = item(parts[2])?;
    let (pa, pb) = match (pa, pb) {
        (Some(x), Some(y)) => (x, y),
        (Some(x), None) => (x, 1.0 - x),
        (None, Some(y)) => (1.0 - y, y),
        (None, None) => (0.5, 0.5),
    };
    let sum = pa + pb;
    if sum <= 0.0 {
        return None;
    }
    let (wa, wb) = (pa / sum, pb / sum);
    let to = |c: Rgba| -> [f64; 3] {
        match space {
            "oklab" | "oklch" => oklab(c),
            _ => [c.r, c.g, c.b],
        }
    };
    let (ca, cb) = (to(a), to(b));
    let alpha = a.a * wa + b.a * wb;
    if alpha == 0.0 {
        return Some(Rgba {
            a: 0.0,
            ..Rgba::rgb(0.0, 0.0, 0.0)
        });
    }
    let mixed: Vec<f64> = (0..3)
        .map(|k| (ca[k] * a.a * wa + cb[k] * b.a * wb) / alpha)
        .collect();
    let rgb = match space {
        "oklab" | "oklch" => from_oklab([mixed[0], mixed[1], mixed[2]]),
        _ => [mixed[0], mixed[1], mixed[2]],
    };
    // A sum under 100% scales the alpha down.
    let alpha = alpha * sum.min(1.0);
    Some(Rgba {
        r: rgb[0],
        g: rgb[1],
        b: rgb[2],
        a: alpha,
    })
}

fn lin(v: f64) -> f64 {
    let c = v / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn gam(c: f64) -> f64 {
    let v = if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (v * 255.0).clamp(0.0, 255.0)
}

fn oklab(c: Rgba) -> [f64; 3] {
    let (r, g, b) = (lin(c.r), lin(c.g), lin(c.b));
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    [
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    ]
}

fn from_oklab([l, a, b]: [f64; 3]) -> [f64; 3] {
    let l_ = (l + 0.3963377774 * a + 0.2158037573 * b).powi(3);
    let m_ = (l - 0.1055613458 * a - 0.0638541728 * b).powi(3);
    let s_ = (l - 0.0894841775 * a - 1.2914855480 * b).powi(3);
    [
        gam(4.0767416621 * l_ - 3.3077115913 * m_ + 0.2309699292 * s_),
        gam(-1.2684380046 * l_ + 2.6097574011 * m_ - 0.3413193965 * s_),
        gam(-0.0041960863 * l_ - 0.7034186147 * m_ + 1.7076147010 * s_),
    ]
}

// ---------------------------------------------------------------------------------------------
// Theme

/// The theme: root tokens plus each `[data-accent="..."]` preset.
#[derive(Debug, Clone, Default)]
pub struct Theme {
    pub root: Tokens,
    /// (name, tokens) in file order; "neutral" (no preset) is implied.
    pub accents: Vec<(String, Tokens)>,
}

impl Theme {
    /// Reads `:root` and `[data-accent]` tokens, ignoring `@media` and `@supports` blocks
    /// (the generated fallback repeats the same tokens).
    pub fn parse(css: &str) -> Theme {
        let mut theme = Theme::default();
        for rule in parse(css) {
            if rule.media.is_some() {
                continue;
            }
            let sel = rule.selector.replace(' ', "");
            let target = if sel == ":root" || sel == "html" {
                Some(&mut theme.root)
            } else if let Some(name) = sel
                .strip_prefix("[data-accent=\"")
                .and_then(|s| s.strip_suffix("\"]"))
                .filter(|n| !n.contains('['))
            {
                if !theme.accents.iter().any(|(n, _)| n == name) {
                    theme.accents.push((name.to_string(), Tokens::default()));
                }
                theme
                    .accents
                    .iter_mut()
                    .find(|(n, _)| n == name)
                    .map(|(_, t)| t)
            } else {
                None
            };
            if let Some(t) = target {
                for (p, v) in &rule.decls {
                    if p.starts_with("--") {
                        t.vars.insert(p.clone(), v.clone());
                    }
                }
            }
        }
        theme
    }

    /// Later files win, like a stylesheet loaded after the kit's theme.
    pub fn overlay(&mut self, other: Theme) {
        self.root.vars.extend(other.root.vars);
        for (name, t) in other.accents {
            match self.accents.iter_mut().find(|(n, _)| *n == name) {
                Some((_, mine)) => mine.vars.extend(t.vars),
                None => self.accents.push((name, t)),
            }
        }
    }

    /// "neutral" (no preset) first, then the presets.
    pub fn accent_names(&self) -> Vec<String> {
        std::iter::once("neutral".to_string())
            .chain(self.accents.iter().map(|(n, _)| n.clone()))
            .collect()
    }

    fn accent(&self, name: &str) -> Option<&Tokens> {
        self.accents.iter().find(|(n, _)| n == name).map(|(_, t)| t)
    }
}

// ---------------------------------------------------------------------------------------------
// The proof

/// The page the kit sits on: what an app's `body` sets (the gallery's, which the docs
/// recommend).
pub const PAGE_BACKGROUND: &str = "var(--primary-color)";
pub const PAGE_TEXT: &str = "var(--secondary-color-4)";

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Text,
    LargeText,
    /// An icon or other graphic drawn in `color` (no text inside): 3:1 (WCAG 1.4.11).
    Graphic,
    Focus,
}

impl Kind {
    pub fn min(&self) -> f64 {
        match self {
            Kind::Text => TEXT,
            Kind::LargeText | Kind::Graphic | Kind::Focus => LARGE,
        }
    }
}

/// One check: a rule's foreground on one background it can sit on, in one mode and accent.
#[derive(Debug, Clone)]
pub struct Pair {
    pub component: String,
    pub selector: String,
    pub kind: Kind,
    pub mode: Mode,
    pub accent: String,
    /// The value as written, before resolving (`var(--secondary-color-5)`).
    pub fg_source: String,
    pub bg_source: String,
    pub fg: Rgba,
    pub bg: Rgba,
    pub ratio: f64,
    pub host: Host,
}

impl Pair {
    pub fn passes(&self) -> bool {
        // Rounded to two places, like every contrast checker reports it.
        (self.ratio * 100.0).round() / 100.0 >= self.kind.min()
    }

    /// The stable key a known-issue list matches: `component selector mode accent`.
    pub fn key(&self) -> String {
        format!(
            "{} {} {} {}",
            self.component,
            self.selector,
            self.mode.name(),
            self.accent
        )
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} `{}` ({}, {}): {} {} on {} {} = {:.2}:1, needs {}:1",
            self.component,
            self.selector,
            self.mode.name(),
            self.accent,
            self.fg_source,
            self.fg.hex(),
            self.bg_source,
            self.bg.hex(),
            self.ratio,
            self.kind.min()
        )
    }
}

/// Things the proof could not evaluate, reported so they can't hide failures silently.
#[derive(Debug, Clone)]
pub struct Skipped {
    pub component: String,
    pub selector: String,
    pub value: String,
}

#[derive(Debug, Default)]
pub struct Report {
    pub pairs: Vec<Pair>,
    pub skipped: Vec<Skipped>,
}

impl Report {
    pub fn failures(&self) -> impl Iterator<Item = &Pair> {
        self.pairs.iter().filter(|p| !p.passes())
    }
}

/// A layer under a rule's element: what painted it, and with what.
#[derive(Clone)]
struct Layer {
    source: String,
    paint: Rgba,
}

/// Where each class appears in rendered markup: every element's classes, then its
/// ancestors' classes, nearest first. Read from the demos' SSR snapshots
/// (`crates/ui/tests/snapshots/*.html`), so "what sits on what" comes from real markup.
#[derive(Debug, Default)]
pub struct Nesting {
    /// Per element: its classes then its ancestors' (nearest first), and whether any text
    /// is rendered inside it.
    elements: Vec<(Vec<Vec<String>>, bool)>,
}

const VOID: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

/// `dx-button-css` (the class `css_module` writes in SSR) -> `dx-button`.
fn normalize_class(c: &str) -> String {
    c.strip_suffix("-css").unwrap_or(c).to_string()
}

impl Nesting {
    pub fn add_html(&mut self, html: &str) {
        // (classes, index in `elements` if it has classes)
        let mut stack: Vec<(Vec<String>, Option<usize>)> = Vec::new();
        let mut rest = html;
        while let Some(lt) = rest.find('<') {
            if !rest[..lt].trim().is_empty() {
                for (_, idx) in &stack {
                    if let Some(i) = idx {
                        self.elements[*i].1 = true;
                    }
                }
            }
            rest = &rest[lt + 1..];
            if let Some(after) = rest.strip_prefix('/') {
                stack.pop();
                rest = after.split_once('>').map(|(_, r)| r).unwrap_or("");
                continue;
            }
            if rest.starts_with('!') || rest.starts_with('?') {
                continue;
            }
            // The tag runs to the first `>` outside quotes.
            let mut quote = None;
            let mut end = rest.len();
            for (i, c) in rest.char_indices() {
                match (quote, c) {
                    (Some(q), c) if c == q => quote = None,
                    (Some(_), _) => {}
                    (None, '"' | '\'') => quote = Some(c),
                    (None, '>') => {
                        end = i;
                        break;
                    }
                    _ => {}
                }
            }
            let tag = &rest[..end];
            rest = rest.get(end + 1..).unwrap_or("");
            let name: String = tag
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
                .collect::<String>()
                .to_ascii_lowercase();
            let classes: Vec<String> = tag
                .find("class=\"")
                .and_then(|i| {
                    let v = &tag[i + 7..];
                    v.find('"').map(|j| &v[..j])
                })
                .map(|v| v.split_whitespace().map(normalize_class).collect())
                .unwrap_or_default();
            let idx = (!classes.is_empty()).then(|| {
                let chain: Vec<Vec<String>> = std::iter::once(classes.clone())
                    .chain(stack.iter().rev().map(|(c, _)| c.clone()))
                    .collect();
                // An <input> or <textarea> shows its value as text.
                let text = matches!(name.as_str(), "input" | "textarea" | "select");
                self.elements.push((chain, text));
                self.elements.len() - 1
            });
            if !(tag.ends_with('/') || VOID.contains(&name.as_str())) {
                stack.push((classes, idx));
            }
        }
    }

    /// Ancestor chains (nearest first, the element itself left out) of every element that
    /// carries all of `classes`.
    fn ancestors<'n>(&'n self, classes: &'n [String]) -> impl Iterator<Item = &'n [Vec<String>]> {
        self.matching(classes).map(|(chain, _)| &chain[1..])
    }

    fn matching<'n>(
        &'n self,
        classes: &'n [String],
    ) -> impl Iterator<Item = &'n (Vec<Vec<String>>, bool)> {
        self.elements
            .iter()
            .filter(move |(chain, _)| classes.iter().all(|c| chain[0].contains(c)))
    }

    /// `Some(true)` if an element with these classes renders text, `Some(false)` if every
    /// one seen renders none (an icon, a dot, a bar), `None` if none was rendered.
    pub fn has_text(&self, classes: &[String]) -> Option<bool> {
        let mut seen = None;
        for (_, text) in self.matching(classes) {
            if *text {
                return Some(true);
            }
            seen = Some(false);
        }
        seen
    }
}

struct Sheet {
    name: String,
    rules: Vec<(Rule, Selector)>,
    own: Tokens,
}

fn is_color_prop(p: &str) -> bool {
    p == "background" || p == "background-color"
}

fn background_of(rule: &Rule) -> Option<&str> {
    rule.decls
        .iter()
        .rev()
        .find(|(p, _)| is_color_prop(p))
        .map(|(_, v)| v.as_str())
}

fn px(v: &str) -> Option<f64> {
    let v = v.trim();
    if let Some(n) = v.strip_suffix("rem").or_else(|| v.strip_suffix("em")) {
        return n.trim().parse::<f64>().ok().map(|n| n * 16.0);
    }
    v.strip_suffix("px")?.trim().parse().ok()
}

/// The classes a selector's subject must carry: `.a.b:hover` -> [a, b].
fn subject_classes(sel: &Selector) -> Vec<String> {
    sel.last()
        .iter()
        .filter_map(|s| s.strip_prefix('.'))
        .map(str::to_string)
        .collect()
}

/// Part names that are surfaces other parts sit on, for parts no snapshot renders (closed
/// overlays): `dx-dropdown-menu-content`, `dx-dialog-popover`, ...
const SURFACES: &[&str] = &[
    "-content",
    "-popover",
    "-panel",
    "-viewport",
    "-list",
    "-popup",
    "-container",
];

impl Sheet {
    fn new(name: &str, css: &str) -> Sheet {
        let rules: Vec<(Rule, Selector)> = parse(css)
            .into_iter()
            .filter(|r| {
                !r.media.as_deref().is_some_and(|m| {
                    m.contains("forced-colors")
                        || m.contains("print")
                        || m.contains("prefers-contrast")
                })
            })
            .map(|mut r| {
                for (_, v) in r.decls.iter_mut() {
                    *v = v.split_whitespace().collect::<Vec<_>>().join(" ");
                }
                let s = Selector::parse(&r.selector);
                (r, s)
            })
            .collect();
        // A component's own custom properties, by first declaration.
        let mut own = Tokens::default();
        for (r, _) in &rules {
            for (p, v) in &r.decls {
                if p.starts_with("--") {
                    own.vars.entry(p.clone()).or_insert_with(|| v.clone());
                }
            }
        }
        Sheet {
            name: name.to_string(),
            rules,
            own,
        }
    }

    /// Rules that also apply to the element `sel` matches, least specific first.
    fn bases(&self, sel: &Selector) -> Vec<&Rule> {
        let mut found: Vec<(usize, usize, &Rule)> = self
            .rules
            .iter()
            .enumerate()
            .filter(|(_, (r, s))| s.is_base_of(sel) && !is_disabled(&r.selector))
            .map(|(i, (r, s))| (s.last().len() + s.compounds.len(), i, r))
            .collect();
        found.sort_by_key(|(spec, i, _)| (*spec, *i));
        found.into_iter().map(|(_, _, r)| r).collect()
    }

    /// The last value of `prop` among the rules applying to `sel` (most specific wins).
    fn cascaded(&self, sel: &Selector, prop: impl Fn(&str) -> bool) -> Option<&str> {
        self.bases(sel).into_iter().rev().find_map(|r| {
            r.decls
                .iter()
                .rev()
                .find(|(p, _)| prop(p))
                .map(|(_, v)| v.as_str())
        })
    }

    /// Custom properties the rules applying to `sel` set themselves.
    fn scoped(&self, sel: &Selector) -> Tokens {
        let mut t = Tokens::default();
        for r in self.bases(sel) {
            for (p, v) in &r.decls {
                if p.starts_with("--") {
                    t.vars.insert(p.clone(), v.clone());
                }
            }
        }
        t
    }

    /// Background of an element carrying `classes`, from this sheet's plain class rules.
    fn plain_background(&self, classes: &[String]) -> Option<&str> {
        self.rules
            .iter()
            .rev()
            .filter(|(r, s)| {
                s.compounds.len() == 1
                    && !s.last().is_empty()
                    && s.last().iter().all(|x| {
                        x.strip_prefix('.')
                            .is_some_and(|c| classes.iter().any(|k| k == c))
                    })
                    && !is_disabled(&r.selector)
            })
            .find_map(|(r, _)| background_of(r))
    }
}

fn describe(layers: &[Layer]) -> String {
    layers
        .iter()
        .map(|l| l.source.as_str())
        .collect::<Vec<_>>()
        .join(" over ")
}

fn flatten(layers: &[Layer], under: Rgba) -> Rgba {
    layers.iter().rev().fold(under, |acc, l| l.paint.over(acc))
}

/// The kit (or an app) as the proof sees it: theme, component stylesheets, markup nesting.
pub struct Kit<'a> {
    pub theme: &'a Theme,
    sheets: Vec<Sheet>,
    pub nesting: &'a Nesting,
}

/// How a rule's host background was found, for the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Host {
    /// From the rule's own element or the selector's ancestors.
    Selector,
    /// From where the snapshots render the element.
    Markup,
    /// Not rendered anywhere: the page and the component's surface parts.
    Assumed,
}

impl<'a> Kit<'a> {
    pub fn new(theme: &'a Theme, sheets: &[(String, String)], nesting: &'a Nesting) -> Kit<'a> {
        Kit {
            theme,
            sheets: sheets.iter().map(|(n, c)| Sheet::new(n, c)).collect(),
            nesting,
        }
    }

    fn ctx<'c>(&'c self, mode: Mode, accent: &str, layers: &[&'c Tokens]) -> Ctx<'c> {
        let mut all: Vec<&Tokens> = layers.to_vec();
        all.extend(self.theme.accent(accent));
        all.push(&self.theme.root);
        Ctx { mode, layers: all }
    }

    /// An ancestor element's background, from any component's plain class rules.
    fn markup_background(&self, classes: &[String], mode: Mode, accent: &str) -> Option<Layer> {
        self.sheets.iter().rev().find_map(|sh| {
            let v = sh.plain_background(classes)?;
            match self.ctx(mode, accent, &[&sh.own]).paint(v) {
                Paint::Color(c) => Some(Layer {
                    source: format!("{v} (`.{}`)", classes.join(".")),
                    paint: c,
                }),
                _ => None,
            }
        })
    }

    /// The backgrounds the subject of `sel` can end up on, each with the layers that make it.
    fn grounds(
        &self,
        sheet: &Sheet,
        sel: &Selector,
        ctx: &Ctx,
        accent: &str,
        include_self: bool,
        page: Rgba,
    ) -> (Vec<(Vec<Layer>, Rgba)>, Host) {
        let mut layers = Vec::new();
        let mut chain: Vec<Selector> = Vec::new();
        if include_self {
            chain.push(sel.clone());
        }
        for k in (1..sel.compounds.len()).rev() {
            chain.push(Selector {
                compounds: sel.compounds[..k].to_vec(),
            });
        }
        for s in &chain {
            if let Some(v) = sheet.cascaded(s, is_color_prop) {
                if let Paint::Color(c) = ctx.paint(v) {
                    layers.push(Layer {
                        source: v.to_string(),
                        paint: c,
                    });
                    if c.a >= 1.0 {
                        return (vec![(layers.clone(), flatten(&layers, c))], Host::Selector);
                    }
                }
            }
        }
        let page_layer = Layer {
            source: PAGE_BACKGROUND.into(),
            paint: page,
        };
        let mut out: Vec<(Vec<Layer>, Rgba)> = Vec::new();
        let mut add = |ls: Vec<Layer>, under: Rgba| {
            let bg = flatten(&ls, under);
            if !out.iter().any(|(_, b)| *b == bg) {
                out.push((ls, bg));
            }
        };
        let subject = subject_classes(sel);
        let mut seen = false;
        if !subject.is_empty() {
            for ancestors in self.nesting.ancestors(&subject) {
                seen = true;
                let mut ls = layers.clone();
                let mut under = page;
                let mut opaque = false;
                for classes in ancestors {
                    if let Some(l) = self.markup_background(classes, ctx.mode, accent) {
                        let a = l.paint.a;
                        ls.push(l);
                        if a >= 1.0 {
                            under = ls.last().map(|l| l.paint).unwrap_or(page);
                            opaque = true;
                            break;
                        }
                    }
                }
                if !opaque {
                    ls.push(page_layer.clone());
                }
                add(ls, under);
            }
        }
        if seen {
            return (out, Host::Markup);
        }
        let mut ls = layers.clone();
        ls.push(page_layer.clone());
        add(ls, page);
        let root = format!("dx-{}", sheet.name.replace('_', "-"));
        for (r, s) in &sheet.rules {
            let [simple] = s.last() else { continue };
            let Some(class) = simple.strip_prefix('.') else {
                continue;
            };
            let surface = class == root || SURFACES.iter().any(|x| class.ends_with(x));
            if s.compounds.len() != 1 || !surface || subject.iter().any(|c| c == class) {
                continue;
            }
            if let Some(v) = background_of(r) {
                if let Paint::Color(c) = ctx.paint(v) {
                    let c = c.over(page);
                    let mut ls = layers.clone();
                    ls.push(Layer {
                        source: format!("{v} (`{}`)", r.selector),
                        paint: c,
                    });
                    add(ls, c);
                }
            }
        }
        (out, Host::Assumed)
    }

    /// Runs the proof: every rule, both modes, every accent.
    pub fn check(&self) -> Report {
        let mut report = Report::default();
        for sheet in &self.sheets {
            for (rule, sel) in &sheet.rules {
                self.check_rule(sheet, rule, sel, &mut report);
            }
        }
        report
    }

    fn check_rule(&self, sheet: &Sheet, rule: &Rule, sel: &Selector, report: &mut Report) {
        if is_disabled(&rule.selector) {
            return;
        }
        let sets_fg = rule.get("color").is_some();
        let sets_bg = background_of(rule).is_some();
        let focus = without_not(&rule.selector).contains(":focus")
            && FOCUS_PROPS.iter().any(|p| rule.get(p).is_some());
        if !sets_fg && !sets_bg && !focus {
            return;
        }
        let subject = subject_classes(sel);
        let text = self.nesting.has_text(&subject);
        // `::before`/`::after` with empty content are decoration (dots, bars, lines).
        let pseudo = sel.last().iter().any(|x| x == "::before" || x == "::after");
        let decorative = pseudo
            && sheet
                .cascaded(sel, |p| p == "content")
                .is_none_or(|c| matches!(c.trim(), "\"\"" | "''" | "none"));
        let scoped = sheet.scoped(sel);
        for mode in Mode::ALL {
            for accent in self.theme.accent_names() {
                let ctx = self.ctx(mode, &accent, &[&scoped, &sheet.own]);
                let page = match ctx.paint(PAGE_BACKGROUND) {
                    Paint::Color(c) => c.over(Rgba::rgb(255.0, 255.0, 255.0)),
                    _ => Rgba::rgb(255.0, 255.0, 255.0),
                };
                let pair =
                    |kind: Kind, fg_source: &str, fg: Rgba, layers: &[Layer], bg: Rgba, host| {
                        let fg = fg.over(bg);
                        Pair {
                            component: sheet.name.clone(),
                            selector: rule.selector.clone(),
                            kind,
                            mode,
                            accent: accent.clone(),
                            fg_source: fg_source.to_string(),
                            bg_source: describe(layers),
                            fg,
                            bg,
                            ratio: ratio(fg, bg),
                            host,
                        }
                    };
                // Text (or an icon drawn in `color`) on its background. A rule that only sets
                // a background is checked with the text colour it inherits, unless nothing
                // with its classes ever renders text.
                let text_matters = !decorative && (sets_fg || text != Some(false));
                if (sets_fg || sets_bg) && text_matters {
                    let mut fg_value = sheet.cascaded(sel, |p| p == "color").unwrap_or(PAGE_TEXT);
                    if ctx.paint(fg_value) == Paint::Inherit {
                        fg_value = PAGE_TEXT;
                    }
                    let size = sheet.cascaded(sel, |p| p == "font-size").and_then(px);
                    let bold = sheet
                        .cascaded(sel, |p| p == "font-weight")
                        .and_then(|w| match w.trim() {
                            "bold" | "bolder" => Some(700),
                            w => w.parse::<u32>().ok(),
                        })
                        .is_some_and(|w| w >= 700);
                    let large = size.is_some_and(|s| s >= 24.0 || (bold && s >= 18.66));
                    let kind = match (text, large) {
                        (Some(false), _) => Kind::Graphic,
                        (_, true) => Kind::LargeText,
                        _ => Kind::Text,
                    };
                    match ctx.paint(fg_value) {
                        Paint::Color(c) => {
                            let (grounds, host) =
                                self.grounds(sheet, sel, &ctx, &accent, true, page);
                            for (layers, bg) in grounds {
                                report.pairs.push(pair(
                                    kind.clone(),
                                    fg_value,
                                    c,
                                    &layers,
                                    bg,
                                    host,
                                ));
                            }
                        }
                        Paint::Unknown(v) => report.skipped.push(Skipped {
                            component: sheet.name.clone(),
                            selector: rule.selector.clone(),
                            value: v,
                        }),
                        _ => {}
                    }
                }
                if focus {
                    let indicators = focus_indicators(sheet, rule, sel, &ctx);
                    if indicators.is_empty() {
                        continue;
                    }
                    let (own, _) = self.grounds(sheet, sel, &ctx, &accent, true, page);
                    let (around, host) = self.grounds(sheet, sel, &ctx, &accent, false, page);
                    // Each surrounding background needs one indicator part at 3:1: an outer
                    // ring against it, or an inner one against the element.
                    for (layers, bg) in around {
                        let best = indicators
                            .iter()
                            .flat_map(|(src, c, inside)| {
                                if *inside {
                                    own.iter()
                                        .map(|(l, b)| pair(Kind::Focus, src, *c, l, *b, host))
                                        .collect::<Vec<_>>()
                                } else {
                                    vec![pair(Kind::Focus, src, *c, &layers, bg, host)]
                                }
                            })
                            .max_by(|a, b| a.ratio.total_cmp(&b.ratio));
                        report.pairs.extend(best);
                    }
                }
            }
        }
    }
}

const FOCUS_PROPS: &[&str] = &[
    "outline",
    "outline-color",
    "border-color",
    "border",
    "box-shadow",
];

/// The parts of a focus style that differ from the unfocused one, as (value, colour, drawn
/// inside the element). A ring the element also has without focus doesn't indicate focus.
fn focus_indicators(
    sheet: &Sheet,
    rule: &Rule,
    sel: &Selector,
    ctx: &Ctx,
) -> Vec<(String, Rgba, bool)> {
    let mut plain = rule.selector.clone();
    for p in [":focus-visible", ":focus-within", ":focus"] {
        plain = plain.replace(p, "");
    }
    let unfocused = Selector::parse(&plain);
    let mut out = Vec::new();
    for prop in FOCUS_PROPS {
        let Some(value) = sheet.cascaded(sel, |p| p == *prop) else {
            continue;
        };
        if sheet.cascaded(&unfocused, |p| p == *prop) == Some(value) {
            continue;
        }
        let resolved = ctx.substitute(value, 0).unwrap_or_default();
        for layer in split_top(&resolved, ',') {
            let colours: Vec<Rgba> = split_top(layer.trim(), ' ')
                .into_iter()
                .filter_map(color)
                .filter(|c| c.a > 0.0)
                .collect();
            if let [c] = colours.as_slice() {
                let inside = *prop == "box-shadow" && layer.contains("inset");
                out.push((value.to_string(), *c, inside));
            }
        }
    }
    out
}

/// Runs the proof over `(component, css)` sheets with `theme` and the markup in `nesting`.
pub fn check(theme: &Theme, sheets: &[(String, String)], nesting: &Nesting) -> Report {
    Kit::new(theme, sheets, nesting).check()
}

/// The proof on a checkout of the kit at `root`: its theme with `themes` loaded after it (like
/// an app's own stylesheet), every component stylesheet, and the SSR snapshots for nesting.
pub fn run(root: &std::path::Path, themes: &[std::path::PathBuf]) -> Result<Report, String> {
    let read = |p: &std::path::Path| {
        std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
    };
    let ui = root.join("crates/ui");
    let mut theme = Theme::parse(&read(&ui.join("assets/theme.css"))?);
    for t in themes {
        theme.overlay(Theme::parse(&read(t)?));
    }
    let sheets = component_sheets(&ui.join("src/components")).map_err(|e| e.to_string())?;
    let nesting = snapshot_nesting(&ui.join("tests/snapshots")).map_err(|e| e.to_string())?;
    Ok(check(&theme, &sheets, &nesting))
}

/// Failures grouped by rule and colours, with the modes and accents each fails in.
pub fn summary(report: &Report) -> Vec<String> {
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for f in report.failures() {
        groups
            .entry(format!(
                "{} `{}`: {} on {}",
                f.component, f.selector, f.fg_source, f.bg_source
            ))
            .or_default()
            .push(format!("{} {} {:.2}:1", f.mode.name(), f.accent, f.ratio));
    }
    groups
        .into_iter()
        .map(|(k, v)| format!("{k}\n    {}", v.join(", ")))
        .collect()
}

/// Reads `components/<name>/style.css` for every component under `dir`, sorted by name.
pub fn component_sheets(dir: &std::path::Path) -> std::io::Result<Vec<(String, String)>> {
    let mut out = BTreeMap::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path().join("style.css");
        if path.is_file() {
            let name = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            out.insert(name, std::fs::read_to_string(&path)?);
        }
    }
    Ok(out.into_iter().collect())
}

/// Reads every `*.html` under `dir` (the SSR snapshots) into a [`Nesting`].
pub fn snapshot_nesting(dir: &std::path::Path) -> std::io::Result<Nesting> {
    let mut nesting = Nesting::default();
    let mut files: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "html"))
        .collect();
    files.sort();
    for f in files {
        nesting.add_html(&std::fs::read_to_string(f)?);
    }
    Ok(nesting)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> Theme {
        Theme::parse(
            ":root { --bg: light-dark(#fff, #000); --fg: light-dark(#000, #fff); --muted: light-dark(#777, #888); }
             [data-accent=\"x\"] { --accent: light-dark(#1d4ed8, #60a5fa); }",
        )
    }

    #[test]
    fn colours_parse() {
        assert_eq!(color("#fff").map(Rgba::hex), Some("#ffffff".into()));
        assert_eq!(color("rgb(0 0 0 / 40%)").map(|c| c.a), Some(0.4));
        assert_eq!(color("rgba(10, 20, 30, 0.5)").map(|c| c.g), Some(20.0));
        assert_eq!(
            color("#FFFFFF26").map(|c| (c.a * 255.0).round()),
            Some(38.0)
        );
        let mid = color("color-mix(in srgb, #000 50%, #fff)").unwrap();
        assert!((mid.r - 127.5).abs() < 0.01);
        // OKLab mix with transparent keeps the colour and halves the alpha.
        let half = color("color-mix(in oklab, #2b7fff 50%, transparent)").unwrap();
        assert!((half.a - 0.5).abs() < 1e-9 && (half.r - 43.0).abs() < 0.5);
    }

    #[test]
    fn ratios_match_wcag_examples() {
        let r = ratio(color("#777").unwrap(), color("#fff").unwrap());
        assert!((r - 4.48).abs() < 0.01, "{r}");
        assert!((ratio(color("#000").unwrap(), color("#fff").unwrap()) - 21.0).abs() < 1e-9);
    }

    #[test]
    fn substitution_handles_toggles_light_dark_and_fallbacks() {
        let t = theme();
        let ctx = |mode| Ctx {
            mode,
            layers: vec![&t.root],
        };
        assert_eq!(
            ctx(Mode::Dark).paint("var(--fg)"),
            Paint::Color(color("#fff").unwrap())
        );
        assert_eq!(
            ctx(Mode::Light).paint("var(--light, var(--fg)) var(--dark, var(--bg))"),
            Paint::Color(color("#000").unwrap())
        );
        assert_eq!(ctx(Mode::Light).paint("var(--dark, #123)"), Paint::None);
        assert_eq!(
            ctx(Mode::Light).paint("var(--nope, #123)"),
            Paint::Color(color("#123").unwrap())
        );
        assert_eq!(ctx(Mode::Light).paint("var(--nope)"), Paint::None);
    }

    #[test]
    fn nesting_and_selectors() {
        let rules = parse(".a { color: red; &:hover { color: blue; } .b { color: green } @supports (x: y) { top: 0; } } @keyframes k { 0% { color: red } }");
        let sels: Vec<&str> = rules.iter().map(|r| r.selector.as_str()).collect();
        assert_eq!(sels, [".a", ".a:hover", ".a .b", ".a"]);
        let base = Selector::parse(".item");
        assert!(base.is_base_of(&Selector::parse(".content .item:hover")));
        assert!(!Selector::parse(".item:hover").is_base_of(&base));
        assert!(is_disabled(".item[data-disabled=\"true\"]"));
        assert!(!is_disabled(".item:hover:not([data-disabled=\"true\"])"));
    }

    #[test]
    fn finds_a_failing_pair_and_its_background() {
        let sheets = vec![(
            "demo".to_string(),
            ".box { background: var(--bg); color: var(--fg); }
             .box .hint { color: var(--muted); }
             .tag { color: var(--muted); }
             .btn:hover { background: var(--accent); }
             .btn:disabled { color: var(--bg); }"
                .to_string(),
        )];
        let report = check(&theme(), &sheets, &Nesting::default());
        let failing: Vec<String> = report.failures().map(Pair::key).collect();
        // #777 on white is 4.48:1; on black (dark mode, #888) it passes.
        assert!(
            failing.contains(&"demo .box .hint light neutral".to_string()),
            "{failing:?}"
        );
        // Page text (--secondary-color-4) is undefined here, so the hover pair has no fg.
        assert!(!failing.iter().any(|k| k.contains("disabled")));
        assert!(report
            .pairs
            .iter()
            .any(|p| p.selector == ".tag" && p.accent == "x"));
    }
}
