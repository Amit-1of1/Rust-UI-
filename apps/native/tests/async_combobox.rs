//! `AsyncCombobox` on headless Blitz: debounced search, the loading line, "Load more results"
//! by keyboard, "Try again" after an error, out-of-order responses, the picked label after the
//! results change, and the screen-reader announcements (which SSR tests can't see, because
//! they need the spawned requests to finish).
//!
//! Time is real: each test runs a Tokio runtime and waits for the debounce and the fake
//! server's delay with `wait`.
//!
//! Run with `cargo test -p native --test async_combobox`.

mod common;
use common::*;

use dioxus::prelude::*;
use std::cell::RefCell;
use std::time::Duration;
use ui::prelude::*;

const FRUIT: &[&str] = &[
    "Apple",
    "Apricot",
    "Avocado",
    "Banana",
    "Blackberry",
    "Blueberry",
    "Cherry",
    "Coconut",
    "Damson",
    "Date",
];

thread_local! {
    /// Every `(query, page)` the fake server was asked for.
    static CALLS: RefCell<Vec<(String, usize)>> = const { RefCell::new(Vec::new()) };
    /// How many of the next requests fail.
    static FAILURES: RefCell<usize> = const { RefCell::new(0) };
}

fn calls() -> Vec<(String, usize)> {
    CALLS.with(|c| c.borrow().clone())
}

/// Pages of three fruits containing the query. A query starting with "slow" takes 300 ms,
/// everything else 20 ms.
async fn fake_server(query: String, page: usize) -> Result<AsyncPage<String>, String> {
    let delay = if query.starts_with("slow") { 300 } else { 20 };
    tokio::time::sleep(Duration::from_millis(delay)).await;
    let fail = FAILURES.with(|f| {
        let mut f = f.borrow_mut();
        let fail = *f > 0;
        *f = f.saturating_sub(1);
        fail
    });
    if fail {
        return Err("offline".into());
    }
    let q = query.trim_start_matches("slow").to_lowercase();
    let found: Vec<String> = FRUIT
        .iter()
        .filter(|f| f.to_lowercase().contains(&q))
        .map(|f| f.to_string())
        .collect();
    Ok(AsyncPage {
        items: found.iter().skip(page * 3).take(3).cloned().collect(),
        has_more: found.len() > (page + 1) * 3,
    })
}

fn app() -> Element {
    let fruit = use_async_options(|query: String, page: usize| {
        CALLS.with(|c| c.borrow_mut().push((query.clone(), page)));
        fake_server(query, page)
    });
    rsx! {
        ui::ThemeStyles {}
        main { style: "padding: 24px; height: 700px;",
            AsyncCombobox::<String> {
                options: fruit,
                label: |f: String| f,
                aria_label: "Fruit",
                list_aria_label: "Fruit",
                id: "fruit",
            }
            button { id: "elsewhere", "Elsewhere" }
        }
    }
}

struct Test {
    h: Harness,
    rt: tokio::runtime::Runtime,
}

impl Test {
    fn new() -> Self {
        CALLS.with(|c| c.borrow_mut().clear());
        FAILURES.with(|f| *f.borrow_mut() = 0);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("runtime");
        let h = {
            let _guard = rt.enter();
            Harness::new(app)
        };
        Self { h, rt }
    }

    /// Let `ms` of real time pass with the runtime driving timers, then settle the document.
    fn wait(&mut self, ms: u64) {
        let _guard = self.rt.enter();
        let step = 10;
        let mut left = ms;
        while left > 0 {
            let d = left.min(step);
            self.rt
                .block_on(tokio::time::sleep(Duration::from_millis(d)));
            self.h.settle();
            left -= d;
        }
    }

    fn act(&mut self, f: impl FnOnce(&mut Harness)) {
        let _guard = self.rt.enter();
        f(&mut self.h);
    }

    fn attr(&self, selector: &str, name: &str) -> Option<String> {
        let id = self.h.find(selector)?;
        self.h
            .doc
            .get_node(id)?
            .attr(blitz_dom::LocalName::from(name))
            .map(str::to_owned)
    }

    fn texts(&self, selector: &str) -> Vec<String> {
        self.h
            .doc
            .query_selector_all(selector)
            .expect("selector")
            .into_iter()
            .map(|id| {
                self.h
                    .doc
                    .get_node(id)
                    .expect("node")
                    .text_content()
                    .trim()
                    .to_string()
            })
            .collect()
    }

    fn options(&self) -> Vec<String> {
        self.texts("[role=listbox] [role=option]")
    }

    fn announced(&self) -> String {
        self.texts("[data-slot=combobox-announcer]").join("|")
    }

    fn press(&mut self, key: Key, code: Code) {
        self.act(|h| h.key(key, code, Modifiers::empty()));
    }
}

#[test]
fn opening_loads_and_announces_the_first_page() {
    let mut t = Test::new();
    assert_eq!(t.announced(), "", "announced before anything happened");
    t.act(|h| h.click("[role=combobox]"));
    assert_eq!(
        t.attr("[role=combobox]", "aria-expanded").as_deref(),
        Some("true")
    );
    // The request is out: the list says so and is busy.
    assert!(
        t.texts("[role=listbox]")
            .join("")
            .contains("Loading results…"),
        "no loading line"
    );
    assert_eq!(
        t.attr("[role=listbox]", "aria-busy").as_deref(),
        Some("true")
    );
    assert_eq!(t.announced(), "Loading results…");

    t.wait(60);
    assert_eq!(
        t.options(),
        ["Apple", "Apricot", "Avocado", "Load more results"]
    );
    assert_eq!(t.attr("[role=listbox]", "aria-busy"), None);
    assert_eq!(t.announced(), "3 results");
    assert_eq!(calls(), [(String::new(), 0)]);
}

