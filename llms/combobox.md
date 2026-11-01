# Combobox

- Copy into your app: `atoll add combobox` or `dx components add combobox --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `icons`, `ui_strings`).
- Import: `use crate::components::combobox::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Combobox component is an autocomplete input with a filterable popup list.

Filtering preserves the order defined by the rendered `ComboboxOption` elements and their `index`
props. If you want query-dependent ranking, control `query`, sort your item data in user code,
render the options in that sorted order, and assign indexes from the sorted list.

## Component Structure

```rust
let mut value = use_signal(|| None::<String>);
let mut query = use_signal(String::new);

Combobox::<String> {
    value: Some(value.into()),
    on_value_change: move |next: Option<String>| {
        value.set(next);
    },
    query: Some(query()),
    on_query_change: move |next| query.set(next),
    placeholder: "Select framework...",
    aria_label: "Select framework",
    list_aria_label: "Frameworks",
    ComboboxEmpty { "No framework found." }
    ComboboxOption::<String> {
        index: 0usize,
        value: "next".to_string(),
        text_value: "Next.js",
        "Next.js"
    }
}
```

## Results from a server

`AsyncCombobox` loads its options from an async function instead of a fixed list.
`use_async_options` wraps a `fetch(query, page)` that returns one `AsyncPage` (the items, and
whether a later page exists) or an error message:

```rust
let cities = use_async_options(|query: String, page: usize| async move {
    let found = api::search_cities(&query, page).await.map_err(|e| e.to_string())?;
    Ok(AsyncPage { items: found.cities, has_more: found.next_page.is_some() })
});

AsyncCombobox::<City> {
    options: cities,
    label: |city: City| city.name,
    on_value_change: move |city: Option<City>| picked.set(city),
    placeholder: "Search cities...",
    aria_label: "City",
    list_aria_label: "Cities",
}
```

- **Debounced search.** Typing waits 250 ms after the last key before it asks (change it with
  `set_debounce`). Opening the list loads the empty query at once.
- **Newest answer wins.** Each request cancels the one before it, and an older response that
  arrives late is dropped.
- **Loading state.** While the first page loads, the list shows "Loading results…" and is
  marked `aria-busy`. Earlier results stay visible until the new ones arrive.
- **Load more.** When `has_more` is true, the last option is "Load more results". It is a
  real option: arrow down to it and press Enter, or click it. The list stays open and the
  highlight moves to the first new result.
- **Errors.** A failed request shows "Couldn't load results. Try again" as one option; picking
  it repeats the request.
- **Announcements.** A polite live region says "Loading results…", "12 results", "No results
  found." or the error, so screen-reader users hear what the list did. The words come from
  `UiStrings` (`loading_results`, `loading_more`, `load_more`, `one_result`, `result_count`,
  `results_failed`, `try_again`).
- The function does the filtering: `AsyncCombobox` shows everything it returns, in order.
  `label` is the text shown and read for an option; `render_option` changes what the row
  shows.

If you wire a plain `Combobox` to your own loading code instead, its `list_busy` and
`announcement` props give you the same `aria-busy` and live region.

## Keyboard and accessibility

Follows the WAI-ARIA [Combobox (list autocomplete)](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/) pattern. Roles: `combobox`, `listbox`, `option`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| ArrowDown | In the input | Opens the list. | yes |
| d | In the input | Typing opens the list and filters it to the matching options. | yes |
| Enter | On a highlighted option | Picks the option and closes the list; focus stays in the input. | yes |
| Escape | In the open list | Closes the list; focus stays in the input. | yes |

States:

- `open`: The list of options is shown.
- `closed`: Only the input is shown.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// One page of results from an async source.
pub struct AsyncPage<T> {
    /// The options on this page, in the order they should be shown.
    pub items: Vec<T>,
    /// Whether a later page exists. Shows the "Load more results" option.
    pub has_more: bool,
}

/// Where an [`AsyncOptions`] source is.
pub enum AsyncStatus {
    /// Nothing requested yet.
    Idle,
    /// The first page for the current query is on its way (after the debounce).
    Loading,
    /// A later page is on its way.
    LoadingMore,
    /// The last request finished.
    Ready,
    /// The last request failed with this message.
    Failed(String),
}

/// How long [`use_async_options`] waits after the last keystroke before it asks for results.
pub const DEFAULT_DEBOUNCE: Duration;

