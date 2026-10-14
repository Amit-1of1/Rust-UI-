use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::slider::{self, RangeSliderProps, SliderProps};

#[css_module("/src/components/slider/style.css")]
struct Styles;

#[component]
pub fn Slider(props: SliderProps) -> Element {
    // The primitive steps with the arrow keys but has no Home and End, which the APG
    // slider pattern asks for. Keep our own copy of the value so those keys can jump
    // to the ends whether or not the caller controls it.
    let mut own = use_signal(|| props.default_value);
    let value = props.value;
    let current = use_memo(move || Some(value().unwrap_or(own())));
    let (min, max, disabled, on_change) =
        (props.min, props.max, props.disabled, props.on_value_change);
    let set = use_callback(move |v: f64| {
        own.set(v);
        on_change.call(v);
    });
    let home_end = attributes!(div {
        onkeydown: move |e: KeyboardEvent| {
            if disabled() {
                return;
            }
            let to = match e.key() {
                Key::Home => min(),
                Key::End => max(),
                _ => return,
            };
            e.prevent_default();
            set.call(to);
        }
    });
    rsx! {
        slider::Slider {
            value: current,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: set,
            label: props.label,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_slider }), props.attributes]),
            slider::SliderTrack {
                class: Styles::dx_slider_track,
                attributes: home_end,
                slider::SliderRange { class: Styles::dx_slider_range }
                slider::SliderThumb { class: Styles::dx_slider_thumb }
            }
        }
    }
}

#[component]
pub fn RangeSlider(props: RangeSliderProps) -> Element {
    rsx! {
        slider::RangeSlider {
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: props.on_value_change,
            label: props.label,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_slider }), props.attributes]),
            slider::SliderTrack { class: Styles::dx_slider_track,
                slider::SliderRange { class: Styles::dx_slider_range }
                slider::SliderThumb { class: Styles::dx_slider_thumb, index: 0usize }
                slider::SliderThumb { class: Styles::dx_slider_thumb, index: 1usize }
            }
        }
    }
}
