//! `rust-ui theme --from-image`: the brand colour found in a logo, and a readable preset from it.

use rust_ui_cli::logo::{from_pixels, from_svg, read};
use rust_ui_cli::theme::{contrast, preset, Rgb};

const WHITE: [u8; 4] = [255, 255, 255, 255];
const CLEAR: [u8; 4] = [0, 0, 0, 0];

fn rgba(hex: &str) -> [u8; 4] {
    let c = Rgb::parse(hex).expect("hex");
    [c.0, c.1, c.2, 255]
}

/// A `w` x `h` canvas of `bg` with rectangles painted on it.
fn canvas(
    w: usize,
    h: usize,
    bg: [u8; 4],
    rects: &[(usize, usize, usize, usize, [u8; 4])],
) -> Vec<[u8; 4]> {
    let mut px = vec![bg; w * h];
    for &(x0, y0, x1, y1, c) in rects {
        for y in y0..y1 {
            for x in x0..x1 {
                px[y * w + x] = c;
            }
        }
    }
    px
}

#[test]
fn a_small_coloured_mark_beats_a_large_grey_wordmark() {
    // Orange mark (10% of the logo) beside a dark grey wordmark (30%), on white.
    let px = canvas(
        100,
        100,
        WHITE,
        &[
            (5, 30, 25, 80, rgba("#f97316")),
            (30, 40, 95, 86, rgba("#374151")),
        ],
    );
    let p = from_pixels(&px, 100, 100).expect("palette");
    assert_eq!(p.brand, Rgb::parse("#f97316").unwrap());
    assert_eq!(p.background, Some(Rgb(255, 255, 255)));
    assert!(!p.monochrome);
}

#[test]
fn a_transparent_logo_with_two_hues_reports_both() {
    let px = canvas(
        100,
        100,
        CLEAR,
        &[
            (10, 10, 60, 90, rgba("#2563eb")),
            (65, 10, 90, 90, rgba("#16a34a")),
        ],
    );
    let p = from_pixels(&px, 100, 100).expect("palette");
    assert_eq!(p.brand, Rgb::parse("#2563eb").unwrap());
    assert_eq!(p.second, Some(Rgb::parse("#16a34a").unwrap()));
    assert_eq!(p.background, None);
}

#[test]
fn a_logo_drawn_on_its_brand_colour_keeps_it() {
    // White mark on a full blue square: blue is the border, but it is the brand, not a backdrop.
    let px = canvas(60, 60, rgba("#1d4ed8"), &[(20, 20, 40, 40, WHITE)]);
    let p = from_pixels(&px, 60, 60).expect("palette");
    assert_eq!(p.brand, Rgb::parse("#1d4ed8").unwrap());
    assert_eq!(p.background, None);
}

#[test]
fn a_black_and_white_logo_is_monochrome() {
    let px = canvas(50, 50, WHITE, &[(10, 10, 40, 40, rgba("#111111"))]);
    let p = from_pixels(&px, 50, 50).expect("palette");
    assert!(p.monochrome);
    assert_eq!(p.second, None);
}

#[test]
fn svg_paints_are_counted_and_the_backdrop_skipped() {
    let svg = r##"<svg viewBox="0 0 10 10"><rect width="100%" height="100%" fill="#FFFFFF"/>
        <circle r="4" fill="#7c3aed"/><path style="fill:#f59e0b;stroke:none"/><path fill="#7C3AED"/>
        <linearGradient><stop offset="0" stop-color="rgb(124, 58, 237)"/></linearGradient></svg>"##;
    let p = from_svg(svg).expect("palette");
    assert_eq!(p.brand, Rgb::parse("#7c3aed").unwrap());
    assert_eq!(p.second, Some(Rgb::parse("#f59e0b").unwrap()));
    assert_eq!(p.background, Some(Rgb(255, 255, 255)));
    // An id or a URL fragment is not a colour.
    assert!(from_svg(r##"<svg><use href="#abc"/></svg>"##).is_err());
}

#[test]
fn png_and_jpeg_files_are_read() {
    let dir = tempfile::tempdir().expect("tempdir");
    let px = canvas(64, 32, WHITE, &[(4, 4, 28, 28, rgba("#db2777"))]);
    let mut img = image::RgbaImage::new(64, 32);
    for (i, p) in img.pixels_mut().enumerate() {
        *p = image::Rgba(px[i]);
    }
    let png = dir.path().join("logo.png");
    img.save(&png).expect("png");
    assert_eq!(
        read(&png).expect("png palette").brand,
        Rgb::parse("#db2777").unwrap()
    );

    // JPEG shifts colours a little; the brand stays the same hue.
    let jpg = dir.path().join("logo.jpg");
    image::DynamicImage::ImageRgba8(img)
        .to_rgb8()
        .save(&jpg)
        .expect("jpeg");
    let brand = read(&jpg).expect("jpeg palette").brand;
    let (h, _, _) = brand.hsl();
    let (want, _, _) = Rgb::parse("#db2777").unwrap().hsl();
    assert!((h - want).abs() < 6.0, "{} vs #db2777", brand.hex());

    assert!(read(&dir.path().join("missing.png")).is_err());
}

#[test]
fn every_extracted_brand_gets_a_readable_preset() {
    for hex in ["#f97316", "#facc15", "#22d3ee", "#1e1b4b", "#f5f5f4"] {
        let px = canvas(40, 40, CLEAR, &[(5, 5, 35, 35, rgba(hex))]);
        let p = preset(from_pixels(&px, 40, 40).expect("palette").brand).expect("preset");
        assert!(contrast(p.light_fill, p.light_text) >= 4.5, "{hex} light");
        assert!(contrast(p.dark_fill, p.dark_text) >= 4.5, "{hex} dark");
    }
}
