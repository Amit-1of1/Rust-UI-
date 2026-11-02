//! The theme builder (`#theme-builder`): pick a brand colour and see the kit wear it, in light
//! and dark mode, with the contrast it reaches and the CSS to copy.
//!
//! It runs the same code as `atoll theme --brand` (the CLI's `theme.rs`, included below), so
//! the page and the command can't disagree, and builds the page from kit components only:
//! `ColorPicker`, `Input`, `Switch`, `Checkbox`, `Slider`, `Progress`, `Table`, `Badge`.

// The CLI's colour maths, shared as source so the docs site doesn't depend on the CLI crate
// (which pulls in clap and an image decoder).
#[path = "../../../crates/cli/src/theme.rs"]
mod brand;

use brand::{contrast, css, preset, Preset, Rgb};
use dioxus::prelude::*;
use palette::{encoding, FromColor, Hsv, Srgb};
use ui::hooks::use_copy_clipboard;
use ui::prelude::*;

/// Brands to try with one click: one that is already readable, and some that are not (pale,
/// neon, mid-tone), which is where the lightness search earns its keep.
const SWATCHES: &[(&str, &str)] = &[
    ("Indigo", "#4f46e5"),
    ("Sunflower", "#facc15"),
    ("Teal", "#14b8a6"),
    ("Coral", "#fb7185"),
    ("Lime", "#a3e635"),
    ("Sky", "#38bdf8"),
];

/// The preset's name in the generated CSS and on the preview.
const NAME: &str = "brand";

/// WCAG AA for normal text.
const AA: f64 = 4.5;

fn to_hsv(c: Rgb) -> Hsv<encoding::Srgb, f64> {
    Hsv::from_color(Srgb::new(c.0, c.1, c.2).into_format::<f64>())
}

fn from_hsv(c: Hsv<encoding::Srgb, f64>) -> Rgb {
    let rgb: Srgb<u8> = Srgb::from_color(c).into_format();
    Rgb(rgb.red, rgb.green, rgb.blue)
}

