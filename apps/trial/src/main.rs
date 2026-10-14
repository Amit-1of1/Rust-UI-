//! Taskboard: a small app that uses the kit end to end. It's the trial app and the Rust side of
//! the comparison in `bench/` (the same app is built in React and Svelte from `bench/SPEC.md`).
//!
//! `?rows=N` sets the number of generated tasks; `?bench` swaps in the js-framework-benchmark
//! style page that `bench/run.mjs` drives.
//!
//! Built with `--wasm-split --features wasm-split` (see bench/README.md), everything but the
//! Tasks screen is a `#[component(lazy)]` in its own wasm file: the Stats and Settings tabs and
//! the command menu download the first time they are shown, and the New task dialog right after
//! the board is up. Without that flag they are ordinary components.

use dioxus::prelude::*;
use std::rc::Rc;
use ui::prelude::*;

const TRIAL_CSS: Asset = asset!("/assets/trial.css");

fn main() {
    if query().contains("bench") {
        dioxus::launch(Bench);
    } else {
        dioxus::launch(App);
    }
}

// ----- data (bench/SPEC.md) ----------------------------------------------------------------

const ADJ: [&str; 10] = [
    "Quick", "Lazy", "Bright", "Quiet", "Brave", "Calm", "Eager", "Fancy", "Gentle", "Happy",
];
const NOUN: [&str; 10] = [
    "report", "review", "deploy", "design", "fix", "test", "sync", "plan", "audit", "launch",
];
const NAMES: [&str; 5] = ["Ada", "Grace", "Linus", "Margaret", "Alan"];
const PRIORITIES: [&str; 3] = ["Low", "Medium", "High"];
const STATUSES: [&str; 4] = ["Todo", "Doing", "Done", "Done"];

#[derive(Clone, PartialEq)]
struct Task {
    id: usize,
    title: String,
    assignee: String,
    priority: String,
    status: String,
    estimate: usize,
}

fn task(i: usize) -> Task {
    Task {
        id: i + 1,
        title: format!("{} {} #{}", ADJ[i % 10], NOUN[(i * 7) % 10], i + 1),
        assignee: NAMES[i % 5].into(),
        priority: PRIORITIES[i % 3].into(),
        status: STATUSES[i % 4].into(),
        estimate: (i * 37) % 13 + 1,
    }
}

// ----- browser glue ------------------------------------------------------------------------

fn query() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.location().search().ok())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    String::new()
}

fn rows_param() -> usize {
    query()
        .trim_start_matches('?')
        .split('&')
        .find_map(|kv| kv.strip_prefix("rows=")?.parse().ok())
        .unwrap_or(200)
}

/// `document.body.dataset.ready = "1"`: the bench scripts wait for it.
fn mark_ready() {
    #[cfg(target_arch = "wasm32")]
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        let _ = body.dataset().set("ready", "1");
    }
}

fn apply_theme(dark: bool) {
    #[cfg(target_arch = "wasm32")]
    if let Some(root) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        let _ = root.set_attribute("data-theme", if dark { "dark" } else { "light" });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = dark;
}

// ----- app ---------------------------------------------------------------------------------

#[component]
fn App() -> Element {
    // The kit's own words, renamed for this app: the filter box is about tasks.
    let strings = use_signal(|| UiStrings {
        filter_rows: "Filter tasks".into(),
        ..UiStrings::default()
    });
    use_effect(mark_ready);
    rsx! {
        ThemeStyles {}
        document::Stylesheet { href: TRIAL_CSS }
        UiStringsProvider { strings,
            ToastProvider { Board {} }
        }
    }
}

