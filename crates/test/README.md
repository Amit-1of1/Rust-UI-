# atoll-test

Test Atoll (and any Dioxus 0.7) screens inside `cargo test`: render them headless on Blitz,
click, type, press keys, let timers run, and check what a user would see. No browser, window,
GPU or JavaScript.

```rust
use atoll_test::prelude::*;

#[test]
fn says_hello_by_name() {
    let mut screen = Screen::render(App);
    screen.type_into(label("Your name"), "Ada");
    screen.click(role("button").name("Say hello"));
    screen.assert_visible(text("Hello, Ada.").within(role("dialog")));
}
```

Set up an app with `atoll setup-tests`. The guide is
[docs/TESTING.md](../../docs/TESTING.md).

Experimental; licensed MIT OR Apache-2.0 like the rest of the kit.
