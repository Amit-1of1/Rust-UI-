# Tool call

- Copy into your app: `atoll add tool_call` or `dx components add tool_call --git https://github.com/Amit-1of1/Rust-UI-` (also copies `chat`, `collapsible`, `icons`, `ui_strings`).
- Import: `use crate::components::tool_call::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

What an agent did and is doing. `ToolCall` is one tool call: its name, a one-line summary and
its status in words, with arguments and output folded away until opened (it is a
[`Collapsible`](collapsible)). `actions` stay visible when closed: put Allow and Deny there
for `ToolStatus::NeedsApproval`. `AgentSteps` and `AgentStep` show a plan as an ordered list.

```rust
ToolCall { name: "read_file", summary: "src/date.rs", status: ToolStatus::Succeeded,
    pre { "{output}" }
}
AgentSteps { label: "Plan",
    AgentStep { status: ToolStatus::Succeeded, "Read the failing test" }
    AgentStep { status: ToolStatus::Running, "Patch the parser" }
}
```

## Accessibility

- Role and name: the header is a disclosure button (`aria-expanded`) whose name includes the
  tool, the summary and the status, so the status is never colour or icon alone. Glyphs are
  decorative.
- Inside a [`ChatLog`](chat), a call that turns `Failed` or `NeedsApproval` is announced
  ("deploy_preview: Needs approval") through the log's announcer; quieter changes are not.
- `AgentSteps` is an `ol` with a required `label`; the running step has
  `aria-current="step"`, and each step's status is read after its text.
- Reduced motion: the running glyph turns slowly instead of spinning (like `Spinner`).

## Source

Original to this repo. The idea of a status glyph, tool steps and an approval card comes from
the agent components in deeplethe/solid-glass (MIT); no code was copied.

## Stability

Experimental: new on 2026-10-30, and the props may change once a real agent app uses them (docs/STABILITY.md).

## Keyboard and accessibility

Follows the WAI-ARIA [Disclosure (Show/Hide)](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/) pattern. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | On a tool call's header | Shows or hides its details. | yes |
| Space | On a tool call's header | Shows or hides its details. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// Where a tool call or agent step is.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum ToolStatus {
    #[default]
    Queued,
    Running,
    Succeeded,
    Failed,
    /// Waits for the reader to allow it (put the Allow and Deny buttons in `actions`).
    NeedsApproval,
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
) -> Element

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
) -> Element

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
) -> Element
```

## Example

```rust
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
```
