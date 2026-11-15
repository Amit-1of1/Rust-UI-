# Color picker

- Copy into your app: `atoll add color_picker` or `dx components add color_picker --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `input`).
- Import: `use crate::components::color_picker::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

Color picker used when the user needs to make a customized color selection.

## Component Structure

```rust
ColorPicker {
    // The currently selected color in the color picker.
    color,
    on_color_change: move |c: Hsv<encoding::Srgb, f64>| {
        // This callback is triggered when a color is selected in the popover.
        // The HSV parameter contains the selected color.
    },

    // Optional label on the trigger button.
    label: "Pick",
}
```

### Custom Popover

`ColorPicker` renders the default trigger and `ColorPickerSelect` content. If you
want to fully replace the default popover UI, compose the picker from
`ColorPickerRoot`, `ColorPickerTrigger`, and `ColorPickerPopover`:

```rust
ColorPickerRoot {
    color,
    on_color_change: move |c: Hsv<encoding::Srgb, f64>| { /* ... */ },

    ColorPickerTrigger {
        label: "Pick",
    }

    ColorPickerPopover {
        div {
            "Custom color picker content"
        }
    }
}
```

If you only need to add content after the default controls, pass children to
`ColorPicker`; they will be appended inside the default popover.

## Keyboard and accessibility

Follows the WAI-ARIA [Slider (hue) and 2D slider (colour area) in a non-modal dialog](https://www.w3.org/WAI/ARIA/apg/patterns/slider/) pattern. Roles: `group`, `dialog`, `slider`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | On the swatch button | Opens the picker. | yes |
| Escape | In the open picker | Closes it and returns focus to the swatch button. | yes |
| ArrowRight | On the hue slider | Increases the hue by 1 degree. | yes |
| ArrowLeft | On the hue slider | Decreases the hue by 1 degree. | yes |
| Shift + ArrowRight | On the hue slider | Increases the hue by 10 degrees, up to 360 (Shift+ArrowLeft decreases). | yes |
| ArrowRight | On the colour area | Raises the saturation one step and moves focus to the saturation input, which announces it. | yes |
| ArrowUp | On the colour area | Raises the brightness (value) one step and moves focus to the value input. | yes |
| ArrowUp | On the saturation input | Raises the value and moves focus to the value input (the cross axis). | yes |
| ArrowRight | On the value input | Raises the saturation and moves focus to the saturation input. | yes |

States:

- `open`: The picker's popover is shown.
- `closed`: Only the swatch button is shown.

Known gaps:

- Hue slider: no Home, End, PageUp or PageDown.
- Colour area: no Home, End, PageUp or PageDown; its focusable thumb has no role (only aria-label "Color area").

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// The props for the [`ColorPickerRoot`] component.
pub struct ColorPickerRootProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Hsv<encoding::Srgb, f64>>,

    /// Callback when color changes
    #[props(default)]
    pub on_color_change: Callback<Hsv<encoding::Srgb, f64>>,

    /// Whether the color picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the popover.
    pub open: ReadSignal<Option<bool>>,

    /// The default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback fired when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes to extend the color picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker element
    pub children: Element,
}

#[component]
pub fn ColorPickerRoot(props: ColorPickerRootProps) -> Element

/// The props for the [`ColorPicker`] component.
pub struct ColorPickerProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Hsv<encoding::Srgb, f64>>,

    /// Callback when color changes
    #[props(default)]
    pub on_color_change: Callback<Hsv<encoding::Srgb, f64>>,

    /// Whether the color picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional label on the trigger button
    #[props(default)]
    pub label: Option<String>,

    /// The controlled open state of the popover.
    pub open: ReadSignal<Option<bool>>,

    /// The default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback fired when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes to extend the color picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Additional content to append to the default color picker popover
    pub children: Element,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element

/// The props for the [`ColorPickerTrigger`] component.
pub struct ColorPickerTriggerProps {
    /// Optional label on the trigger button
    #[props(default)]
    pub label: Option<String>,

    /// Additional attributes to extend the trigger button
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Additional content to render inside the trigger button
    pub children: Element,
}

#[component]
pub fn ColorPickerTrigger(props: ColorPickerTriggerProps) -> Element

/// The props for the [`ColorPickerPopover`] component.
pub struct ColorPickerPopoverProps {
    /// Additional attributes to extend the popover content
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker popover
    pub children: Element,
}

#[component]
pub fn ColorPickerPopover(props: ColorPickerPopoverProps) -> Element

/// The props for the [`ColorField`] component.
pub struct ColorFieldProps {
    /// Optional label above the input field
    #[props(default)]
    pub label: Option<String>,

    /// Optional props for the text description element
    #[props(default)]
    pub description: Option<String>,

    /// Additional attributes to extend the color field element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color field element
    pub children: Element,
}

/// The props for the [`ColorSwatch`] component.
pub struct ColorSwatchProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Hsv<encoding::Srgb, f64>>,

    /// Additional attributes to extend the color swatch element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color swatch element
    pub children: Element,
}

/// The props for the [`ColorSlider`] component.
pub struct ColorSliderProps {
    pub title: ReadSignal<String>,

    /// Additional attributes to extend the color slider element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color slider element
    pub children: Element,
}

/// The props for the [`ColorPickerSelect`] component.
pub struct ColorPickerSelectProps {
    /// Additional attributes to extend the color picker select element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker select element
    pub children: Element,
}

#[component]
pub fn ColorPickerSelect(props: ColorPickerSelectProps) -> Element
```

## Example

```rust
use dioxus::prelude::*;

use crate::components::color_picker::*;
use dioxus_primitives::color_picker::Color;
use palette::{encoding, Hsv, IntoColor};

#[component]
pub fn Demo() -> Element {
    let mut color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(155, 128, 255).into_format::<f64>().into_color()
    });

    rsx! {
        ColorPicker {
            label: "Pick a color",
            color: color(),
            on_color_change: move |c| {
                tracing::info!("Color changed: {:?}", c);
                color.set(c);
            },
        }
    }
}
```
