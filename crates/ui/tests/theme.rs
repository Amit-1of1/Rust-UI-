//! Every accent preset in `assets/theme.css` must keep its label text readable on the fill
//! (WCAG AA, 4.5:1) in both light and dark.

const THEME: &str = include_str!("../assets/theme.css");

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

/// `var(--dark, #a) var(--light, #b)` -> (dark, light)
fn pair(value: &str) -> (String, String) {
    let pick = |marker: &str| {
        let start = value.find(marker).unwrap() + marker.len();
        let rest = &value[start..];
        rest[..rest.find(')').unwrap()].trim().to_string()
    };
    (pick("var(--dark,"), pick("var(--light,"))
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
    for chunk in THEME.split("[data-accent=\"").skip(1) {
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
