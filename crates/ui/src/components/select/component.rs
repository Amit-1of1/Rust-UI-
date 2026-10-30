use crate::components::icons::{Check, ChevronDown};
use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;
use dioxus_primitives::select::{self, SelectGroupLabelProps, SelectMultiProps, SelectOptionProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

pub use dioxus_primitives::select::SelectGroup;

#[css_module("/src/components/select/style.css")]
struct Styles;

/// How the trigger button is named. `aria-label` and `aria-labelledby` given to `Select`
/// arrive with the root's attributes; on the root `div` they would name nothing, so they
/// are moved to the trigger. The trigger also lists itself in `aria-labelledby`, so a
/// screen reader announces the label and the current value ("Fruit, Apple"), the way a
/// native `<select>` does. A plain `aria-label` on a button would replace the value.
///
/// The same goes for what makes it a form control: `id` (so a `<label for>` and focus-on-error
/// reach the button), `aria-describedby`, `aria-invalid` and `aria-required` are moved to the
/// trigger too, and `placeholder` is shown while nothing is selected.
///
/// The open list is labelled by the same label (without the trigger), so a screen reader
/// names the popup ("Fruit, list box") instead of an unnamed list.
struct TriggerName {
    root: Vec<Attribute>,
    trigger: Vec<Attribute>,
    placeholder: Option<String>,
    trigger_id: String,
    label_id: String,
    label: Option<String>,
    labelledby: Option<String>,
    list_labelledby: Option<String>,
}

/// Attributes that belong on the trigger button rather than the wrapper.
const TRIGGER_ATTRIBUTES: [&str; 3] = ["aria-describedby", "aria-invalid", "aria-required"];

fn use_trigger_name(attributes: Vec<Attribute>) -> TriggerName {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let (trigger_id, label_id) = (
        format!("rui-select-{id}-trigger"),
        format!("rui-select-{id}-label"),
    );
    let mut label = None;
    let mut by = None;
    let mut id = None;
    let mut placeholder = None;
    let mut trigger = Vec::new();
    let root = attributes
        .into_iter()
        .filter_map(|a| {
            if TRIGGER_ATTRIBUTES.contains(&a.name) {
                trigger.push(a);
                return None;
            }
            let slot = match a.name {
                "aria-label" => &mut label,
                "aria-labelledby" => &mut by,
                "id" => &mut id,
                "placeholder" => &mut placeholder,
                _ => return Some(a),
            };
            if let AttributeValue::Text(t) = &a.value {
                *slot = Some(t.clone()).filter(|t| !t.trim().is_empty());
            }
            None
        })
        .collect();
    let trigger_id = id.unwrap_or(trigger_id);
    let labelledby = match (&by, &label) {
        (Some(ids), _) => Some(format!("{ids} {trigger_id}")),
        (None, Some(_)) => Some(format!("{label_id} {trigger_id}")),
        (None, None) => None,
    };
    let list_labelledby = match (&by, &label) {
        (Some(ids), _) => Some(ids.clone()),
        (None, Some(_)) => Some(label_id.clone()),
        (None, None) => None,
    };
    TriggerName {
        root,
        trigger,
        placeholder,
        label: label.filter(|_| by.is_none()),
        trigger_id,
        label_id,
        labelledby,
        list_labelledby,
    }
}

/// Props for [`Select`]: dioxus-primitives' `SelectProps` plus `selected`, a plain value.
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// The selected option as a plain value: the select is controlled and shows whatever the
    /// parent passes on each render (`selected: filters.read().sort`), with no signal or memo
    /// needed. Wins over `value`.
    #[props(default, into)]
    pub selected: Option<T>,

    /// The controlled value as a signal; its `None` means no option is selected.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// The initial value when uncontrolled. `None` shows the placeholder.
    #[props(default)]
    pub default_value: Option<T>,

    /// Called when the selected value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the list.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Name for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Whether arrow keys wrap around at the ends of the list.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// How long typed letters are remembered for typeahead.
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// The trigger's element, so closing from the keyboard can put focus back on it. Through
/// the element handle rather than the DOM, so it also works on native, where focus would
/// otherwise stay on the removed list.
#[derive(Clone, Copy)]
struct TriggerEl(Signal<Option<std::rc::Rc<MountedData>>>);

fn refocus_trigger() {
    if let Some(TriggerEl(el)) = try_consume_context::<TriggerEl>() {
        if let Some(el) = el() {
            spawn(async move {
                _ = el.set_focus(true).await;
            });
        }
    }
}

fn is_close_key(e: &KeyboardEvent) -> bool {
    matches!(e.key(), Key::Escape | Key::Enter) || e.key() == Key::Character(" ".into())
}

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select
    });
    let mut trigger_el = use_context_provider(|| TriggerEl(Signal::new(None))).0;
    let name = use_trigger_name(props.attributes);
    let trigger_attrs = merge_attributes(vec![
        attributes!(button {
            onmounted: move |e: MountedEvent| trigger_el.set(Some(e.data())),
        }),
        name.trigger,
    ]);
    let merged = merge_attributes(vec![base, name.root]);
    // A plain `selected` becomes the signal the primitive wants. The memo follows the prop.
    let selected = props.selected.clone();
    let has_selected = selected.is_some();
    let from_prop = use_memo(use_reactive!(|selected| selected));
    let value = if has_selected {
        Some(from_prop.into())
    } else {
        props.value
    };

    rsx! {
        select::Select {
            value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            if let Some(label) = name.label {
                span { id: name.label_id, hidden: true, "{label}" }
            }
            select::SelectTrigger {
                class: Styles::dx_select_trigger,
                id: name.trigger_id,
                aria_labelledby: name.labelledby,
                attributes: trigger_attrs,
                if let Some(placeholder) = name.placeholder {
                    select::SelectValue { placeholder }
                } else {
                    select::SelectValue {}
                }
                ChevronDown { "aria-hidden": "true",
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: Styles::dx_select_list,
                aria_labelledby: name.list_labelledby,
                // The primitive closes on Escape, Enter and Space but leaves focus on the
                // removed list; the APG select-only combobox returns it to the trigger.
                attributes: attributes!(div {
                    onkeydown: move |e: KeyboardEvent| {
                        if is_close_key(&e) {
                            refocus_trigger();
                        }
                    },
                }),
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select
    });
    let name = use_trigger_name(props.attributes);
    let merged = merge_attributes(vec![base, name.root]);

    rsx! {
        select::SelectMulti {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            if let Some(label) = name.label {
                span { id: name.label_id, hidden: true, "{label}" }
            }
            select::SelectTrigger {
                class: Styles::dx_select_trigger,
                id: name.trigger_id,
                aria_labelledby: name.labelledby,
                attributes: name.trigger,
                if let Some(placeholder) = name.placeholder {
                    select::SelectValue { placeholder }
                } else {
                    select::SelectValue {}
                }
                ChevronDown { "aria-hidden": "true",
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: Styles::dx_select_list,
                aria_labelledby: name.list_labelledby,
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_group_label
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectGroupLabel {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_option,
        // Choosing an option closes a single select (the primitive stops the key here).
        // `SelectMulti` provides no `TriggerEl`, so its list stays open and keeps focus.
        onkeydown: move |e: KeyboardEvent| {
            if is_close_key(&e) {
                refocus_trigger();
            }
        },
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            select::SelectItemIndicator {
                Check { "aria-hidden": "true",
                    size: "1rem",
                    stroke: "var(--secondary-color-5)",
                }
            }
        }
    }
}
