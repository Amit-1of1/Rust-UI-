//! Tests for `contract.rs`, kept apart because `crates/ui/build.rs` and the native tests
//! include that file with `#[path]`.

use crate::contract::*;
use std::path::Path;

const SOUND: &str = r#"
pattern = "Dialog (Modal)"
apg = "dialog-modal"
roles = ["dialog"]

[parts]
trigger = "button"
dialog = "[role=dialog]"

[[states]]
name = "open"
means = "Shown."
check = "visible dialog"

[[keys]]
key = "Escape"
when = "Inside the dialog"
does = "Closes it."
native = true
test = """
focus trigger
press Enter   # open it first
state open
press Escape
hidden dialog
focused trigger
"""
"#;

fn with(test: &str) -> Contract {
    let text = SOUND.replace(
        SOUND
            .split("test = \"\"\"")
            .nth(1)
            .and_then(|t| t.split("\"\"\"").next())
            .unwrap(),
        test,
    );
    Contract::parse("dialog", &text).unwrap()
}

#[test]
fn a_sound_contract_has_no_problems_and_parses_its_steps() {
    let c = Contract::parse("dialog", SOUND).unwrap();
    assert_eq!(c.problems(), Vec::<String>::new());
    let steps = c.steps(&c.keys[0]).unwrap();
    assert_eq!(steps.len(), 6);
    assert_eq!(
        steps[2],
        Step::Visible {
            at: PartRef {
                part: "dialog".into(),
                selector: "[role=dialog]".into(),
                nth: 1
            }
        }
    );
    assert_eq!(
        steps[1],
        Step::Press {
            key: "Enter".into(),
            times: 1
        }
    );
}

#[test]
fn a_documented_key_must_be_pressed_and_then_checked() {
    let never = with("\nfocus trigger\npress Enter\nstate open\n");
    assert!(
        never.problems()[0].contains("never presses Escape"),
        "{:?}",
        never.problems()
    );
    let unchecked = with("\nfocus trigger\npress Enter\nstate open\npress Escape\n");
    assert!(
        unchecked.problems()[0].contains("nothing is checked after pressing Escape"),
        "{:?}",
        unchecked.problems()
    );
}

#[test]
fn roles_and_states_must_be_tested() {
    let c = with("\nfocus trigger\npress Escape\nfocused trigger\n");
    let p = c.problems().join("\n");
    assert!(
        p.contains("state `open` is listed but no test checks it"),
        "{p}"
    );
    assert!(
        p.contains("role `dialog` is listed but no test asserts"),
        "{p}"
    );
}

#[test]
fn bad_steps_are_reported() {
    let c = Contract::parse("dialog", SOUND).unwrap();
    for (line, err) in [
        ("focus nowhere", "unknown part `nowhere`"),
        ("press Enterr", "unknown key `Enterr`"),
        ("press Tab*0", "bad repeat count"),
        ("focused trigger@0", "starts at 1"),
        ("wiggle trigger", "unknown step `wiggle`"),
        ("attr dialog open", "write `attr <part> <name>=<value>`"),
        ("state shut", "no state `shut`"),
    ] {
        let e = c.step(line).unwrap_err();
        assert!(e.contains(err), "{line}: {e}");
    }
    assert_eq!(
        c.step("attr dialog data-state!=open").unwrap(),
        Step::Attr {
            at: c.part_ref("dialog").unwrap(),
            name: "data-state".into(),
            value: "open".into(),
            negate: true
        }
    );
    assert_eq!(
        c.step("press Shift+Tab*3").unwrap(),
        Step::Press {
            key: "Shift+Tab".into(),
            times: 3
        }
    );
    assert!(Contract::parse("dialog", &format!("{SOUND}\nextra = 1\n")).is_err());
}

#[test]
fn the_table_says_where_it_works() {
    let mut c = Contract::parse("dialog", SOUND).unwrap();
    let md = c.markdown();
    assert!(md.contains("[Dialog (Modal)](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/)"));
    assert!(
        md.contains("| Escape | Inside the dialog | Closes it. | yes |"),
        "{md}"
    );
    assert!(md.contains("Works on native (Blitz): yes."));
    c.keys[0].native = false;
    assert!(c.markdown().contains("Works on native (Blitz): not yet."));
}

#[test]
fn every_contract_in_the_repo_is_sound() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../ui/src/components");
    let all = Contract::load_all(&dir).unwrap();
    assert!(all.len() >= 10, "only {} contracts", all.len());
    for c in all {
        assert_eq!(c.problems(), Vec::<String>::new(), "{}", c.name);
        c.runnable().unwrap();
    }
}
