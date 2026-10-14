use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    self, MenubarContentProps, MenubarItemProps, MenubarMenuProps, MenubarProps,
    MenubarTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
#[css_module("/src/components/menubar/style.css")]
struct Styles;

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    rsx! {
        menubar::Menubar {
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_menubar }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    rsx! {
        menubar::MenubarMenu {
            index: props.index,
            disabled: props.disabled,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_menubar_menu }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    rsx! {
        menubar::MenubarTrigger { attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_menubar_trigger }), props.attributes]), {props.children} }
    }
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    rsx! {
        menubar::MenubarContent {
            id: props.id,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_menubar_content }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    rsx! {
        menubar::MenubarItem {
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            on_select: props.on_select,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_menubar_item }), props.attributes]),
            {props.children}
        }
    }
}
