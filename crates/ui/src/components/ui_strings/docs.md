# UiStrings

Every word the kit shows or announces on its own ("Next", "Clear", "No results found.", "Go to next page", screen-reader names) lives in one `UiStrings` struct with English defaults. Wrap the app in `UiStringsProvider` to translate them, and pass a signal to switch language at runtime. Text you pass as props (labels, placeholders) always wins.

```rust
let strings = use_signal(|| UiStrings { next: "Suivant".into(), ..UiStrings::default() });
rsx! { UiStringsProvider { strings, App {} } }
```

Templates use `{name}` placeholders (`"Select {row}"`, `"Showing {first}–{last} of {total} rows"`), filled with `fill`.

Not covered: calendar weekday names (use the calendar's `day_name` prop), month names (dioxus-primitives), and the field validators' messages (pass your own validator).

## Accessibility

- Role and name: renders no element of its own; it changes the accessible names inside it.
- Keyboard: none.
- Reduced motion: no motion.

## Source

Original to this repo. The idea follows thaw's `ConfigProvider` (MIT).