#[component]
pub fn ThemeBuilderPage() -> Element {
    let mut color = use_signal(|| Rgb(0xfa, 0xcc, 0x15));
    // What the user typed, kept separately so a half-typed hex isn't overwritten.
    let mut typed = use_signal(|| color().hex());
    // The page's own light/dark mode: light-dark() tokens resolve on <html>, so the preview
    // follows the whole page rather than a data-theme on its own box.
    let theme = use_theme();
    let (copy, copied) = use_copy_clipboard();

    let brand = color();
    let p: Option<Preset> = preset(brand).ok();
    let sheet = p.as_ref().map(|p| css(NAME, p)).unwrap_or_default();
    let typed_ok = Rgb::parse(&typed()).is_ok();

    rsx! {
        document::Title { "Theme builder - Atoll" }
        h1 { id: "page-title", tabindex: "-1", "Theme builder" }
        p { class: "docs-lede",
            "Pick your brand colour and the kit wears it. Atoll keeps the colour's hue and saturation and only moves its lightness until button labels on it pass WCAG AA (4.5:1), separately for light and dark mode. This page runs the same code as "
            code { "atoll theme --brand" }
            "."
        }

        // The generated preset, scoped to its own name, so it only recolours the preview.
        style { {sheet.clone()} }

        section { "aria-labelledby": "tb-look",
            h2 { id: "tb-look", "Start from a look" }
            p {
                "A look sets the fonts, neutrals, corners and density; your brand colour below then replaces only its accent. In your app: "
                code { "ThemeStyles {{ preset: \"reef\" }}" }
                " or "
                code { "atoll theme --preset reef" }
                "."
            }
            PresetPicker {}
        }

        section { class: "tb-controls", "aria-labelledby": "tb-pick",
            h2 { id: "tb-pick", "1. Your brand colour" }
            div { class: "tb-row",
                ColorPicker {
                    label: "Pick a colour",
                    color: to_hsv(brand),
                    on_color_change: move |c| {
                        let rgb = from_hsv(c);
                        color.set(rgb);
                        typed.set(rgb.hex());
                    },
                }
                div { class: "tb-field",
                    Label { html_for: "tb-hex", "Hex" }
                    Input {
                        id: "tb-hex",
                        value: typed(),
                        autocomplete: "off",
                        "aria-invalid": if !typed_ok { "true" },
                        "aria-describedby": "tb-hex-help",
                        oninput: move |e: FormEvent| {
                            let v = e.value();
                            if let Ok(c) = Rgb::parse(&v) {
                                color.set(c);
                            }
                            typed.set(v);
                        },
                    }
                    p { id: "tb-hex-help", class: "tb-help",
                        if typed_ok { "3 or 6 hex digits, like #7c3aed." } else { "Not a hex colour yet: use 3 or 6 digits, like #7c3aed." }
                    }
                }
            }
            div { class: "tb-swatches", role: "group", "aria-label": "Example brand colours",
                for (label , hex) in SWATCHES {
                    button {
                        key: "{hex}",
                        r#type: "button",
                        class: "tb-swatch",
                        "aria-pressed": if brand.hex() == *hex { "true" } else { "false" },
                        onclick: move |_| {
                            if let Ok(c) = Rgb::parse(hex) {
                                color.set(c);
                                typed.set(c.hex());
                            }
                        },
                        span { class: "tb-chip", style: "background: {hex}", "aria-hidden": "true" }
                        "{label}"
                    }
                }
            }
        }

        if let Some(p) = p {
            section { "aria-labelledby": "tb-proof",
                h2 { id: "tb-proof", "2. What it changed, and why it is readable" }
                Table {
                    TableHeader {
                        TableRow {
                            TableHead { "Mode" }
                            TableHead { "Fill" }
                            TableHead { "Label" }
                            TableHead { "Contrast" }
                        }
                    }
                    TableBody {
                        ProofRow { mode: "Light", brand, fill: p.light_fill, text: p.light_text }
                        ProofRow { mode: "Dark", brand, fill: p.dark_fill, text: p.dark_text }
                    }
                }
            }

            section { "aria-labelledby": "tb-preview",
                h2 { id: "tb-preview", "3. The kit in your colour" }
                div { class: "tb-row",
                    Switch {
                        checked: theme.is_dark(),
                        aria_label: "Dark mode",
                        on_checked_change: move |v| theme.set_mode(if v { ThemeMode::Dark } else { ThemeMode::Light }),
                    }
                    span { "Dark mode (the whole page)" }
                }
                div {
                    class: "tb-preview",
                    "data-accent": NAME,
                    div { class: "tb-row",
                        Button { variant: ButtonVariant::Primary, "Start free trial" }
                        Button { variant: ButtonVariant::Outline, "Talk to sales" }
                    }
                    div { class: "tb-row",
                        Checkbox { id: "tb-check", default_checked: CheckboxState::Checked }
                        Label { html_for: "tb-check", "Email me product news" }
                    }
                    div { class: "tb-row tb-wide",
                        Slider {
                            label: "Volume",
                            horizontal: true,
                            min: 0.0,
                            max: 100.0,
                            step: 1.0,
                            default_value: 60.0,
                        }
                    }
                    div { class: "tb-row tb-wide",
                        Progress { aria_label: "Setup progress", value: 72.0 }
                    }
                }
            }

            section { "aria-labelledby": "tb-use",
                h2 { id: "tb-use", "4. Use it" }
                p {
                    "Add this after the kit's theme, then put "
                    code { "data-accent=\"{NAME}\"" }
                    " on "
                    code { "<html>" }
                    " (or on any element, to colour only that part of the page)."
                }
                pre { class: "tb-code", tabindex: "0", code { {sheet.clone()} } }
                div { class: "tb-row",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| copy.call(sheet.clone()),
                        if copied() { "Copied" } else { "Copy CSS" }
                    }
                    span { class: "tb-help", role: "status",
                        if copied() { "The CSS is on your clipboard." }
                    }
                }
                p {
                    "From a terminal: "
                    code { "atoll theme --brand {brand.hex()} --name {NAME}" }
                    ". From a logo: "
                    code { "atoll theme --from-image logo.png" }
                    " picks the brand colour for you. Then "
                    code { "atoll contrast --theme brand.css" }
                    " proves every text colour in every component stays readable with it."
                }
            }
        } else {
            Callout { variant: CalloutVariant::Warning, title: "No readable preset",
                "This colour has no lightness at which labels pass AA. Try another."
            }
        }
    }
}

#[component]
fn ProofRow(mode: &'static str, brand: Rgb, fill: Rgb, text: Rgb) -> Element {
    let ratio = contrast(fill, text);
    let moved = fill != brand;
    rsx! {
        TableRow {
            TableCell { "{mode}" }
            TableCell {
                span { class: "tb-chip", style: "background: {fill.hex()}", "aria-hidden": "true" }
                code { "{fill.hex()}" }
                if moved { span { class: "tb-help", " (lightness moved from {brand.hex()})" } } else { span { class: "tb-help", " (your colour, unchanged)" } }
            }
            TableCell {
                span { class: "tb-chip", style: "background: {text.hex()}", "aria-hidden": "true" }
                code { "{text.hex()}" }
            }
            TableCell {
                "{ratio:.1}:1 "
                if ratio >= AA {
                    Badge { variant: BadgeVariant::Secondary, "Passes AA" }
                } else {
                    Badge { variant: BadgeVariant::Destructive, "Fails AA" }
                }
            }
        }
    }
}
