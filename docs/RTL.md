# Right-to-left languages and translations

For Arabic, Hebrew, Persian and Urdu pages the kit mirrors its layout, and it ships its own
words in nine languages.

## Turn it on

Set `lang` and `dir` on `<html>`, not on a wrapper: dialogs, sheets and menus render at the
page root and read the direction from there.

```rust
let lang = "ar";
let strings = use_signal(move || UiStrings::locale(lang).unwrap_or_default());
// web: document.documentElement.lang / dir; fullstack: render them in your index.html
let _ = document::eval(&format!(
    "document.documentElement.lang='{lang}';document.documentElement.dir='{}'",
    text_direction(lang)
));
rsx! { UiStringsProvider { strings, App {} } }
```

`text_direction("ar")` is `"rtl"`; it takes any BCP 47 tag (`"he-IL"`, `"fa"`).

## What mirrors

- Spacing, borders and positions use logical CSS properties (`margin-inline-start`,
  `inset-inline-end`), so they follow the reading direction.
- **Sidebar and sheet sides are start and end.** `SidebarSide::Left` and `SheetSide::Left`
  mean the start edge: the left in English, the right in Arabic. The layout mirrors with no
  code change. For a panel that must stay on one physical side, pick the other side when
  `text_direction(lang) == "rtl"`.
- The switch thumb, the sidebar rail, the stepper connectors, progress and shimmer animations
  move the other way.
- The carousel's Next button scrolls towards the left.
- Icons that point along the reading direction (arrows, chevrons) flip with
  `mirror_rtl: true` (docs/ICONS.md); the kit's own chevrons already do.

Tooltip, popover and hover card sides (`ContentSide::Left`) stay physical, as in Radix.

Checked by `tests/e2e/rtl.mjs` (sheet edges, switch thumb, carousel) and axe on the whole
gallery with `dir="rtl"` (`scripts/e2e.sh`).

Not mirrored yet: the slider (the fill and arrow keys still run left to right; this lives in
dioxus-primitives) and chart axes (time still runs left to right, which is what most RTL
dashboards show).

## Built-in translations

`UiStrings::locale(tag)` returns the kit's own words (Next, Close, "Showing 1–5 of 14 rows",
screen-reader hints) in French (`fr`), German (`de`), Spanish (`es`), Brazilian Portuguese
(`pt`), Arabic (`ar`), Hebrew (`he`), Japanese (`ja`) and Simplified Chinese (`zh`), or
`None` for other languages (English is `UiStrings::default()`). Each language also has its
own constructor (`UiStrings::arabic()`), and `LOCALES` lists the codes.

Override single words with struct update syntax:

```rust
UiStrings { next: "Weiter".into(), ..UiStrings::german() }
```

The translations were drafted by an AI model, not yet reviewed by native speakers;
`crates/ui/tests/locales.rs` checks that each keeps every `{placeholder}` of the English text.
Month and weekday names in the calendar come from dioxus-primitives and are English.
