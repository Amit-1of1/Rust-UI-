# Image

A picture with a fixed shape, sharp on every screen, and a fallback when it can't load.

```rust
Image {
    src: "/cabin-800.jpg",
    srcset: "/cabin-800.jpg 800w, /cabin-1600.jpg 1600w",
    sizes: "(min-width: 40rem) 40rem, 100vw",
    alt: "The cabin from the lake",
    ratio: 16.0 / 9.0,
}
```

- `ratio` (width / height) keeps the box's shape at any width, so the page doesn't jump when the picture arrives; `fit` is `Cover` (crop, the default) or `Contain` (show it all). Set the width with `style` or a class; the box never grows past its container.
- `srcset` and `sizes` let the browser pick the file for the screen's density and the layout width.
- `alt` is required: describe what the picture shows, or pass `alt: ""` for a decorative picture.
- When `src` is empty or fails to load, a muted box with an "image missing" icon takes its place, named by `alt`. Pass `fallback` for your own (initials, a placeholder picture).
- Pictures load lazily (`loading="lazy"`); set `lazy: false` for the first one a visitor sees.

## Component Structure

```rust
// A box that keeps the shape and holds the picture, or the fallback when it fails.
Image { src, alt, srcset, sizes, ratio, fit, lazy,
    fallback: rsx! { /* shown instead when src is empty or fails to load */ },
}
```

## Accessibility

- Role and name: a native `img` named by `alt`; the fallback is `role="img"` with the same name, or hidden when `alt` is empty
- Keyboard: none (not interactive). For a clickable picture, wrap it in a link or button with its own name
- Reduced motion: none

## Source

Original to this repo.