/// The state of an async option source: the query, the options loaded so far and the request
/// status. Made by [`use_async_options`]; pass it to [`AsyncCombobox`], or read it yourself.
///
/// Every request supersedes the one before it: a slower, older response never overwrites a
/// newer one, and the older request's future is dropped.
pub struct AsyncOptions<T: 'static> {
    query: Signal<String>,
    items: Signal<Vec<T>>,
    status: Signal<AsyncStatus>,
    has_more: Signal<bool>,
    next_page: Signal<usize>,
    debounce: Signal<Duration>,
    /// `(page, debounced)`: starts a request, created in the hook's scope so its task lives
    /// as long as the component that owns the source.
    request: Callback<(usize, bool)>,
}

/// An option source that loads pages from `fetch(query, page)`, with `page` counting from 0.
///
/// ```rust,ignore
/// let cities = use_async_options(|query: String, page: usize| async move {
///     let found = api::search_cities(&query, page).await.map_err(|e| e.to_string())?;
///     Ok(AsyncPage { items: found.cities, has_more: found.next_page.is_some() })
/// });
/// ```
///
/// Typing is debounced ([`DEFAULT_DEBOUNCE`], change it with
/// [`AsyncOptions::set_debounce`]); "load more" and "try again" go out at once. The source
/// does no filtering of its own: `fetch` returns what matches `query`.
pub fn use_async_options<T, F, Fut>(fetch: F) -> AsyncOptions<T>
where
    T: Clone + 'static,
    F: Fn(String, usize) -> Fut + 'static,
    Fut: Future<Output = Result<AsyncPage<T>, String>> + 'static,


pub struct AsyncComboboxProps<T: Clone + PartialEq + 'static> {
    /// Where the options come from: [`use_async_options`].
    pub options: AsyncOptions<T>,

    /// The text shown for an option, searched by screen readers and shown in the input once
    /// it is picked.
    pub label: Callback<T, String>,

    /// What an option shows in the list. Defaults to its `label`.
    #[props(default)]
    pub render_option: Option<Callback<T, Element>>,

    /// The controlled value; `None` inside means nothing is picked.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// The initial value when uncontrolled.
    #[props(default)]
    pub default_value: Option<T>,

    /// Called when the user picks an option.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Called when the list opens or closes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub placeholder: ReadSignal<String>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(default)]
    pub list_aria_label: Option<String>,

    /// Shown when the source returned nothing. Defaults to
    /// [`UiStrings::no_results`](crate::components::ui_strings::UiStrings).
    #[props(default)]
    pub empty: Option<String>,

    /// Like [`Combobox`]: `id` and the `aria-*` form attributes go to the input, the rest
    /// to the root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A [`Combobox`] fed by an [`AsyncOptions`] source: debounced search, a loading state,
/// "Load more results", "Try again" after an error, and screen-reader announcements.
#[component]
pub fn AsyncCombobox<T: Clone + PartialEq + 'static>(props: AsyncComboboxProps<T>) -> Element

pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    #[props(default)]
    pub default_value: Option<T>,

    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    #[props(default)]
    pub default_query: ReadSignal<String>,

    #[props(default)]
    pub on_query_change: Callback<String>,

    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    #[props(default)]
    pub placeholder: ReadSignal<String>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(default)]
    pub list_aria_label: Option<String>,

    /// Marks the list `aria-busy` while its options are loading (see [`AsyncCombobox`]).
    #[props(default)]
    pub list_busy: ReadSignal<bool>,

    /// Text for a polite live region next to the input, such as "Loading results…" or
    /// "12 results". `None` (the default) renders no live region; pass `Some` from the first
    /// render (an empty string is fine) so screen readers notice later changes.
    #[props(default)]
    pub announcement: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element

#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element

#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(props: ComboboxOptionProps<T>) -> Element
```

## Example

```rust
use crate::components::combobox::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut query = use_signal(String::new);
    let frameworks: &[(&str, &str)] = &[
        ("next", "Next.js"),
        ("svelte", "SvelteKit"),
        ("nuxt", "Nuxt.js"),
        ("remix", "Remix"),
        ("astro", "Astro"),
        ("solid", "SolidStart"),
        ("dioxus", "Dioxus"),
    ];

    rsx! {
        Combobox::<String> {
            query: Some(query()),
            on_query_change: move |next| query.set(next),
            placeholder: "Select framework...",
            aria_label: "Select framework",
            list_aria_label: "Frameworks",
            ComboboxEmpty { "No framework found." }
            for (i , (value , label)) in frameworks.iter().enumerate() {
                ComboboxOption::<String> {
                    index: i,
                    value: value.to_string(),
                    text_value: label.to_string(),
                    {*label}
                }
            }
        }
    }
}
```
