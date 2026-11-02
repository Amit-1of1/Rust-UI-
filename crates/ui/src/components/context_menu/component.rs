use dioxus::prelude::*;
use crate::components::icons::ChevronRight;
use dioxus_primitives::context_menu::{
    self, ContextMenuContentProps, ContextMenuItemProps, ContextMenuProps,
    ContextMenuSubContentProps, ContextMenuSubProps, ContextMenuSubTriggerProps,
    ContextMenuTriggerProps,
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
            padding: "20px",
            background: "var(--primary-color)",
            border: "1px dashed var(--primary-color-6)",
            border_radius: ".5rem",
            cursor: "context-menu",
            user_select: "none",
            text_align: "center",
            attributes: props.attributes,
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

/// A nested menu inside [`ContextMenuContent`] (or another submenu). Its trigger is an item of
/// the surrounding menu at `index`; the content holds ordinary [`ContextMenuItem`]s.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ContextMenuSub(props: ContextMenuSubProps) -> Element {
    rsx! {
        context_menu::ContextMenuSub {
            index: props.index,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_context_menu_sub }), props.attributes]),
            {props.children}
        }
    }
}

/// The item that opens a [`ContextMenuSub`], with a chevron after its label.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ContextMenuSubTrigger(props: ContextMenuSubTriggerProps) -> Element {
    let class = format!("{} {}", Styles::dx_context_menu_item, Styles::dx_context_menu_sub_trigger);
    rsx! {
        context_menu::ContextMenuSubTrigger {
            attributes: merge_attributes(vec![attributes!(div { class }), props.attributes]),
            {props.children}
            ChevronRight { size: 16, class: Styles::dx_context_menu_sub_chevron }
        }
    }
}

/// The menu of a [`ContextMenuSub`]: the context menu's own look, beside its trigger.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ContextMenuSubContent(props: ContextMenuSubContentProps) -> Element {
    let class = format!("{} {}", Styles::dx_context_menu_content, Styles::dx_context_menu_sub_content);
    rsx! {
        context_menu::ContextMenuSubContent {
            id: props.id,
            attributes: merge_attributes(vec![attributes!(div { class }), props.attributes]),
            {props.children}
        }
    }
}
