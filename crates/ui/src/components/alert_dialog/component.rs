use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogDescriptionProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/alert_dialog/style.css")]
struct Styles;

/// The props for [`AlertDialog`].
///
/// `title` is required: it names the alert dialog for screen readers. Unlike a
/// plain `Dialog` it can't be hidden, since an interruption
/// should say what it is about.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { AlertDialog { open: true, title: "Rename file", p { "…" } } }
/// # }
/// ```
///
/// Leaving `title` out is a compile error ("Missing required field title"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { AlertDialog { open: true, p { "…" } } }
/// # }
/// ```
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogProps {
    /// The heading, and the dialog's accessible name (`aria-labelledby`).
    #[props(into)]
    pub title: String,

    /// The message read after the title (`aria-describedby`), such as what will be lost. For
    /// richer text, put an [`AlertDialogDescription`] in the children instead (not both).
    #[props(default, into)]
    pub description: Option<String>,

    /// The id of the alert dialog root element. If not provided, a unique id will be generated.
    pub id: ReadSignal<Option<String>>,

    /// Whether the alert dialog should be open by default. Only used if `open` is not provided.
    #[props(default)]
    pub default_open: bool,

    /// The controlled open state of the alert dialog.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// Called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Attributes for the backdrop element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The content below the title: usually [`AlertDialogActions`].
    pub children: Element,
}

#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogRoot {
            id: props.id,
            default_open: props.default_open,
            open: props.open,
            on_open_change: props.on_open_change,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_alert_dialog_backdrop }), props.attributes]),
            alert_dialog::AlertDialogContent {
                class: Styles::dx_alert_dialog.to_string(),
                alert_dialog::AlertDialogTitle {
                    attributes: attributes!(div { class: Styles::dx_alert_dialog_title }),
                    {props.title}
                }
                if let Some(description) = props.description {
                    AlertDialogDescription { {description} }
                }
                {props.children}
            }
        }

    }
}

/// A longer message for an [`AlertDialog`] (links, formatting). For plain text, use the
/// alert dialog's `description` prop instead.
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
