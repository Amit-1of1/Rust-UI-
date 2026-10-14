use crate::components::overlay::ModalPortal;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{
    self, DialogCtx, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use motion::SwipeDismiss;

#[css_module("/src/components/drawer/style.css")]
struct Styles;

/// A panel that slides up from the bottom of the screen and can be dragged down to dismiss.
///
/// It is a modal dialog underneath (focus is trapped, Escape closes, the page behind is inert),
/// so it needs a `DrawerTitle`. Dragging is an extra: the Close button and Escape always work.
#[component]
pub fn Drawer(props: DialogRootProps) -> Element {
    let on_open_change = props.on_open_change;
    let base = attributes!(div {
        class: Styles::dx_drawer,
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        ModalPortal {
            dialog::DialogRoot {
                class: Styles::dx_drawer_backdrop,
                id: props.id,
                is_modal: props.is_modal,
                open: props.open,
                default_open: props.default_open,
                on_open_change: props.on_open_change,
                dialog::DialogContent { class: None, attributes: merged,
                    SwipeDismiss {
                        vertical: true,
                        one_way: true,
                        width: 320.0,
                        on_dismiss: move |_| on_open_change.call(false),
                        class: Styles::dx_drawer_body,
                        div { class: Styles::dx_drawer_handle, "aria-hidden": "true" }
                        {props.children}
                    }
                }
            }
        }
    }
}

#[component]
pub fn DrawerTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: Styles::dx_drawer_title,
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        dialog::DialogTitle { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DrawerDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_drawer_description,
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        dialog::DialogDescription { id: props.id, attributes: merged, {props.children} }
    }
}

/// A button that closes the drawer.
#[component]
pub fn DrawerClose(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx: DialogCtx = use_context();
    rsx! {
        button {
            r#type: "button",
            onclick: move |_| ctx.set_open(false),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_drawer_close }), attributes]),
            {children}
        }
    }
}
