## What and why

<!-- One change per PR. Link the issue or accepted proposal: "Closes #123". -->

## How it was checked

<!-- Commands you ran and what you looked at (gallery demo, light and dark, keyboard). -->

- [ ] `scripts/check.sh` passes
- [ ] `scripts/check.sh --a11y` passes (any UI change)

## Checklist

- [ ] A changelog fragment in `changes/<topic>.md` (and `<topic>.migration.md` if breaking)
- [ ] New public items are marked experimental (docs/STABILITY.md)
- [ ] Theme tokens only; reduced motion handled for any animation
- [ ] Keyboard works; primitive markup and ARIA untouched
- [ ] Docs updated (`docs.md`, guides) and generated files regenerated, not hand-edited
- [ ] `THIRD_PARTY.md` row for anything copied
