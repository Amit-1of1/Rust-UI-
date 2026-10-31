//! `rust-ui theme --from-image <logo>`: find the brand colour in a logo (PNG, JPEG or SVG) and
//! hand it to the same contrast-fitting generator as `--brand`.
//!
//! The colours are counted, the logo's background (the colour of its border pixels, or a
//! transparent one) is left out, and the most visible saturated colour wins: share of the logo
//! times saturation, so a small orange mark beats a large near-grey wordmark. A second colour
//! that is clearly a different hue is reported too, as an alternative preset.

use crate::theme::Rgb;
use std::collections::HashMap;
use std::path::Path;

/// One colour of the logo and how much of it there is (0..1 of the counted pixels or
/// SVG paints).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Swatch {
    pub rgb: Rgb,
    pub share: f64,
}

/// What was found in a logo.
#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    /// Every colour, most common first (background excluded).
    pub swatches: Vec<Swatch>,
    /// The brand colour: the most visible saturated colour, or the most common one when the
    /// logo has no saturated colour.
    pub brand: Rgb,
    /// A second, clearly different hue, when the logo has one.
    pub second: Option<Rgb>,
    /// The background that was left out, if any.
    pub background: Option<Rgb>,
    /// The logo has no saturated colour (black, white or grey only).
    pub monochrome: bool,
}

/// Colours closer than this (per channel, 0..255) count as one.
const BUCKET: u8 = 16;
/// HSL saturation below this is grey.
const MIN_SATURATION: f64 = 0.25;
/// A second colour must be at least this far round the colour wheel from the brand.
const MIN_HUE_GAP: f64 = 40.0;
/// ... and at least this share of the logo.
const MIN_SECOND_SHARE: f64 = 0.03;
/// Raster logos are sampled down to about this many pixels.
const SAMPLE: usize = 160_000;

pub fn read(path: &Path) -> Result<Palette, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if ext == "svg" {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        return from_svg(&text);
    }
    let img = image::open(path)
        .map_err(|e| format!("{}: {e} (PNG, JPEG and SVG are supported)", path.display()))?
        .to_rgba8();
    let (w, h) = img.dimensions();
    let pixels: Vec<[u8; 4]> = img.pixels().map(|p| p.0).collect();
    from_pixels(&pixels, w as usize, h as usize)
}

/// From RGBA pixels, row by row.
pub fn from_pixels(pixels: &[[u8; 4]], width: usize, height: usize) -> Result<Palette, String> {
    if width == 0 || height == 0 || pixels.len() != width * height {
        return Err("the image is empty".into());
    }
    let step = ((width * height) as f64 / SAMPLE as f64)
        .sqrt()
        .ceil()
        .max(1.0) as usize;
    // The background is whatever most of the border is (transparent counts as background).
    let mut border: HashMap<Rgb, usize> = HashMap::new();
    let mut transparent_border = 0usize;
    let mut border_total = 0usize;
    for y in 0..height {
        for x in 0..width {
            if x != 0 && y != 0 && x != width - 1 && y != height - 1 {
                continue;
            }
            border_total += 1;
            let p = pixels[y * width + x];
            if p[3] < 128 {
                transparent_border += 1;
            } else {
                *border.entry(bucket(Rgb(p[0], p[1], p[2]))).or_default() += 1;
            }
        }
    }
    // Only a neutral border is a backdrop: a logo drawn on its own brand colour (white mark on
    // a blue square) keeps the blue.
    let background = border
        .into_iter()
        .max_by_key(|(_, n)| *n)
        .filter(|(_, n)| *n * 2 > border_total && transparent_border * 2 < border_total)
        .map(|(c, _)| c)
        .filter(|c| !is_saturated(*c));

    let mut counts: HashMap<Rgb, (usize, [u64; 3])> = HashMap::new();
    let mut backdrop = (0u64, [0u64; 3]);
    for y in (0..height).step_by(step) {
        for x in (0..width).step_by(step) {
            let p = pixels[y * width + x];
            if p[3] < 128 {
                continue;
            }
            let c = Rgb(p[0], p[1], p[2]);
            let key = bucket(c);
            if Some(key) == background {
                backdrop.0 += 1;
                backdrop.1[0] += u64::from(c.0);
                backdrop.1[1] += u64::from(c.1);
                backdrop.1[2] += u64::from(c.2);
                continue;
            }
            let e = counts.entry(key).or_default();
            e.0 += 1;
            e.1[0] += u64::from(c.0);
            e.1[1] += u64::from(c.1);
            e.1[2] += u64::from(c.2);
        }
    }
    // Each bucket's colour is the mean of its pixels, not the bucket corner.
    let weighted = counts
        .into_values()
        .map(|(n, sum)| {
            let m = |s: u64| (s / n as u64) as u8;
            (Rgb(m(sum[0]), m(sum[1]), m(sum[2])), n as f64)
        })
        .collect();
    // Report the backdrop as its real colour, not its bucket.
    let background = background.map(|b| match backdrop {
        (0, _) => b,
        (n, sum) => Rgb((sum[0] / n) as u8, (sum[1] / n) as u8, (sum[2] / n) as u8),
    });
    pick(weighted, background)
}

