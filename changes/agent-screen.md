### Added

- **test:** `atoll_test::agent`: an MCP server that lets an AI agent read and drive a headless copy of your app (`outline`, `click`, `fill`, `press`, `type_text`, `wait`, `read`, `screenshot`). See docs/AGENTS.md; try it with `cargo run -p atoll-test --example agent`.
- **test:** `Screen::outline` (the screen as a screen reader meets it), `Screen::find` (a query without panicking), `Screen::png` and `Screen::size`.
