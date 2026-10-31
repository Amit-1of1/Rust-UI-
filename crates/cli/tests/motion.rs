//! The motion audit (`atoll_cli::motion`) on the kit: no animation or transition in any kit
//! stylesheet makes the browser lay the page out on every frame unless its rule says why in
//! an `atoll-motion: layout` comment. Overlays (dialogs, menus, popovers, toasts...) animate
//! only `transform` and `opacity`, which the compositor runs off the main thread.
//! `tests/e2e/frames.mjs` checks the same in the browser. `cargo run -p atoll-cli -- motion
//! --all` prints the table.

use atoll_cli::motion::{self, Cost, Kind};
use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn no_unexplained_layout_animation_in_the_kit() {
    let report = motion::run(&repo(), &[]).expect("audit runs");
    let errors: Vec<String> = report.errors().iter().map(|f| f.to_string()).collect();
    assert!(
        errors.is_empty(),
        "{} animated properties lay the page out every frame. Animate transform or opacity \
         instead, or explain the rule with an `atoll-motion: layout <props> (why)` comment:\n  {}",
        errors.len(),
        errors.join("\n  ")
    );
    for f in &report.findings {
        if let Some(why) = &f.allowed {
            assert!(
                !why.is_empty(),
                "{f}: the atoll-motion comment needs a reason"
            );
        }
    }
    println!(
        "{} stylesheets: {} composite, {} paint, {} layout (all explained)",
        report.files,
        report.count(Cost::Composite),
        report.count(Cost::Paint),
        report.count(Cost::Layout)
    );
}

#[test]
fn keyframe_animations_are_composite_only() {
    let report = motion::run(&repo(), &[]).expect("audit runs");
    let keyframes: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.kind == Kind::Keyframes)
        .collect();
    assert!(
        keyframes.len() > 100,
        "found only {} keyframe properties",
        keyframes.len()
    );
    // Every enter/exit animation of every overlay is transform and opacity. The only
    // exception is the accordion, which has to grow to its content's height.
    let other: Vec<String> = keyframes
        .iter()
        .filter(|f| f.cost != Cost::Composite && !f.file.contains("accordion"))
        .map(|f| f.to_string())
        .collect();
    assert!(
        other.is_empty(),
        "keyframes off the compositor:\n  {}",
        other.join("\n  ")
    );
}

#[test]
fn an_unexplained_layout_transition_fails() {
    let dir = std::env::temp_dir().join(format!("atoll-motion-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let css = dir.join("app.css");
    std::fs::write(&css, ".panel { transition: height 200ms ease; }").unwrap();
    let report = motion::run(&dir, &[css]).unwrap();
    assert_eq!(report.errors().len(), 1);
    assert_eq!(report.errors()[0].property, "height");
}