/// From an SVG's paints: every `#rgb`, `#rrggbb` and `rgb(r, g, b)` in `fill`, `stroke`,
/// `stop-color` and `style`. Each paint counts once; a white `fill` on the root rectangle is
/// the background.
pub fn from_svg(svg: &str) -> Result<Palette, String> {
    let mut found: Vec<Rgb> = Vec::new();
    let lower = svg.to_ascii_lowercase();
    let b = lower.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'#' {
            let hex: String = lower[i + 1..]
                .chars()
                .take_while(|c| c.is_ascii_hexdigit())
                .collect();
            // Only paint positions: `fill="#..."`, `fill:#...`, `stop-color=...`.
            let before = lower[..i].trim_end_matches(['"', '\'', ' ', ':', '=']);
            let paint = ["fill", "stroke", "stop-color", "color", "flood-color"]
                .iter()
                .any(|p| before.ends_with(p));
            if paint && (hex.len() == 3 || hex.len() == 6) {
                if let Ok(c) = Rgb::parse(&hex) {
                    found.push(c);
                }
            }
            i += 1 + hex.len();
        } else if lower[i..].starts_with("rgb(") {
            let end = lower[i..].find(')').map_or(b.len(), |e| i + e);
            let parts: Vec<u8> = lower[i + 4..end]
                .split(',')
                .filter_map(|p| p.trim().parse::<f64>().ok())
                .map(|v| v.clamp(0.0, 255.0) as u8)
                .collect();
            if let [r, g, b] = parts[..] {
                found.push(Rgb(r, g, b));
            }
            i = end;
        } else {
            i += 1;
        }
    }
    if found.is_empty() {
        return Err("no colours found in the SVG's fill, stroke or stop-color".into());
    }
    let mut counts: HashMap<Rgb, f64> = HashMap::new();
    for c in &found {
        *counts.entry(*c).or_default() += 1.0;
    }
    let background = (counts.len() > 1)
        .then(|| {
            found
                .first()
                .copied()
                .filter(|c| c.luminance() > 0.9 || c.luminance() < 0.01)
        })
        .flatten()
        .filter(|_| svg_starts_with_backdrop(&lower));
    if let Some(bg) = background {
        if let Some(n) = counts.get_mut(&bg) {
            *n -= 1.0;
        }
        counts.retain(|_, n| *n > 0.0);
    }
    pick(counts.into_iter().collect(), background)
}

/// A first shape that fills the whole canvas (`width="100%"` or a `rect` with no position).
fn svg_starts_with_backdrop(svg: &str) -> bool {
    let Some(i) = svg.find("<rect") else {
        return false;
    };
    let shape_before = ["<path", "<circle", "<ellipse", "<polygon", "<text", "<g "]
        .iter()
        .any(|t| svg[..i].contains(t));
    let tag = &svg[i..svg[i..].find('>').map_or(svg.len(), |e| i + e)];
    !shape_before && (tag.contains("100%") || !(tag.contains(" x=") || tag.contains(" y=")))
}

/// Colourful enough to be a brand colour: not grey, and not nearly white or black (where HSL
/// saturation means little).
fn is_saturated(c: Rgb) -> bool {
    let (_, sat, l) = c.hsl();
    sat >= MIN_SATURATION && (0.12..=0.92).contains(&l)
}

