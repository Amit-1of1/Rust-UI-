use crate::components::hooks::use_unique_id;
use dioxus::prelude::*;
use dioxus_primitives::progress;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/progress/style.css")]
struct Styles;

/// A [`Progress`] bar's thickness.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ProgressSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl ProgressSize {
    pub fn class(&self) -> &'static str {
        match self {
            ProgressSize::Sm => "sm",
            ProgressSize::Default => "default",
            ProgressSize::Lg => "lg",
        }
    }
}

/// The props for [`Progress`].
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current value, between 0 and `max`. `None` is indeterminate: the bar moves until
    /// the work finishes or fails, for waits whose length is unknown.
    pub value: ReadSignal<Option<f64>>,

    /// The maximum value. Defaults to 100.
    #[props(default = ReadSignal::new(Signal::new(100.0)))]
    pub max: ReadSignal<f64>,

    /// A visible label above the bar ("Uploading photos"), which also names it. Without one,
    /// give the bar an `aria-label`.
    #[props(default, into)]
    pub label: Option<String>,

    /// Shows the percentage next to the label (or above the bar).
    #[props(default)]
    pub show_value: bool,

    #[props(default)]
    pub size: ProgressSize,

    /// Additional attributes for the bar (`role="progressbar"`), such as `aria-label`.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Unused; kept so code that passes children still builds.
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let label_id = use_unique_id("dx-progress-label");
    let value = props.value;
    let max = props.max;
    let percent = use_memo(move || {
        value().map(|v| {
            let m = max();
            if m > 0.0 {
                (v / m * 100.0).clamp(0.0, 100.0).round()
            } else {
                0.0
            }
        })
    });
    let labelled = props.label.is_some();
    let mut bar_attrs = vec![
        attributes!(div { class: Styles::dx_progress, "data-size": props.size.class() }),
    ];
    if labelled {
        bar_attrs.push(attributes!(div { aria_labelledby: label_id.clone() }));
    }
    bar_attrs.push(props.attributes);
    let bar = rsx! {
        progress::Progress {
            value: props.value,
            max: props.max,
            attributes: merge_attributes(bar_attrs),
            progress::ProgressIndicator { class: Styles::dx_progress_indicator }
        }
    };
    if !labelled && !props.show_value {
        return bar;
    }
    rsx! {
        div { class: Styles::dx_progress_field,
            div { class: Styles::dx_progress_header,
                if let Some(label) = props.label {
                    span { id: label_id, class: Styles::dx_progress_label, {label} }
                }
                if props.show_value {
                    if let Some(p) = percent() {
                        // The bar already reports its value; this copy is for sighted users.
                        span { class: Styles::dx_progress_value, aria_hidden: "true", "{p}%" }
                    }
                }
            }
            {bar}
        }
    }
}
