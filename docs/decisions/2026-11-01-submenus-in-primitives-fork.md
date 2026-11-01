# 2026-11-01: Nested submenus live in our primitives fork

**Status.** Accepted

**Decision.** Dropdown menu, context menu and menubar get `*Sub`, `*SubTrigger` and `*SubContent` parts. Their behaviour (focus, keyboard, hover intent, closing) is written once in the primitives fork (`primitives/src/submenu.rs`, marked `PATCH(rust-ui)`); the kit only adds classes and a chevron, reusing each menu's existing look.

**Context.** Nested submenus are the loudest gap in the November complaints research (`research/atoll-complaints-gap-list-2026-11.md`, gap 1: MUI #11723, open since 2018; Radix ships `Sub`). Desktop apps, Atoll's target, need File > Open recent > ... Neither the kit nor the primitives had a submenu.

**Options considered.**
- **In the primitives fork** (chosen): a submenu trigger must be an item of the parent menu's focus collection, and must keep the parent open while focus is inside the submenu. Both need the menus' private context and `CollectionState`, which only code inside the primitives crate can reach.
- **In the kit only**: the kit cannot register an item in a primitive's collection, so the trigger would not take part in the parent's arrow keys and the parent would close the moment focus moved into the submenu. It would also mean the kit writing menu markup of its own, which the "never change a primitive's structure" guardrail is there to stop.
- **Do nothing**: menus stay one level deep; apps nest a second dropdown by hand, without keyboard support.

**Why.** [GUARDRAILS.md](../GUARDRAILS.md) says forks are for fixes, not features, and a patch "doesn't add new API". This sets that rule aside for one feature, recorded here as the rule asks. It is the only place the behaviour can be correct; the patch is additive (no existing part changes), shared by the three menus, written so it could go upstream as it is, and uses element events only, so it also runs on native without `eval`. Existing items work inside a submenu unchanged, because the submenu content re-provides the menu's own context.

**Consequences.** Three new parts per menu, experimental ([STABILITY.md](../STABILITY.md)). The fork carries one more feature patch to rebase on upstream moves (`scripts/fork-drift.sh` reports it); if upstream ships submenus, drop ours and follow theirs. Known limits: submenus don't flip at screen edges (the parent panel is transformed, so it is the submenu's containing block), the dropdown's trigger can't say `aria-expanded` (its items are `option`s), and the arrow keys are not mirrored in right-to-left text.

**Approved by.** The owner's standing goal (2026-10-31, full autonomy) and the coordinator's brief for this thread, which asked for the primitives fork change.
