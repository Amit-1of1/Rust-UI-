# NativeSelect

- Copy into your app: `rust-ui add native_select` or `dx components add native_select --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`, `input`).
- Import: `use crate::components::native_select::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The browser's own `<select>`, styled like `Input`. Choose it over `Select` on phones (the OS picker), in long forms, for long option lists, and in server-rendered pages, where it works before the app has loaded. Use `Select` when the open list needs custom styling.

```rust
NativeSelect { id: "plan", value: plan(), onchange: move |e: FormEvent| plan.set(e.value()),
    NativeSelectOption { value: "starter", "Starter" }
    NativeSelectOption { value: "team", "Team" }
    NativeSelectGroup { label: "Legacy",
        NativeSelectOption { value: "pro-2024", disabled: true, "Pro (2024)" }
    }
}
```

Every `select` attribute (`name`, `value`, `required`, `disabled`, `multiple`, `aria-*`) goes to the `<select>`, and `class` is added to the kit's. Label it with a `Label { html_for }` or `aria_label`. `aria-invalid="true"` draws the error border.

## Accessibility

- Role and name: a native `select` (combobox or listbox), named by its label; options and `optgroup` labels are read by screen readers as usual. The chevron is `aria-hidden`.
- Keyboard: the browser's own (arrow keys, type to jump, Enter or Space to open).
- Reduced motion: no animation.

## Source

Original to this repo; shadcn/ui's `native-select` was the reference (not copied).

## Stability

Experimental. New on 2026-10-31.

## API

```rust
/// The browser's own `<select>`, styled like [`Input`](crate::components::input::Input).
///
/// Use it where the platform picker is the better choice: phones (the OS wheel or sheet),
/// long forms, very long option lists, and server-rendered pages, where it works before the
/// app has loaded. Use `Select` when the list itself needs custom styling.
///
/// ```rust,ignore
/// NativeSelect { aria_label: "Country", onchange: move |e: FormEvent| country.set(e.value()),
///     NativeSelectOption { value: "", "Choose a country" }
///     NativeSelectGroup { label: "Europe",
///         NativeSelectOption { value: "fr", "France" }
///     }
/// }
/// ```
///
/// Any `select` attribute (`name`, `value`, `required`, `disabled`, `multiple`, `id`,
/// `aria-*`) goes to the `<select>`; `class` is added to the kit's.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn NativeSelect(
    onchange: Option<EventHandler<FormEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=select)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// One `<option>` of a [`NativeSelect`]. Takes `value`, `selected` and `disabled`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn NativeSelectOption(
    #[props(extends=GlobalAttributes)]
    #[props(extends=option)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// A labelled group of options (`<optgroup>`), read by screen readers with its options.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn NativeSelectGroup(
    #[props(into)] label: String,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::label::Label;
use crate::components::native_select::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut plan = use_signal(|| "team".to_string());
    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 16rem;",
            Label { html_for: "native-select-plan", "Plan" }
            NativeSelect {
                id: "native-select-plan",
                value: plan(),
                onchange: move |e: FormEvent| plan.set(e.value()),
                NativeSelectOption { value: "starter", "Starter" }
                NativeSelectOption { value: "team", "Team" }
                NativeSelectOption { value: "business", "Business" }
                NativeSelectGroup { label: "Legacy",
                    NativeSelectOption { value: "pro-2024", disabled: true, "Pro (2024)" }
                }
            }
            p { style: "margin: 0; font-size: 0.875rem;", "Selected plan: {plan}" }
        }
    }
}
```
