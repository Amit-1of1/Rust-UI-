use crate::components::icons::ChevronDown;
use dioxus::prelude::*;
use dioxus_primitives::navbar::{
    self, NavbarContentProps, NavbarItemProps, NavbarNavProps, NavbarProps, NavbarTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
#[css_module("/src/components/navbar/style.css")]
struct Styles;

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        navbar::Navbar {
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_navbar }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    rsx! {
        navbar::NavbarNav {
            index: props.index,
            disabled: props.disabled,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_navbar_nav }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    rsx! {
        navbar::NavbarTrigger { attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_navbar_trigger }), props.attributes]),
            {props.children}
            ChevronDown { "aria-hidden": "true",
                class: Styles::dx_navbar_expand_icon,
                size: "20px",
                stroke: "var(--secondary-color-4)",
            }
        }
    }
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    rsx! {
        navbar::NavbarContent {
            id: props.id,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_navbar_content }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    rsx! {
        navbar::NavbarItem {
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            new_tab: props.new_tab,
            to: props.to,
            active_class: props.active_class,
            class: match props.class {
                Some(class) => format!("{} {class}", Styles::dx_navbar_item),
                None => Styles::dx_navbar_item.to_string(),
            },
            attributes: props.attributes,
            on_select: props.on_select,
            onclick: props.onclick,
            onmounted: props.onmounted,
            {props.children}
        }
    }
}
