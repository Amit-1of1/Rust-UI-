//! `#[derive(Form)]`: the generated fields, rules, messages and typed value.

use dioxus::prelude::*;
use ui::prelude::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn plans() -> Vec<FieldOption> {
    vec![
        FieldOption::new("free", "Free"),
        FieldOption::new("pro", "Pro"),
    ]
}

fn not_admin(v: &str) -> Result<(), String> {
    if v.eq_ignore_ascii_case("admin") {
        Err("That name is taken.".into())
    } else {
        Ok(())
    }
}

#[derive(Form, Clone, PartialEq, Debug)]
struct SignUp {
    #[form(required, min_len = 3, validate = not_admin)]
    user_name: String,
    #[form(required, email, autocomplete = "email")]
    email: String,
    #[form(min = 18, max = 120)]
    age: u32,
    #[form(max = 5.0)]
    rating: Option<f64>,
    #[form(select = plans, placeholder = "Choose a plan")]
    plan: Option<String>,
    #[form(textarea, max_len = 5)]
    bio: Option<String>,
    #[form(required)]
    terms: bool,
}

fn sample() -> SignUp {
    SignUp {
        user_name: "ada".into(),
        email: "ada@example.com".into(),
        age: 36,
        rating: Some(4.5),
        plan: Some("pro".into()),
        bio: None,
        terms: true,
    }
}

/// Renders every field's error as `name=message;`.
fn errors(form: &SignUpForm) -> String {
    [
        ("user_name", form.user_name),
        ("email", form.email),
        ("age", form.age),
        ("rating", form.rating),
        ("plan", form.plan),
        ("bio", form.bio),
        ("terms", form.terms),
    ]
    .iter()
    .filter_map(|(n, f)| f.error().map(|e| format!("{n}={e};")))
    .collect()
}

#[test]
fn empty_form_reports_named_messages_for_required_fields_only() {
    fn app() -> Element {
        let form = SignUp::use_form();
        let valid = use_hook(|| form.validate());
        rsx! { p { "{valid} {errors(&form)}" } }
    }
    assert_eq!(
        render(app),
        "<p>false user_name=User name is required.;email=Email is required.;\
         age=Age is required.;terms=Tick “Terms” to continue.;</p>"
    );
}

#[test]
fn rules_run_in_order_and_optional_fields_check_only_what_was_typed() {
    fn app() -> Element {
        let form = SignUp::use_form();
        use_hook(|| {
            form.user_name.set("admin");
            form.email.set("nope");
            form.age.set("17");
            form.rating.set("lots");
            form.bio.set("too long");
            form.terms.set("true");
            form.validate();
        });
        rsx! { p { "{errors(&form)}" } }
    }
    assert_eq!(
        render(app),
        "<p>user_name=That name is taken.;email=Enter a valid email address.;\
         age=Must be at least 18.;rating=Enter a number.;bio=Must be at most 5 characters.;</p>"
    );
}

#[test]
fn value_is_typed_and_load_round_trips() {
    fn app() -> Element {
        let form = SignUp::use_form();
        let before = use_hook(|| form.value());
        let after = use_hook(|| {
            form.load(&sample());
            (form.validate(), form.value())
        });
        assert_eq!(before, None, "an empty number does not parse");
        assert_eq!(after, (true, Some(sample())));
        rsx! {}
    }
    render(app);
}

#[test]
fn use_form_with_starts_from_a_value_and_reset_returns_to_it() {
    fn app() -> Element {
        let form = SignUp::use_form_with(&sample());
        let value = use_hook(|| {
            form.age.set("50");
            form.reset();
            form.value()
        });
        assert_eq!(value, Some(sample()));
        rsx! {}
    }
    render(app);
}

#[test]
fn fields_are_labelled_and_marked_up_for_their_type() {
    fn app() -> Element {
        let form = SignUp::use_form();
        rsx! {
            Form { state: form, {form.fields()} }
        }
    }
    let html = render(app);
    for label in [
        "User name",
        "Email",
        "Age",
        "Rating",
        "Plan",
        "Bio",
        "Terms",
    ] {
        assert!(
            html.contains(&format!(">{label}</label>")),
            "{label}: {html}"
        );
    }
    let inputs: Vec<&str> = html.split("<input").skip(1).collect();
    // Just the tag's attributes.
    let input = |needle: &str| {
        inputs
            .iter()
            .map(|i| i.split('>').next().unwrap_or(i))
            .find(|i| i.contains(needle))
            .unwrap_or_else(|| panic!("no input with {needle}: {html}"))
    };
    let email = input("type=\"email\"");
    assert!(email.contains("autocomplete=\"email\""), "{email}");
    assert!(email.contains("aria-required=\"true\""), "{email}");
    let age = input("inputmode=\"numeric\"");
    assert!(age.contains("aria-required=\"true\""), "{age}");
    let rating = input("inputmode=\"decimal\"");
    assert!(!rating.contains("aria-required"), "optional: {rating}");
    assert!(html.contains("<textarea"), "{html}");
    assert!(html.contains("Choose a plan"), "{html}");
    let checkbox = html
        .split("<button")
        .find(|b| b.contains("role=\"checkbox\""))
        .and_then(|b| b.split('>').next())
        .expect("checkbox");
    assert!(checkbox.contains("aria-required=true"), "{checkbox}");
}

#[test]
fn messages_are_linked_to_their_control() {
    fn app() -> Element {
        let form = SignUp::use_form();
        use_hook(|| form.validate());
        rsx! { {form.fields()} }
    }
    let html = render(app);
    let email = html
        .split("<input")
        .find(|i| i.contains("type=\"email\""))
        .expect("email input");
    assert!(email.contains("aria-invalid=\"true\""), "{email}");
    let described = email
        .split("aria-describedby=\"")
        .nth(1)
        .and_then(|r| r.split('"').next())
        .expect("aria-describedby");
    assert!(
        html.contains(&format!("id=\"{described}\"")),
        "{described} must name the error: {html}"
    );
    assert!(html.contains("Email is required."), "{html}");
}

#[derive(Form)]
struct Braces {
    #[form(label = "Code {x}", description = "Use {braces}")]
    code: String,
}

#[test]
fn braces_in_labels_are_text_not_formatting() {
    fn app() -> Element {
        let form = Braces::use_form();
        rsx! { {form.fields()} }
    }
    let html = render(app);
    assert!(html.contains("Code {x}"), "{html}");
    assert!(html.contains("Use {braces}"), "{html}");
}
