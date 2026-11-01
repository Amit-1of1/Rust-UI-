# How Atoll is maintained

How the project runs day to day: who it's for, how requests are handled and how fast, how we measure whether it's working, and how we talk to the people who use it. Contributors read [CONTRIBUTING.md](../CONTRIBUTING.md); this page is the maintainer's side of the same deal.

Atoll has **one maintainer** (the repository owner), who reviews and merges every change. Much of the implementation is written by AI coding agents, one pull request at a time, under [GUARDRAILS.md](GUARDRAILS.md) and [PROTOCOL.md](PROTOCOL.md), with CI on every change ([HARD_QUESTIONS.md](HARD_QUESTIONS.md#who-builds-this-and-how-old-is-it)). Where a design-system team would split work across people, this page says how one person does it.

## Stakeholders

Who Atoll serves, what each needs, and where we hear from them.

| Who | What they need | Where we hear from them |
|---|---|---|
| **App developers** using the free kit (Dioxus web, desktop, mobile) | Components that work, honest docs, upgrades that don't break them | Issues, Discussions, the example apps' `GAPS.md` |
| **Design partners** (up to five early teams) | Their top missing pieces built first, a direct line | Issues triaged within 2 working days, calls at day 14 and day 45 |
| **Pro buyers** (personal and team licences) | Polished blocks and templates, answers when stuck | Pro discussions in the private Pro repository (priority answers) |
| **Designers** on those teams | A clear set of parts, variants and tokens to design with | The gallery, the Theme builder, [BEST_PRACTICES.md](BEST_PRACTICES.md) |
| **Evaluators and tech leads** deciding whether to adopt | Evidence, limits, track record | [HARD_QUESTIONS.md](HARD_QUESTIONS.md), [CLAIMS.md](CLAIMS.md), the docs site |
| **Contributors** | Clear rules, fast reviews, credit | Pull requests, [CONTRIBUTING.md](../CONTRIBUTING.md) |
| **Upstream projects**: Dioxus, DioxusLabs/components, Blitz | Bugs reported well; fixes offered upstream | Their issue trackers, only with the owner's go-ahead ([GUARDRAILS.md](GUARDRAILS.md)) |
| **Coding agents** writing UI with the kit | The real API, not guesses | `llms.txt`, `atoll docs`, `atoll mcp` |

## Support channels

| Channel | For | Notes |
|---|---|---|
| [Discussions](https://github.com/Amit-1of1/Rust-UI-/discussions), category **Q&A** | Questions, "how do I", help with an error | Needs Discussions turned on (owner step) |
| Discussions, categories **Web**, **Desktop**, **Mobile**, **Native (Blitz)** | Platform-specific questions, so details like OS and webview stay together | Same |
| Discussions, **Ideas** | Early ideas before they're a feature request | Same |
| Discussions, **Show and tell** | Apps built with Atoll | Same |
| [Issues](https://github.com/Amit-1of1/Rust-UI-/issues/new/choose) | Bugs, feature requests, component proposals | Templates in `.github/ISSUE_TEMPLATE/` ask for what triage needs |
| Private security advisory | Vulnerabilities | [SECURITY.md](../SECURITY.md); never a public issue |
| Pro discussions (private Pro repository) | Pro buyers' questions | Priority answers |
| Office hours (bookable call) | Questions that don't fit a ticket: migrations, architecture, a design review | Link in `.github/ISSUE_TEMPLATE/config.yml` once the owner sets up a booking page |

Until Discussions are on, questions go to issues with the "Question" link in the issue chooser, which points to the same place.

## Service levels

What you can expect, so you can decide whether to wait for a release or work around a problem. These are targets for one maintainer, measured from when a report has the information its template asks for. "Working days" are Monday to Friday.

| Request | First response | Resolution target |
|---|---|---|
| Security vulnerability (private advisory) | 3 working days | Fix or mitigation within 14 days; released as a patch and backported to the previous minor |
| Regression, crash, or broken keyboard or screen-reader behaviour in a **stable** item | 3 working days | Fixed in the next patch release |
| Other bug | 7 days | Triaged (accepted, needs info, won't fix) within 7 days; fix scheduled by severity |
| Feature request or component proposal | 14 days | Accepted onto the roadmap, sent back for details, or declined with a reason |
| Question in Discussions | 7 days | Best effort; answered questions that come up twice go into the docs |
| Pull request | 7 days for a first review | Merged, changes requested or closed with a reason |
| Design partner's issue | 2 working days | As agreed in the partnership |
| Pro buyer's question | 2 working days | Best effort |

**Labels:** `bug`, `enhancement`, `proposal`, `question`, `a11y`, `regression`, `docs`, plus `platform: web|desktop|mobile|native` and `needs info`. An issue waiting on its author for 30 days with `needs info` is closed, and reopened when the information arrives.

**Workaround guidance.** If an issue won't be fixed within its target, the triage reply says so and suggests a workaround (usually: copy the component with `atoll add` and patch your copy, then `atoll update` when the fix lands).

## Support time ("shifts")

With one maintainer there is no rota; there is a fixed slice of time instead, so support doesn't eat the roadmap:

- **About 30% of the maintainer's time** goes to support: triage, answers, reviews and small fixes. The rest is roadmap work ([ROADMAP.md](ROADMAP.md)).
- **Two triage sessions a week** (for example Tuesday and Friday): new issues get labels, a first response and a target; Discussions are answered; pull requests get a first review. Security reports are checked every working day.
- Requests that arrive between sessions wait for the next one unless they're security reports or regressions in stable items.
- If more maintainers join, triage rotates weekly and the person on rotation owns all first responses that week.

## Decision log

Decisions that shape the kit, and the reasoning behind them, are recorded so they don't have to be argued again. Each decision is one file in [docs/decisions/](decisions/README.md), written from [the template](decisions/TEMPLATE.md); older ones are in [GUARDRAILS.md, Decision log](GUARDRAILS.md#decision-log).

Write a decision record when:

- a principle or a guardrail is set aside ([PRINCIPLES.md](PRINCIPLES.md));
- a dependency, a fork or a platform is added or dropped;
- a public API, naming scheme or token family is chosen that other components will follow;
- a recurring question gets a standard answer ("why plain CSS and not Tailwind?").

Recurring calls that already have a written rule don't need a record each time: the rule is in [VERSIONING.md](VERSIONING.md) (is this breaking?), [STABILITY.md](STABILITY.md) (stable or experimental?), [LOCAL_LIBRARIES.md](LOCAL_LIBRARIES.md) (kit or local component?) and [CONTRIBUTING.md](../CONTRIBUTING.md#what-gets-accepted) (accept or decline a proposal?).

## Analytics

The goal is to know whether Atoll is used and useful **without tracking anyone**. Rules:

- **No telemetry in the CLI, the components or the example apps. Ever.** Nothing in Atoll phones home ([PRINCIPLES.md](PRINCIPLES.md)).
- **No cookies and no personal data on the docs site.** If page analytics are added, they must be cookieless and aggregate-only (no fingerprinting, no cross-site identifiers), so no consent banner is needed; anything that would need consent isn't used. The service and what it collects are named on the docs site.
- Every number we publish from these sources follows [CLAIMS.md](CLAIMS.md): dated, with how to reproduce it.

What we measure, from sources that exist without tracking:

| Question | Metric | Source | Cadence |
|---|---|---|---|
| Are people finding it? | Repository views, unique visitors, top referrers | GitHub Insights > Traffic (owner only; keeps 14 days, so record it) | Weekly, into the monthly update |
| Are people installing it? | Clones; crates.io downloads per crate and version (after v0.1.0) | GitHub Insights > Traffic; crates.io (public) | Monthly |
| Are people building with it? | Public repositories depending on `atoll-ui`; GitHub "Used by" | GitHub code search for `atoll-ui` in `Cargo.toml`; dependency graph | Monthly |
| Which components matter? | Components named in issues, discussions and `GAPS.md`; most-read docs pages (once page analytics exist) | Issue labels; docs analytics | Monthly |
| Is it good to use? | Answers to a short survey (5 questions, below); design partner feedback questions | A pinned Discussions post or a form linked from the monthly update | Each quarter |
| Is support keeping up? | Median first-response time; open bugs by age; share of service levels met | GitHub issue data | Monthly |
| Is it paying for itself? | Pro sales and refunds | Polar dashboard (owner only) | Monthly |

The quarterly survey: (1) What did you build with Atoll? (2) Which component saved you the most time? (3) What was missing or broken? (4) How likely are you to recommend it (0 to 10)? (5) Anything else? Answers are anonymous unless someone signs one.

## Regular updates

People only use what they hear about. The cadence:

- **Every release**: GitHub release notes from the CHANGELOG section, crediting contributors by handle.
- **Monthly "What's new in Atoll"**: a Discussions post in the **Announcements** category on the first working day of the month, pinned until the next one. Template:

  ```markdown
  # What's new in Atoll: <Month Year>

  **Released:** <versions, with links to release notes>
  **New:** <components, props, tools, with one line each and a docs link>
  **Fixed:** <the fixes people asked about, with issue links>
  **Thanks:** <contributors and reporters by handle>
  **Coming next:** <the top of the roadmap's "Now" and "Next">
  **Numbers:** <the month's metrics from the table above, dated>
  **Tell us:** <one question, or the survey link in survey months>
  ```

- **Breaking changes** are announced when they land on `main` (behind deprecations), again in the release notes, and in the next monthly post, each time with the MIGRATION.md link.

## The monthly cycle

On the first working day of each month the maintainer:

1. Writes the monthly update (above), collecting the metrics.
2. Reviews [ROADMAP.md](ROADMAP.md): moves finished items out, re-orders "Next" from new gaps, votes and partner requests, updates the "Last reviewed" date.
3. Checks the service levels for the past month and adjusts support time if they slipped.
4. Cuts a patch release if fixes are waiting ([RELEASE.md](RELEASE.md)).

## Recognition

- Every merged contribution is credited by handle in its CHANGELOG fragment (`changes/<topic>.md`), and so in the release notes.
- The monthly update thanks contributors and the people whose reports led to fixes.
- Contributors who help repeatedly may be offered triage rights, and a Pro licence as thanks.
- A contributor who wants a note for their manager or a portfolio gets one, linking their merged work.
