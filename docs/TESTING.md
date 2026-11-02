# Testing your screens

`atoll-test` renders a screen inside `cargo test`, lets the test click, type and press keys
the way a user would, and checks what ends up on the screen. There is no browser, window,
GPU or JavaScript, so the tests run anywhere `cargo test` runs (CI included) and take well
under a second each.

The screen is drawn by Blitz, the engine behind `dioxus-native`, with the kit's real CSS. A
click lands on whatever is drawn at that point, so a modal dialog really does block the page
behind it, and something hidden with `display: none` really can't be clicked.

Experimental (new on 2026-11-01): method names may still change ([STABILITY.md](STABILITY.md)).

## Set up (once per app)

In the app's folder:

```sh
atoll setup-tests
```

It edits `Cargo.toml` twice and changes nothing else:

- `atoll-test` under `[dev-dependencies]`, from the same place as your `ui` line (the same
  git URL and `rev`, or the `crates/test` folder beside a local kit). It reads the kit's CSS
  from its own checkout, so the two must always match; if you move the kit's `rev`, move
  this one with it, or run the command again after deleting the line.
- The kit's `[patch.crates-io]` section: small fixes to the Blitz renderer that the
  screen needs ([FORKS.md](FORKS.md)). Web builds don't use these crates, so this changes
  nothing in your app.

Running it twice changes nothing. To do it by hand, copy the `[patch.crates-io]` section of
the kit's root `Cargo.toml` and add
`atoll-test = { git = "https://github.com/Amit-1of1/Rust-UI-", rev = "<the kit's rev>" }`.

