# Combobox

- Copy into your app: `rust-ui add combobox` or `dx components add combobox --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::combobox::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Combobox component is an autocomplete input with a filterable popup list.

Filtering preserves the order defined by the rendered `ComboboxOption` elements and their `index`
props. If you want query-dependent ranking, control `query`, sort your item data in user code,
render the options in that sorted order, and assign indexes from the sorted list.

## Component Structure

```rust
let mut value = use_signal(|| None::<String>);
let mut query = use_signal(String::new);

Combobox::<String> {
    value: Some(value.into()),
    on_value_change: move |next: Option<String>| {
        value.set(next);
    },
    query: Some(query()),
    on_query_change: move |next| query.set(next),
    placeholder: "Select framework...",
    aria_label: "Select framework",
    list_aria_label: "Frameworks",
    ComboboxEmpty { "No framework found." }
    ComboboxOption::<String> {
        index: 0usize,
        value: "next".to_string(),
        text_value: "Next.js",
        "Next.js"
    }
}
```

## API

```rust
pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    #[props(default)]
    pub default_value: Option<T>,

    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    #[props(default)]
    pub default_query: ReadSignal<String>,

    #[props(default)]
    pub on_query_change: Callback<String>,

    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    #[props(default)]
    pub placeholder: ReadSignal<String>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(default)]
    pub list_aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element

#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element

#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(props: ComboboxOptionProps<T>) -> Element
```

## Example

```rust
use crate::components::combobox::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut query = use_signal(String::new);
    let frameworks: &[(&str, &str)] = &[
        ("next", "Next.js"),
        ("svelte", "SvelteKit"),
        ("nuxt", "Nuxt.js"),
        ("remix", "Remix"),
        ("astro", "Astro"),
        ("solid", "SolidStart"),
        ("dioxus", "Dioxus"),
    ];

    rsx! {
        Combobox::<String> {
            query: Some(query()),
            on_query_change: move |next| query.set(next),
            placeholder: "Select framework...",
            aria_label: "Select framework",
            list_aria_label: "Frameworks",
            ComboboxEmpty { "No framework found." }
            for (i , (value , label)) in frameworks.iter().enumerate() {
                ComboboxOption::<String> {
                    index: i,
                    value: value.to_string(),
                    text_value: label.to_string(),
                    {*label}
                }
            }
        }
    }
}
```
