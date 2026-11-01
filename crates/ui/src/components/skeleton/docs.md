The Skeleton component is used to display a placeholder preview of content before the data gets loaded. This helps improve perceived performance and provides users with a visual indication that content is being loaded.

## Component Structure

```rust
Skeleton {
    // Accepts all GlobalAttributes, commonly used with style for sizing
    style: "width: 15rem; height: 1rem;",
}
```

## Shapes and composition

`shape` is `Rect` (the default, with the theme's corner radius), `Circle` (give it a width; the height follows) or `Text` (one line at the current font size, so it lines up with the text it replaces; `lines: 3` draws a paragraph with a shorter last line). Size skeletons like the content they replace so nothing moves when it loads, and compose them freely: a circle and a few text lines make a list row.

Skeletons are hidden from screen readers. Put `aria-busy="true"` on the region that is loading and remove it when the content arrives.

When the system asks for reduced motion the pulse stops (the kit's theme turns animations off).