atoll-test is not on crates.io yet (the kit isn't either; [PUBLISHING.md](PUBLISHING.md)).

## A first test

An app made by `atoll create` keeps its screen in `src/main.rs`, which is a binary, so put the
tests at the bottom of that file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use atoll_test::prelude::*;

    #[test]
    fn says_hello_by_name() {
        let mut screen = Screen::render(App);
        screen.type_into(label("Your name"), "Ada");
        screen.click(role("button").name("Say hello"));
        screen.assert_visible(text("Hello, Ada.").within(role("dialog")));
    }
}
```

Then `cargo test`. If the app has a `src/lib.rs`, tests can also live in `tests/*.rs` and
import the components from the library.

`Screen::render` takes a component function with no props. For one with props, or to pass
data in, use a closure:

```rust
let screen = Screen::render_with(move || rsx! { InvoiceRow { amount: 42.0 } });
```

## Finding things

Find elements the way a user does, by what they see and what a screen reader announces.
That also catches accessibility bugs: a button with no name can't be found by name.

| Query | Finds |
|---|---|
| `role("button").name("Save")` | An element with that role (explicit `role="..."`, or implied: `button`, `link`, `textbox`, `checkbox`, `heading`, `dialog`, `switch`, ...) and that accessible name. `.name_containing("Sav")` matches part of it. |
| `label("Email")` | The form control that label names (`<label for>`, a wrapping `<label>`, `aria-labelledby` or `aria-label`). |
| `text("Saved")` | The innermost element whose text is exactly that. `text_containing("Sav")` for part of it. |
| `placeholder("Search")` | A field with that placeholder. |
| `test_id("total")` | `data-testid="total"`, when nothing a user sees fits. |
| `css(".row > td")`, or just `".row > td"` | A CSS selector, as a last resort. |

Narrow any query with `.within(role("dialog"))` (only inside that element), `.nth(2)` or
`.first()`. Role, label, text and placeholder queries skip what a user can't perceive
(`display: none`, `visibility: hidden`, `hidden`, inside `aria-hidden="true"`);
`.hidden()` includes it. CSS and test-id queries see everything.

Every query used to act or assert must find **exactly one** element. When it finds none, the
failure lists the roles and names on the screen; when it finds several, it lists them and
suggests `.name(..)`, `.within(..)` or `.nth(..)`. `screen.get(q)` returns the element,
`screen.query(q)` returns `None` instead of failing when there is none, and `screen.all(q)`
returns every match.

The accessible name is a simplified version of the browser's computation: `aria-labelledby`,
`aria-label`, the label of a form control, `alt`, the text inside (buttons, links, headings,
options, tabs, menu items, checkboxes, cells), then `title` and `placeholder`.

## Acting

| Method | Does |
|---|---|
| `click(q)`, `right_click(q)`, `hover(q)` | Moves the mouse to the middle of the element and presses there. Fails if it isn't visible; whatever is drawn on top gets the click. |
| `type_into(q, "text")` | Focuses the field and types at the end of what it holds, one key per character. |
| `fill(q, "text")` | Focuses the field, selects everything and types over it (`""` clears it). |
| `press("Enter")` | One key on the focused element: `Escape`, `Tab`, `Shift+Tab`, `ArrowDown`, `Space`, `Home`, `Control+a`, a single character. A typo fails the test. |
| `tab()`, `shift_tab()`, `focus(q)` | Moves keyboard focus. |
| `drag(from, &[points])` | Drags with the main button held (sliders, drag-and-drop, swipe to dismiss). |
| `scroll_into_view(q)` | Scrolls the page until the element is in the window. |
| `advance(Duration)` | Lets time pass on the app's clock: timers, toast timeouts, debounces and animations fire in order, without the test waiting. |
| `wait_for(q, timeout)` | Moves the clock in small steps until the element shows, for async work. |

Every action runs the app's effects and re-renders before it returns, so the next line sees
the result.

## Checking

| Method | Checks |
|---|---|
| `assert_visible(q)`, `assert_not_visible(q)` | Present, not hidden, with a size, inside the window (missing counts as not visible). |
| `assert_text(q, "...")` | The element's text, whitespace collapsed. |
| `assert_value(q, "...")` | What a field holds (what was typed). |
| `assert_checked(q, true)` | `aria-checked`, `aria-pressed`, `aria-selected`, `data-state` or a native checkbox. |
| `assert_focused(q)` | The element has keyboard focus (the failure says where focus is). |

The readers behind them are there for your own `assert!`s: `text`, `value`, `attr`, `name`,
`role`, `is_visible`, `is_checked`, `is_focused`, `focused`, `rect`, `exists`, `html`.
Failures point at the test's line, not at atoll-test.

## Window, theme and styles

```rust
let screen = Screen::builder(App).size(390, 844).dark().render();
```

The default window is 1024 x 768 in light mode. `.dark()` is the system dark mode, which the
kit follows unless the app picks a mode.

The kit's `theme.css`, `reset.css` and every component's stylesheet are loaded for you, and
so are the components copied into `src/components/` with `atoll add` (their classes are
hashed the way `#[css_module]` hashes them). Your own CSS isn't, because `dx` normally
serves it: add it with `.stylesheet("assets/main.css")` (plain CSS) or
`.css_module("src/pages/home.css")` (a CSS module). Paths are relative to the package.
`.without_reset()`, `.without_kit_css()` and `.without_app_components()` leave things out.

`screenshot("target/shots/home.png")` saves a picture of the screen (painted on the CPU) for
a human to look at, and `pixel(x, y)` reads one pixel.

## What it doesn't do

- **JavaScript.** Anything that goes through `document::eval` does nothing, as in a native
  build ([NATIVE.md](NATIVE.md) lists what in the kit still uses it). Web-only listeners the
  kit adds through `web-sys` (Ctrl+K for the command palette, clipboard) don't run either.
- **Browser engines.** This is Blitz, not Chromium, Firefox or WebKit. Layout is very close
  but not identical, so check pixel-exact layout and browser-specific behaviour in a browser
  (the kit's own `tests/e2e` does that with Playwright).
- **Network.** `<img src>` and other fetched resources don't load.
- **Real time.** The clock only moves when the test calls `advance` or `wait_for` (and one
  second per action for CSS animations, so they have finished when the test looks).

Anything else the API misses: `screen.document()` is the Blitz document itself.

## How the kit uses it

`crates/test/tests/screen.rs` is atoll-test's own test file and doubles as a set of
examples: a counter, the `atoll create` starter, switches, keyboard focus order, toasts timing
out, an async load and an animated drawer. The kit's deeper renderer tests
(`apps/native/tests/`) use the same engine.
