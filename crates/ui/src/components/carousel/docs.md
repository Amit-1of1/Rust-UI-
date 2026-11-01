# Carousel

A horizontally scrolling row of slides with scroll snapping and Previous/Next buttons. Nothing auto-plays, so there is no motion to pause.

```rust
Carousel { label: "Featured plans",
    CarouselItem { label: "1 of 3", "..." }
    CarouselItem { label: "2 of 3", "..." }
}
```

Slides are 80% of the carousel's width by default, so the next one peeks in and shows there is more. `item_size` takes any CSS length: `"16rem"`, `"33%"` for three in view, or `"max(40%, 11rem)"` to adapt to the space. Touch and trackpads scroll it natively; the buttons scroll by about one viewport. Slides centre their content; `CarouselItem { media: true, img { .. } }` makes a full-bleed photo slide instead (the content fills the slide and an `img` is cropped to cover it).

## Accessibility

- Role and name: a `section` with `aria-roledescription="carousel"` named by `label`; the scrolling track is a focusable group; each slide is `aria-roledescription="slide"` with your `label`.
- Keyboard: focus the track and use the arrow keys (native scrolling), or Tab to the Previous/Next buttons.
- Reduced motion: scrolling is instant instead of smooth.

## Source

Original to this repo. The pattern follows the WAI-ARIA carousel practices; Rust/UI's `carousel.rs` was used only to see the feature set.
