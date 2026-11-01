# 2026-11-01: how the project is maintained

**Status.** Accepted

**Decision.** Atoll adopts written maintenance processes, adapted from the design-system maintenance checklist (designsystemchecklist.com, "Maintenance") for a project with one maintainer: published service levels, a fixed share of time for support (about 30%) in two triage sessions a week, issue templates for bugs, feature requests and component proposals, a monthly "What's new" post and roadmap review, a decision template, and an analytics plan with no telemetry. The processes live in [MAINTAINING.md](../MAINTAINING.md), [CONTRIBUTING.md](../../CONTRIBUTING.md), [ROADMAP.md](../ROADMAP.md) and [LOCAL_LIBRARIES.md](../LOCAL_LIBRARIES.md); [MAINTENANCE_CHECKLIST.md](../MAINTENANCE_CHECKLIST.md) maps every checklist item to where it's met.

**Context.** The owner asked for the kit to be audited against that checklist (project thread, 2026-11-01). Release and API rules already existed (VERSIONING, STABILITY, RELEASE, PROTOCOL); what was missing was everything facing users and contributors: how to ask for help, what response to expect, how to propose a component, what the project values, and where it's going.

**Options considered.**
- **Team-style processes as the checklist describes them** (rotas, stakeholder meetings, Slack channels): written for a design-system team inside a company; with one maintainer they would be fiction.
- **Adapted processes** (chosen): the same intent, sized for one person, with promises small enough to keep.
- **Nothing written**: adopters can't tell whether the project is maintained, which is the first thing an evaluator asks ([HARD_QUESTIONS.md](../HARD_QUESTIONS.md)).

**Why.** Honest claims ([PRINCIPLES.md](../PRINCIPLES.md), 2): a service level that's published and met is worth more than a generous one that isn't. No telemetry follows principle 4 (you own the code): nothing in Atoll phones home, so usage is measured from public and owner-only sources instead.

**Consequences.** The maintainer commits to the response targets in MAINTAINING.md and to a monthly update. Enabling Discussions, private vulnerability reporting, a booking page for office hours and any docs-site analytics need the owner's accounts and are listed as owner steps in MAINTENANCE_CHECKLIST.md. Revisit the targets after three months of data, or when a second maintainer joins.

**Approved by.** The owner asked for the audit and gave the project full autonomy on non-visual decisions (2026-10-31).
