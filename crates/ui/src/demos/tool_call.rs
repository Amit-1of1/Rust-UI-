use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::tool_call::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut approval = use_signal(|| ToolStatus::NeedsApproval);
    let deploy_step = match approval() {
        ToolStatus::NeedsApproval => ToolStatus::Queued,
        other => other,
    };
    rsx! {
        div { display: "flex", flex_direction: "column", gap: "1rem", max_width: "36rem",
            AgentSteps { label: "Plan",
                AgentStep { status: ToolStatus::Succeeded, "Read the failing test" }
                AgentStep { status: ToolStatus::Running, "Patch the date parser" }
                AgentStep { status: deploy_step, "Deploy the preview" }
            }
            ToolCall {
                name: "read_file",
                summary: "src/date.rs",
                status: ToolStatus::Succeeded,
                pre { "fn parse(s: &str) -> Option<Date> {{ … }}" }
            }
            ToolCall {
                name: "run_tests",
                summary: "cargo test date",
                status: ToolStatus::Running,
                pre { "running 12 tests…" }
            }
            ToolCall {
                name: "deploy_preview",
                summary: "preview-142",
                status: approval(),
                actions: (approval() == ToolStatus::NeedsApproval).then(|| {
                    rsx! {
                        Button {
                            size: ButtonSize::Sm,
                            onclick: move |_| approval.set(ToolStatus::Running),
                            "Allow"
                        }
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Outline,
                            onclick: move |_| approval.set(ToolStatus::Failed),
                            "Deny"
                        }
                    }
                }),
                pre { "{{ \"branch\": \"fix/date-parser\" }}" }
            }
        }
    }
}
