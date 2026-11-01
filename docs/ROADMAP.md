# Roadmap

What Atoll is working on, in three horizons. It's a plan, not a promise: dates are targets, and items move when a design partner or a gap in a real app shows something more urgent. Last reviewed **2026-11-01**; it's reviewed at the start of every month ([MAINTAINING.md](MAINTAINING.md#the-monthly-cycle)).

**How time is split.** About 70% goes to roadmap work below and 30% to support: answering issues and discussions, triage, and bug fixes, within the [service levels](MAINTAINING.md#service-levels). Support work that keeps coming back (the same question, the same workaround) becomes a roadmap item.

**How items get here.** From the example apps' `GAPS.md` files, design partners' top three missing pieces, issues and discussions (a 👍 on the first post counts as a vote), and the hard questions an evaluator raises ([HARD_QUESTIONS.md](HARD_QUESTIONS.md)). Larger items get a decision record ([decisions/](decisions/README.md)).

## Now (November 2026)

| Item | Why | State |
|---|---|---|
| **First release, v0.1.0** on crates.io | Lets apps depend on a version instead of a git commit | Ready; needs the owner's account steps in [RELEASE.md](RELEASE.md#what-needs-the-owners-accounts) |
| **Docs site and gallery online** (GitHub Pages) | The first thing an evaluator opens | Workflow ready; waiting on a run by the owner |
| **Atoll Pro v1** to buyers | Paid blocks and the SaaS starter fund the free kit | Built; delivery target 2026-11-14 ([pro/README.md](pro/README.md)) |
| **Visual identity** (default look, presets, fonts, mark) | The default look decides first impressions | On hold for the owner's choice of design-system direction |
| **Maintenance foundations** (this roadmap, support, templates, contribution guide) | Makes the project safe to depend on and to contribute to | In review |

## Next (December 2026 to February 2027)

| Item | Why |
|---|---|
| **Dioxus 0.8** as kit minor `0.2.0`, once 0.8 is stable | Dioxus minors are breaking; it ships alone, with a migration guide ([VERSIONING.md](VERSIONING.md)) |
| **Keyboard contracts for every interactive component** (28 of 81 components have one today) | Turns more of "accessible" from a promise into a test |
| **Component Structure for every docs page** (36 components still lack the snippet) | Component anatomy on every page ([ANATOMY.md](ANATOMY.md)) |
| **Screen-reader pass** with NVDA and VoiceOver on the items in [SCREEN_READERS.md](SCREEN_READERS.md) | The one accessibility layer tests can't cover |
| **Desktop builds on Windows and macOS in CI** | Moves them from tier 3 to tier 2 ([SUPPORT_MATRIX.md](SUPPORT_MATRIX.md)) |
| **Open gaps** from `apps/dashboard/GAPS.md` and `apps/booking/GAPS.md` | Found by building real apps |

## Later (2027)

| Item | Why |
|---|---|
| **1.0** | When the stable API has gone three minor releases without a break, the kit is on the Dioxus line it will live on, and the primitives fork is either upstream or settled as ours ([RELEASE.md](RELEASE.md#after-v010)) |
| **Native (Blitz) out of preview** | Inline SVG styling, pointer capture and the remaining `eval` uses ([NATIVE.md](NATIVE.md)) |
| **iOS and native Android** | Untested or blocked today ([SUPPORT_MATRIX.md](SUPPORT_MATRIX.md)) |
| **Design-tool library** (Figma) matching the kit's components and tokens | Lets designers work in the same parts and names as the code |
| **Upstreaming** the primitive and renderer fixes | Fewer forks to carry ([FORKS.md](FORKS.md), [notes/upstreaming.md](notes/upstreaming.md)) |

## Not planned

- A second framework (Leptos, Yew, GPUI ports) or a renderer of our own ([GUARDRAILS.md](GUARDRAILS.md)).
- Telemetry in the CLI or the components ([MAINTAINING.md](MAINTAINING.md#analytics)).
