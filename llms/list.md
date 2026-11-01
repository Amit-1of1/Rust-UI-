# List

- Copy into your app: `atoll add list` or `dx components add list --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::list::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A bulleted, numbered or plain list of items. It renders a real `<ul>` or `<ol>`, so screen readers announce "list, 3 items" and count as you move through it. Items hold any content, other components included, and lists nest.

```rust
List { marker: ListMarker::Number,
    ListItem { "Install the CLI" }
    ListItem { "Run " Kbd { "atoll create" } }
}
```

- `marker`: `Bullet` (default), `Number`, `Letter`, `Roman` or `None`. The ordered markers render an `<ol>`.
- `spaced: true` puts more room between items, for lists of rows or cards.
- `ListMarker::None` keeps the list role (`role="list"`), which Safari drops from lists drawn without markers.

## Accessibility

- Role and name: native `list` and `listitem`; give a plain list an `aria-label` when it is a group of its own ("Plans")
- Keyboard: none (not interactive); controls inside keep their own keys
- Reduced motion: none

## Source

Original to this repo.

## API

```rust
/// How a [`List`] marks its items.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum ListMarker {
    /// Bullets (an unordered list).
    #[default]
    Bullet,
    /// 1, 2, 3 (an ordered list).
    Number,
    /// a, b, c (an ordered list).
    Letter,
    /// i, ii, iii (an ordered list).
    Roman,
    /// No marker, for lists of rich rows (an unordered list that keeps its list role).
    None,
}

/// A bulleted, numbered or plain list: a real `<ul>` or `<ol>`, so screen readers say "list,
/// 3 items" and count as you move through it. Items hold any content, components included.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     List { marker: ListMarker::Number,
///         ListItem { "Install the CLI" }
///         ListItem { "Run " Kbd { "atoll create" } }
///     }
/// }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn List(
    #[props(default)] marker: ListMarker,
    /// Space between items: `true` for lists of rows or cards.
    #[props(default)]
    spaced: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// One item of a [`List`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ListItem(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::badge::*;
use crate::components::kbd::*;
use crate::components::list::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 2rem; align-items: flex-start;",
            List {
                ListItem { "Light and dark themes" }
                ListItem { "Keyboard first"
                    List { ListItem { "Tab, arrows, Escape" } }
                }
                ListItem { "No JavaScript of your own" }
            }
            List { marker: ListMarker::Number,
                ListItem { "Install the CLI" }
                ListItem { "Run " Kbd { "atoll create" } }
                ListItem { "Open the gallery" }
            }
            List { marker: ListMarker::None, spaced: true, aria_label: "Plans",
                ListItem { Badge { variant: BadgeVariant::Secondary, "Free" } " Every component" }
                ListItem { Badge { "Pro" } " Blocks and starters" }
            }
        }
    }
}
```
