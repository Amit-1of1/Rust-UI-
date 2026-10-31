use crate::components::chat::use_chat_announcer;
use crate::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::icons::{CircleAlert, CircleCheck, CircleDashed, CircleX, LoaderCircle};
use crate::components::ui_strings::{fill, use_ui_strings, UiStrings};
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/tool_call/style.css")]
struct Styles;

/// Where a tool call or agent step is.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[non_exhaustive]
pub enum ToolStatus {
    #[default]
    Queued,
    Running,
    Succeeded,
    Failed,
    /// Waits for the reader to allow it (put the Allow and Deny buttons in `actions`).
    NeedsApproval,
}

impl ToolStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::NeedsApproval => "needs-approval",
        }
    }

    /// The status in words, from the kit's strings.
    pub fn label(self, strings: &UiStrings) -> String {
        match self {
            Self::Queued => strings.tool_queued.clone(),
            Self::Running => strings.tool_running.clone(),
            Self::Succeeded => strings.tool_succeeded.clone(),
            Self::Failed => strings.tool_failed.clone(),
            Self::NeedsApproval => strings.tool_needs_approval.clone(),
        }
    }

    /// Worth interrupting the reader for: something went wrong or needs them.
    fn wants_attention(self) -> bool {
        matches!(self, Self::Failed | Self::NeedsApproval)
    }
}

#[component]
fn StatusGlyph(status: ToolStatus) -> Element {
    rsx! {
        span { class: Styles::dx_tool_glyph, "data-status": status.as_str(), aria_hidden: "true",
            match status {
                ToolStatus::Queued => rsx! { CircleDashed { size: "1rem" } },
                ToolStatus::Running => rsx! { LoaderCircle { size: "1rem" } },
                ToolStatus::Succeeded => rsx! { CircleCheck { size: "1rem" } },
                ToolStatus::Failed => rsx! { CircleX { size: "1rem" } },
                ToolStatus::NeedsApproval => rsx! { CircleAlert { size: "1rem" } },
            }
        }
    }
}

/// One tool call an agent made: its name, a one-line summary and its status in words, with
/// the input and output folded away until opened. The status is never colour alone, and a call
/// that fails or needs approval is announced through the [`ChatLog`](crate::components::chat::ChatLog)
/// it sits in.
///
/// Opens and closes like [`Collapsible`] (Enter or Space on the header). `actions` stay visible
/// when it is closed: put Allow and Deny there for [`ToolStatus::NeedsApproval`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ToolCall(
    /// The tool's name, shown in code type ("search_docs").
    #[props(into)]
    name: String,
    #[props(default)] status: ToolStatus,
    /// One line about this call ("3 results", "src/main.rs").
    #[props(into)]
    summary: Option<String>,
    /// Start open.
    #[props(default)]
    open: bool,
    /// Buttons under the header, shown even when closed.
    actions: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    /// Details: arguments, output, logs.
    children: Element,
) -> Element {
    let strings = use_ui_strings();
    let status_text = status.label(&strings);
    let announcer = use_chat_announcer();
    let said = strings.clone();
    let tool = name.clone();
    use_effect(use_reactive!(|status| {
        if status.wants_attention() {
            if let Some(announcer) = announcer {
                let words = status.label(&said);
                announcer.announce(fill(&said.tool_status, &[("name", &tool), ("status", &words)]));
            }
        }
    }));

    rsx! {
        Collapsible {
            default_open: open,
            attributes: merge_attributes(vec![
                attributes!(div {
                    class: Styles::dx_tool_call,
                    "data-slot": "tool-call",
                    "data-status": status.as_str(),
                }),
                attributes,
            ]),
            CollapsibleTrigger { class: Styles::dx_tool_call_trigger,
                StatusGlyph { status }
                span { class: Styles::dx_tool_call_name, "{name}" }
                if let Some(summary) = summary {
                    span { class: Styles::dx_tool_call_summary, "{summary}" }
                }
                span { class: Styles::dx_tool_call_status, "{status_text}" }
            }
            if let Some(actions) = actions {
                div { class: Styles::dx_tool_call_actions, {actions} }
            }
            CollapsibleContent { class: Styles::dx_tool_call_body, {children} }
        }
    }
}

/// An agent's plan or progress as an ordered list. The running step is marked
/// `aria-current="step"`, and each step's status is read after its text.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     AgentSteps { label: "Plan",
///         AgentStep { status: ToolStatus::Succeeded, "Read the issue" }
///         AgentStep { status: ToolStatus::Running, "Write the fix" }
///         AgentStep { "Run the tests" }
///     }
/// }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn AgentSteps(
    /// The list's accessible name ("Plan", "Progress").
    #[props(into)]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        ol {
            "data-slot": "agent-steps",
            aria_label: label,
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_agent_steps }), attributes]),
            {children}
        }
    }
}

/// One step of [`AgentSteps`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn AgentStep(
    #[props(default)] status: ToolStatus,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let strings = use_ui_strings();
    let status_text = status.label(&strings);
    rsx! {
        li {
            "data-slot": "agent-step",
            "data-status": status.as_str(),
            aria_current: (status == ToolStatus::Running).then_some("step"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_agent_step }), attributes]),
            StatusGlyph { status }
            span { class: Styles::dx_agent_step_text, {children} }
            span { class: "sr-only", ", {status_text}" }
        }
    }
}
