# Let an AI agent see and drive your app

AI coding agents write UI code blind: they guess what the screen looks like from the source,
and they can't click the button they just wired up. `atoll_test::agent` fixes that. It is a
Model Context Protocol (MCP) server that renders your app headless (the same engine and CSS
as `atoll-test`, no browser or window) and gives the agent tools to read the screen, click,
type, press keys, let time pass and take screenshots. The agent checks its own work.

## Set it up (2 minutes)

1. `atoll setup-tests` (adds `atoll-test` to `[dev-dependencies]`; docs/TESTING.md).
2. Add `examples/agent.rs`:

   ```rust
   use dioxus::prelude::*;

   fn main() {
       // Your root component; size and dark mode are optional.
       atoll_test::agent::serve(atoll_test::Screen::builder(my_app::App).size(1280, 800));
   }
   ```

3. Register it with your agent. Claude Code:

   ```sh
   claude mcp add my-app -- cargo run -q --example agent
   ```

   Any MCP client works: the command is `cargo run -q --example agent`, transport stdio.

Try the kit's own: `claude mcp add atoll-demo -- cargo run -q -p atoll-test --example agent`,
then ask "sign up with ada@example.com on the yearly plan and tell me what the page says".

## Tools

| Tool | What it does |
| --- | --- |
| `outline` | The screen as a user or screen reader meets it: one line per element with a role (role, name, value, `[selected=true]`, `[focused]`, `[disabled]`), indented, plus loose text. Hidden parts are left out. |
| `click` | Clicks an element (role + name, label, placeholder, text, test id or CSS). Fails, with what *is* on screen, if nothing matches or it is hidden. |
| `fill` | Replaces a field's text. |
| `press` | Presses a key (`Enter`, `Tab`, `Shift+Tab`, `Escape`, `ArrowDown`, `Ctrl+K`), optionally several times; says where focus went. |
| `type_text` | Types into whatever has focus. |
| `wait` | Moves the app's clock on (toasts, debounces, timers). |
| `read` | One element: description, visibility, box, HTML. |
| `screenshot` | A PNG of the frame, painted on the CPU with the app's real CSS. |

Every action returns the new outline, so one call is enough to see what changed. A bad query
is an error message for the agent, never a crash.

Example outline after the agent filled the form and clicked "Sign up":

```text
main
  textbox "Email" value="ada@example.com"
  tablist
    tab "Monthly" [selected=false]
    tab "Yearly" [selected=true]
  button "Sign up" [focused]
  status "Thanks, ada@example.com! Plan: Yearly."
```

## Why this is safe to give an agent

It is a copy of your app in memory: no network access of its own, no browser profile, no
real clicks on your desktop. Whatever your components do when clicked (a server call, a file
write) still happens, so point it at a test configuration if that matters.

## Limits

Same as `atoll-test` (docs/TESTING.md): Blitz is not a browser, so JavaScript run through
`document::eval` does nothing, and platform widgets like `<select>` options are drawn as
text. For pixel-exact checks in a real browser use the e2e suite.

`outline` is also handy in your own tests: `println!("{}", screen.outline())`.
