# Tabs

- Copy into your app: `atoll add tabs` or `dx components add tabs --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`).
- Import: `use crate::components::tabs::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Tabs component is used to create a tabbed interface, allowing users to switch between different views or sections of content.

## Component Structure

```rust
// The Tabs component wraps all tab triggers and contents and orders them based on their index.
Tabs {
    // The TabList component contains all the tab triggers
    TabList {
        // The TabTrigger component is used to create a clickable tab button that switches the active tab.
        TabTrigger {
            // The index of the tab trigger, used to determine the focus order of the tabs.
            index: 0,
            // The value of the tab trigger, which must be unique and is used to identify the active tab.
            value: "tab1",
            // The contents of the tab trigger button
            {children}
        }
    }
    // The TabContent component contains the content that is displayed when the corresponding tab is active.
    TabContent {
        // The index of the tab content, used to determine the focus order of the tabs.
        index: 0,
        // The value of the tab content, which must match the value of the corresponding TabTrigger to be displayed.
        value: "tab1",
        // The content of the tab, which is displayed when the tab is active.
        {children}
    }
}
```

## Accessibility

Each `TabContent` is labelled by the `TabTrigger` with the same `index` (`aria-labelledby`). If you give a `TabTrigger` your own `id`, set `aria_labelledby` to that id on its `TabContent`.

## Variants, icons and width

`variant` is `TabsVariant::Default` (pills on a track) or `Ghost` (for tabs inside cards). Icons go inside `TabTrigger`, decorative. `stretch: true` makes the list fill its container and gives every tab the same width. `TabTrigger { disabled: true }` keeps a tab visible but out of reach.

## Keyboard and accessibility

Follows the WAI-ARIA [Tabs](https://www.w3.org/WAI/ARIA/apg/patterns/tabs/) pattern. Roles: `tablist`, `tab`, `tabpanel`. Works on native (Blitz): partly.

| Key | When | What it does | Native |
|---|---|---|---|
| Tab | On a tab | Leaves the tab list and moves focus to the shown panel. | not yet |
| ArrowRight | On a tab | Moves focus to the next tab without selecting it (manual activation). | yes |
| ArrowLeft | On a tab | Moves focus to the previous tab. | yes |
| Enter | On a tab | Selects the focused tab and shows its panel. | yes |
| Space | On a tab | Selects the focused tab and shows its panel. | yes |
| Home | On a tab | Moves focus to the first tab. | yes |
| End | On a tab | Moves focus to the last tab. | yes |

States:

- `first-selected`: The first tab is selected and its panel is shown.
- `second-selected`: The second tab is selected and its panel is shown.

Native: Tab moves to the next tab instead of leaving the tab list for the panel.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// The props for the [`Tabs`] component.
pub struct TabsProps {
    /// The class of the tabs component.
    #[props(default)]
    pub class: String,

    /// The controlled value of the active tab.
    pub value: ReadSignal<Option<String>>,

    /// The default active tab value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback fired when the active tab changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the tabs are disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the tabs are horizontal.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// The variant of the tabs component.
    #[props(default)]
    pub variant: TabsVariant,

    /// Stretch the tab list to the container's width, every tab the same width.
    #[props(default)]
    pub stretch: bool,

    /// Additional attributes to apply to the tabs element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs component.
    pub children: Element,
}

/// The variant of the tabs component.
pub enum TabsVariant {
    /// The default variant.
    #[default]
    Default,
    /// The ghost variant.
    Ghost,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element

#[component]
pub fn TabList(props: TabListProps) -> Element

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element

#[component]
pub fn TabContent(props: TabContentProps) -> Element
```

## Example

```rust
use crate::components::tabs::*;
use crate::components::icons::{Bell, CreditCard, User};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1.5rem;",
        Tabs { default_value: "account".to_string(), horizontal: true, max_width: "24rem",
            TabList {
                TabTrigger { value: "account".to_string(), index: 0usize, "Account" }
                TabTrigger { value: "password".to_string(), index: 1usize, "Password" }
            }
            TabContent { index: 0usize, value: "account".to_string(),
                p { "Make changes to your account here." }
            }
            TabContent { index: 1usize, value: "password".to_string(),
                p { "Change your password here." }
            }
        }
        // Equal-width tabs with icons, and one you can't open yet.
        Tabs { default_value: "profile".to_string(), horizontal: true, stretch: true, variant: TabsVariant::Ghost, max_width: "24rem",
            TabList {
                TabTrigger { value: "profile".to_string(), index: 0usize, User { "aria-hidden": "true", size: "1rem" } "Profile" }
                TabTrigger { value: "alerts".to_string(), index: 1usize, Bell { "aria-hidden": "true", size: "1rem" } "Alerts" }
                TabTrigger { value: "billing".to_string(), index: 2usize, disabled: true, CreditCard { "aria-hidden": "true", size: "1rem" } "Billing" }
            }
            TabContent { index: 0usize, value: "profile".to_string(), p { "Your name and photo." } }
            TabContent { index: 1usize, value: "alerts".to_string(), p { "What we email you about." } }
            TabContent { index: 2usize, value: "billing".to_string(), p { "Plans and invoices." } }
        }
        }
    }
}
```
