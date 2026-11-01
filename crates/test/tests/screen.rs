//! atoll-test's own tests: each one is also an example of testing a screen.
//!
//! Run with `cargo test -p atoll-test`, not as part of `cargo test --workspace`: there the
//! gallery's `web` feature reaches the motion crate, which then calls JavaScript and panics.

use atoll_test::prelude::*;
use dioxus::prelude::*;
use ui::prelude::*;

fn counter() -> Element {
    let mut n = use_signal(|| 0);
    rsx! {
        Button { onclick: move |_| n += 1, "Add one" }
        p { "Count: {n}" }
    }
}

#[test]
fn clicks_reach_the_handler() {
    let mut s = Screen::render(counter);
    s.assert_text(text_containing("Count:"), "Count: 0");
    s.click(role("button").name("Add one"));
    s.click(role("button").name("Add one"));
    s.assert_visible(text("Count: 2"));
}

/// The screen `atoll create` writes (crates/cli/src/create.rs), trimmed to what the test needs.
fn starter() -> Element {
    let mut name = use_signal(String::new);
    let mut open = use_signal(|| false);
    rsx! {
        ThemeStyles { reset: true }
        main { style: "padding: 1.5rem;",
            Card { style: "max-width: 26rem;",
                CardContent {
                    Label { html_for: "name", "Your name" }
                    Input {
                        id: "name",
                        placeholder: "Ada Lovelace",
                        value: name,
                        oninput: move |e: FormEvent| name.set(e.value()),
                    }
                }
                CardFooter {
                    Button { onclick: move |_| open.set(true), "Say hello" }
                }
            }
        }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Hello",
            description: if name().trim().is_empty() { "Hello, stranger.".to_string() } else { format!("Hello, {}.", name().trim()) },
            Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Close" }
        }
    }
}

#[test]
fn starter_says_hello_by_name() {
    let mut s = Screen::render(starter);
    s.assert_not_visible(role("dialog"));

    s.type_into(label("Your name"), "Ada");
    s.assert_value(placeholder("Ada Lovelace"), "Ada");

    s.click(role("button").name("Say hello"));
    s.assert_visible(role("dialog"));
    s.assert_visible(text("Hello, Ada.").within(role("dialog")));

    s.click(role("button").name("Close"));
    s.assert_not_visible(role("dialog"));
}

#[test]
fn fill_replaces_what_the_field_holds() {
    let mut s = Screen::render(starter);
    s.type_into(label("Your name"), "Grace");
    s.fill(label("Your name"), "Ada");
    s.assert_value(label("Your name"), "Ada");
    s.fill(label("Your name"), "");
    s.assert_value(label("Your name"), "");
}

fn settings() -> Element {
    let mut email = use_signal(|| false);
    rsx! {
        div { style: "display: flex; gap: 0.5rem; align-items: center;",
            Switch {
                checked: email(),
                aria_label: "Email me",
                on_checked_change: move |v| email.set(v),
            }
            span { "Email me" }
        }
        div { style: "display: flex; gap: 0.5rem; align-items: center; margin-top: 1rem;",
            Checkbox { aria_label: "Accept terms" }
        }
        if email() {
            p { "data-testid": "note", "We will email you." }
        }
    }
}

#[test]
fn switches_and_checkboxes_toggle() {
    let mut s = Screen::render(settings);
    s.assert_checked(role("switch").name("Email me"), false);
    assert!(!s.exists(test_id("note")));

    s.click(role("switch").name("Email me"));
    s.assert_checked(role("switch").name("Email me"), true);
    s.assert_text(test_id("note"), "We will email you.");

    s.click(role("checkbox").name("Accept terms"));
    s.assert_checked(role("checkbox").name("Accept terms"), true);
}

fn three_buttons() -> Element {
    rsx! {
        Button { "One" }
        Button { "Two" }
        Button { "Three" }
    }
}

#[test]
fn tab_moves_focus_in_order() {
    let mut s = Screen::render(three_buttons);
    s.focus(role("button").name("One"));
    s.assert_focused(role("button").name("One"));
    s.tab();
    s.assert_focused(role("button").name("Two"));
    s.tab();
    s.assert_focused(role("button").nth(2));
    s.shift_tab();
    s.assert_focused(role("button").name("Two"));
}

