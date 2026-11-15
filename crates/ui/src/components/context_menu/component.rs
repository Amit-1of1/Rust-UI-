use dioxus::prelude::*;
use dioxus_primitives::context_menu::{
    self, ContextMenuContentProps, ContextMenuItemProps, ContextMenuProps, ContextMenuTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/context_menu/style.css")]
struct Styles;

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    rsx! {
        context_menu::ContextMenu {
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    rsx! {
        context_menu::ContextMenuTrigger {
            // Styled by class: the primitive sets its own `style` (it stops iOS's long-press
            // menu), and a second `style` attribute would replace it or be dropped.
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_context_menu_trigger }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    rsx! {
        context_menu::ContextMenuContent {
            id: props.id,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_context_menu_content }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    rsx! {
        context_menu::ContextMenuItem {
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select: props.on_select,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_context_menu_item }), props.attributes]),
            {props.children}
        }
    }
}
