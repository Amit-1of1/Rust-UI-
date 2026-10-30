use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

#[test]
fn tool_call_header_says_name_summary_and_status_in_words() {
    let html = render(|| {
        rsx! { ToolCall { name: "read_file", summary: "src/main.rs", status: ToolStatus::Failed, "out" } }
    });
    assert!(html.contains("aria-expanded=false"), "{html}");
    for text in ["read_file", "src/main.rs", ">Failed<"] {
        assert!(html.contains(text), "missing {text} in {html}");
    }
    assert!(html.contains("data-status=\"failed\""), "{html}");
}

#[test]
fn tool_call_actions_show_while_closed() {
    let html = render(|| {
        rsx! {
            ToolCall { name: "deploy", status: ToolStatus::NeedsApproval,
                actions: rsx! { button { "Allow" } },
                "details"
            }
        }
    });
    assert!(html.contains("Allow"), "{html}");
    assert!(html.contains("Needs approval"), "{html}");
}

#[test]
fn agent_steps_mark_the_running_step_current() {
    let html = render(|| {
        rsx! {
            AgentSteps { label: "Plan",
                AgentStep { status: ToolStatus::Succeeded, "One" }
                AgentStep { status: ToolStatus::Running, "Two" }
                AgentStep { "Three" }
            }
        }
    });
    assert!(html.starts_with("<ol"), "{html}");
    assert!(html.contains("aria-label=\"Plan\""), "{html}");
    assert_eq!(html.matches("aria-current=\"step\"").count(), 1, "{html}");
    for text in [", Done", ", Running", ", Queued"] {
        assert!(html.contains(text), "missing {text} in {html}");
    }
}
