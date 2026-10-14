use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogDescriptionProps, AlertDialogRootProps, AlertDialogTitleProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/alert_dialog/style.css")]
struct Styles;

#[component]
pub fn AlertDialog(props: AlertDialogRootProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogRoot {
            id: props.id,
            default_open: props.default_open,
            open: props.open,
            on_open_change: props.on_open_change,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_alert_dialog_backdrop }), props.attributes]),
            alert_dialog::AlertDialogContent {
                class: Styles::dx_alert_dialog.to_string(),
                {props.children}
            }
        }

    }
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogTitle {
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_alert_dialog_title }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogDescription {
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_alert_dialog_description }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogActions { attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_alert_dialog_actions }), props.attributes]), {props.children} }
    }
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogCancel {
            on_click: props.on_click,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_alert_dialog_cancel }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogAction {
            on_click: props.on_click,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_alert_dialog_action }), props.attributes]),
            {props.children}
        }
    }
}
