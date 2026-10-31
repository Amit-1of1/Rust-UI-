# First-customer trial: a fullstack SaaS app from the public docs (2026-10-31)

A skeptical team's view. "Ledgerly", a small invoicing SaaS, was written from an empty
`cargo new` using only the README, docs/INSTALL.md and the component `docs.md` pages:
Dioxus fullstack (server rendering, server functions), the router, a landing page with
pricing and FAQ, sign-in, a sidebar app shell, an overview with KPI cards and a chart, a
20,000-row customers table, an invoice form with line items, settings tabs, a dark-mode
switch and English, French and Arabic. It was built with `dx build --fullstack` and driven in
headless Chromium. Source: project files `rust-ui/first-customer/ledgerly-src/`.

Severity: **High** blocks production or breaks accessibility; **Medium** needs a workaround
that is easy to get wrong; **Low** is friction.

| # | Sev | Friction | Status |
|---|---|---|---|
| 1 | High | Server-rendered pages broke after hydration: ids came from process-wide counters, so from the second request on the server's `for`/`aria-describedby` didn't match the browser's | Fixed: per-app ids (`use_unique_id`), `tests/ssr.rs` (PR "fullstack-ready") |
| 2 | High | A 20,000-row table was a 4.4 MB HTML page: `DataTable` only filtered, sorted and paged in the browser | Fixed: server mode (`total_rows`, `on_query_change`); 44 KB |
| 3 | High | Arabic page: the desktop sidebar overlapped the content, the switch thumb left its track, sheets slid the wrong way | Fixed: logical CSS, `tests/e2e/rtl.mjs` (PR "RTL") |
| 4 | Medium | Every app had to translate ~85 kit strings itself | Fixed: `UiStrings::locale` for 8 languages (PR "RTL") |
| 5 | Medium | Dark mode reset on every reload, and a server-rendered page flashed light first | Fixed: `PersistTheme` + `THEME_INIT_SCRIPT` |
| 6 | Medium | Invoice line items: form fields are hooks, so a variable-length list fell back to raw `Input`s with no validation | Fixed: `FormState::add_field`/`remove_field` |
| 7 | Medium | Media queries (mobile sidebar, dark preference, reduced motion) used the browser value during hydration, unlike the server | Fixed: start from the server's value, update after hydration |
| 8 | Medium | Missing next to shadcn: native select, resizable panels | Added (PR "components") |
| 9 | Medium | Charts scale their text with their width: a full-width bar chart had ~25 px axis labels | Open: needs measured width (see below) |
| 10 | Medium | A fresh project resolves the newest `wasm-bindgen`; `dx` then refuses to build ("requires 0.2.130 but 0.2.129 is installed") | Documented in docs/SSR.md |
| 11 | Low | Without a reset the app renders in Times New Roman | Open PR #63 adds `ThemeStyles { reset: true }` |
| 12 | Low | `ToggleGroup` docs don't show the controlled single-choice pattern (`pressed`, `on_pressed_change`) | Open |
| 13 | Low | `BarChart` takes `label`, where most chart libraries say `title` | Open (naming) |
| 14 | Low | `TableQuery` isn't `Serialize`, so a server function takes its fields one by one | Open (would add serde to the kit) |

What worked well: the prelude, `use_form` with label/error/focus wiring, `SidebarProvider`
with icon collapse, the router-aware links, `ToastsExt`, theme tokens (dark mode needed no app
CSS), and `table_view` being reusable on the server.

## Performance budget

Debug fullstack build, Chromium, 20,000 customers: client-side filtering took ~200 ms per
keystroke; server mode answers a page in one request. Release-size budgets are tracked in
the download-size notes (docs/notes) and `scripts/release-web.sh`.
