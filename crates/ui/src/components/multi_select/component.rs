use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/multi_select/style.css")]
struct Styles;

#[derive(Clone, PartialEq, Debug)]
pub struct MultiOption {
    pub value: String,
    pub label: String,
}

impl MultiOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Button text for the current selection: the placeholder, the labels (up to two), or
/// "A, B +3".
pub fn summary(labels: &[&str], placeholder: &str) -> String {
    match labels {
        [] => placeholder.to_string(),
        [a] => (*a).to_string(),
        [a, b] => format!("{a}, {b}"),
        [a, b, rest @ ..] => format!("{a}, {b} +{}", rest.len()),
    }
}

/// Options whose label contains every word of the query, ignoring case.
pub fn filter_options<'a>(options: &'a [MultiOption], query: &str) -> Vec<&'a MultiOption> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    options
        .iter()
        .filter(|o| {
            let label = o.label.to_lowercase();
            words.iter().all(|w| label.contains(w))
        })
        .collect()
}

/// Pick several options from a list. A button shows the selection and opens a panel of native
/// checkboxes with a filter box. Escape or "Done" closes it and returns focus to the button.
#[component]
pub fn MultiSelect(
    options: Vec<MultiOption>,
    /// Accessible name, for example "Teams".
    #[props(into)]
    label: String,
    /// Shown when nothing is selected. Defaults to "Select…" ([`UiStrings`](crate::components::ui_strings::UiStrings)).
    #[props(into)]
    placeholder: Option<String>,
    /// Values selected at first.
    #[props(default)]
    default_selected: Vec<String>,
    #[props(default)] disabled: bool,
    /// Called with the selected values (in option order) after every change.
    #[props(default)]
    on_change: Callback<Vec<String>>,
) -> Element {
    let base = use_hook(|| {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        format!("dx-multi-{}", NEXT.fetch_add(1, Ordering::Relaxed))
    });
    let t = use_ui_strings();
    let placeholder = placeholder.unwrap_or_else(|| t.select_placeholder.clone());
    let mut open = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut selected =
        use_signal(move || default_selected.iter().cloned().collect::<BTreeSet<_>>());

    let ordered = {
        let set = selected.read();
        options
            .iter()
            .filter(|o| set.contains(&o.value))
            .map(|o| o.value.clone())
            .collect::<Vec<_>>()
    };
    let chosen_labels: Vec<&str> = options
        .iter()
        .filter(|o| selected.read().contains(&o.value))
        .map(|o| o.label.as_str())
        .collect();
    let button_text = summary(&chosen_labels, &placeholder);
    let visible = filter_options(&options, &query());
    let visible_count = visible.len();

    let button_id = format!("{base}-button");
    let panel_id = format!("{base}-panel");
    let mut button_el = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let close = Callback::new(move |()| {
        open.set(false);
        query.set(String::new());
        // Through the element handle rather than a script, so it also works on native.
        if let Some(button) = button_el() {
            spawn(async move {
                _ = button.set_focus(true).await;
            });
        }
    });

    rsx! {
        div {
            class: Styles::dx_multi_select,
            onkeydown: move |e: KeyboardEvent| {
                if open() && e.key() == Key::Escape {
                    e.prevent_default();
                    close.call(());
                }
            },
            button {
                id: "{button_id}",
                class: Styles::dx_multi_select_button,
                r#type: "button",
                disabled,
                "aria-expanded": "{open()}",
                "aria-controls": "{panel_id}",
                "aria-label": "{label}: {button_text}",
                onclick: move |_| open.set(!open()),
                onmounted: move |e: MountedEvent| button_el.set(Some(e.data())),
                span { class: Styles::dx_multi_select_text, "{button_text}" }
                span { "aria-hidden": "true", "▾" }
            }
            if open() {
                div {
                    class: Styles::dx_multi_select_backdrop,
                    "aria-hidden": "true",
                    onclick: move |_| close.call(()),
                }
                div {
                    id: "{panel_id}",
                    class: Styles::dx_multi_select_panel,
                    role: "group",
                    "aria-label": fill(&t.options_named, &[("label", &label)]),
                    input {
                        class: Styles::dx_multi_select_filter,
                        r#type: "search",
                        "aria-label": fill(&t.filter_named, &[("label", &label)]),
                        placeholder: t.filter.clone(),
                        value: "{query()}",
                        oninput: move |e: FormEvent| query.set(e.value()),
                    }
                    ul { class: Styles::dx_multi_select_list,
                        for option in visible {
                            li { key: "{option.value}",
                                label { class: Styles::dx_multi_select_option,
                                    input {
                                        r#type: "checkbox",
                                        checked: selected.read().contains(&option.value),
                                        onchange: {
                                            let value = option.value.clone();
                                            let options = options.clone();
                                            move |e: FormEvent| {
                                                selected
                                                    .with_mut(|set| {
                                                        if e.checked() {
                                                            set.insert(value.clone());
                                                        } else {
                                                            set.remove(&value);
                                                        }
                                                    });
                                                let set = selected.read();
                                                on_change
                                                    .call(
                                                        options
                                                            .iter()
                                                            .filter(|o| set.contains(&o.value))
                                                            .map(|o| o.value.clone())
                                                            .collect(),
                                                    );
                                            }
                                        },
                                    }
                                    "{option.label}"
                                }
                            }
                        }
                    }
                    p { class: Styles::dx_multi_select_status, role: "status",
                        if visible_count == 0 { {t.no_options.clone()} } else { {fill(&t.selected_count, &[("count", &ordered.len().to_string())])} }
                    }
                    div { class: Styles::dx_multi_select_footer,
                        button {
                            class: Styles::dx_multi_select_action,
                            r#type: "button",
                            onclick: move |_| {
                                selected.set(BTreeSet::new());
                                on_change.call(Vec::new());
                            },
                            {t.clear.clone()}
                        }
                        button {
                            class: Styles::dx_multi_select_action,
                            r#type: "button",
                            onclick: move |_| close.call(()),
                            {t.done.clone()}
                        }
                    }
                }
            }
        }
    }
}
