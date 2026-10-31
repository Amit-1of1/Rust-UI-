# Resizable

- Copy into your app: `atoll add resizable` or `dx components add resizable --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::resizable::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

Panels in a row or column with dividers between them: drag a divider, or focus it and use the arrow keys. For split views such as folders | messages | preview.

```rust
div { style: "height: 20rem;",
    ResizablePanelGroup { on_layout: move |sizes: Vec<f32>| save(sizes),
        ResizablePanel { id: "folders", default_size: 25.0, min_size: 15.0, max_size: 50.0, Folders {} }
        ResizableHandle { aria_label: "Resize folders", with_handle: true }
        ResizablePanel { default_size: 75.0, min_size: 30.0,
            ResizablePanelGroup { direction: ResizableDirection::Vertical,
                ResizablePanel { default_size: 60.0, Messages {} }
                ResizableHandle { aria_label: "Resize preview" }
                ResizablePanel { default_size: 40.0, Preview {} }
            }
        }
    }
}
```

Sizes are percentages of the group and should add up to 100; `min_size` and `max_size` limit each panel. The group fills its parent, so give the parent a height for a horizontal group. `on_layout` reports every panel's size after a change (store it to restore the layout with `default_size`). `resize_pair` is the pure rule a divider follows.

## Accessibility

- Role and name: each `ResizableHandle` is a focusable `role="separator"` (the WAI-ARIA window splitter pattern) with `aria-valuenow` (the size of the panel before it, in percent), `aria-valuemin`, `aria-valuemax`, `aria-orientation` and the `aria_label` you give it ("Resize" by default). Give the panel before it an `id` and the handle points at it with `aria-controls`.
- Keyboard: Tab reaches each divider. Left and Right arrows (Up and Down in a vertical group) move it by 5%, with Shift by 10%; Home and End move it to the panel's minimum and maximum.
- Pointer: drag the divider (a 9 px hit area around the 1 px line). The drag ends when the pointer leaves the group.
- Right to left: not mirrored yet. In a `dir="rtl"` row the first panel is on the right, and the arrow keys and dragging still change its size as in a left-to-right row, so they feel reversed. Use a vertical group, or `dir="ltr"` on the group, until this is fixed.
- Reduced motion: only the divider's colour changes; that transition is off under `prefers-reduced-motion`.

## Source

Original to this repo. The API follows shadcn/ui's `Resizable` (built on react-resizable-panels), which was not copied.

## Stability

Experimental. New on 2026-10-31.

## API

```rust
/// How far one arrow-key press moves a divider, in percent of the group.
pub const RESIZE_STEP: f32;

/// Moves the divider between two neighbouring panels by `delta` percent, keeping both
/// inside their `(min, max)` limits and their total unchanged. Returns the new sizes.
///
/// ```
/// use ui::components::resizable::resize_pair;
/// assert_eq!(resize_pair((30.0, 70.0), 10.0, (10.0, 80.0), (10.0, 90.0)), (40.0, 60.0));
/// assert_eq!(resize_pair((30.0, 70.0), 90.0, (10.0, 80.0), (10.0, 90.0)), (80.0, 20.0));
/// ```
pub fn resize_pair(sizes: (f32, f32), delta: f32, a: (f32, f32), b: (f32, f32)) -> (f32, f32)

pub enum ResizableDirection {
    /// Panels side by side, dividers dragged left and right.
    #[default]
    Horizontal,
    /// Panels stacked, dividers dragged up and down.
    Vertical,
}

/// A row (or column) of [`ResizablePanel`]s with a [`ResizableHandle`] between each pair.
///
/// ```rust,ignore
/// ResizablePanelGroup {
///     ResizablePanel { default_size: 25.0, min_size: 15.0, "Folders" }
///     ResizableHandle { aria_label: "Resize folders" }
///     ResizablePanel { default_size: 75.0, "Messages" }
/// }
/// ```
///
/// Sizes are percentages of the group and should add up to 100. Give the group a size (a
/// height for horizontal groups, as with any flex row).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ResizablePanelGroup(
    #[props(default)] direction: ResizableDirection,
    /// Called with every panel's size (percent) after a divider moves.
    #[props(default)]
    on_layout: Callback<Vec<f32>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// One panel of a [`ResizablePanelGroup`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ResizablePanel(
    /// Starting size in percent of the group.
    #[props(default = 50.0)]
    default_size: f32,
    #[props(default = 0.0)] min_size: f32,
    #[props(default = 100.0)] max_size: f32,
    /// The panel's element id. The handle after it points at it with `aria-controls`, as
    /// the window splitter pattern asks; without an id there is no `aria-controls`.
    #[props(into)]
    id: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The divider between two panels: drag it, or focus it and use the arrow keys (Home and End
/// jump to the limits). It is a focusable `role="separator"` whose value is the size of the
/// panel before it, as the WAI-ARIA window splitter pattern describes.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ResizableHandle(
    /// Read by screen readers, e.g. "Resize sidebar". Defaults to "Resize".
    #[props(into, default = "Resize".to_string())]
    aria_label: String,
    /// Show a grip in the middle of the divider.
    #[props(default)]
    with_handle: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
use crate::components::resizable::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut layout = use_signal(|| vec![30.0_f32, 70.0]);
    let pane = "padding: 1rem; font-size: 0.875rem;";
    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 36rem;",
            div { style: "height: 14rem; border: 1px solid var(--primary-color-6); border-radius: var(--radius);",
                ResizablePanelGroup { on_layout: move |sizes| layout.set(sizes),
                    ResizablePanel { id: "resizable-folders", default_size: 30.0, min_size: 15.0, max_size: 60.0,
                        div { style: pane, "Folders" }
                    }
                    ResizableHandle { aria_label: "Resize folders", with_handle: true }
                    ResizablePanel { default_size: 70.0, min_size: 30.0,
                        ResizablePanelGroup { direction: ResizableDirection::Vertical,
                            ResizablePanel { id: "resizable-messages", default_size: 60.0, min_size: 20.0,
                                div { style: pane, "Messages" }
                            }
                            ResizableHandle { aria_label: "Resize preview" }
                            ResizablePanel { default_size: 40.0, min_size: 20.0,
                                div { style: pane, "Preview" }
                            }
                        }
                    }
                }
            }
            p { style: "margin: 0; font-size: 0.875rem;",
                "Folders take {layout()[0].round()}%"
            }
        }
    }
}
```
