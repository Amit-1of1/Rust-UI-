# Tool call

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
