//! Words the kit itself shows or announces ("Next", "Clear", "No results found."), so an
//! app can translate them in one place.
//!
//! ```rust,ignore
//! let strings = use_signal(|| UiStrings { next: "Suivant".into(), ..UiStrings::default() });
//! rsx! { UiStringsProvider { strings, App {} } }
//! ```
//!
//! Without a provider every component uses [`UiStrings::default`] (English). Text the app
//! passes in (labels, placeholders given as props) always wins over these. Templates use
//! `{name}` placeholders, filled by [`fill`]. Calendar weekday names have their own
//! `day_name` prop; month names come from dioxus-primitives.

use dioxus::prelude::*;

/// Every string the kit renders on its own. Fields are grouped by component.
#[derive(Clone, Debug, PartialEq)]
pub struct UiStrings {
    // Shared
    pub previous: String,
    pub next: String,
    pub loading: String,
    pub filter: String,
    /// Name of icon-only close buttons (sheet).
    pub close: String,
    // Pagination
    pub go_to_previous_page: String,
    pub go_to_next_page: String,
    // Data table
    pub filter_rows: String,
    pub select_all_rows: String,
    /// `{row}` is the row's first cell.
    pub select_row: String,
    pub no_rows: String,
    pub table_pages: String,
    /// `{count}` is the number of rows.
    pub row_count: String,
    /// `{first}`, `{last}` and `{total}` are row numbers.
    pub showing_rows: String,
    /// `{page}` and `{pages}` are page numbers.
    pub page_of: String,
    // Multi-select
    pub select_placeholder: String,
    /// `{label}` is the multi-select's name.
    pub filter_named: String,
    /// `{label}` is the multi-select's name.
    pub options_named: String,
    pub no_options: String,
    /// `{count}` is the number of selected options.
    pub selected_count: String,
    pub clear: String,
    pub done: String,
    // Command palette
    pub command_palette: String,
    pub command_help: String,
    pub search_commands: String,
    pub search_commands_placeholder: String,
    pub commands: String,
    pub no_results: String,
    // One-time code
    pub otp_label: String,
    /// `{label}` is the code's name, `{n}` the box number and `{total}` the box count.
    pub otp_character: String,
    // Marquee
    pub play: String,
    pub pause: String,
    pub scrolling_content: String,
    /// Read after Play or Pause by screen readers.
    pub scrolling: String,
    // Dropzone
    pub drop_files: String,
    pub selected_files: String,
    pub only_one_file: String,
    /// `{types}` is the accepted file types.
    pub accepts: String,
    /// `{name}` is the file name.
    pub not_accepted: String,
    /// `{name}` is the file name, `{size}` the limit.
    pub too_large: String,
    // Navigation
    pub breadcrumb: String,
    pub main_navigation: String,
    pub toggle_sidebar: String,
    pub sidebar: String,
    pub sidebar_description: String,
    pub sortable_list: String,
    /// `{label}` is the carousel's name.
    pub slides: String,
    // Date picker
    pub show_calendar: String,
    // Time picker
    pub time: String,
    pub date_and_time: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    /// Name of the AM/PM segment.
    pub day_period: String,
    pub am: String,
    pub pm: String,
    /// Read for a time segment that has no value yet.
    pub empty_value: String,
    // Chart
    pub no_data: String,
    /// Opens a chart's data table.
    pub show_data: String,
    /// Header of a chart table's first column.
    pub chart_category: String,
    /// Header of a single-series chart's value column.
    pub chart_value: String,
    /// Header of a pie chart's percentage column.
    pub chart_share: String,
    /// Radial chart summary. `{value}`, `{max}` and `{percent}` are filled in.
    pub value_of_max: String,
    /// Read when a keyboard-explorable chart gets focus.
    pub chart_keys_hint: String,
    /// The focused point of a chart. `{point}`, `{index}` and `{count}` are filled in.
    pub chart_point: String,
    /// Read for a time picker whose time is outside `min`/`max`. `{min}` and `{max}` are
    /// filled in.
    pub time_out_of_range: String,
    // Link
    /// Read after a link that opens in a new tab.
    pub opens_in_new_tab: String,
    // Skip link
    pub skip_to_content: String,
    // Chat
    /// The chat log's button when the reader has scrolled up.
    pub jump_to_latest: String,
    /// The same button once a message arrived below.
    pub new_messages: String,
    /// Typing indicator. `{author}` is who is typing.
    pub typing: String,
    /// Announced when a streamed reply ends. `{author}` and `{text}` are filled in.
    pub message_from: String,
    /// Read after the composer's name: how to send.
    pub composer_hint: String,
    pub send: String,
    pub stop: String,
    // Tool calls
    pub tool_queued: String,
    pub tool_running: String,
    pub tool_succeeded: String,
    pub tool_failed: String,
    pub tool_needs_approval: String,
    /// Announced when a tool call fails or needs approval. `{name}` and `{status}`.
    pub tool_status: String,
    // Number field
    /// The - button's name. `{label}` is the field's name.
    pub decrease: String,
    /// The + button's name. `{label}` is the field's name.
    pub increase: String,
}

