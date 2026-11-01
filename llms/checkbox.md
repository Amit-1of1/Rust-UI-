# Checkbox

- Copy into your app: `atoll add checkbox` or `dx components add checkbox --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::checkbox::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Checkbox component is used to create an accessible checkbox input.

## Component Structure

```rust
// The checkbox component creates a checkbox button
Checkbox {
    // The checkbox indicator is a child component which will only be visible when the checkbox is checked.
    CheckboxIndicator {
        // The content of the checkbox indicator, typically an icon or image.
        {children}
    }
}
```

## Keyboard and accessibility

Follows the WAI-ARIA [Checkbox](https://www.w3.org/WAI/ARIA/apg/patterns/checkbox/) pattern. Roles: `checkbox`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Space | On the checkbox | Checks or unchecks it. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
pub use dioxus_primitives::checkbox::CheckboxState;

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element

/// A set of related checkboxes under one visible label: a `<fieldset>` with a `<legend>`, so a
/// screen reader reads the group's name when focus enters it. Put each `Checkbox` with its
/// `Label` inside. For a "select all" box, give it `CheckboxState::Indeterminate` while only
/// some are checked.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn CheckboxGroup(
    /// The group's visible name ("Notify me about").
    #[props(into)]
    label: String,
    /// Shows the group as invalid; pair it with a text error that says what to fix.
    #[props(default)]
    invalid: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::checkbox::*;
use crate::components::label::Label;
use dioxus::prelude::*;

const TOPICS: [&str; 3] = ["Invoices", "Payments", "Team changes"];

#[component]
pub fn Demo() -> Element {
    let mut picked = use_signal(|| vec![true, false, false]);
    let all = use_memo(move || {
        let p = picked();
        if p.iter().all(|v| *v) {
            CheckboxState::Checked
        } else if p.iter().any(|v| *v) {
            CheckboxState::Indeterminate
        } else {
            CheckboxState::Unchecked
        }
    });
    let row = "display: flex; align-items: center; gap: 0.5rem;";
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: flex-start; gap: 2rem;",
            div { style: row,
                Checkbox { id: "terms", name: "terms" }
                Label { html_for: "terms", "Accept terms" }
            }
            CheckboxGroup { label: "Email me about",
                div { style: row,
                    Checkbox { id: "topic-all",
                        checked: Some(all()),
                        on_checked_change: move |state| {
                            let on = state != CheckboxState::Unchecked;
                            picked.set(vec![on; TOPICS.len()]);
                        },
                    }
                    Label { html_for: "topic-all", "Everything" }
                }
                for (i, topic) in TOPICS.iter().enumerate() {
                    div { key: "{topic}", style: "{row} padding-inline-start: 1.5rem;",
                        Checkbox { id: "topic-{i}",
                            checked: Some(if picked()[i] { CheckboxState::Checked } else { CheckboxState::Unchecked }),
                            on_checked_change: move |state| picked.write()[i] = state == CheckboxState::Checked,
                        }
                        Label { html_for: "topic-{i}", "{topic}" }
                    }
                }
            }
        }
    }
}
```
