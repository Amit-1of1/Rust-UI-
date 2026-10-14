use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/stepper/style.css")]
struct Styles;

/// Shared state of a [`Stepper`]. Get it with [`use_stepper`] inside the stepper to build
/// Back/Next buttons.
#[derive(Clone, Copy)]
pub struct StepperContext {
    pub current: Signal<usize>,
    pub total: usize,
}

impl StepperContext {
    pub fn can_prev(&self) -> bool {
        (self.current)() > 0
    }

    pub fn can_next(&self) -> bool {
        (self.current)() + 1 < self.total
    }

    pub fn prev(&self) {
        let mut current = self.current;
        if current() > 0 {
            current.set(current() - 1);
        }
    }

    pub fn next(&self) {
        let mut current = self.current;
        if self.can_next() {
            current.set(current() + 1);
        }
    }

    /// Jump to `step`; out-of-range values are ignored.
    pub fn go_to(&self, step: usize) {
        let mut current = self.current;
        if step < self.total {
            current.set(step);
        }
    }
}

/// The stepper context. Call inside a [`Stepper`].
pub fn use_stepper() -> StepperContext {
    use_context::<StepperContext>()
}

#[derive(Copy, Clone, PartialEq)]
enum StepState {
    Done,
    Current,
    Upcoming,
}

impl StepState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Done => "done",
            Self::Current => "current",
            Self::Upcoming => "upcoming",
        }
    }
}

/// A multi-step progress indicator. Steps are an ordered list; the current one carries
/// `aria-current="step"`. Put [`StepperItem`]s inside, one per step.
#[component]
pub fn Stepper(
    /// Number of steps.
    total: usize,
    #[props(default = 0)] default_step: usize,
    /// Drive the step from outside (for example, move on only after a form validates). When
    /// given, `default_step` is ignored and the stepper reads and writes this signal.
    #[props(default)]
    current: Option<Signal<usize>>,
    #[props(default)] vertical: bool,
    /// Called with the new index whenever the step changes.
    #[props(default)]
    on_change: Callback<usize>,
    /// Rendered after the list, inside the stepper, so it can call [`use_stepper`]
    /// (for example Back/Next buttons).
    #[props(default)]
    footer: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let own = use_signal(|| default_step.min(total.saturating_sub(1)));
    let current = current.unwrap_or(own);
    use_context_provider(|| StepperContext { current, total });
    let mut last = use_signal(|| *current.peek());
    use_effect(move || {
        let now = current();
        if now != last() {
            last.set(now);
            on_change.call(now);
        }
    });
    rsx! {
        div { class: Styles::dx_stepper_root,
            ol {
                "data-orientation": if vertical { "vertical" } else { "horizontal" },
                ..merge_attributes(vec![attributes!(div { class: Styles::dx_stepper }), attributes]),
                {children}
            }
            {footer}
        }
    }
}

/// One step. Clicking its button jumps there unless `disabled`.
#[component]
pub fn StepperItem(
    /// Zero-based index of this step.
    step: usize,
    #[props(into)] title: String,
    #[props(into, default)] description: Option<String>,
    #[props(default)] disabled: bool,
) -> Element {
    let ctx = use_stepper();
    let state = match step.cmp(&(ctx.current)()) {
        std::cmp::Ordering::Less => StepState::Done,
        std::cmp::Ordering::Equal => StepState::Current,
        std::cmp::Ordering::Greater => StepState::Upcoming,
    };
    rsx! {
        li { class: Styles::dx_stepper_item, "data-state": state.as_str(),
            button {
                class: Styles::dx_stepper_trigger,
                r#type: "button",
                disabled,
                "aria-current": (state == StepState::Current).then_some("step"),
                onclick: move |_| ctx.go_to(step),
                span { class: Styles::dx_stepper_indicator, "aria-hidden": "true",
                    if state == StepState::Done {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "3",
                            path { d: "M5 12l5 5L20 7" }
                        }
                    } else {
                        "{step + 1}"
                    }
                }
                span { class: Styles::dx_stepper_text,
                    span { class: Styles::dx_stepper_title, "{title}" }
                    if let Some(description) = description {
                        span { class: Styles::dx_stepper_description, "{description}" }
                    }
                    if state == StepState::Done {
                        span { class: Styles::dx_stepper_sr, "(completed)" }
                    }
                }
            }
        }
    }
}
