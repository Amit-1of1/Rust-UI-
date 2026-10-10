use ui::components::form::*;

#[test]
fn first_failing_rule_wins_in_order() {
    let rules = [required(), email(), min_len(10)];
    assert_eq!(
        first_error(&rules, ""),
        Some("This field is required.".into())
    );
    assert_eq!(
        first_error(&rules, "nope"),
        Some("Enter a valid email address.".into())
    );
    assert_eq!(
        first_error(&rules, "a@b.co"),
        Some("Must be at least 10 characters.".into())
    );
    assert_eq!(first_error(&rules, "someone@example.com"), None);
}

#[test]
fn whitespace_only_is_empty_and_max_len_counts_chars() {
    assert!(first_error(&[required()], "   ").is_some());
    assert!(first_error(&[max_len(3)], "héé").is_none());
    assert!(first_error(&[max_len(3)], "héél").is_some());
}

#[test]
fn custom_rules_report_their_own_message() {
    let rules = [custom(|v| {
        if v.starts_with('#') {
            Ok(())
        } else {
            Err("Start with #".into())
        }
    })];
    assert_eq!(first_error(&rules, "tag"), Some("Start with #".into()));
    assert_eq!(first_error(&rules, "#tag"), None);
}