#[test]
fn typing_is_debounced_to_one_request() {
    let mut t = Test::new();
    t.act(|h| h.click("[role=combobox]"));
    t.wait(60);
    t.act(|h| h.type_text("berr"));
    // Still inside the debounce: nothing new was asked for yet, the old results stay.
    assert_eq!(calls().len(), 1, "{:?}", calls());
    t.wait(400);
    assert_eq!(
        calls(),
        [(String::new(), 0), ("berr".to_string(), 0)],
        "one request per pause in typing"
    );
    assert_eq!(t.options(), ["Blackberry", "Blueberry"]);
    assert_eq!(t.announced(), "2 results");
}

#[test]
fn nothing_found_shows_the_empty_text() {
    let mut t = Test::new();
    t.act(|h| {
        h.click("[role=combobox]");
        h.type_text("zzz");
    });
    t.wait(400);
    assert!(t.options().is_empty(), "{:?}", t.options());
    assert!(
        t.texts("[role=listbox]")
            .join("")
            .contains("No results found."),
        "no empty text"
    );
    assert_eq!(t.announced(), "No results found.");
}

#[test]
fn load_more_is_an_option_and_keeps_the_list_open() {
    let mut t = Test::new();
    t.act(|h| h.click("[role=combobox]"));
    t.wait(60);
    // Down to "Load more results" (the fourth option) and pick it with Enter.
    for _ in 0..4 {
        t.press(Key::ArrowDown, Code::ArrowDown);
    }
    let more_id = t.attr("[role=combobox]", "aria-activedescendant");
    assert_eq!(
        more_id,
        t.attr("[role=listbox] [role=option]:last-child", "id"),
        "Load more is not highlighted"
    );
    t.press(Key::Enter, Code::Enter);
    assert_eq!(
        t.attr("[role=combobox]", "aria-expanded").as_deref(),
        Some("true"),
        "picking Load more closed the list"
    );
    assert!(t.options().contains(&"Loading more results…".to_string()));
    assert_eq!(t.announced(), "Loading more results…");

    t.wait(60);
    assert_eq!(
        t.options(),
        [
            "Apple",
            "Apricot",
            "Avocado",
            "Banana",
            "Blackberry",
            "Blueberry",
            "Load more results"
        ]
    );
    assert_eq!(t.announced(), "6 results");
    assert_eq!(calls(), [(String::new(), 0), (String::new(), 1)]);
    // The highlight sits on the first new result, so Enter there picks Banana.
    t.press(Key::Enter, Code::Enter);
    assert_eq!(
        t.attr("[role=combobox]", "aria-expanded").as_deref(),
        Some("false")
    );
    assert_eq!(
        t.attr("[role=combobox]", "value").as_deref(),
        Some("Banana")
    );
}

#[test]
fn a_failed_request_offers_try_again() {
    let mut t = Test::new();
    FAILURES.with(|f| *f.borrow_mut() = 1);
    t.act(|h| h.click("[role=combobox]"));
    t.wait(60);
    // The error and the way out are one option, so both are read when it is highlighted.
    assert_eq!(t.options(), ["Couldn't load results. Try again"]);
    assert_eq!(t.announced(), "Couldn't load results.");
    assert!(
        !t.texts("[role=listbox]")
            .join("")
            .contains("No results found."),
        "the empty text showed for a failure"
    );

    t.press(Key::ArrowDown, Code::ArrowDown);
    t.press(Key::Enter, Code::Enter);
    assert_eq!(
        t.attr("[role=combobox]", "aria-expanded").as_deref(),
        Some("true")
    );
    t.wait(60);
    assert_eq!(
        t.options(),
        ["Apple", "Apricot", "Avocado", "Load more results"]
    );
    assert_eq!(calls().len(), 2);
}

#[test]
fn a_slow_old_response_never_replaces_a_newer_one() {
    let mut t = Test::new();
    t.act(|h| h.click("[role=combobox]"));
    t.wait(60);
    // "slowap" goes out after the debounce and takes 300 ms ...
    t.act(|h| h.type_text("slowap"));
    t.wait(300);
    assert_eq!(calls().last().map(|c| c.0.as_str()), Some("slowap"));
    // ... meanwhile the user clears it and types "da", which answers fast.
    t.act(|h| {
        for _ in 0..6 {
            h.key(Key::Backspace, Code::Backspace, Modifiers::empty());
        }
        h.type_text("da");
    });
    t.wait(700);
    assert_eq!(t.options(), ["Damson", "Date"]);
}

#[test]
fn the_picked_label_stays_when_the_results_change() {
    let mut t = Test::new();
    t.act(|h| {
        h.click("[role=combobox]");
        h.type_text("cher");
    });
    t.wait(400);
    t.press(Key::ArrowDown, Code::ArrowDown);
    t.press(Key::Enter, Code::Enter);
    assert_eq!(
        t.attr("[role=combobox]", "value").as_deref(),
        Some("Cherry")
    );

    // Reopening searches for "" again, whose first page has no Cherry; closing must still
    // show the pick.
    t.act(|h| h.click("[role=combobox]"));
    t.wait(400);
    assert!(!t.options().contains(&"Cherry".to_string()));
    t.press(Key::Escape, Code::Escape);
    t.wait(200);
    assert_eq!(
        t.attr("[role=combobox]", "aria-expanded").as_deref(),
        Some("false")
    );
    assert_eq!(
        t.attr("[role=combobox]", "value").as_deref(),
        Some("Cherry"),
        "the input lost the picked label"
    );
}