impl Default for UiStrings {
    fn default() -> Self {
        let s = |v: &str| v.to_string();
        Self {
            previous: s("Previous"),
            next: s("Next"),
            loading: s("Loading"),
            filter: s("Filter"),
            go_to_previous_page: s("Go to previous page"),
            go_to_next_page: s("Go to next page"),
            filter_rows: s("Filter rows"),
            select_all_rows: s("Select all rows on this page"),
            select_row: s("Select {row}"),
            no_rows: s("No rows match."),
            table_pages: s("Table pages"),
            row_count: s("{count} rows"),
            showing_rows: s("Showing {first}–{last} of {total} rows"),
            page_of: s("Page {page} of {pages}"),
            select_placeholder: s("Select…"),
            filter_named: s("Filter {label}"),
            options_named: s("{label} options"),
            no_options: s("No options match."),
            selected_count: s("{count} selected"),
            clear: s("Clear"),
            done: s("Done"),
            close: s("Close"),
            command_palette: s("Command palette"),
            command_help: s("Search for a command and press Enter to run it."),
            search_commands: s("Search commands"),
            search_commands_placeholder: s("Search commands..."),
            commands: s("Commands"),
            no_results: s("No results found."),
            otp_label: s("One-time code"),
            otp_character: s("{label}, character {n} of {total}"),
            play: s("Play"),
            pause: s("Pause"),
            scrolling_content: s("Scrolling content"),
            scrolling: s(" scrolling"),
            drop_files: s("Drop files here or click to browse"),
            selected_files: s("Selected files"),
            only_one_file: s("Only one file can be added"),
            accepts: s("Accepts {types}"),
            not_accepted: s("{name} is not an accepted type"),
            too_large: s("{name} is larger than {size}"),
            breadcrumb: s("Breadcrumb"),
            main_navigation: s("Main"),
            toggle_sidebar: s("Toggle Sidebar"),
            sidebar: s("Sidebar"),
            sidebar_description: s("Displays the mobile sidebar."),
            sortable_list: s("Sortable list"),
            slides: s("{label} slides"),
            show_calendar: s("Show Calendar"),
            time: s("Time"),
            date_and_time: s("Date and time"),
            hours: s("Hours"),
            minutes: s("Minutes"),
            seconds: s("Seconds"),
            day_period: s("AM/PM"),
            am: s("AM"),
            pm: s("PM"),
            empty_value: s("Empty"),
            no_data: s("No data."),
            show_data: s("Show data"),
            chart_category: s("Category"),
            chart_value: s("Value"),
            chart_share: s("Share"),
            value_of_max: s("{value} of {max} ({percent})"),
            chart_keys_hint: s("Use the arrow keys to move between points."),
            chart_point: s("{point}. {index} of {count}."),
            time_out_of_range: s("Choose a time from {min} to {max}."),
            opens_in_new_tab: s("(opens in a new tab)"),
            skip_to_content: s("Skip to content"),
            jump_to_latest: s("Jump to latest"),
            new_messages: s("New messages below"),
            typing: s("{author} is typing"),
            message_from: s("{author}: {text}"),
            composer_hint: s("Enter sends, Shift+Enter adds a line."),
            send: s("Send"),
            stop: s("Stop"),
            tool_queued: s("Queued"),
            tool_running: s("Running"),
            tool_succeeded: s("Done"),
            tool_failed: s("Failed"),
            tool_needs_approval: s("Needs approval"),
            tool_status: s("{name}: {status}"),
            decrease: s("Decrease {label}"),
            increase: s("Increase {label}"),
        }
    }
}

/// Provides `strings` to every kit component below it. Pass a signal to switch language
/// at runtime.
#[component]
pub fn UiStringsProvider(strings: ReadSignal<UiStrings>, children: Element) -> Element {
    use_context_provider(|| strings);
    rsx! {
        {children}
    }
}

/// The strings from the nearest [`UiStringsProvider`], or English.
pub fn use_ui_strings() -> UiStrings {
    try_use_context::<ReadSignal<UiStrings>>()
        .map(|s| s.read().clone())
        .unwrap_or_default()
}

/// Replaces each `{name}` in `template` with its value.
///
/// ```
/// assert_eq!(ui::components::ui_strings::fill("Select {row}", &[("row", "Ada")]), "Select Ada");
/// ```
pub fn fill(template: &str, values: &[(&str, &str)]) -> String {
    values
        .iter()
        .fold(template.to_string(), |out, (name, value)| {
            out.replace(&format!("{{{name}}}"), value)
        })
}
