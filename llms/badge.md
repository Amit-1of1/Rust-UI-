# Badge

- Copy into your app: `atoll add badge` or `dx components add badge --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`, `ui_strings`).
- Import: `use crate::components::badge::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Badge is a component designed to display small, distinct labels that help highlight important content or status indicators. Perfect for use cases like notifications, status labels, or categorization.

## Component Structure

```rust
Badge {
    {children}
}
```

## Colours, sizes and icons

`variant` picks the role: `Primary`, `Secondary` (a faded fill that draws less attention), `Outline`, `Destructive`, `Success` and `Warning`. The status colours come from the theme's success, warning and error tokens, and their text passes WCAG AA on the fill (`atoll contrast`). `size` is `Sm`, `Default` or `Lg`. Put a decorative icon before the text (`aria-hidden`); it follows the badge's text size and colour, so add words as well: colour and icon alone don't say what a badge means.

## Dismissible badges

`on_dismiss` adds a remove button after the text, named "Remove {dismiss_label}":

```rust
Badge { variant: BadgeVariant::Outline, dismiss_label: "Rust", on_dismiss: move |_| remove("Rust"), "Rust" }
```

For a whole set of removable, selectable values with arrow-key navigation, use `TagGroup`.

## Dots and positioning

`dot: true` draws a round marker with no text. Give it an `aria-label` when it means something on its own ("Online"); without one it is decorative. `BadgeAnchor` pins a badge to a corner of another element, such as a count on a bell button. Put the count in the button's own name too, and hide the badge itself (`aria_hidden: "true"`) so it isn't read twice:

```rust
BadgeAnchor { badge: rsx! { Badge { size: BadgeSize::Sm, aria_hidden: "true", "3" } },
    IconButton { label: "Notifications, 3 unread", Bell { "aria-hidden": "true" } }
}
```

## API

```rust
/// A badge's role, shown by its colour. Success and warning use the theme's status tokens.
pub enum BadgeVariant {
    #[default]
    Primary,
    /// A faded fill that draws less attention.
    Secondary,
    Destructive,
    Outline,
    Success,
    Warning,
}

/// A badge's size: `Sm` for dense tables, `Lg` for marketing pages.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum BadgeSize {
    Sm,
    #[default]
    Default,
    Lg,
}

/// The props for the [`Badge`] component.
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,

    #[props(default)]
    pub size: BadgeSize,

    /// A dot with no text, such as an "unread" marker. Give it an `aria-label` when it means
    /// something on its own; without one it is decorative and hidden from screen readers.
    #[props(default)]
    pub dot: bool,

    /// Adds a remove button after the text (for selected values and filters). It is named
    /// "Remove {dismiss_label}", so pass the badge's text as `dismiss_label`.
    #[props(default)]
    pub on_dismiss: Option<EventHandler<MouseEvent>>,

    /// What the remove button removes ("Rust" gives "Remove Rust"). Needed with `on_dismiss`.
    #[props(default, into)]
    pub dismiss_label: Option<String>,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element

/// Where a [`BadgeAnchor`] puts its badge. `End` follows the reading direction (right in
/// English, left in Arabic).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum BadgePlacement {
    #[default]
    TopEnd,
    BottomEnd,
    TopStart,
    BottomStart,
}

/// Pins a badge to the corner of another element, such as a count on an icon button or a
/// status dot on an avatar.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # use ui::icons::Bell;
/// # fn app() -> Element {
/// rsx! {
///     BadgeAnchor { badge: rsx! { Badge { size: BadgeSize::Sm, "3" } },
///         IconButton { label: "Notifications, 3 unread", variant: ButtonVariant::Outline,
///             Bell { "aria-hidden": "true" }
///         }
///     }
/// }
/// # }
/// ```
///
/// The badge is drawn over the element, so put what it says into the element's own name too
/// (as above) and mark it `aria-hidden` if it would be read twice.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn BadgeAnchor(
    /// The badge (or any small element) to pin.
    badge: Element,
    #[props(default)] placement: BadgePlacement,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn VerifiedIcon() -> Element
```

## Example

```rust
use crate::components::badge::*;
use crate::components::button::*;
use crate::components::icons::{Bell, Check, TriangleAlert};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut tags = use_signal(|| vec!["Rust", "Dioxus", "WebAssembly"]);
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge { variant: BadgeVariant::Success, Check { "aria-hidden": "true", size: "12px" } "Paid" }
            Badge { variant: BadgeVariant::Warning, TriangleAlert { "aria-hidden": "true", size: "12px" } "Overdue" }
            Badge { variant: BadgeVariant::Secondary, VerifiedIcon {} "Verified" }
        }
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; margin-top: 0.75rem;",
            Badge { size: BadgeSize::Sm, "Small" }
            Badge { "Default" }
            Badge { size: BadgeSize::Lg, "Large" }
            Badge { dot: true, variant: BadgeVariant::Success, aria_label: "Online" }
            span { "Online" }
        }
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.5rem; margin-top: 0.75rem;",
            for tag in tags() {
                Badge { key: "{tag}", variant: BadgeVariant::Outline, dismiss_label: tag,
                    on_dismiss: move |_| tags.write().retain(|t| *t != tag),
                    "{tag}"
                }
            }
        }
        div { style: "display: flex; align-items: center; gap: 1.5rem; margin-top: 1rem;",
            BadgeAnchor {
                badge: rsx! { Badge { size: BadgeSize::Sm, variant: BadgeVariant::Destructive, aria_hidden: "true", "3" } },
                IconButton { label: "Notifications, 3 unread", variant: ButtonVariant::Outline,
                    Bell { "aria-hidden": "true" }
                }
            }
            BadgeAnchor { placement: BadgePlacement::BottomEnd,
                badge: rsx! { Badge { dot: true, variant: BadgeVariant::Success } },
                IconButton { label: "Messages", variant: ButtonVariant::Ghost,
                    Bell { "aria-hidden": "true" }
                }
            }
        }
    }
}
```
