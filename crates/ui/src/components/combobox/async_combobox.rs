//! A combobox whose options come from an async source (a server search, a database): results
//! load as the user types (debounced), a "Load more results" option fetches the next page,
//! and a status line tells screen readers when results are loading, how many came back, or
//! that the request failed.
//!
//! The styled [`Combobox`] and [`ComboboxOption`] do the rendering, so the markup, keyboard
//! model and look are the same as the plain combobox. "Load more results" and "Try again" are
//! real options: they are reached with the arrow keys and picked with Enter like any other.

use super::component::{Combobox, ComboboxEmpty, ComboboxOption, Styles};
use crate::components::hooks::use_unique_id;
use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::core::Task;
use dioxus::prelude::*;
use dioxus_sdk_time::sleep;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

/// One page of results from an async source.
#[derive(Clone, Debug, PartialEq)]
pub struct AsyncPage<T> {
    /// The options on this page, in the order they should be shown.
    pub items: Vec<T>,
    /// Whether a later page exists. Shows the "Load more results" option.
    pub has_more: bool,
}

impl<T> AsyncPage<T> {
    /// The last page: `items` and nothing after them.
    pub fn last(items: Vec<T>) -> Self {
        Self {
            items,
            has_more: false,
        }
    }
}

/// Where an [`AsyncOptions`] source is.
#[derive(Clone, Debug, PartialEq, Eq)]
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

type PageFuture<T> = Pin<Box<dyn Future<Output = Result<AsyncPage<T>, String>>>>;

/// How long [`use_async_options`] waits after the last keystroke before it asks for results.
pub const DEFAULT_DEBOUNCE: Duration = Duration::from_millis(250);

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

impl<T: 'static> Clone for AsyncOptions<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for AsyncOptions<T> {}

impl<T: 'static> PartialEq for AsyncOptions<T> {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query && self.items == other.items && self.request == other.request
    }
}

impl<T: Clone + 'static> AsyncOptions<T> {
    /// The current query.
    pub fn query(&self) -> String {
        self.query.cloned()
    }

    /// The options loaded so far for the current query (every page, in order).
    pub fn items(&self) -> Vec<T> {
        self.items.cloned()
    }

    /// The options as a signal, for reading without cloning.
    pub fn items_signal(&self) -> ReadSignal<Vec<T>> {
        self.items.into()
    }

    /// Where the source is.
    pub fn status(&self) -> AsyncStatus {
        self.status.cloned()
    }

    /// Whether any request is on its way.
    pub fn is_loading(&self) -> bool {
        matches!(
            *self.status.read(),
            AsyncStatus::Loading | AsyncStatus::LoadingMore
        )
    }

    /// Whether the source said another page exists.
    pub fn has_more(&self) -> bool {
        (self.has_more)()
    }

    /// The last request's error, if it failed.
    pub fn error(&self) -> Option<String> {
        match &*self.status.read() {
            AsyncStatus::Failed(message) => Some(message.clone()),
            _ => None,
        }
    }

    /// Sets the query and asks for its first page after the debounce. Typing again before
    /// the debounce ends restarts the wait, so only the last query is fetched.
    pub fn search(&self, query: String) {
        self.set_query(query, true);
    }

    /// Like [`search`](Self::search) without the debounce: for a query that wasn't typed,
    /// such as the empty query [`AsyncCombobox`] sets when its list opens.
    pub fn search_now(&self, query: String) {
        self.set_query(query, false);
    }

    fn set_query(&self, query: String, debounced: bool) {
        let mut q = self.query;
        if *q.peek() == query && *self.status.peek() != AsyncStatus::Idle {
            return;
        }
        q.set(query);
        self.request.call((0, debounced));
    }

    /// Asks for the first page of the current query now, if nothing has been requested yet.
    /// [`AsyncCombobox`] calls this when its list opens.
    pub fn load(&self) {
        if *self.status.peek() == AsyncStatus::Idle {
            self.request.call((0, false));
        }
    }

    /// Asks for the next page now. Does nothing while a request is on its way or when the
    /// source has no more pages.
    pub fn load_more(&self) {
        let busy = matches!(
            *self.status.peek(),
            AsyncStatus::Loading | AsyncStatus::LoadingMore
        );
        if !busy && *self.has_more.peek() {
            self.request.call((*self.next_page.peek(), false));
        }
    }

    /// Repeats the request that failed (or reloads the first page).
    pub fn retry(&self) {
        let page = if self.items.peek().is_empty() {
            0
        } else {
            *self.next_page.peek()
        };
        self.request.call((page, false));
    }

    /// Changes the wait after the last keystroke ([`DEFAULT_DEBOUNCE`] by default).
    pub fn set_debounce(&self, debounce: Duration) {
        let mut d = self.debounce;
        d.set(debounce);
    }
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
{
    let fetch = use_callback(move |(query, page): (String, usize)| -> PageFuture<T> {
        Box::pin(fetch(query, page))
    });
    let query = use_signal(String::new);
    let mut items = use_signal(Vec::<T>::new);
    let mut status = use_signal(|| AsyncStatus::Idle);
    let mut has_more = use_signal(|| false);
    let mut next_page = use_signal(|| 0usize);
    let debounce = use_signal(|| DEFAULT_DEBOUNCE);
    let mut generation = use_signal(|| 0u64);
    let mut task = use_signal(|| None::<Task>);

    let request = use_callback(move |(page, debounced): (usize, bool)| {
        if let Some(previous) = task.write().take() {
            previous.cancel();
        }
        let id = *generation.peek() + 1;
        generation.set(id);
        status.set(if page == 0 {
            AsyncStatus::Loading
        } else {
            AsyncStatus::LoadingMore
        });
        let wait = if debounced {
            *debounce.peek()
        } else {
            Duration::ZERO
        };
        let handle = spawn(async move {
            if !wait.is_zero() {
                sleep(wait).await;
                if *generation.peek() != id {
                    return;
                }
            }
            let q = query.peek().clone();
            let result = fetch.call((q, page)).await;
            // A newer request started while this one was out: its answer wins.
            if *generation.peek() != id {
                return;
            }
            match result {
                Ok(found) => {
                    if page == 0 {
                        items.set(found.items);
                    } else {
                        items.write().extend(found.items);
                    }
                    has_more.set(found.has_more);
                    next_page.set(page + 1);
                    status.set(AsyncStatus::Ready);
                }
                Err(message) => status.set(AsyncStatus::Failed(message)),
            }
            task.set(None);
        });
        task.set(Some(handle));
    });

    AsyncOptions {
        query,
        items,
        status,
        has_more,
        next_page,
        debounce,
        request,
    }
}

