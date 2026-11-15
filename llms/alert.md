# Alert

- Copy into your app: `atoll add alert` or `dx components add alert --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::alert::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A message about the whole page or one part of it. By default it interrupts: it is announced as soon as it appears. Use `Callout` for a quiet inline note.

```rust
Alert { variant: AlertVariant::Destructive, icon: rsx! { CircleAlert {} },
    AlertTitle { "Payment failed" }
    AlertDescription { "Your card was declined." }
    AlertActions { Button { size: ButtonSize::Sm, "Update card" } }
}
```

- `variant`: `Default`, `Destructive`, `Success`, `Warning` or `Info`. The status colours come from the theme's tokens and pass WCAG AA.
- `icon`: an icon in its own column before the text (hidden from screen readers; the title carries the meaning). Use one for status variants so colour isn't the only cue.
- `AlertTitle` for a short summary, `AlertDescription` for the details, `AlertActions` for buttons that answer the message.
- `live`: `Assertive` (`role="alert"`, the default) for errors, `Polite` (`role="status"`) for good news, `Off` for a message that is part of the page from the start.
- The alert fills its container's width, so on a phone it is full width.

## Accessibility

- Role and name: `role="alert"`, `role="status"` or none, from `live`
- Keyboard: none of its own; actions inside are ordinary buttons
- Reduced motion: none

## API

```rust
/// An alert's role, shown by its colour (and, with `icon`, by an icon, so colour is not the
/// only cue).
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Success,
    Warning,
    Info,
}

/// How an [`Alert`] is announced.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum AlertLive {
    /// `role="alert"`: read at once, interrupting. For errors and anything urgent.
    #[default]
    Assertive,
    /// `role="status"`: read when the screen reader is idle. For success and progress notes.
    Polite,
    /// No live region: for a message that is on the page from the start (a live region only
    /// announces changes, and many alerts on load are noise).
    Off,
}

/// A message about the page or a part of it. Announced to screen readers (`role="alert"`)
/// when it appears, unless `live` says otherwise.
#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    /// How it is announced: at once (default), politely, or not at all.
    #[props(default)]
    live: AlertLive,
    /// An icon before the text, such as `CircleAlert`. Mark it `aria-hidden`; the title says
    /// what the icon shows.
    #[props(default)]
    icon: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn AlertTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn AlertDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Buttons or links that answer the alert ("Retry", "Undo", "View details"), below the text.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn AlertActions(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::alert::*;
use crate::components::button::*;
use crate::components::icons::{CircleAlert, CircleCheck, Info, TriangleAlert};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 28rem;",
            Alert { live: AlertLive::Off,
                AlertTitle { "Heads up" }
                AlertDescription { "You can add components to your app with the registry." }
            }
            Alert { variant: AlertVariant::Destructive,
                icon: rsx! { CircleAlert {} },
                AlertTitle { "Something went wrong" }
                AlertDescription { "Your session has expired. Sign in again to continue." }
                AlertActions { Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Sign in" } }
            }
            Alert { variant: AlertVariant::Success, live: AlertLive::Polite,
                icon: rsx! { CircleCheck {} },
                AlertTitle { "Invoice sent" }
                AlertDescription { "Ledgerly emailed the invoice to your client." }
            }
            Alert { variant: AlertVariant::Warning, live: AlertLive::Off,
                icon: rsx! { TriangleAlert {} },
                AlertTitle { "Trial ends in 3 days" }
            }
            Alert { variant: AlertVariant::Info, live: AlertLive::Off,
                icon: rsx! { Info {} },
                AlertTitle { "Maintenance on Sunday" }
                AlertDescription { "Sync pauses from 02:00 to 03:00 UTC." }
            }
        }
    }
}
```
