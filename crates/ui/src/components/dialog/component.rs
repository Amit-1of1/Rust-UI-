use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogDescriptionProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/dialog/style.css")]
struct Styles;

/// The props for [`Dialog`].
///
/// `title` is required: a dialog without a name is announced as just "dialog". Hide it with
/// `hide_title` when the content already makes the purpose clear to sighted users.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Dialog { open: true, title: "Rename file", p { "…" } } }
/// # }
/// ```
///
/// Leaving `title` out is a compile error ("Missing required field title"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Dialog { open: true, p { "…" } } }
/// # }
/// ```
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// The dialog's heading, and its accessible name (`aria-labelledby`).
    #[props(into)]
    pub title: String,

    /// One line read after the title (`aria-describedby`). For richer text, put a
    /// [`DialogDescription`] in the children instead (not both).
    #[props(default, into)]
    pub description: Option<String>,

    /// Hide the title and `description` visually; screen readers still read them.
    #[props(default)]
    pub hide_title: bool,

    /// The ID of the dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the dialog is modal. If true, it will trap focus within the dialog when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the dialog.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the dialog if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// Called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Attributes for the dialog panel.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The dialog's content, shown below the title.
    pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_dialog,
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let hide = props.hide_title;
    let title_class = with_sr_only(Styles::dx_dialog_title.to_string(), hide);
    let description_class = with_sr_only(Styles::dx_dialog_description.to_string(), hide);

    rsx! {
        dialog::DialogRoot {
            class: Styles::dx_dialog_backdrop,
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent {
                class: None,
                attributes: merged,
                dialog::DialogTitle {
                    attributes: attributes!(h2 { class: title_class }),
                    {props.title}
                }
                if let Some(description) = props.description {
                    dialog::DialogDescription {
                        attributes: attributes!(p { class: description_class }),
                        {description}
                    }
                }
                {props.children}
            }
        }

    }
}

/// A longer description for a [`Dialog`] (links, formatting). For one line of text, use the
/// dialog's `description` prop instead.
#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_dialog_description,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogDescription {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

/// Adds the theme's `.sr-only` class (visually hidden, still read) when `hide` is set.
fn with_sr_only(class: String, hide: bool) -> String {
    if hide {
        format!("{class} sr-only")
    } else {
        class
    }
}