/// What an option of the inner combobox stands for.
#[derive(Clone, PartialEq)]
enum Row<T> {
    Item(T),
    LoadMore,
    Retry,
}

/// Index of the hidden option that keeps the selected label on the input while the selected
/// item is not among the loaded results. Far past any page.
const PINNED_INDEX: usize = usize::MAX / 2;

#[derive(Props, Clone, PartialEq)]
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
pub fn AsyncCombobox<T: Clone + PartialEq + 'static>(props: AsyncComboboxProps<T>) -> Element {
    let t = use_ui_strings();
    let base_id = use_unique_id("rui-async-combobox");
    let options = props.options;
    let label = props.label;
    let controlled = props.value;
    let on_value_change = props.on_value_change;
    let on_open_change = props.on_open_change;

    let mut own_value = use_signal(|| props.default_value.clone());
    let selected = use_memo(move || match controlled {
        Some(value) => value.cloned(),
        None => own_value.cloned(),
    });
    let row_value = use_memo(move || selected().map(Row::Item));

    let mut open = use_signal(|| false);
    // Picking "Load more" or "Try again" makes the inner combobox close the list; this
    // swallows that one close so the list stays open on the new results.
    let mut keep_open = use_signal(|| false);

    let status = options.status();
    let items = options.items();
    let loading_first = status == AsyncStatus::Loading;
    let loading_more = status == AsyncStatus::LoadingMore;
    let failed = matches!(status, AsyncStatus::Failed(_));
    let count = items.len();
    let mut rows: Vec<Row<T>> = items.iter().cloned().map(Row::Item).collect();
    if failed {
        rows.push(Row::Retry);
    } else if options.has_more() {
        rows.push(Row::LoadMore);
    }
    let selected_now = selected();
    let pin = (!open())
        .then_some(selected_now)
        .flatten()
        .filter(|s| !items.contains(s));

    let announcement = if !open() {
        String::new()
    } else {
        match &status {
            AsyncStatus::Idle => String::new(),
            AsyncStatus::Loading => t.loading_results.clone(),
            AsyncStatus::LoadingMore => t.loading_more.clone(),
            AsyncStatus::Failed(_) => t.results_failed.clone(),
            AsyncStatus::Ready if count == 0 => t.no_results.clone(),
            AsyncStatus::Ready if count == 1 => t.one_result.clone(),
            AsyncStatus::Ready => fill(&t.result_count, &[("count", &count.to_string())]),
        }
    };
    let empty = props.empty.clone().unwrap_or(t.no_results.clone());

    rsx! {
        Combobox::<Row<T>> {
            value: Some(row_value.into()),
            on_value_change: move |row: Option<Row<T>>| match row {
                Some(Row::Item(item)) => {
                    own_value.set(Some(item.clone()));
                    on_value_change.call(Some(item));
                }
                Some(Row::LoadMore) => {
                    keep_open.set(true);
                    options.load_more();
                }
                Some(Row::Retry) => {
                    keep_open.set(true);
                    options.retry();
                }
                None => {
                    own_value.set(None);
                    on_value_change.call(None);
                }
            },
            open: Some(open()),
            on_open_change: move |next: bool| {
                if !next && *keep_open.peek() {
                    keep_open.set(false);
                    return;
                }
                open.set(next);
                if next {
                    options.load();
                }
                on_open_change.call(next);
            },
            query: Some(options.query()),
            on_query_change: move |q: String| {
                // Opening the list resets the query to "" before it opens: load that at once.
                // Only typing waits for the debounce.
                if q.is_empty() && !*open.peek() {
                    options.search_now(q);
                } else {
                    options.search(q);
                }
            },
            // The source already filtered on the server.
            filter: |_: (String, String)| true,
            disabled: props.disabled,
            placeholder: props.placeholder,
            aria_label: props.aria_label.clone(),
            list_aria_label: props.list_aria_label.clone(),
            list_busy: loading_first || loading_more,
            announcement,
            attributes: props.attributes.clone(),
            // One keyed list for results and the trailing "Load more" or "Try again", so the
            // option at a given position is the same component across pages. The highlight
            // follows an option's id: "Load more" at position n has the id the first result
            // of the next page gets, and when that page arrives the same option turns into
            // that result, so the highlight lands on it. "Load more" stays enabled while
            // loading for the same reason (a disabled option loses the highlight); picking it
            // again then does nothing.
            for (i, row) in rows.into_iter().enumerate() {
                ComboboxOption::<Row<T>> {
                    key: "{i}",
                    index: i,
                    id: format!("{base_id}-{i}"),
                    value: row.clone(),
                    text_value: match &row {
                        Row::Item(item) => label.call(item.clone()),
                        Row::LoadMore if loading_more => t.loading_more.clone(),
                        Row::LoadMore => t.load_more.clone(),
                        Row::Retry => t.try_again.clone(),
                    },
                    class: if matches!(row, Row::Item(_)) { None } else { Some(Styles::dx_combobox_more.to_string()) },
                    {row_content(props.render_option, label, row, loading_more, &t)}
                }
            }
            if loading_first {
                div { role: "presentation", class: Styles::dx_combobox_status,
                    {t.loading_results.clone()}
                }
            } else if !failed && status != AsyncStatus::Idle {
                ComboboxEmpty { {empty} }
            }
            if let Some(item) = pin {
                ComboboxOption::<Row<T>> {
                    index: PINNED_INDEX,
                    value: Row::Item(item.clone()),
                    text_value: label.call(item),
                    disabled: true,
                    hidden: true,
                    ""
                }
            }
        }
    }
}

fn row_content<T: Clone + 'static>(
    render: Option<Callback<T, Element>>,
    label: Callback<T, String>,
    row: Row<T>,
    loading_more: bool,
    t: &crate::components::ui_strings::UiStrings,
) -> Element {
    match row {
        Row::Item(item) => match render {
            Some(render) => render.call(item),
            None => rsx! {
                {label.call(item)}
            },
        },
        Row::LoadMore if loading_more => rsx! {
            {t.loading_more.clone()}
        },
        Row::LoadMore => rsx! {
            {t.load_more.clone()}
        },
        // The error and the way out in one option, so a screen reader reads both.
        Row::Retry => rsx! {
            span { class: Styles::dx_combobox_error, {t.results_failed.clone()} }
            " "
            {t.try_again.clone()}
        },
    }
}
