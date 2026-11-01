### Breaking

- **ui:** `ComboboxProps` and `UiStrings` have new fields, so code that builds either with a struct literal and no `..` must add them. `rsx!` users and `..UiStrings::default()` users are unaffected. (#92)

### Added

- **ui:** `AsyncCombobox` and `use_async_options`: a combobox whose options come from an async `fetch(query, page)`. Typing is debounced (250 ms, `set_debounce`), each request cancels the previous one and late answers are dropped, the list shows a loading line and is `aria-busy` while the first page loads, "Load more results" is a real option (Enter or click; the list stays open and the highlight lands on the first new result), a failed request offers "Try again", and a polite live region announces loading, the result count, "no results" and errors. The picked label stays on the input when later results no longer contain it (experimental). (#92)
- **ui:** `Combobox` takes `list_busy` (marks the list `aria-busy`) and `announcement` (a polite live region next to the input), for apps that load options themselves (experimental). (#92)
- **ui:** New `UiStrings` fields `loading_results`, `loading_more`, `load_more`, `one_result`, `result_count`, `results_failed` and `try_again`, translated for every bundled locale. (#92)