#[component]
fn Board() -> Element {
    let mut tasks = use_signal(|| (0..rows_param()).map(task).collect::<Vec<_>>());
    let mut tab = use_signal(|| "tasks".to_string());
    let mut dark = use_signal(|| false);
    let mut palette = use_signal(|| false);
    let mut new_open = use_signal(|| false);
    let mut page_size = use_signal(|| 20usize);
    use_hotkey("k", move || palette.set(true));
    // The command menu mounts on first open, so its code downloads then, and stays mounted so
    // closing still animates. The New task dialog's code downloads in the background as soon
    // as the board is up. A dialog that mounts already open misses its focus trap, so each one
    // stays closed until it has mounted (`on_ready`).
    let mut palette_seen = use_signal(|| false);
    let mut palette_ready = use_signal(|| false);
    let mut new_ready = use_signal(|| false);
    use_effect(move || {
        if palette() {
            palette_seen.set(true);
        }
    });

    let mut toggle_dark = move || {
        let v = !dark();
        dark.set(v);
        apply_theme(v);
    };

    rsx! {
        header { class: "tb-header",
            h1 { "Taskboard" }
            div { class: "tb-tools",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm,
                    onclick: move |_| palette.set(true),
                    "Command menu (Ctrl+K)"
                }
                Switch {
                    checked: dark(),
                    aria_label: "Dark mode",
                    on_checked_change: move |v| {
                        dark.set(v);
                        apply_theme(v);
                    },
                }
            }
        }
        main { class: "tb-main",
            Tabs {
                value: Some(tab()),
                on_value_change: move |v| tab.set(v),
                horizontal: true,
                TabList {
                    TabTrigger { value: "tasks".to_string(), index: 0usize, "Tasks" }
                    TabTrigger { value: "stats".to_string(), index: 1usize, "Stats" }
                    TabTrigger { value: "settings".to_string(), index: 2usize, "Settings" }
                }
                TabContent { index: 0usize, value: "tasks".to_string(),
                    TasksTab { tasks: tasks(), page_size: page_size(), on_new: move |_| new_open.set(true) }
                }
                TabContent { index: 1usize, value: "stats".to_string(),
                    SuspenseBoundary { fallback: |_| rsx! { Spinner {} },
                        StatsTab { tasks: tasks() }
                    }
                }
                TabContent { index: 2usize, value: "settings".to_string(),
                    SuspenseBoundary { fallback: |_| rsx! { Spinner {} },
                        SettingsTab { page_size: page_size(), on_page_size: move |n| page_size.set(n) }
                    }
                }
            }
        }
        SuspenseBoundary { fallback: |_| rsx! {},
            NewTask {
                open: new_open() && new_ready(),
                on_open_change: move |v| new_open.set(v),
                on_ready: move |_| new_ready.set(true),
                next_id: tasks.read().len() + 1,
                on_save: move |t: Task| tasks.write().insert(0, t),
            }
        }
        if palette_seen() {
            SuspenseBoundary { fallback: |_| rsx! {},
                Palette {
                    open: palette() && palette_ready(),
                    on_open_change: move |v| palette.set(v),
                    on_ready: move |_| palette_ready.set(true),
                    on_select: move |id: String| match id.as_str() {
                        "new" => {
                            tab.set("tasks".into());
                            new_open.set(true);
                        }
                        "theme" => toggle_dark(),
                        other => tab.set(other.into()),
                    },
                }
            }
        }
    }
}

#[component(lazy)]
fn Palette(
    open: bool,
    on_open_change: EventHandler<bool>,
    on_ready: EventHandler<()>,
    on_select: EventHandler<String>,
) -> Element {
    use_effect(move || on_ready.call(()));
    let items = vec![
        CommandItem::new("new", "New task"),
        CommandItem::new("theme", "Toggle dark mode"),
        CommandItem::new("stats", "Go to Stats"),
        CommandItem::new("settings", "Go to Settings"),
    ];
    rsx! {
        CommandDialog {
            open,
            on_open_change: move |v| on_open_change.call(v),
            items,
            on_select: move |id| on_select.call(id),
        }
    }
}

