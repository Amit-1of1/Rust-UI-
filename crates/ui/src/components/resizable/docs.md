# Resizable

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
