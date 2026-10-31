//! The contrast proof (`atoll_cli::contrast`) on the kit: every text colour each component
//! stylesheet draws, on the background it sits on, in light and dark, under every accent
//! preset, meets WCAG 2 AA. Focus rings and icons need 3:1. Run `cargo run -p atoll-cli --
//! contrast` for the same report, or add `--theme your-theme.css` to prove your own theme.

use atoll_cli::contrast::{self, check, Kind, Mode, Nesting, Theme};
use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn kit_theme() -> String {
    std::fs::read_to_string(repo().join("crates/ui/assets/theme.css")).expect("theme.css")
}

#[test]
fn every_pair_meets_wcag_aa_in_both_modes_and_every_accent() {
    let report = contrast::run(&repo(), &[]).expect("proof runs");
    let failures = contrast::summary(&report);
    assert!(
        failures.is_empty(),
        "{} rules draw unreadable colours (fix the CSS, or the token):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
    // Values the proof could not evaluate would be holes in it.
    let skipped: Vec<String> = report
        .skipped
        .iter()
        .map(|s| format!("{} `{}`: {}", s.component, s.selector, s.value))
        .collect();
    assert!(
        skipped.is_empty(),
        "unevaluated colours:\n  {}",
        skipped.join("\n  ")
    );
}

#[test]
fn the_proof_covers_every_component_mode_and_accent() {
    let report = contrast::run(&repo(), &[]).expect("proof runs");
    let theme = Theme::parse(&kit_theme());
    let accents = theme.accent_names();
    assert!(
        accents.len() >= 6,
        "neutral and the five presets: {accents:?}"
    );
    for mode in Mode::ALL {
        for accent in &accents {
            let n = report
                .pairs
                .iter()
                .filter(|p| p.mode == mode && &p.accent == accent)
                .count();
            assert!(n > 300, "{} {accent}: only {n} pairs", mode.name());
        }
    }
    // Components with text or focus styles of their own all take part.
    let mut seen: Vec<&str> = report.pairs.iter().map(|p| p.component.as_str()).collect();
    seen.sort();
    seen.dedup();
    for name in [
        "button",
        "badge",
        "toast",
        "calendar",
        "dropdown_menu",
        "sidebar",
        "tabs",
    ] {
        assert!(seen.contains(&name), "{name} was not checked");
    }
    assert!(report.pairs.iter().any(|p| p.kind == Kind::Focus));
    assert!(report.pairs.iter().any(|p| p.kind == Kind::Graphic));
}

/// The proof has to fail on the bugs it was written for: put back values the kit used to
/// ship and check each one is caught.
#[test]
fn the_proof_catches_regressions() {
    let sheets = contrast::component_sheets(&repo().join("crates/ui/src/components")).unwrap();
    let nesting = contrast::snapshot_nesting(&repo().join("crates/ui/tests/snapshots")).unwrap();
    let cases: &[(&str, &str, &str)] = &[
        // Upstream's muted text, 3.7:1 on white (axe found it in week 1).
        (
            "--secondary-color-5: light-dark(#6b6b6b, #a1a1a1);",
            "--secondary-color-5: light-dark(#848484, #a1a1a1);",
            "var(--secondary-color-5)",
        ),
        // Success text that used to be the fill colour.
        (
            "--success-text-color: light-dark(#047857, #b6fae3);",
            "--success-text-color: light-dark(#10b981, #b6fae3);",
            "var(--success-text-color)",
        ),
        // A focus colour too close to the page.
        (
            "--focused-border-color: #2b7fff;",
            "--focused-border-color: #c6dcff;",
            "var(--focused-border-color)",
        ),
    ];
    for (good, bad, expect) in cases {
        let css = kit_theme();
        assert!(css.contains(good), "theme.css no longer has `{good}`");
        let theme = Theme::parse(&css.replace(good, bad));
        let report = check(&theme, &sheets, &nesting);
        assert!(
            report.failures().any(|f| f.fg_source.contains(expect)),
            "`{bad}` was not caught"
        );
    }
}

/// `atoll contrast --theme brand.css` checks a preset of your own like the shipped ones.
#[test]
fn a_users_preset_is_proven_too() {
    let dir = tempfile::tempdir().unwrap();
    let good = dir.path().join("good.css");
    let preset =
        atoll_cli::theme::preset(atoll_cli::theme::Rgb::parse("#0ea5e9").unwrap()).unwrap();
    std::fs::write(&good, atoll_cli::theme::css("sky", &preset)).unwrap();
    let report = contrast::run(&repo(), &[good]).unwrap();
    assert!(report.pairs.iter().any(|p| p.accent == "sky"));
    assert_eq!(
        report.failures().count(),
        0,
        "{:?}",
        contrast::summary(&report)
    );

    let bad = dir.path().join("bad.css");
    std::fs::write(
        &bad,
        "[data-accent=\"pale\"] {\n  --accent-color: light-dark(#7dd3fc, #0c4a6e);\n  --accent-contrast: light-dark(#fff, #fff);\n}\n",
    )
    .unwrap();
    let report = contrast::run(&repo(), &[bad]).unwrap();
    assert!(report.failures().any(|f| f.accent == "pale"));
}

#[test]
fn the_command_reports_and_fails() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_atoll"))
        .args(["--source"])
        .arg(repo())
        .arg("contrast")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}");
    assert!(
        stdout.contains("pairs checked, 0 rules failing"),
        "{stdout}"
    );
}

/// Markup nesting: what a part sits on comes from where the snapshots render it.
#[test]
fn nesting_comes_from_rendered_markup() {
    let theme = Theme::parse(
        ":root { --primary-color: light-dark(#fff, #000); --secondary-color-4: light-dark(#111, #ddd); --ink: light-dark(#777, #777); --panel: light-dark(#eee, #222); }",
    );
    let sheets = vec![(
        "demo".to_string(),
        ".dx-panel { background: var(--panel); } .dx-hint { color: var(--ink); }".to_string(),
    )];
    let mut nesting = Nesting::default();
    nesting.add_html(r#"<div class="dx-panel-css"><p class="dx-hint-css">hi</p></div>"#);
    let report = check(&theme, &sheets, &nesting);
    let hint: Vec<_> = report
        .pairs
        .iter()
        .filter(|p| p.selector == ".dx-hint" && p.mode == Mode::Light)
        .collect();
    // On the panel (#eee), not on the page: one background only.
    assert_eq!(hint.len(), 1);
    assert_eq!(hint[0].bg.hex(), "#eeeeee");
    assert!(!hint[0].passes(), "#777 on #eee is 3.9:1");
}
