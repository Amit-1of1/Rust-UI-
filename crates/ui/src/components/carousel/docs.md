# Carousel

A horizontally scrolling row of slides with scroll snapping and Previous/Next buttons. Nothing auto-plays, so there is no motion to pause.

```rust
Carousel { label: "Featured plans",
    CarouselItem { label: "1 of 3", "..." }
    CarouselItem { label: "2 of 3", "..." }
}
```

Slide width and spacing are CSS (`.dx-carousel-item` flex-basis); the buttons scroll by about one viewport.

## Accessibility

- Role and name: a `section` with `aria-roledescription="carousel"` named by `label`; the scrolling track is a focusable group; each slide is `aria-roledescription="slide"` with your `label`.
- Keyboard: focus the track and use the arrow keys (native scrolling), or Tab to the Previous/Next buttons.
- Reduced motion: scrolling is instant instead of smooth.

## Source

Original to this repo. The pattern follows the WAI-ARIA carousel practices; Rust/UI's `carousel.rs` was used only to see the feature set.
