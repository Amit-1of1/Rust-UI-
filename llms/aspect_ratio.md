# Aspect ratio

- Copy into your app: `rust-ui add aspect_ratio` or `dx components add aspect_ratio --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::aspect_ratio::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The AspectRatio component is used to maintain a specific aspect ratio for its children. This is particularly useful for responsive designs where you want to ensure that an element retains its proportions regardless of the screen size.

## Component Structure

```rust
AspectRatio {
    // The aspect ratio to maintain (width / height)
    ratio: 4. / 3.,
    // The children of the AspectRatio component will be rendered within it.
    {children}
}
```

## API

```rust
#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element
```

## Example

```rust
use crate::components::aspect_ratio::AspectRatio;
use dioxus::prelude::*;

#[css_module("/src/components/aspect_ratio/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            class: Styles::dx_aspect_ratio_container,
            width: "20rem",
            max_width: "30vw",
            AspectRatio { ratio: 4.0 / 3.0,
                div {
                    background: "linear-gradient(to bottom right, var(--primary-color-5), var(--primary-color-3))",
                    width: "100%",
                    height: "100%",
                }
            }
        }
    }
}
```
