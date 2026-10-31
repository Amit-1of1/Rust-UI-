# Slider

- Copy into your app: `atoll add slider` or `dx components add slider --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::slider::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The slider component allows users to select a value from a range by sliding a handle along a track.

## Component Structure

```rust
Slider {
    value: 0.0,
    horizontal: true,
    on_value_change: |value: f64| {
        // Handle the change in slider value.
    },
}
```

For a two-thumb range selector, use `RangeSlider`:

```rust
RangeSlider {
    default_value: 20.0..80.0,
    on_value_change: |value: std::ops::Range<f64>| {
        // value.start and value.end give the two endpoints
    },
}
```

## Keyboard and accessibility

Follows the WAI-ARIA [Slider](https://www.w3.org/WAI/ARIA/apg/patterns/slider/) pattern. Roles: `slider`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| ArrowRight | On the thumb | Increases the value by one step. | yes |
| ArrowLeft | On the thumb | Decreases the value by one step. | yes |
| Home | On the thumb | Sets the minimum. | yes |
| End | On the thumb | Sets the maximum. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
#[component]
pub fn Slider(props: SliderProps) -> Element

#[component]
pub fn RangeSlider(props: RangeSliderProps) -> Element
```

## Example

```rust
use crate::components::slider::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut current_value = use_signal(|| 50.0);

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",

            // Display the current value
            div { style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;", "{current_value:.0}%" }

            Slider {
                label: "Volume",
                horizontal: true,
                min: 0.0,
                max: 100.0,
                step: 1.0,
                default_value: 50.0,
                on_value_change: move |value: f64| {
                    current_value.set(value);
                },
            }
        }
    }
}
```
