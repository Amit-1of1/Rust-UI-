# Layout

Page structure on the kit's foundations ([Foundations](../../../../docs/FOUNDATIONS.md)):

- `Container`: centres the page at `--layout-max-width` with the responsive page margin.
- `Stack`: children in a column or a row with an even `gap` in 4 px units (`gap: 4` is 16 px). Use it instead of margins between siblings.
- `Grid` and `GridItem`: the page grid (4 columns on phones, 8 from 768 px, 12 from 1024 px) with the matching gutter, `columns` for a fixed count, or `min_item: "16rem"` to fit as many columns as there is room for. `GridItem { span, span_m, span_l }` spans columns per breakpoint.
- `Text`: the type scale (`Display`, `Title1` to `Title4`, `Body1` to `Body3`, `Caption`) with line heights on the 4 px grid and the semantic text colours. `as_` picks the element, so a title can be a real heading.

```rust
Container {
    Stack { gap: 6,
        Text { variant: TextVariant::Title1, as_: TextAs::H1, "Settings" }
        Grid { min_item: "16rem",
            Card { "Profile" }
            Card { "Billing" }
        }
    }
}
```

## Accessibility

- Role and name: plain `div`s; `Text` renders the element you choose with `as_`, so pick headings by meaning (`H1` to `H4`) and the look with `variant`.
- Keyboard: nothing to focus.
- Reduced motion: no motion.
- Reading order follows the source order; `Grid` and `Stack` never reorder children.

## Source

Original to this repo; the spacing, grid and type tokens follow Reshaped (MIT).
