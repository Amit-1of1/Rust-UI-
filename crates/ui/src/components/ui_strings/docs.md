# UiStrings

Every word the kit shows or announces on its own ("Next", "Clear", "No results found.", "Go to next page", screen-reader names) lives in one `UiStrings` struct with English defaults. Wrap the app in `UiStringsProvider` to translate them, and pass a signal to switch language at runtime. Text you pass as props (labels, placeholders) always wins.

```rust
let strings = use_signal(|| UiStrings { next: "Suivant".into(), ..UiStrings::default() });
rsx! { UiStringsProvider { strings, App {} } }
```

Built-in translations: `UiStrings::locale("fr")` returns French, and likewise `de`, `es`, `pt`, `ar`, `he`, `ja` and `zh` (`None` for other codes; `LOCALES` lists them). Start from one and override words: `UiStrings { next: "Weiter".into(), ..UiStrings::german() }`. `text_direction(tag)` gives `"rtl"` or `"ltr"` for `<html dir>`; see `docs/RTL.md` in the kit repository. The translations were drafted by an AI model and are checked for placeholders, not yet by native speakers.

```rust
let strings = use_signal(|| UiStrings::locale("ar").unwrap_or_default());
```

Templates use `{name}` placeholders (`"Select {row}"`, `"Showing {first}–{last} of {total} rows"`), filled with `fill`.

Not covered: calendar weekday names (use the calendar's `day_name` prop), month names (dioxus-primitives), and the field validators' messages (pass your own validator).

## Accessibility

- Role and name: renders no element of its own; it changes the accessible names inside it.
- Keyboard: none.
- Reduced motion: no motion.

## Source

Original to this repo. The idea follows thaw's `ConfigProvider` (MIT).
