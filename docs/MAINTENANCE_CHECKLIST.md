# Maintenance checklist

Atoll audited against the "Maintenance" section of the [Design System Checklist](https://www.designsystemchecklist.com/) on 2026-11-01. Each item says where it's met. The checklist is written for a design-system team inside a company; where Atoll has one maintainer, an open-source kit and a paid Pro pack, the item is adapted and the adaptation is said. `crates/cli/tests/maintenance.rs` fails if a file this page points to disappears.

**Score: 24 of 28 met, 2 partly met, 2 waiting on the owner's accounts.** Before this audit: 4 met (getting started, sandbox apps, release cycle, decision log).

## Documentation (10 of 10 in place, 1 partly)

| Item | State | Where |
|---|---|---|
| Design system principles | Met | [PRINCIPLES.md](PRINCIPLES.md): eight principles and how they rank when they collide |
| Getting started | Met (existed) | [GETTING_STARTED.md](GETTING_STARTED.md), checked from scratch in CI by `scripts/getting-started-check.sh`; [EXISTING_APP.md](EXISTING_APP.md) for existing apps |
| Design best practices | Met | [BEST_PRACTICES.md, Designing with Atoll](BEST_PRACTICES.md). Adapted: there is no Figma library yet, so the design source is the gallery and the Theme builder (a design-tool library is on the [roadmap](ROADMAP.md)) |
| Development best practices | Met | [BEST_PRACTICES.md, Developing with Atoll](BEST_PRACTICES.md), with recommended dependencies and pitfalls |
| Component anatomy | Partly | [ANATOMY.md](ANATOMY.md): the layers, the files, what's customizable and what isn't, layout limits. A Component Structure snippet is on 44 of 80 component pages; the other 36 are listed in `crates/cli/tests/maintenance.rs`, which stops the list growing |
| Component properties | Met in code | Every component page has a **Properties** table per part (prop, type, default or required, description), generated from the source by `atoll registry` (`crates/cli/src/props.rs`), so it can't drift. Parts whose props come from `atoll-primitives` name that struct. Design-side properties wait for a design-tool library |
| Component composition examples | Met | [COMPOSITION.md](COMPOSITION.md): five compositions (card with field, dialog with actions, table slots, empty state, menu row), compiled as doctests |
| Sandbox product example | Met (existed) | `apps/dashboard`, `apps/booking`, `apps/site`, `apps/showcase`, `apps/trial`, each with its browser checks and a `GAPS.md` |
| Browser / OS support | Met | [SUPPORT_MATRIX.md](SUPPORT_MATRIX.md): tiers, what CI checks on each browser, desktop OS, mobile and native target, and what's untested |
| Release cycle | Met (existed) | [VERSIONING.md](VERSIONING.md) (breaking changes batched, at most one minor every six weeks, previous minor supported three months) and [RELEASE.md](RELEASE.md) |

## Local libraries (4 of 4)

| Item | State | Where |
|---|---|---|
| When to build | Met | [LOCAL_LIBRARIES.md, When to build your own](LOCAL_LIBRARIES.md) |
| Horizontal and vertical libraries | Met | [LOCAL_LIBRARIES.md, Horizontal and vertical libraries](LOCAL_LIBRARIES.md) |
| Library expectations | Met | [LOCAL_LIBRARIES.md, Library expectations](LOCAL_LIBRARIES.md), naming the kit's tools that check each one |
| Release cycle alignment | Met | [LOCAL_LIBRARIES.md, Keeping in step](LOCAL_LIBRARIES.md): support a new Atoll minor within a month |

## Team processes (5 of 6, 1 partly)

| Item | State | Where |
|---|---|---|
| Decision-making log | Met (existed, extended) | [decisions/](decisions/README.md), now with an index, a [template](decisions/TEMPLATE.md) and the rules for when to write one ([MAINTAINING.md](MAINTAINING.md#decision-log)) |
| Roadmap | Met | [ROADMAP.md](ROADMAP.md): now, next, later, not planned; reviewed monthly; 70/30 split between roadmap and support |
| Stakeholder mapping | Met | [MAINTAINING.md, Stakeholders](MAINTAINING.md#stakeholders). Adapted: users, design partners, Pro buyers, evaluators, contributors and upstream projects instead of a company's product teams and leadership |
| Analytics | Partly | [MAINTAINING.md, Analytics](MAINTAINING.md#analytics): what to measure and from where, with no telemetry and no cookies. Docs-site page analytics and recording GitHub traffic need the owner |
| Ongoing support "shifts" | Met (adapted) | [MAINTAINING.md, Support time](MAINTAINING.md#support-time-shifts): one maintainer, so a fixed 30% of time in two triage sessions a week, with a weekly rotation if maintainers join |
| SLA | Met | [MAINTAINING.md, Service levels](MAINTAINING.md#service-levels): response and resolution targets per kind of request, labels, workaround guidance |

## Community support (2 of 4, 2 waiting on the owner)

| Item | State | Where |
|---|---|---|
| Support channels | Waiting on the owner | Planned in [MAINTAINING.md, Support channels](MAINTAINING.md#support-channels): Discussions with Q&A and per-platform categories (Web, Desktop, Mobile, Native), issues, private security advisories, Pro discussions. Discussions and private reporting must be switched on |
| Templates | Met | `.github/ISSUE_TEMPLATE/`: bug report (reproduction, platform, versions), feature request, component proposal; `.github/pull_request_template.md` |
| Regular updates | Met | [MAINTAINING.md, Regular updates](MAINTAINING.md#regular-updates): release notes every release, a monthly "What's new" post with a template, breaking changes announced three times |
| Open hours | Waiting on the owner | Planned as a bookable call in the issue chooser ([MAINTAINING.md](MAINTAINING.md#support-channels)); needs a booking page |

## Contribution (4 of 4)

| Item | State | Where |
|---|---|---|
| House rules for the system | Met | [CONTRIBUTING.md, House rules](../CONTRIBUTING.md): why the kit moves slower, the non-negotiables, proposals first, one change per PR |
| Contribution guidelines | Met | [CONTRIBUTING.md](../CONTRIBUTING.md): set-up, making a change, the gate, a new component's checklist, licence; [SECURITY.md](../SECURITY.md) |
| Feature proposal template | Met | `.github/ISSUE_TEMPLATE/component_proposal.yml`: use cases, why in the kit, anatomy and API sketch, WAI-ARIA pattern and keys, platforms, effect on existing apps |
| Engagement | Met | [MAINTAINING.md, Recognition](MAINTAINING.md#recognition): credit in changelog and release notes, thanks in the monthly post, triage rights |

## Owner steps

These need the repository owner's accounts. Everything else is in the repository.

1. **Turn on Discussions** (Settings > General > Features > Discussions) and set the categories: Announcements, Q&A (answer format), Ideas, Show and tell, Web, Desktop, Mobile, Native (Blitz). The issue chooser already links to Q&A.
2. **Turn on private vulnerability reporting** (Settings > Code security > Private vulnerability reporting). [SECURITY.md](../SECURITY.md) points to it.
3. **Create the labels** the triage rules use (Issues > Labels): `proposal`, `a11y`, `regression`, `needs info`, `platform: web`, `platform: desktop`, `platform: mobile`, `platform: native`. GitHub already has `bug`, `enhancement`, `question` and `documentation`.
4. **Office hours** (optional): make a free booking page (for example Cal.com or Calendly) with a few 30-minute slots a month, and add its link to `.github/ISSUE_TEMPLATE/config.yml` as a contact link.
5. **Analytics** (optional): once a month, copy GitHub's Insights > Traffic numbers into the monthly update (GitHub keeps only 14 days). If you want docs-site page counts, pick a cookieless, aggregate-only service and tell a thread to add it; nothing is added without that choice.
6. **First monthly update** on the first working day of December, in Discussions > Announcements, from the template in [MAINTAINING.md](MAINTAINING.md#regular-updates).