fn bucket(c: Rgb) -> Rgb {
    let q = |v: u8| v / BUCKET * BUCKET;
    Rgb(q(c.0), q(c.1), q(c.2))
}

fn hue_gap(a: f64, b: f64) -> f64 {
    let d = (a - b).abs() % 360.0;
    d.min(360.0 - d)
}

fn pick(weighted: Vec<(Rgb, f64)>, background: Option<Rgb>) -> Result<Palette, String> {
    let total: f64 = weighted.iter().map(|(_, n)| n).sum();
    if total <= 0.0 {
        return Err("the logo has no colours besides its background".into());
    }
    let mut swatches: Vec<Swatch> = weighted
        .into_iter()
        .map(|(rgb, n)| Swatch {
            rgb,
            share: n / total,
        })
        .collect();
    swatches.sort_by(|a, b| {
        b.share
            .total_cmp(&a.share)
            .then(a.rgb.hex().cmp(&b.rgb.hex()))
    });
    // Score: how much of the logo it is, times how colourful. Group near hues first so a
    // gradient or anti-aliased edge counts as one colour.
    let saturated: Vec<(Swatch, f64, f64)> = swatches
        .iter()
        .filter(|s| is_saturated(s.rgb))
        .map(|s| {
            let (h, sat, _) = s.rgb.hsl();
            (*s, h, sat)
        })
        .collect();
    let score_of = |hue: f64| -> f64 {
        saturated
            .iter()
            .filter(|(_, h, _)| hue_gap(*h, hue) < 15.0)
            .map(|(s, _, sat)| s.share * sat)
            .sum()
    };
    // The hue group that scores highest, represented by its most common swatch (not by an
    // anti-aliased edge that happens to sit between two groups).
    let representative = |hue: f64| {
        saturated
            .iter()
            .filter(|(_, h, _)| hue_gap(*h, hue) < 15.0)
            .max_by(|a, b| a.0.share.total_cmp(&b.0.share))
    };
    let best = saturated
        .iter()
        .max_by(|a, b| score_of(a.1).total_cmp(&score_of(b.1)))
        .and_then(|(_, h, _)| representative(*h));
    let (brand, monochrome) = match best {
        Some((s, ..)) => (s.rgb, false),
        None => (swatches[0].rgb, true),
    };
    let second = best.and_then(|(_, brand_hue, _)| {
        saturated
            .iter()
            .filter(|(_, h, _)| hue_gap(*h, *brand_hue) >= MIN_HUE_GAP)
            .filter(|(_, h, _)| {
                saturated
                    .iter()
                    .filter(|(_, h2, _)| hue_gap(*h, *h2) < 15.0)
                    .map(|(s, ..)| s.share)
                    .sum::<f64>()
                    >= MIN_SECOND_SHARE
            })
            .max_by(|a, b| score_of(a.1).total_cmp(&score_of(b.1)))
            .and_then(|(_, h, _)| representative(*h))
            .map(|(s, ..)| s.rgb)
    });
    Ok(Palette {
        swatches,
        brand,
        second,
        background,
        monochrome,
    })
}

/// A CSS comment listing the colours found, for the top of the printed preset.
pub fn summary(p: &Palette) -> String {
    let mut out = String::from("/* Colours found in the logo (share of the logo):\n");
    for s in p.swatches.iter().filter(|s| s.share >= 0.01).take(8) {
        let mark = if s.rgb == p.brand {
            "  <- brand"
        } else if Some(s.rgb) == p.second {
            "  <- second colour"
        } else {
            ""
        };
        out.push_str(&format!(
            " *   {}  {:>5.1}%{mark}\n",
            s.rgb.hex(),
            s.share * 100.0
        ));
    }
    if let Some(bg) = p.background {
        out.push_str(&format!(" * Background {} left out.\n", bg.hex()));
    }
    if p.monochrome {
        out.push_str(" * The logo has no saturated colour, so the accent is its main colour.\n");
    }
    out.push_str(" * Fills are adjusted until labels on them reach 4.5:1 (WCAG AA). */\n");
    out
}
