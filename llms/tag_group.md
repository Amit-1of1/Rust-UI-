# Tag group

- Copy into your app: `rust-ui add tag_group` or `dx components add tag_group --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::tag_group::*;` after copying, or `use ui::components::tag_group::*;` with the `rust-ui-kit` crate.

A Tag Group is a focusable group of tags (labels, categories, filters and similar items) with keyboard navigation, optional selection and removal.

## Structure

Single selection with [`TagGroup`](component.rs):

```rust
TagGroup {
    value: Some(value.into()),
    on_value_change: move |value| { /* ... */ },
    TagGroupLabel { "Labels" }
    TagList {
        TagGroupEmpty { "No tags" }
        Tag { index: 0usize, value: "bug", "bug" RemoveButton {} }
        Tag { index: 1usize, value: "feature", disabled: true, "feature" }
    }
}
```

## API

```rust
#[component]
pub fn TagGroup(props: TagGroupProps<String>) -> Element

#[component]
pub fn TagGroupMulti(props: TagGroupMultiProps<String>) -> Element

#[component]
pub fn TagGroupLabel(props: TagGroupLabelProps) -> Element

#[component]
pub fn TagGroupEmpty(props: TagGroupEmptyProps) -> Element

#[component]
pub fn TagList(props: TagListProps) -> Element

pub struct TagProps {
    pub value: ReadSignal<String>,
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,
    pub index: ReadSignal<usize>,
    #[props(default)]
    pub id: ReadSignal<Option<String>>,
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn Tag(props: TagProps) -> Element

#[component]
pub fn RemoveButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use dioxus::prelude::*;

use crate::components::tag_group::*;

#[component]
pub fn Demo() -> Element {
    let labels = ["bug", "feature", "core", "desktop", "example", "duplicate"];
    let tags = labels.iter().enumerate().map(|(index, &t)| {
        rsx! {
            Tag {
                index,
                value: t,
                "{t}"
                RemoveButton {}
            }
        }
    });

    let mut value = use_signal(|| Some("core".to_string()));

    rsx! {
        TagGroup {
            value: Some(value.into()),
            on_value_change: move |v| value.set(v),
            allow_empty_selection: false,
            TagGroupLabel { "Labels" }
            TagList {
                TagGroupEmpty { "No tags" }
                {tags}
            }
        }
    }
}
```
