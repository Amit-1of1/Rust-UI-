//! Proof that `atoll theme --brand <colour>` gives a readable preset for **every** brand colour,
//! not a sample: all 16,777,216 sRGB colours. The checks use `contrast::ratio` (the contrast
//! proof's own code) as the judge, not the generator's `theme::contrast`, and pin both to
//! published WCAG values, so a wrong formula can't agree with itself.
//!
//! The full sweep takes about a minute in release and much longer in debug, so it is
//! `#[ignore]`d in `cargo test` and run by `scripts/proofs.sh` (the "Proofs" workflow):
//! `cargo test --release -p atoll-cli --test theme_proof -- --ignored`.

use atoll_cli::contrast::{ratio, Rgba};
use atoll_cli::theme::{contrast, preset, Rgb};
use std::sync::atomic::{AtomicU64, Ordering};

fn rgba(c: Rgb) -> Rgba {
    Rgba {
        r: f64::from(c.0),
        g: f64::from(c.1),
        b: f64::from(c.2),
        a: 1.0,
    }
}

fn close(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

/// Values from the WCAG 2 definition and its well-known examples (relative luminance of sRGB
/// grey levels, as published by W3C and WebAIM's checker), so the formula is checked against the
/// outside world, not against itself.
#[test]
fn contrast_matches_published_wcag_values() {
    let c = |a: &str, b: &str| contrast(Rgb::parse(a).unwrap(), Rgb::parse(b).unwrap());
    assert_eq!(c("#000000", "#ffffff"), 21.0);
    assert_eq!(c("#ffffff", "#ffffff"), 1.0);
    // Order doesn't matter.
    assert_eq!(c("#1d4ed8", "#ffffff"), c("#ffffff", "#1d4ed8"));
    // #777777 on white is the classic "just fails AA": 4.48:1.
    assert!(
        close(c("#777777", "#ffffff"), 4.478, 0.001),
        "{}",
        c("#777777", "#ffffff")
    );
    // #767676 is the lightest grey that passes: 4.54:1.
    assert!(
        close(c("#767676", "#ffffff"), 4.542, 0.001),
        "{}",
        c("#767676", "#ffffff")
    );
    // Pure primaries on white (WebAIM): red 4.00, green 1.37, blue 8.59.
    assert!(
        close(c("#ff0000", "#ffffff"), 3.998, 0.001),
        "{}",
        c("#ff0000", "#ffffff")
    );
    assert!(
        close(c("#00ff00", "#ffffff"), 1.372, 0.001),
        "{}",
        c("#00ff00", "#ffffff")
    );
    assert!(
        close(c("#0000ff", "#ffffff"), 8.592, 0.001),
        "{}",
        c("#0000ff", "#ffffff")
    );
    // Each channel's weight: luminance of the primaries is 0.2126, 0.7152, 0.0722.
    assert!(close(Rgb(255, 0, 0).luminance(), 0.2126, 1e-12));
    assert!(close(Rgb(0, 255, 0).luminance(), 0.7152, 1e-12));
    assert!(close(Rgb(0, 0, 255).luminance(), 0.0722, 1e-12));
    // The linear segment at the dark end: 8/255 is below the knee, so it is (8/255)/12.92.
    assert!(close(Rgb(8, 8, 8).luminance(), 8.0 / 255.0 / 12.92, 1e-15));
    // The curve above it: 128/255 is 0.2158605 linear.
    assert!(close(Rgb(128, 128, 128).luminance(), 0.215_860_5, 1e-7));
    // The kit's dark text, #0a0a0a, on white: 19.8:1.
    assert!(
        close(c("#0a0a0a", "#ffffff"), 19.80, 0.01),
        "{}",
        c("#0a0a0a", "#ffffff")
    );
}

#[test]
fn hsl_corners_and_every_hue_sextant() {
    // One colour in each sixth of the hue wheel, so no arm of `from_hsl` goes untested.
    for (hex, h) in [
        ("#ff0000", 0.0),
        ("#ffff00", 60.0),
        ("#00ff00", 120.0),
        ("#00ffff", 180.0),
        ("#0000ff", 240.0),
        ("#ff00ff", 300.0),
        ("#ff8000", 30.0),
        ("#80ff00", 90.0),
        ("#00ff80", 150.0),
        ("#0080ff", 210.0),
        ("#8000ff", 270.0),
        ("#ff0080", 330.0),
    ] {
        let c = Rgb::parse(hex).unwrap();
        let (hh, s, l) = c.hsl();
        assert!(close(hh, h, 0.5), "{hex}: hue {hh}");
        assert!(close(s, 1.0, 1e-9) && close(l, 0.5, 0.01), "{hex}: {s} {l}");
        assert_eq!(Rgb::from_hsl(hh, s, l), c, "{hex}");
    }
}

/// Split all 2^24 colours over the machine's cores; `check` returns an error message or None.
fn every_colour(check: impl Fn(Rgb) -> Option<String> + Sync) -> Vec<String> {
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let failures = std::sync::Mutex::new(Vec::new());
    let done = AtomicU64::new(0);
    std::thread::scope(|scope| {
        for t in 0..threads {
            let (check, failures, done) = (&check, &failures, &done);
            scope.spawn(move || {
                let mut c = t as u32;
                while c < 1 << 24 {
                    if let Some(msg) = check(Rgb((c >> 16) as u8, (c >> 8) as u8, c as u8)) {
                        let mut f = failures.lock().unwrap();
                        if f.len() < 20 {
                            f.push(msg);
                        }
                    }
                    done.fetch_add(1, Ordering::Relaxed);
                    c += threads as u32;
                }
            });
        }
    });
    assert_eq!(done.load(Ordering::Relaxed), 1 << 24);
    failures.into_inner().unwrap()
}

/// Every brand colour gets a preset whose text reaches WCAG AA (4.5:1) on its fill, in light
/// and dark mode, judged by the contrast proof's own `ratio`.
#[test]
#[ignore]
fn every_one_of_16_7_million_brand_colours_gets_a_readable_preset() {
    let failures = every_colour(|brand| match preset(brand) {
        Err(e) => Some(format!("{}: {e}", brand.hex())),
        Ok(p) => {
            let dark = ratio(rgba(p.dark_fill), rgba(p.dark_text));
            let light = ratio(rgba(p.light_fill), rgba(p.light_text));
            (dark < 4.5 || light < 4.5)
                .then(|| format!("{}: dark {dark:.3}, light {light:.3}", brand.hex()))
        }
    });
    assert!(failures.is_empty(), "{failures:#?}");
}

/// The generator's `contrast` and the contrast proof's `ratio` agree on every colour, against
/// both text colours the presets use.
#[test]
#[ignore]
fn generator_and_proof_agree_on_every_colour() {
    let (white, ink) = (Rgb(255, 255, 255), Rgb(10, 10, 10));
    let failures = every_colour(|c| {
        for t in [white, ink] {
            let (a, b) = (contrast(c, t), ratio(rgba(c), rgba(t)));
            if !close(a, b, 1e-12) {
                return Some(format!("{} on {}: {a} vs {b}", c.hex(), t.hex()));
            }
        }
        None
    });
    assert!(failures.is_empty(), "{failures:#?}");
}

/// HSL round trip: every colour comes back exactly from its own hue, saturation, lightness.
#[test]
#[ignore]
fn every_colour_survives_the_hsl_round_trip() {
    let failures = every_colour(|c| {
        let (h, s, l) = c.hsl();
        let back = Rgb::from_hsl(h, s, l);
        (back != c).then(|| format!("{} came back as {}", c.hex(), back.hex()))
    });
    assert!(failures.is_empty(), "{failures:#?}");
}
