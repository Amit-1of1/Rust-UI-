# Badge

- Copy into your app: `rust-ui add badge` or `dx components add badge --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::badge::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Badge is a component designed to display small, distinct labels that help highlight important content or status indicators. Perfect for use cases like notifications, status labels, or categorization.

## Component Structure

```rust
Badge {
    {children}
}
```

## API

```rust
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
}

/// The props for the [`Badge`] component.
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element

#[component]
pub fn VerifiedIcon() -> Element
```

## Example

```rust
use crate::components::badge::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge { variant: BadgeVariant::Secondary, VerifiedIcon {} "Verified" }
        }
    }
}
```