#[component]
fn TasksTab(tasks: Vec<Task>, page_size: usize, on_new: EventHandler<()>) -> Element {
    let columns = vec![
        DataColumn::new("Title").sortable(),
        DataColumn::new("Assignee").sortable(),
        DataColumn::new("Priority").sortable(),
        DataColumn::new("Status"),
        DataColumn::new("Estimate").sortable().numeric(),
    ];
    let rows: Vec<Vec<String>> = tasks
        .iter()
        .map(|t| {
            vec![
                t.title.clone(),
                t.assignee.clone(),
                t.priority.clone(),
                t.status.clone(),
                format!("{}h", t.estimate),
            ]
        })
        .collect();
    rsx! {
        div { class: "tb-toolbar",
            Button { id: "new-task", onclick: move |_| on_new.call(()), "New task" }
        }
        DataTable { columns, rows, caption: "Tasks", page_size, selectable: true }
    }
}

#[component(lazy)]
fn NewTask(
    open: bool,
    on_open_change: EventHandler<bool>,
    on_ready: EventHandler<()>,
    next_id: usize,
    on_save: EventHandler<Task>,
) -> Element {
    use_effect(move || on_ready.call(()));
    let form = use_form();
    let title = form.field(vec![custom(|v| {
        if v.trim().is_empty() {
            Err("Title is required".into())
        } else {
            Ok(())
        }
    })]);
    let mut assignee = use_signal(|| NAMES[0].to_string());
    let mut priority = use_signal(|| "Medium".to_string());
    let toasts = use_toast();

    let names = NAMES.iter().enumerate().map(|(i, n)| {
        rsx! {
            SelectOption::<String> { index: i, value: n.to_string(), text_value: "{n}", "{n}" }
        }
    });

    rsx! {
        Dialog { open, on_open_change: move |v| on_open_change.call(v),
            DialogTitle { "New task" }
            DialogDescription { "Add a task to the top of the board." }
            Form {
                state: form,
                on_valid: move |_| {
                    on_save.call(Task {
                        id: next_id,
                        title: title.value(),
                        assignee: assignee(),
                        priority: priority(),
                        status: "Todo".into(),
                        estimate: 1,
                    });
                    form.reset();
                    on_open_change.call(false);
                    toasts.success("Task added".to_string(), ToastOptions::new());
                },
                TextField { field: title, label: "Title", autocomplete: "off" }
                div { class: "tb-field",
                    span { id: "assignee-label", class: "tb-label", "Assignee" }
                    div { role: "group", "aria-labelledby": "assignee-label",
                        Select::<String> {
                            default_value: Some(NAMES[0].to_string()),
                            on_value_change: move |v: Option<String>| {
                                if let Some(v) = v {
                                    assignee.set(v);
                                }
                            },
                            {names}
                        }
                    }
                }
                div { class: "tb-field",
                    span { id: "priority-label", class: "tb-label", "Priority" }
                    RadioGroup {
                        "aria-labelledby": "priority-label",
                        value: Some(priority()),
                        on_value_change: move |v| priority.set(v),
                        horizontal: true,
                        for (i , p) in PRIORITIES.iter().enumerate() {
                            RadioItem { value: p.to_string(), index: i, "{p}" }
                        }
                    }
                }
                div { class: "tb-actions",
                    Button { r#type: "button", variant: ButtonVariant::Outline,
                        onclick: move |_| on_open_change.call(false),
                        "Cancel"
                    }
                    Button { r#type: "submit", "Save" }
                }
            }
        }
    }
}

#[component(lazy)]
fn StatsTab(tasks: Vec<Task>) -> Element {
    let count = |s: &str| tasks.iter().filter(|t| t.status == s).count() as f32;
    let data = vec![
        ChartDatum::new("Todo", count("Todo")),
        ChartDatum::new("Doing", count("Doing")),
        ChartDatum::new("Done", count("Done")),
    ];
    let done = if tasks.is_empty() {
        0.0
    } else {
        (f64::from(count("Done")) / tasks.len() as f64 * 100.0).round()
    };
    rsx! {
        div { class: "tb-stats",
            BarChart { data, label: "Tasks per status" }
            div { class: "tb-field",
                span { id: "done-label", class: "tb-label", "Done: {done}%" }
                Progress { "aria-labelledby": "done-label", value: done }
            }
        }
    }
}

#[component(lazy)]
fn SettingsTab(page_size: usize, on_page_size: EventHandler<usize>) -> Element {
    rsx! {
        div { class: "tb-settings",
            div { class: "tb-field",
                span { class: "tb-label", "Rows per page: {page_size}" }
                Slider {
                    label: "Rows per page",
                    min: 10.0,
                    max: 50.0,
                    step: 10.0,
                    default_value: page_size as f64,
                    on_value_change: move |v: f64| on_page_size.call(v as usize),
                }
            }
            div { class: "tb-field",
                Label { html_for: "notes", "Notes" }
                Textarea { id: "notes", autosize: true, placeholder: "Anything to remember" }
            }
            ThemeCustomizer {}
        }
    }
}

// ----- bench page (js-framework-benchmark style) --------------------------------------------

#[derive(Clone, PartialEq)]
struct BenchItem {
    id: usize,
    title: Rc<str>,
}

#[component]
fn Bench() -> Element {
    rsx! {
        SuspenseBoundary { fallback: |_| rsx! {}, BenchPage {} }
    }
}

#[component(lazy)]
fn BenchPage() -> Element {
    let mut rows = use_signal(Vec::<BenchItem>::new);
    let mut next = use_signal(|| 0usize);
    let selected = use_selection::<usize>();
    use_effect(mark_ready);

    let mut build = move |n: usize| {
        let start = next();
        next.set(start + n);
        (start..start + n)
            .map(|i| BenchItem {
                id: i + 1,
                title: task(i).title.into(),
            })
            .collect::<Vec<_>>()
    };

    rsx! {
        ThemeStyles {}
        document::Stylesheet { href: TRIAL_CSS }
        div { class: "bench-buttons",
            button { id: "run", onclick: move |_| { let r = build(1_000); rows.set(r); }, "Create 1,000 rows" }
            button { id: "runlots", onclick: move |_| { let r = build(10_000); rows.set(r); }, "Create 10,000 rows" }
            button { id: "add", onclick: move |_| { let r = build(1_000); rows.write().extend(r); }, "Append 1,000 rows" }
            button { id: "update",
                onclick: move |_| {
                    for t in rows.write().iter_mut().step_by(10) {
                        t.title = format!("{} !!!", t.title).into();
                    }
                },
                "Update every 10th row"
            }
            button { id: "swaprows",
                onclick: move |_| {
                    let mut r = rows.write();
                    if r.len() > 998 {
                        r.swap(1, 998);
                    }
                },
                "Swap rows"
            }
            button { id: "clear", onclick: move |_| rows.write().clear(), "Clear" }
        }
        table { id: "bench",
            tbody {
                for t in rows.read().iter() {
                    BenchRow { key: "{t.id}", id: t.id, title: t.title.clone(), selected, rows }
                }
            }
        }
    }
}

/// One bench row. Every prop compares cheaply and stays equal unless this row changed,
/// so when the list re-renders Dioxus skips the rows that didn't change, and the
/// selection only touches the two rows that flip.
#[component]
fn BenchRow(
    id: usize,
    title: Rc<str>,
    selected: Selection<usize>,
    rows: Signal<Vec<BenchItem>>,
) -> Element {
    let is_selected = use_selected(selected, id);
    rsx! {
        tr { class: if is_selected() { "selected" },
            td { "{id}" }
            td { onclick: move |_| selected.select_only(id), "{title}" }
            td {
                button {
                    "aria-label": "Remove row {id}",
                    onclick: move |_| rows.write().retain(|t| t.id != id),
                    "×"
                }
            }
        }
    }
}
