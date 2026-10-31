# Accessibility Conformance Report: `atoll acr`

Software sold to public bodies, universities and most large companies is asked for an
Accessibility Conformance Report (ACR): the VPAT® template filled in, one row per WCAG success
criterion, saying "Supports", "Partially Supports" and so on, with remarks. Teams usually write
it by hand after an audit, months after the code.

For the parts of an app built from this kit, most of the answers are already proven by the
repo's own checks: the ARIA check over every demo in `cargo test`, axe in light, dark, RTL and
every accent, the keyboard contracts run in three browsers, the compile-time rules for titles,
labels and `alt`. `atoll acr` turns those into a draft ACR for the components your app uses.

```sh
atoll acr --product "Taskboard" --out acr.md      # components in atoll-lock.json
atoll acr dialog select data_table --out acr.md    # or name them (blocks work too)
```

## What is in it

- **Both WCAG 2.2 tables** (Level A: 31 criteria, Level AA: 24), in the VPAT 2.5 column layout:
  Criteria, Conformance Level, Remarks and Explanations.
- **What the kit does** for each criterion, from [`docs/acr/wcag22.toml`](acr/wcag22.toml), and
  an Evidence section naming the test or file behind each row.
  `crates/cli/tests/acr.rs` fails if a cited file or test is removed, so the report can't keep
  pointing at a check that no longer exists.
- **Rows computed from your components.** 2.1.1 Keyboard, 2.4.3 Focus Order and 4.1.2 Name, Role,
  Value list the contracts of the components you use and every gap they record (for example
  "toolbar: Home works, End does not"), and drop to "Partially Supports" when there is one.
  Keyboard-operable components without a contract yet are named. Rows about components you don't
  use (toasts for 2.2.1 Timing Adjustable) become Not Applicable for the kit.
- **What your app must still do**, marked *review* in the Conformance column and spelled out
  after "Your app:" in the remarks: page titles, `lang`, link text, media, your own colours.
- **Not Evaluated** where the kit has no check yet (reflow at 320px, text spacing, target size,
  focus not obscured). These are not claimed either way.

## What it is not

It is a draft for you to check and finish, not an audit. It knows nothing about your app's own
markup, content or custom components, and automated checks never cover everything a screen
reader user meets (docs/SCREEN_READERS.md lists the manual checks). Edit the file before you
send it to anyone, and have it reviewed if it goes into a contract.

VPAT is a registered service mark of the Information Technology Industry Council (ITI). The
report follows the layout of its WCAG edition; it is not an ITI template.