#[test]
#[should_panic(expected = "3 elements match role(\"button\")")]
fn ambiguous_queries_fail_and_list_the_matches() {
    let s = Screen::render(three_buttons);
    s.get(role("button"));
}

#[test]
#[should_panic(expected = "button \"Three\"")]
fn missing_queries_list_what_is_on_screen() {
    let s = Screen::render(three_buttons);
    s.get(role("button").name("Four"));
}

fn hidden_things() -> Element {
    rsx! {
        div { style: "display: none;", button { "Ghost" } }
        div { "aria-hidden": "true", button { "Decor" } }
        button { "Real" }
    }
}

#[test]
fn hidden_elements_are_skipped_unless_asked_for() {
    let s = Screen::render(hidden_things);
    assert_eq!(s.all(role("button")).len(), 1);
    assert_eq!(s.all(role("button").hidden()).len(), 3);
    assert!(!s.is_visible(role("button").name("Ghost").hidden()));
    s.assert_visible(role("button").name("Real"));
}

fn with_toasts() -> Element {
    rsx! {
        ToastProvider { SaveButton {} }
    }
}

#[component]
fn SaveButton() -> Element {
    let toasts = use_toast();
    rsx! {
        Button { onclick: move |_| toasts.notify_success(("Saved", "Your changes are stored.")), "Save" }
    }
}

#[test]
fn toasts_appear_and_time_out_on_the_test_clock() {
    let mut s = Screen::render(with_toasts);
    s.click(role("button").name("Save"));
    s.assert_visible(text("Saved"));
    // The default toast lasts five seconds; the test doesn't wait for them.
    s.advance(Duration::from_secs(6));
    s.assert_not_visible(text("Saved"));
}

fn slow_load() -> Element {
    let mut loaded = use_signal(|| false);
    use_future(move || async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        loaded.set(true);
    });
    rsx! {
        if loaded() { p { "Ready" } } else { Spinner {} }
    }
}

#[test]
fn wait_for_moves_the_clock_until_something_shows() {
    let mut s = Screen::render(slow_load);
    assert!(!s.exists(text("Ready")));
    s.wait_for(text("Ready"), Duration::from_secs(5));
}

#[test]
fn closures_pass_data_in() {
    let who = "Lin".to_string();
    let s = Screen::render_with(move || rsx! { h1 { "Hi {who}" } });
    s.assert_text(role("heading"), "Hi Lin");
}

#[test]
fn phone_size_and_dark_mode() {
    let s = Screen::builder(counter).size(390, 844).dark().render();
    let r = s.rect(role("button"));
    assert!(r.w > 0.0 && r.x + r.w <= 390.0, "{r:?}");
    // The page background is the dark theme's, not white.
    let [r, g, b, _] = s.pixel(380, 830);
    assert!(
        u32::from(r) + u32::from(g) + u32::from(b) < 200,
        "{r},{g},{b}"
    );
}

#[test]
fn kit_css_is_found_and_hashed() {
    let css = atoll_test::kit_css(false);
    assert!(css.contains("--primary-color"), "theme.css missing");
    // Component classes are renamed like #[css_module] does (`.dx-button` gets a hash).
    assert!(
        !css.contains(".dx-button {"),
        "component classes not hashed"
    );
    assert!(css.contains(".dx-button"), "component modules missing");
}

fn drawer() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Open drawer" }
        Drawer {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Move goal",
            description: "Set your daily goal.",
            DrawerClose { "Close" }
        }
    }
}

/// The drawer slides with `ui::motion` springs, which run on the test clock (and, since the
/// motion crate only uses its browser timer on wasm, with the app's `web` feature on too).
#[test]
fn animated_drawer_opens_and_closes() {
    let mut s = Screen::render(drawer);
    s.click(role("button").name("Open drawer"));
    s.assert_visible(role("dialog").name("Move goal"));
    s.click(role("button").name("Close"));
    s.advance(Duration::from_secs(1));
    s.assert_not_visible(role("dialog"));
}
