//! `atoll theme --brand <hex>` (or `--from-image <logo>`, see `logo.rs`): turn one brand colour into an accent preset that is
//! readable in both modes. The hue and saturation stay; only lightness moves, until the
//! label text on the fill reaches WCAG AA (4.5:1). The preset test in `crates/ui` holds
//! the same bar for every shipped preset.

const DARK_TEXT: &str = "#0a0a0a";
const LIGHT_TEXT: &str = "#ffffff";
const AA: f64 = 4.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn parse(hex: &str) -> Result<Rgb, String> {
        let h = hex.trim().trim_start_matches('#');
        let full: String = match h.len() {
            3 => h.chars().flat_map(|c| [c, c]).collect(),
            6 => h.to_string(),
            _ => return Err(format!("`{hex}` is not a 3 or 6 digit hex colour")),
        };
        let v = |i: usize| {
            u8::from_str_radix(&full[i..i + 2], 16).map_err(|_| format!("`{hex}` is not hex"))
        };
        Ok(Rgb(v(0)?, v(2)?, v(4)?))
    }

    pub fn hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }

    pub fn luminance(self) -> f64 {
        let c = |v: u8| {
            let c = f64::from(v) / 255.0;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * c(self.0) + 0.7152 * c(self.1) + 0.0722 * c(self.2)
    }

    /// (hue 0..360, saturation 0..1, lightness 0..1)
    pub fn hsl(self) -> (f64, f64, f64) {
        let (r, g, b) = (
            f64::from(self.0) / 255.0,
            f64::from(self.1) / 255.0,
            f64::from(self.2) / 255.0,
        );
        let (max, min) = (r.max(g).max(b), r.min(g).min(b));
        let l = (max + min) / 2.0;
        let d = max - min;
        if d == 0.0 {
            return (0.0, 0.0, l);
        }
        let s = d / (1.0 - (2.0 * l - 1.0).abs());
        let h = if max == r {
            ((g - b) / d).rem_euclid(6.0)
        } else if max == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };
        (h * 60.0, s, l)
    }

    pub fn from_hsl(h: f64, s: f64, l: f64) -> Rgb {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        let (r, g, b) = match (h / 60.0) as u32 % 6 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        let q = |v: f64| ((v + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        Rgb(q(r), q(g), q(b))
    }
}

pub fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (a.luminance(), b.luminance());
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// Move the brand's lightness (toward `dir`: -1 darker, +1 lighter) until `text` on the
/// fill reaches AA. Returns `None` if no lightness works (cannot happen for black/white).
fn fit(brand: Rgb, text: Rgb, dir: f64) -> Option<Rgb> {
    let (h, s, l) = brand.hsl();
    let mut l = l;
    while (0.0..=1.0).contains(&l) {
        let fill = Rgb::from_hsl(h, s, l);
        if contrast(fill, text) >= AA {
            return Some(fill);
        }
        l += dir * 0.005;
    }
    None
}

#[derive(Debug, Clone, PartialEq)]
pub struct Preset {
    pub dark_fill: Rgb,
    pub dark_text: Rgb,
    pub light_fill: Rgb,
    pub light_text: Rgb,
}

pub fn preset(brand: Rgb) -> Result<Preset, String> {
    let dark_text = Rgb::parse(DARK_TEXT)?;
    let light_text = Rgb::parse(LIGHT_TEXT)?;
    // Dark mode: a lighter fill with near-black text. Light mode: a deeper fill with white.
    let dark_fill = fit(brand, dark_text, 1.0).ok_or("no readable dark-mode fill")?;
    let light_fill = fit(brand, light_text, -1.0).ok_or("no readable light-mode fill")?;
    Ok(Preset {
        dark_fill,
        dark_text,
        light_fill,
        light_text,
    })
}

/// The CSS block, in the same shape as the presets in `theme.css`.
pub fn css(name: &str, p: &Preset) -> String {
    let fill = format!("light-dark({}, {})", p.light_fill.hex(), p.dark_fill.hex());
    format!(
        "[data-accent=\"{name}\"] {{\n  --accent-color: {fill};\n  --accent-contrast: light-dark({}, {});\n  --focused-border-color: {fill};\n}}\n",
        p.light_text.hex(),
        p.dark_text.hex()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_prints_hex() {
        assert_eq!(Rgb::parse("#fff").unwrap(), Rgb(255, 255, 255));
        assert_eq!(Rgb::parse("1d4ed8").unwrap().hex(), "#1d4ed8");
        assert!(Rgb::parse("#12").is_err());
        assert!(Rgb::parse("#gggggg").is_err());
    }

    #[test]
    fn hsl_round_trips() {
        for hex in ["#1d4ed8", "#fb923c", "#15803d", "#808080"] {
            let c = Rgb::parse(hex).unwrap();
            let (h, s, l) = c.hsl();
            let back = Rgb::from_hsl(h, s, l);
            assert!(
                (i32::from(c.0) - i32::from(back.0)).abs() <= 1
                    && (i32::from(c.1) - i32::from(back.1)).abs() <= 1
                    && (i32::from(c.2) - i32::from(back.2)).abs() <= 1,
                "{hex} -> {}",
                back.hex()
            );
        }
    }

    #[test]
    fn every_brand_colour_gets_a_readable_preset() {
        // Sweep the colour wheel at several saturations and lightnesses, plus the extremes.
        let mut brands = vec![Rgb(0, 0, 0), Rgb(255, 255, 255), Rgb(255, 255, 0)];
        for h in (0..360).step_by(15) {
            for (s, l) in [(1.0, 0.5), (0.6, 0.3), (0.9, 0.8), (0.2, 0.5)] {
                brands.push(Rgb::from_hsl(f64::from(h), s, l));
            }
        }
        for brand in brands {
            let p = preset(brand).unwrap();
            assert!(contrast(p.dark_fill, p.dark_text) >= AA, "{}", brand.hex());
            assert!(
                contrast(p.light_fill, p.light_text) >= AA,
                "{}",
                brand.hex()
            );
        }
    }

    #[test]
    fn a_readable_brand_is_left_alone() {
        // Already AA with white text, so light mode keeps it.
        let p = preset(Rgb::parse("#1d4ed8").unwrap()).unwrap();
        assert_eq!(p.light_fill.hex(), "#1d4ed8");
    }

    #[test]
    fn css_has_the_shape_the_theme_tests_parse() {
        let p = preset(Rgb::parse("#7c3aed").unwrap()).unwrap();
        let css = css("brand", &p);
        assert!(css.starts_with("[data-accent=\"brand\"] {"));
        assert!(css.contains("--accent-color: light-dark(#"));
        assert!(css.contains("--accent-contrast: light-dark(#"));
    }
}
