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
