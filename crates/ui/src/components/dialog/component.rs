use dioxus::prelude::*;
use crate::components::icons::X;
use crate::components::ui_strings::use_ui_strings;
use dioxus_primitives::dialog::{self, DialogCtx, DialogDescriptionProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::components::hooks::ReturnFocus;

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

    /// How wide the dialog may grow: `Sm` for a confirmation, `Lg`/`Xl` for forms and tables,
    /// `Full` for most of the screen. On phones every size fills the width.
    #[props(default)]
    pub size: DialogSize,

    /// Adds a close button (an X, named "Close") in the top corner.
    #[props(default)]
    pub show_close: bool,

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
        "data-size": props.size.class(),
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
                if props.show_close {
                    DialogContentClose {}
                }
                // Modal panels keep focus inside while open (the primitive's trap), so
                // closing always returns it. Not on the panel's own listeners: those are
                // the trap's.
                if (props.is_modal)() {
                    ReturnFocus { inside: None }
                }
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

/// How wide a [`Dialog`] may grow.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum DialogSize {
    /// 24rem: a short confirmation.
    Sm,
    /// 32rem.
    #[default]
    Default,
    /// 42rem: a form.
    Lg,
    /// 56rem: a table or a two-column form.
    Xl,
    /// Nearly the whole screen.
    Full,
}

impl DialogSize {
    pub fn class(&self) -> &'static str {
        match self {
            DialogSize::Sm => "sm",
            DialogSize::Default => "default",
            DialogSize::Lg => "lg",
            DialogSize::Xl => "xl",
            DialogSize::Full => "full",
        }
    }
}

/// The row of buttons at the bottom of a [`Dialog`] ("Cancel", "Save"), aligned to the end and
/// stacked on narrow screens.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DialogFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { "data-slot": "dialog-footer", ..merge_attributes(vec![attributes!(div { class: Styles::dx_dialog_footer }), attributes]), {children} }
    }
}

/// A button that closes the [`Dialog`] it is in ("Cancel", "Done"). Style it by passing a
/// `class`, or render your own element with `as`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DialogClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let ctx: DialogCtx = use_context();
    let base = attributes! {
        button {
            r#type: "button",
            onclick: move |_| ctx.set_open(false),
        }
    };
    let merged = merge_attributes(vec![base, attributes]);
    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged, {children} }
        }
    }
}

/// The X in a [`Dialog`]'s corner, named "Close" (translatable). `show_close: true` adds it.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DialogContentClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let close = use_ui_strings().close;
    let base = attributes!(button {
        class: Styles::dx_dialog_close,
        aria_label: close,
    });
    rsx! {
        DialogClose { attributes: merge_attributes(vec![base, attributes]),
            X { size: "20px", "aria-hidden": "true" }
        }
    }
}
